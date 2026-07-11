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
    assert!(
        result.is_ok(),
        "authorized disable should succeed: {result:?}"
    );

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
    assert!(
        result.is_ok(),
        "authorized re-enable should succeed: {result:?}"
    );

    let state = account_data::<hedwig_sol::Role>(&svm, &role);
    assert!(state.enabled, "role.enabled should be true after re-enable");

    let caller = funded_keypair(&mut svm);
    let check = send(&mut svm, &caller, &[], ix_check_role(member, role, holder));
    assert!(
        check.is_ok(),
        "check_role should pass once re-enabled: {check:?}"
    );
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
}
