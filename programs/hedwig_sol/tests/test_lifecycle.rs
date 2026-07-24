//! Full state-machine lifecycle: create_org -> create_role -> assign_role
//! -> check_role -> revoke_role. Asserts every field, every counter
//! transition, and that the Member PDA is actually closed at the end.

mod common;

use common::*;
use hedwig_sol::{Member, Org, Role};

#[test]
fn test_full_lifecycle() {
    let mut svm = new_svm();

    // 1. create_org
    let authority = funded_keypair(&mut svm);
    let (org, org_bump) = org_pda(&authority.pubkey());
    send(
        &mut svm,
        &authority,
        &[],
        ix_create_org(org, authority.pubkey(), "Acme"),
    )
    .expect("create_org should succeed");

    let org_state = account_data::<Org>(&svm, &org);
    assert_eq!(org_state.authority, authority.pubkey());
    assert_eq!(org_state.name, "Acme");
    assert_eq!(org_state.role_count, 0);
    assert_eq!(org_state.bump, org_bump);

    // 2. create_role
    let (role, role_bump) = role_pda(&org, "admin");
    send(
        &mut svm,
        &authority,
        &[],
        ix_create_role(role, org, authority.pubkey(), "admin"),
    )
    .expect("create_role should succeed");

    let role_state = account_data::<Role>(&svm, &role);
    assert_eq!(role_state.org, org);
    assert_eq!(role_state.name, "admin");
    assert_eq!(role_state.admin, authority.pubkey());
    assert_eq!(role_state.member_count, 0);
    assert!(role_state.enabled);
    assert_eq!(role_state.bump, role_bump);

    let org_state = account_data::<Org>(&svm, &org);
    assert_eq!(
        org_state.role_count, 1,
        "role_count should increment exactly once"
    );

    // 3. assign_role
    let holder = funded_keypair(&mut svm).pubkey();
    let (member, member_bump) = member_pda(&role, &holder);
    let before_assign_ts = current_unix_timestamp(&svm);
    send(
        &mut svm,
        &authority,
        &[],
        ix_assign_role(member, role, holder, authority.pubkey(), 0),
    )
    .expect("assign_role should succeed");

    let member_state = account_data::<Member>(&svm, &member);
    assert_eq!(member_state.role, role);
    assert_eq!(member_state.holder, holder);
    assert!(member_state.granted_at >= before_assign_ts);
    assert_eq!(member_state.expires_at, 0);
    assert_eq!(member_state.bump, member_bump);

    let role_state = account_data::<Role>(&svm, &role);
    assert_eq!(
        role_state.member_count, 1,
        "member_count should increment exactly once"
    );

    // 4. check_role
    let caller = funded_keypair(&mut svm);
    let check = send(&mut svm, &caller, &[], ix_check_role(member, role, holder));
    check.expect("check_role should pass for an active membership");

    // 5. revoke_role
    let admin_lamports_before_revoke = svm
        .get_account(&authority.pubkey())
        .expect("authority should exist")
        .lamports;
    send(
        &mut svm,
        &authority,
        &[],
        ix_revoke_role(member, role, authority.pubkey()),
    )
    .expect("revoke_role should succeed");

    let role_state = account_data::<Role>(&svm, &role);
    assert_eq!(
        role_state.member_count, 0,
        "member_count should decrement exactly once"
    );
    let admin_lamports_after_revoke = svm
        .get_account(&authority.pubkey())
        .expect("authority should exist")
        .lamports;
    assert!(
        admin_lamports_after_revoke > admin_lamports_before_revoke,
        "returned member rent should exceed the revoke transaction fee"
    );

    // LiteSVM normally prunes the zero-lamport account. If it retains it,
    // assert the exact closed-account shape.
    match svm.get_account(&member) {
        None => {}
        Some(account) => {
            assert_eq!(account.lamports, 0);
            assert_eq!(account.owner, anchor_lang::system_program::ID);
            assert!(!account.executable);
            assert!(account.data.iter().all(|byte| *byte == 0));
        }
    }

    // 6. Re-granting the same holder reuses the closed PDA and restores count.
    send(
        &mut svm,
        &authority,
        &[],
        ix_assign_role(member, role, holder, authority.pubkey(), 0),
    )
    .expect("re-grant after close should succeed");

    let member_state = account_data::<Member>(&svm, &member);
    assert_eq!(member_state.role, role);
    assert_eq!(member_state.holder, holder);
    assert_eq!(member_state.expires_at, 0);
    assert_eq!(account_data::<Role>(&svm, &role).member_count, 1);
}
