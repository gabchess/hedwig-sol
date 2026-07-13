//! Authorization and expiration invariants for assign_role.

mod common;

use common::*;
use hedwig_sol::error::HedwigError;

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

    assert!(
        result.is_err(),
        "re-assigning the same holder+role should fail (duplicate Member PDA)"
    );
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

    assert!(
        result.is_ok(),
        "expires_at = 0 (never-expires) should be accepted: {result:?}"
    );
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

    assert!(
        result.is_ok(),
        "future expires_at should be accepted: {result:?}"
    );
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
}
