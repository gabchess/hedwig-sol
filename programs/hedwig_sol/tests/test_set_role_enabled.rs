//! Tests for the set_role_enabled circuit breaker instruction. Covers the
//! authorized disable/re-enable path and the unauthorized-caller rejection.

mod common;

use common::*;
use hedwig_sol::error::HedwigError;

#[test]
fn test_set_role_enabled_disable_blocks_check_role() {
    let mut svm = new_svm();
    let (_org, admin, role, holder, member) = setup_member(&mut svm, "Acme", "admin", 0);

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_set_role_enabled(role, admin.pubkey(), false),
    );
    result.expect("authorized disable should succeed");

    let state = account_data::<hedwig_sol::Role>(&svm, &role);
    assert!(!state.enabled, "role.enabled should be false after disable");

    let caller = funded_keypair(&mut svm);
    let check = send(&mut svm, &caller, &[], ix_check_role(member, role, holder));
    assert_hedwig_error(check, HedwigError::RoleDisabled);
}

#[test]
fn test_set_role_enabled_reenable_allows_check_role() {
    let mut svm = new_svm();
    let (_org, admin, role, holder, member) = setup_member(&mut svm, "Acme", "admin", 0);

    send(
        &mut svm,
        &admin,
        &[],
        ix_set_role_enabled(role, admin.pubkey(), false),
    )
    .expect("disable should succeed");

    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_set_role_enabled(role, admin.pubkey(), true),
    );
    result.expect("authorized re-enable should succeed");

    let state = account_data::<hedwig_sol::Role>(&svm, &role);
    assert!(state.enabled, "role.enabled should be true after re-enable");

    let caller = funded_keypair(&mut svm);
    let check = send(&mut svm, &caller, &[], ix_check_role(member, role, holder));
    check.expect("check_role should pass once re-enabled");
}

#[test]
fn test_disabled_role_rejects_fresh_assignment_without_state_change() {
    let mut svm = new_svm();
    let (_org, admin, role) = setup_role(&mut svm, "Acme", "admin");
    send(
        &mut svm,
        &admin,
        &[],
        ix_set_role_enabled(role, admin.pubkey(), false),
    )
    .expect("disable should succeed");

    let holder = funded_keypair(&mut svm).pubkey();
    let (member, _bump) = member_pda(&role, &holder);
    let result = send(
        &mut svm,
        &admin,
        &[],
        ix_assign_role(member, role, holder, admin.pubkey(), 0),
    );

    assert_hedwig_error(result, HedwigError::RoleDisabled);
    assert!(svm.get_account(&member).is_none());
    let role_state = account_data::<hedwig_sol::Role>(&svm, &role);
    assert!(!role_state.enabled);
    assert_eq!(role_state.member_count, 0);
}

#[test]
fn test_set_role_enabled_rejects_non_admin() {
    let mut svm = new_svm();
    let (_org, _admin, role) = setup_role(&mut svm, "Acme", "admin");
    let impostor = funded_keypair(&mut svm);

    let result = send(
        &mut svm,
        &impostor,
        &[],
        ix_set_role_enabled(role, impostor.pubkey(), false),
    );

    assert_hedwig_error(result, HedwigError::NotRoleAdmin);
    assert!(account_data::<hedwig_sol::Role>(&svm, &role).enabled);
}
