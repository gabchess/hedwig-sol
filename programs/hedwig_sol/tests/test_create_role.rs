//! Authorization invariants for create_role.

mod common;

use anchor_lang::{error::ErrorCode, prelude::Pubkey};
use common::*;
use hedwig_sol::{error::HedwigError, Org, Role};
use solana_instruction_error::InstructionError;

#[test]
fn test_create_role_rejects_non_authority() {
    let mut svm = new_svm();
    let (org, _authority) = setup_org(&mut svm, "Acme");
    let impostor = funded_keypair(&mut svm);
    let (role, _bump) = role_pda(&org, "admin");

    // impostor is not the org authority: the org PDA re-derivation (seeded
    // on the passed-in authority) will not match the real org account.
    let result = send(
        &mut svm,
        &impostor,
        &[],
        ix_create_role(role, org, impostor.pubkey(), "admin"),
    );

    assert_anchor_constraint_error(result, ErrorCode::ConstraintSeeds);
    assert!(svm.get_account(&role).is_none());
    assert_eq!(account_data::<Org>(&svm, &org).role_count, 0);
}

#[test]
fn test_create_role_rejects_empty_name() {
    let mut svm = new_svm();
    let (org, authority) = setup_org(&mut svm, "Acme");
    let (role, _bump) = role_pda(&org, "");

    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_role(role, org, authority.pubkey(), ""),
    );

    assert_hedwig_error(result, HedwigError::InvalidRoleName);
    assert!(svm.get_account(&role).is_none());
    assert_eq!(account_data::<Org>(&svm, &org).role_count, 0);
}

#[test]
fn test_create_role_accepts_32_byte_name() {
    let mut svm = new_svm();
    let (org, authority) = setup_org(&mut svm, "Acme");
    let name = "a".repeat(32);
    let (role, _bump) = role_pda(&org, &name);

    send(
        &mut svm,
        &authority,
        &[],
        ix_create_role(role, org, authority.pubkey(), &name),
    )
    .expect("32-byte role name should succeed");

    assert_eq!(account_data::<Role>(&svm, &role).name, name);
    assert_eq!(account_data::<Org>(&svm, &org).role_count, 1);
}

#[test]
fn test_create_role_rejects_33_byte_name() {
    let mut svm = new_svm();
    let (org, authority) = setup_org(&mut svm, "Acme");
    let name = "a".repeat(33);
    let role = Pubkey::new_unique();

    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_role(role, org, authority.pubkey(), &name),
    );

    assert_instruction_error(result, InstructionError::ProgramFailedToComplete);
    assert!(svm.get_account(&role).is_none());
    assert_eq!(account_data::<Org>(&svm, &org).role_count, 0);
}

#[test]
fn test_create_role_counts_multibyte_utf8_bytes() {
    let mut svm = new_svm();
    let (org, authority) = setup_org(&mut svm, "Acme");
    let accepted_name = "é".repeat(16);
    let (accepted_role, _bump) = role_pda(&org, &accepted_name);

    send(
        &mut svm,
        &authority,
        &[],
        ix_create_role(accepted_role, org, authority.pubkey(), &accepted_name),
    )
    .expect("32-byte multibyte role name should succeed");

    let rejected_name = "é".repeat(17);
    let rejected_role = Pubkey::new_unique();
    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_role(rejected_role, org, authority.pubkey(), &rejected_name),
    );

    assert_instruction_error(result, InstructionError::ProgramFailedToComplete);
    assert!(svm.get_account(&rejected_role).is_none());
    assert_eq!(account_data::<Org>(&svm, &org).role_count, 1);
}

#[test]
fn test_create_role_rejects_duplicate_without_incrementing_count() {
    let mut svm = new_svm();
    let (org, authority, role) = setup_role(&mut svm, "Acme", "admin");

    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_role(role, org, authority.pubkey(), "admin"),
    );

    assert_account_already_in_use(result);
    assert_eq!(account_data::<Org>(&svm, &org).role_count, 1);
    assert_eq!(account_data::<Role>(&svm, &role).name, "admin");
}
