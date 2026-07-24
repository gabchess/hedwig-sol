//! check_role invariants (wrong holder, wrong role, expiration,
//! revocation). Disabled-role behavior lives in test_set_role_enabled.rs.

mod common;

use anchor_lang::error::ErrorCode;
use common::*;
use hedwig_sol::error::HedwigError;

#[test]
fn test_check_role_rejects_wrong_holder() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");

    // Two distinct holders under the same role.
    let holder_a = funded_keypair(&mut svm).pubkey();
    let (member_a, _bump_a) = member_pda(&role, &holder_a);
    send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member_a, role, holder_a, admin.pubkey(), 0),
    )
    .expect("assign holder_a should succeed");

    let holder_b = funded_keypair(&mut svm).pubkey();
    let (member_b, _bump_b) = member_pda(&role, &holder_b);
    send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member_b, role, holder_b, admin.pubkey(), 0),
    )
    .expect("assign holder_b should succeed");

    // holder_a's Member PDA presented, but claiming to be holder_b.
    let caller = funded_keypair(&mut svm);
    let result = send(
        &mut svm,
        &caller,
        &[],
        ix_check_role(member_a, role, holder_b),
    );

    assert_anchor_constraint_error(result, ErrorCode::ConstraintSeeds);
    assert_eq!(
        account_data::<hedwig_sol::Role>(&svm, &role).member_count,
        2
    );
}

#[test]
fn test_check_role_rejects_wrong_role() {
    let mut svm = new_svm();
    let (org, admin) = setup_org(&mut svm, "Acme");

    let (role_a, _bump_a) = role_pda(&org, "admin");
    send(
        &mut svm,
        &admin,
        &[],
        ix_create_role(role_a, org, admin.pubkey(), "admin"),
    )
    .expect("create_role admin should succeed");

    let (role_b, _bump_b) = role_pda(&org, "viewer");
    send(
        &mut svm,
        &admin,
        &[],
        ix_create_role(role_b, org, admin.pubkey(), "viewer"),
    )
    .expect("create_role viewer should succeed");

    let holder = funded_keypair(&mut svm).pubkey();
    let (member_a, _bump) = member_pda(&role_a, &holder);
    send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member_a, role_a, holder, admin.pubkey(), 0),
    )
    .expect("assign to role_a should succeed");

    // member_a belongs to role_a; present it against role_b.
    let caller = funded_keypair(&mut svm);
    let result = send(
        &mut svm,
        &caller,
        &[],
        ix_check_role(member_a, role_b, holder),
    );

    assert_anchor_constraint_error(result, ErrorCode::ConstraintSeeds);
    assert_eq!(
        account_data::<hedwig_sol::Role>(&svm, &role_a).member_count,
        1
    );
    assert_eq!(
        account_data::<hedwig_sol::Role>(&svm, &role_b).member_count,
        0
    );
}

#[test]
fn test_check_role_expiry_boundary() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");
    let holder = funded_keypair(&mut svm).pubkey();
    let (member, _bump) = member_pda(&role, &holder);

    let assign_expiry = current_unix_timestamp(&svm) + 5;
    send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), assign_expiry),
    )
    .expect("assign_role with a near-future expiry should succeed");

    let caller = funded_keypair(&mut svm);

    warp_unix_timestamp(&mut svm, assign_expiry - 1);
    send(&mut svm, &caller, &[], ix_check_role(member, role, holder))
        .expect("membership should be valid immediately before expiry");

    warp_unix_timestamp(&mut svm, assign_expiry);
    send(&mut svm, &caller, &[], ix_check_role(member, role, holder))
        .expect("membership should be valid exactly at expiry");

    warp_unix_timestamp(&mut svm, assign_expiry + 1);
    let expired = send(&mut svm, &caller, &[], ix_check_role(member, role, holder));
    assert_hedwig_error(expired, HedwigError::MembershipExpired);

    assert_eq!(
        account_data::<hedwig_sol::Role>(&svm, &role).member_count,
        1
    );
    assert!(svm.get_account(&member).is_some());
}

#[test]
fn test_check_role_rejects_revoked_membership() {
    let mut svm = new_svm();
    let (_org, admin, role, holder, member) = setup_member(&mut svm, "Acme", "admin", 0);

    send(
        &mut svm,
        &admin,
        &[],
        ix_revoke_role(member, role, admin.pubkey()),
    )
    .expect("revoke_role should succeed");

    let caller = funded_keypair(&mut svm);
    let result = send(&mut svm, &caller, &[], ix_check_role(member, role, holder));

    assert_anchor_constraint_error(result, ErrorCode::AccountNotInitialized);
    assert!(svm.get_account(&member).is_none());
    assert_eq!(
        account_data::<hedwig_sol::Role>(&svm, &role).member_count,
        0
    );
}
