use borsh::BorshDeserialize;
use multi_sig_wallet::instruction::ProgramInstruction;
use multi_sig_wallet::process_instruction;
use multi_sig_wallet::state::CampaignState;
use solana_program::instruction::AccountMeta;
use solana_program::rent::Rent;
use solana_program::{instruction::Instruction, msg, pubkey::Pubkey, sysvar};
use solana_program_test::*;
use solana_sdk::account::Account;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::transport::TransportError;
use std::mem::size_of;

#[tokio::test]
async fn test_create_campaign() -> Result<(), TransportError> {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "crowdfunding_program",
        program_id,
        processor!(process_instruction),
    );

    let creator_keypair = Keypair::new();
    let campaign_keypair = Keypair::new();

    let rent = Rent::default();
    let account_size = size_of::<CampaignState>();
    let campaign_account_rent = rent.minimum_balance(account_size);

    program_test.add_account(
        campaign_keypair.pubkey(),
        Account {
            lamports: campaign_account_rent,
            data: vec![0; account_size],
            owner: program_id,
            ..Account::default()
        },
    );

    // Add admin account to test environment
    program_test.add_account(
        creator_keypair.pubkey(),
        Account {
            lamports: 0,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::CreateCampaign {
            creator: creator_keypair.pubkey(),
            goal: 100_000_000_000, // equal to 1 sol
            amount_raised: 0,
            deadline: 0,
        },
        vec![
            AccountMeta::new(creator_keypair.pubkey(), true),
            AccountMeta::new(campaign_keypair.pubkey(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &creator_keypair], recent_blockhash);

    match banks_client.process_transaction(transaction).await {
        Ok(_) => {
            // get updated campaign account
            let campaign_account = banks_client
                .get_account(campaign_keypair.pubkey())
                .await?
                .unwrap();

            assert_eq!(campaign_account.owner, program_id);
            assert_eq!(campaign_account.data.len(), account_size);
            assert_eq!(campaign_account.lamports, campaign_account_rent);
            assert!(rent.is_exempt(campaign_account.lamports, campaign_account.data.len()));

            // deserialize campaign state
            let campaign_state = CampaignState::try_from_slice(&campaign_account.data).unwrap();

            assert_eq!(campaign_state.creator, creator_keypair.pubkey());
            assert_eq!(campaign_state.goal, 100_000_000_000);
            assert_eq!(campaign_state.amount_raised, 0);
            assert_eq!(campaign_state.deadline, 0);

            Ok(())
        }
        Err(e) => {
            msg!("Transaction processing error: {:?}", e);
            Err(TransportError::from(e))
        }
    }
}
