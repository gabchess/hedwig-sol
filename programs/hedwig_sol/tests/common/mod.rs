//! Shared LiteSVM helpers for the hedwig_sol integration test suite.
//!
//! Not every test file uses every helper here, so unused-function warnings
//! are expected per test binary; suppressed below rather than per-fn.
#![allow(dead_code)]

use anchor_lang::{
    prelude::{Clock, Pubkey},
    solana_program::{instruction::Instruction, system_program},
    AccountDeserialize, InstructionData, ToAccountMetas,
};
use litesvm::{types::TransactionResult, LiteSVM};
use solana_instruction_error::InstructionError;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
// Re-exported (not just `use`d) so `use common::*;` in each test file also
// brings the `Signer::pubkey()` method into scope for `Keypair` values.
pub use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_transaction_error::TransactionError;

use hedwig_sol::error::HedwigError;

/// Boots a fresh LiteSVM instance with the hedwig_sol program loaded.
pub fn new_svm() -> LiteSVM {
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../../target/deploy/hedwig_sol.so");
    svm.add_program(hedwig_sol::id(), bytes).unwrap();
    svm
}

/// Creates a fresh keypair funded with 1 SOL.
pub fn funded_keypair(svm: &mut LiteSVM) -> Keypair {
    let kp = Keypair::new();
    svm.airdrop(&kp.pubkey(), 1_000_000_000).unwrap();
    kp
}

/// Reads the current onchain unix timestamp from the LiteSVM clock sysvar.
pub fn current_unix_timestamp(svm: &LiteSVM) -> i64 {
    svm.get_sysvar::<Clock>().unix_timestamp
}

/// Advances the LiteSVM clock sysvar to the given unix timestamp, leaving
/// every other Clock field untouched.
pub fn warp_unix_timestamp(svm: &mut LiteSVM, unix_timestamp: i64) {
    let mut clock = svm.get_sysvar::<Clock>();
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar(&clock);
}

/// Deserializes an Anchor account at `pubkey` into `T`. Panics if the
/// account is missing or doesn't match `T`'s discriminator/layout.
pub fn account_data<T: AccountDeserialize>(svm: &LiteSVM, pubkey: &Pubkey) -> T {
    let account = svm
        .get_account(pubkey)
        .unwrap_or_else(|| panic!("account {pubkey} should exist"));
    T::try_deserialize(&mut account.data.as_slice()).expect("account should deserialize as T")
}

// --- PDA derivation (mirrors the seeds documented on each state struct) ---

pub fn org_pda(authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"org", authority.as_ref()], &hedwig_sol::id())
}

pub fn role_pda(org: &Pubkey, name: &str) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"role", org.as_ref(), name.as_bytes()], &hedwig_sol::id())
}

pub fn member_pda(role: &Pubkey, holder: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"member", role.as_ref(), holder.as_ref()],
        &hedwig_sol::id(),
    )
}

// --- transaction sending ---

/// Sends a single instruction. `payer` is the fee payer and always signs;
/// `extra_signers` are appended (e.g. a role admin distinct from the payer).
pub fn send(
    svm: &mut LiteSVM,
    payer: &Keypair,
    extra_signers: &[&Keypair],
    ix: Instruction,
) -> TransactionResult {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let mut signers: Vec<&Keypair> = Vec::with_capacity(1 + extra_signers.len());
    signers.push(payer);
    signers.extend_from_slice(extra_signers);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &signers).unwrap();
    svm.send_transaction(tx)
}

/// Asserts the transaction failed with the given HedwigError's custom
/// program error code (anchor_lang's declared-error-order + 6000 offset).
pub fn assert_hedwig_error(result: TransactionResult, expected: HedwigError) {
    let failed = result.expect_err("expected transaction to fail");
    match failed.err {
        TransactionError::InstructionError(_, InstructionError::Custom(code)) => {
            assert_eq!(
                code,
                u32::from(expected),
                "expected error {expected:?} ({}), got custom code {code}",
                u32::from(expected),
            );
        }
        other => panic!("expected a custom program error, got {other:?}"),
    }
}

// --- instruction builders ---

pub fn ix_create_org(org: Pubkey, authority: Pubkey, name: &str) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_sol::id(),
        &hedwig_sol::instruction::CreateOrg {
            name: name.to_string(),
        }
        .data(),
        hedwig_sol::accounts::CreateOrg {
            org,
            authority,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}

pub fn ix_create_role(role: Pubkey, org: Pubkey, authority: Pubkey, name: &str) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_sol::id(),
        &hedwig_sol::instruction::CreateRole {
            name: name.to_string(),
        }
        .data(),
        hedwig_sol::accounts::CreateRole {
            role,
            org,
            authority,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}

pub fn ix_assign_role(
    member: Pubkey,
    role: Pubkey,
    holder: Pubkey,
    admin: Pubkey,
    expires_at: i64,
) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_sol::id(),
        &hedwig_sol::instruction::AssignRole { expires_at }.data(),
        hedwig_sol::accounts::AssignRole {
            member,
            role,
            holder,
            admin,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}

pub fn ix_revoke_role(member: Pubkey, role: Pubkey, admin: Pubkey) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_sol::id(),
        &hedwig_sol::instruction::RevokeRole {}.data(),
        hedwig_sol::accounts::RevokeRole {
            member,
            role,
            admin,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}

pub fn ix_check_role(member: Pubkey, role: Pubkey, holder: Pubkey) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_sol::id(),
        &hedwig_sol::instruction::CheckRole {}.data(),
        hedwig_sol::accounts::CheckRole {
            member,
            role,
            holder,
        }
        .to_account_metas(None),
    )
}

pub fn ix_set_role_enabled(role: Pubkey, admin: Pubkey, enabled: bool) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_sol::id(),
        &hedwig_sol::instruction::SetRoleEnabled { enabled }.data(),
        hedwig_sol::accounts::SetRoleEnabled { role, admin }.to_account_metas(None),
    )
}

// --- higher-level scenario builders (compose the instruction builders above) ---

/// Creates an org. Returns (org_pda, authority_keypair).
pub fn setup_org(svm: &mut LiteSVM, name: &str) -> (Pubkey, Keypair) {
    let authority = funded_keypair(svm);
    let (org, _bump) = org_pda(&authority.pubkey());
    let ix = ix_create_org(org, authority.pubkey(), name);
    send(svm, &authority, &[], ix).expect("create_org should succeed");
    (org, authority)
}

/// Creates an org + role. role.admin == authority (create_role always
/// defaults admin to the org authority). Returns (org, authority, role).
pub fn setup_role(svm: &mut LiteSVM, org_name: &str, role_name: &str) -> (Pubkey, Keypair, Pubkey) {
    let (org, authority) = setup_org(svm, org_name);
    let (role, _bump) = role_pda(&org, role_name);
    let ix = ix_create_role(role, org, authority.pubkey(), role_name);
    send(svm, &authority, &[], ix).expect("create_role should succeed");
    (org, authority, role)
}

/// Creates an org + role + assigns it to a fresh holder pubkey.
/// Returns (org, admin, role, holder, member).
pub fn setup_member(
    svm: &mut LiteSVM,
    org_name: &str,
    role_name: &str,
    expires_at: i64,
) -> (Pubkey, Keypair, Pubkey, Pubkey, Pubkey) {
    let (org, admin, role) = setup_role(svm, org_name, role_name);
    let holder = Keypair::new().pubkey();
    let (member, _bump) = member_pda(&role, &holder);
    let ix = ix_assign_role(member, role, holder, admin.pubkey(), expires_at);
    send(svm, &admin, &[], ix).expect("assign_role should succeed");
    (org, admin, role, holder, member)
}
