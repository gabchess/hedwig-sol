//! P2.8: deep create_org coverage: happy path (state), bounds, and duplicates.

mod common;

use common::*;
use hedwig_sol::Org;

#[test]
fn test_create_org_state_matches_input() {
    let mut svm = new_svm();
    let (org, authority) = setup_org(&mut svm, "Acme");

    let (_expected_org, expected_bump) = org_pda(&authority.pubkey());
    let state = account_data::<Org>(&svm, &org);

    assert_eq!(state.authority, authority.pubkey());
    assert_eq!(state.name, "Acme");
    assert_eq!(state.role_count, 0);
    assert_eq!(state.bump, expected_bump);
}

#[test]
fn test_create_org_rejects_empty_name() {
    let mut svm = new_svm();
    let authority = funded_keypair(&mut svm);
    let (org, _bump) = org_pda(&authority.pubkey());

    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_org(org, authority.pubkey(), ""),
    );

    assert_hedwig_error(result, hedwig_sol::error::HedwigError::InvalidOrgName);
}

#[test]
fn test_create_org_accepts_64_byte_name() {
    let mut svm = new_svm();
    let authority = funded_keypair(&mut svm);
    let (org, _bump) = org_pda(&authority.pubkey());
    let name = "a".repeat(64);

    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_org(org, authority.pubkey(), &name),
    );
    assert!(
        result.is_ok(),
        "64-byte name should be accepted: {result:?}"
    );

    let state = account_data::<Org>(&svm, &org);
    assert_eq!(state.name.len(), 64);
}

#[test]
fn test_create_org_rejects_65_byte_name() {
    let mut svm = new_svm();
    let authority = funded_keypair(&mut svm);
    let (org, _bump) = org_pda(&authority.pubkey());
    let name = "a".repeat(65);

    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_org(org, authority.pubkey(), &name),
    );

    assert_hedwig_error(result, hedwig_sol::error::HedwigError::InvalidOrgName);
}

#[test]
fn test_create_org_rejects_duplicate() {
    let mut svm = new_svm();
    let (org, authority) = setup_org(&mut svm, "Acme");

    // Same authority, same derived org PDA, second create_org must fail
    // (Anchor's `init` constraint rejects an already-initialized account).
    let result = send(
        &mut svm,
        &authority,
        &[],
        ix_create_org(org, authority.pubkey(), "Acme Again"),
    );

    assert!(result.is_err(), "duplicate create_org should fail");
}
