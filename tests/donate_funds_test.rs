use solana_program_test::{processor, tokio, ProgramTest};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::Rent,
    signature::{Keypair, Signer},
    sysvar,
    transaction::Transaction,
    transport::TransportError,
};
use multi_sig_wallet::{
    instruction::ProgramInstruction,
    process_instruction,
    state::{CampaignState, DonationState},
};

#[tokio::test]
async fn donate_to_campaign_test() -> Result<(), TransportError> {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "crowdfunding_program::donate",
        program_id,
        processor!(process_instruction),
    );

    let campaign_keypair = Keypair::new();
    let donation_keypair = Keypair::new();

    let rent = Rent::default();
    let campaign_account_size = size_of::<CampaignState>();
    let donation_account_size = size_of::<DonationState>();
    let campaign_account_rent = rent.minimum_balance(campaign_account_size);

    program_test.add_account(
        campaign_keypair.pubkey(),
        Account {
            lamports: campaign_account_rent,
            data: vec![0; campaign_account_size],
            owner: program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        donation_keypair.pubkey(),
        Account {
            lamports: 1_000_000_000_000,
            data: vec![0; donation_account_size],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // create a new campaign
    let create_instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::CreateCampaign {
            creator: donation_keypair.pubkey(),
            goal: 10_000_000_000,
            amount_raised: 0,
            deadline: 10_000_000_000,
        },
        vec![
            AccountMeta::new(campaign_keypair.pubkey(), false),
            AccountMeta::new(donation_keypair.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    );

    let mut create_tx = Transaction::new_with_payer(&[create_instruction], Some(&payer.pubkey()));
    create_tx.sign(&[&payer, &donation_keypair], recent_blockhash);
    banks_client.process_transaction(create_tx).await?;

    // donate funds to created campaign
    let instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::DonateFunds {
            campaign: campaign_keypair.pubkey(),
            donor: donation_keypair.pubkey(),
            amount: 30_000_000_000,
        },
        vec![
            AccountMeta::new(campaign_keypair.pubkey(), false),
            AccountMeta::new(donation_keypair.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &donation_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await?;

    Ok(())
}
