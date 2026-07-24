//! Authorization and expiration invariants for assign_role.

mod common;

use common::*;
use hedwig_sol::{error::HedwigError, Role};

#[test]
fn test_assign_role_rejects_non_admin() {
    let mut svm = new_svm();
    let (_org, _admin, role) = setup_role(&mut svm, "Acme", "admin");
    let impostor = funded_keypair(&mut svm);
    let holder = impostor.pubkey();
    let (member, _bump) = member_pda(&role, &holder);

    let result = send(
        &mut svm,
        &impostor,
        &[],
        ix_assign_role(member, role, holder, impostor.pubkey(), 0),
    );

    assert_hedwig_error(result, HedwigError::NotRoleAdmin);
    assert!(svm.get_account(&member).is_none());
    assert_eq!(account_data::<Role>(&svm, &role).member_count, 0);
}

#[test]
fn test_assign_role_rejects_duplicate() {
    let mut svm = new_svm();
    let (_org, admin, role, holder, member) = setup_member(&mut svm, "Acme", "admin", 0);

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), 0),
    );

    assert_account_already_in_use(result);

    let role_state = account_data::<Role>(&svm, &role);
    assert_eq!(role_state.member_count, 1);
}

#[test]
fn test_assign_role_expires_at_zero_accepted() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");
    let holder = funded_keypair(&mut svm).pubkey();
    let (member, _bump) = member_pda(&role, &holder);

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), 0),
    );

    result.expect("expires_at = 0 (never-expires) should be accepted");
    assert_eq!(account_data::<Role>(&svm, &role).member_count, 1);
}

#[test]
fn test_assign_role_expires_at_future_accepted() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");
    let holder = funded_keypair(&mut svm).pubkey();
    let (member, _bump) = member_pda(&role, &holder);
    let future = current_unix_timestamp(&svm) + 3600;

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), future),
    );

    result.expect("future expires_at should be accepted");
    assert_eq!(account_data::<Role>(&svm, &role).member_count, 1);
}

#[test]
fn test_assign_role_expires_at_exact_now_rejected() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");
    let holder = funded_keypair(&mut svm).pubkey();
    let (member, _bump) = member_pda(&role, &holder);
    warp_unix_timestamp(&mut svm, 1_000_000);
    let now = current_unix_timestamp(&svm);

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), now),
    );

    assert_hedwig_error(result, HedwigError::InvalidExpiration);
    assert!(svm.get_account(&member).is_none());
    assert_eq!(account_data::<Role>(&svm, &role).member_count, 0);
}

#[test]
fn test_assign_role_expires_at_past_rejected() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");
    let holder = funded_keypair(&mut svm).pubkey();
    let (member, _bump) = member_pda(&role, &holder);
    let past = current_unix_timestamp(&svm) - 3600;

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), past),
    );

    assert_hedwig_error(result, HedwigError::InvalidExpiration);
    assert!(svm.get_account(&member).is_none());
    assert_eq!(account_data::<Role>(&svm, &role).member_count, 0);
}

#[test]
fn test_assign_role_expires_at_negative_rejected() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");
    let holder = funded_keypair(&mut svm).pubkey();
    let (member, _bump) = member_pda(&role, &holder);

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), -1),
    );

    assert_hedwig_error(result, HedwigError::InvalidExpiration);
    assert!(svm.get_account(&member).is_none());
    assert_eq!(account_data::<Role>(&svm, &role).member_count, 0);
}
