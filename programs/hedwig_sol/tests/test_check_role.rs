//! P1.3: check_role invariants (wrong holder, wrong role, expiration,
//! revocation). "Disabled role" lives in test_set_role_enabled.rs since
//! it's the new circuit-breaker instruction's own behavior.

mod common;

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

    assert!(
        result.is_err(),
        "check_role should reject a Member PDA that doesn't belong to the claimed holder"
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

    assert!(
        result.is_err(),
        "check_role should reject a Member PDA that belongs to a different role"
    );
}

#[test]
fn test_check_role_rejects_expired_membership() {
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

    // Warp the clock past the membership's expiry.
    warp_unix_timestamp(&mut svm, assign_expiry + 100);

    let caller = funded_keypair(&mut svm);
    let result = send(&mut svm, &caller, &[], ix_check_role(member, role, holder));

    assert_hedwig_error(result, HedwigError::MembershipExpired);
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

    assert!(
        result.is_err(),
        "check_role should reject a revoked (closed) Member PDA"
    );
}
