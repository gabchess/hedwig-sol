//! P1.3: create_role authorization invariant.

mod common;

use common::*;

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

    assert!(
        result.is_err(),
        "non-authority create_role should be rejected"
    );
}
