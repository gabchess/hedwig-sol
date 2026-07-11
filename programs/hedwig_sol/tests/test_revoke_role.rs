//! P1.3: revoke_role authorization invariant.

mod common;

use common::*;
use hedwig_sol::error::HedwigError;

#[test]
fn test_revoke_role_rejects_non_admin() {
    let mut svm = new_svm();
    let (_org, _admin, role, _holder, member) = setup_member(&mut svm, "Acme", "admin", 0);
    let impostor = funded_keypair(&mut svm);

    let result = send(
        &mut svm,
        &impostor,
        &[],
        ix_revoke_role(member, role, impostor.pubkey()),
    );

    assert_hedwig_error(result, HedwigError::NotRoleAdmin);
}
