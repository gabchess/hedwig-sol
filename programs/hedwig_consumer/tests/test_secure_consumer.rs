#[path = "../../hedwig_sol/tests/common/mod.rs"]
mod hedwig_common;

use anchor_lang::{
    error::ErrorCode,
    prelude::Pubkey,
    solana_program::{instruction::Instruction, system_program},
    InstructionData, ToAccountMetas,
};
use hedwig_common::*;
use hedwig_consumer::{ConsumerError, Counter};
use hedwig_sol::error::HedwigError;
use litesvm::LiteSVM;
use solana_instruction_error::InstructionError;
use solana_keypair::Keypair;
use solana_transaction_error::TransactionError;

const COUNTER_SEED: &[u8] = b"counter";

struct Scenario {
    svm: LiteSVM,
    org: Pubkey,
    admin: Keypair,
    authority: Keypair,
    role: Pubkey,
    member: Pubkey,
    counter: Pubkey,
}

fn new_consumer_svm() -> LiteSVM {
    let mut svm = hedwig_common::new_svm();
    let bytes = include_bytes!("../../../target/deploy/hedwig_consumer.so");
    svm.add_program(hedwig_consumer::id(), bytes).unwrap();
    svm
}

fn counter_pda(authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[COUNTER_SEED, authority.as_ref()], &hedwig_consumer::id())
}

fn ix_initialize_counter(counter: Pubkey, authority: Pubkey, required_role: Pubkey) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_consumer::id(),
        &hedwig_consumer::instruction::InitializeCounter {}.data(),
        hedwig_consumer::accounts::InitializeCounter {
            counter,
            authority,
            required_role,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}

fn ix_increment_counter(
    counter: Pubkey,
    authority: Pubkey,
    member: Pubkey,
    required_role: Pubkey,
) -> Instruction {
    ix_increment_counter_with_program(counter, authority, member, required_role, hedwig_sol::id())
}

fn ix_increment_counter_with_program(
    counter: Pubkey,
    authority: Pubkey,
    member: Pubkey,
    required_role: Pubkey,
    hedwig_program: Pubkey,
) -> Instruction {
    Instruction::new_with_bytes(
        hedwig_consumer::id(),
        &hedwig_consumer::instruction::IncrementCounter {}.data(),
        hedwig_consumer::accounts::IncrementCounter {
            counter,
            authority,
            member,
            required_role,
            hedwig_program,
        }
        .to_account_metas(None),
    )
}

fn setup_scenario(expires_at: i64) -> Scenario {
    let mut svm = new_consumer_svm();
    let (org, admin, role) = setup_role(&mut svm, "Acme", "operator");
    let authority = funded_keypair(&mut svm);
    let (member, _) = member_pda(&role, &authority.pubkey());
    send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, authority.pubkey(), admin.pubkey(), expires_at),
    )
    .expect("assign_role should succeed");

    let (counter, _) = counter_pda(&authority.pubkey());
    send(
        &mut svm,
        &authority,
        &[],
        ix_initialize_counter(counter, authority.pubkey(), role),
    )
    .expect("initialize_counter should succeed");

    Scenario {
        svm,
        org,
        admin,
        authority,
        role,
        member,
        counter,
    }
}

fn assert_counter_value(svm: &LiteSVM, counter: &Pubkey, expected: u64) {
    let state = account_data::<Counter>(svm, counter);
    assert_eq!(state.value, expected);
}

enum ExpectedIncrementError {
    Anchor(ErrorCode),
    Hedwig(HedwigError),
}

fn assert_anchor_error(result: litesvm::types::TransactionResult, expected: ErrorCode) {
    let failed = result.expect_err("increment_counter should fail");
    match failed.err {
        TransactionError::InstructionError(_, InstructionError::Custom(code)) => {
            assert_eq!(code, expected as u32, "unexpected Anchor error");
        }
        other => panic!("expected an Anchor custom program error, got {other:?}"),
    }
}

fn assert_failed_without_increment(
    scenario: &Scenario,
    result: litesvm::types::TransactionResult,
    expected: ExpectedIncrementError,
) {
    match expected {
        ExpectedIncrementError::Anchor(error) => assert_anchor_error(result, error),
        ExpectedIncrementError::Hedwig(error) => assert_hedwig_error(result, error),
    }
    assert_counter_value(&scenario.svm, &scenario.counter, 0);
}

#[test]
fn authenticated_member_increments_counter() {
    let mut scenario = setup_scenario(0);

    send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            scenario.authority.pubkey(),
            scenario.member,
            scenario.role,
        ),
    )
    .expect("authorized increment should succeed");

    assert_counter_value(&scenario.svm, &scenario.counter, 1);
}

#[test]
fn duplicate_counter_initialization_is_rejected_without_state_change() {
    let mut scenario = setup_scenario(0);

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_initialize_counter(scenario.counter, scenario.authority.pubkey(), scenario.role),
    );

    assert_account_already_in_use(result);
    let counter = account_data::<Counter>(&scenario.svm, &scenario.counter);
    assert_eq!(counter.authority, scenario.authority.pubkey());
    assert_eq!(counter.required_role, scenario.role);
    assert_eq!(counter.value, 0);
}

#[test]
fn counter_overflow_is_rejected_without_wrapping() {
    let mut scenario = setup_scenario(0);
    let mut account = scenario
        .svm
        .get_account(&scenario.counter)
        .expect("counter account should exist");
    let value_offset = 8 + 32 + 32;
    account.data[value_offset..value_offset + 8].copy_from_slice(&u64::MAX.to_le_bytes());
    scenario
        .svm
        .set_account(scenario.counter, account)
        .expect("counter fixture should be writable");

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            scenario.authority.pubkey(),
            scenario.member,
            scenario.role,
        ),
    );

    assert_instruction_error(
        result,
        InstructionError::Custom(
            anchor_lang::error::ERROR_CODE_OFFSET + ConsumerError::CounterOverflow as u32,
        ),
    );
    assert_counter_value(&scenario.svm, &scenario.counter, u64::MAX);
}

#[test]
fn substituted_hedwig_program_is_rejected_without_state_change() {
    let mut scenario = setup_scenario(0);

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter_with_program(
            scenario.counter,
            scenario.authority.pubkey(),
            scenario.member,
            scenario.role,
            hedwig_consumer::id(),
        ),
    );

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Anchor(ErrorCode::InvalidProgramId),
    );
}

#[test]
fn wrong_owned_required_role_is_rejected_at_initialization() {
    let mut svm = new_consumer_svm();
    let (_org, _admin, role) = setup_role(&mut svm, "Acme", "operator");
    let mut wrong_owned_role = svm.get_account(&role).expect("role fixture should exist");
    wrong_owned_role.owner = system_program::ID;
    let wrong_role = Pubkey::new_unique();
    svm.set_account(wrong_role, wrong_owned_role)
        .expect("wrong-owner fixture should be writable");

    let authority = funded_keypair(&mut svm);
    let (counter, _) = counter_pda(&authority.pubkey());
    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_initialize_counter(counter, authority.pubkey(), wrong_role),
    );

    assert_anchor_error(result, ErrorCode::AccountOwnedByWrongProgram);
    assert!(svm.get_account(&counter).is_none());
}

#[test]
fn privileged_third_party_member_cannot_be_substituted() {
    let mut scenario = setup_scenario(0);
    let privileged_third_party = funded_keypair(&mut scenario.svm);
    let (third_party_member, _) = member_pda(&scenario.role, &privileged_third_party.pubkey());
    send(
        &mut scenario.svm,
        &scenario.admin,
        &[],
        ix_assign_role(
            third_party_member,
            scenario.role,
            privileged_third_party.pubkey(),
            scenario.admin.pubkey(),
            0,
        ),
    )
    .expect("third-party membership should be active");

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            scenario.authority.pubkey(),
            third_party_member,
            scenario.role,
        ),
    );

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Anchor(ErrorCode::ConstraintSeeds),
    );
}

#[test]
fn expired_membership_cannot_increment() {
    let mut svm = new_consumer_svm();
    let expires_at = current_unix_timestamp(&svm) + 5;
    let mut scenario = {
        let (org, admin, role) = setup_role(&mut svm, "Acme", "operator");
        let authority = funded_keypair(&mut svm);
        let (member, _) = member_pda(&role, &authority.pubkey());
        send(
            &mut svm,
            &admin,
            &[],
            ix_assign_role(member, role, authority.pubkey(), admin.pubkey(), expires_at),
        )
        .expect("assign_role should succeed");
        let (counter, _) = counter_pda(&authority.pubkey());
        send(
            &mut svm,
            &authority,
            &[],
            ix_initialize_counter(counter, authority.pubkey(), role),
        )
        .expect("initialize_counter should succeed");
        Scenario {
            svm,
            org,
            admin,
            authority,
            role,
            member,
            counter,
        }
    };
    warp_unix_timestamp(&mut scenario.svm, expires_at + 1);

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            scenario.authority.pubkey(),
            scenario.member,
            scenario.role,
        ),
    );

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Hedwig(HedwigError::MembershipExpired),
    );
}

#[test]
fn disabled_role_cannot_increment() {
    let mut scenario = setup_scenario(0);
    send(
        &mut scenario.svm,
        &scenario.admin,
        &[],
        ix_set_role_enabled(scenario.role, scenario.admin.pubkey(), false),
    )
    .expect("set_role_enabled should succeed");

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            scenario.authority.pubkey(),
            scenario.member,
            scenario.role,
        ),
    );

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Hedwig(HedwigError::RoleDisabled),
    );
}

#[test]
fn revoked_membership_cannot_increment() {
    let mut scenario = setup_scenario(0);
    send(
        &mut scenario.svm,
        &scenario.admin,
        &[],
        ix_revoke_role(scenario.member, scenario.role, scenario.admin.pubkey()),
    )
    .expect("revoke_role should succeed");

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            scenario.authority.pubkey(),
            scenario.member,
            scenario.role,
        ),
    );

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Anchor(ErrorCode::AccountNotInitialized),
    );
}

#[test]
fn wrong_role_cannot_increment() {
    let mut scenario = setup_scenario(0);
    let (wrong_role, _) = role_pda(&scenario.org, "viewer");
    send(
        &mut scenario.svm,
        &scenario.admin,
        &[],
        ix_create_role(wrong_role, scenario.org, scenario.admin.pubkey(), "viewer"),
    )
    .expect("create_role should succeed");
    let (wrong_member, _) = member_pda(&wrong_role, &scenario.authority.pubkey());
    send(
        &mut scenario.svm,
        &scenario.admin,
        &[],
        ix_assign_role(
            wrong_member,
            wrong_role,
            scenario.authority.pubkey(),
            scenario.admin.pubkey(),
            0,
        ),
    )
    .expect("wrong-role membership should be active");

    let result = send(
        &mut scenario.svm,
        &scenario.authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            scenario.authority.pubkey(),
            wrong_member,
            wrong_role,
        ),
    );

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Anchor(ErrorCode::ConstraintHasOne),
    );
}

#[test]
fn missing_consumer_authority_signature_fails() {
    let mut scenario = setup_scenario(0);
    let payer = funded_keypair(&mut scenario.svm);
    let mut instruction = ix_increment_counter(
        scenario.counter,
        scenario.authority.pubkey(),
        scenario.member,
        scenario.role,
    );
    let authority_meta = instruction
        .accounts
        .iter_mut()
        .find(|meta| meta.pubkey == scenario.authority.pubkey())
        .expect("authority account meta should exist");
    authority_meta.is_signer = false;

    let result = send(&mut scenario.svm, &payer, &[], instruction);

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Anchor(ErrorCode::AccountNotSigner),
    );
}

#[test]
fn wrong_consumer_authority_fails() {
    let mut scenario = setup_scenario(0);
    let wrong_authority = funded_keypair(&mut scenario.svm);

    let result = send(
        &mut scenario.svm,
        &wrong_authority,
        &[],
        ix_increment_counter(
            scenario.counter,
            wrong_authority.pubkey(),
            scenario.member,
            scenario.role,
        ),
    );

    assert_failed_without_increment(
        &scenario,
        result,
        ExpectedIncrementError::Anchor(ErrorCode::ConstraintSeeds),
    );
}
