use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

/// Smoke test: create an org using LiteSVM (no network required).
#[test]
fn test_create_org() {
    let program_id = hedwig_sol::id();
    let authority = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/hedwig_sol.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&authority.pubkey(), 1_000_000_000).unwrap();

    let (org_pda, _bump) = anchor_lang::prelude::Pubkey::find_program_address(
        &[b"org", authority.pubkey().as_ref()],
        &program_id,
    );

    let args = hedwig_sol::instruction::CreateOrg {
        name: "TestOrg".to_string(),
    };

    let instruction = Instruction::new_with_bytes(
        program_id,
        &args.data(),
        hedwig_sol::accounts::CreateOrg {
            org: org_pda,
            authority: authority.pubkey(),
            system_program: anchor_lang::solana_program::system_program::ID,
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&authority.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[authority]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "create_org failed: {:?}", res.err());
}
