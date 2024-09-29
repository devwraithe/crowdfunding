use multi_sig_wallet::{
    instruction::ProgramInstruction,
    process_instruction,
    state::{CampaignState, DonationState, WithdrawState},
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    rent::Rent,
    sysvar,
};
use solana_program_test::{processor, tokio, ProgramTest};
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};

#[tokio::test]
async fn crowdfunding_program_test() -> Result<(), TransportError> {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "crowdfunding_program",
        program_id,
        processor!(process_instruction),
    );

    let campaign_keypair = Keypair::new();
    let donation_keypair = Keypair::new();
    let withdraw_keypair = Keypair::new();
    let recipient_keypair = Keypair::new();

    let rent = Rent::default();
    let campaign_account_size = size_of::<CampaignState>();
    let donation_account_size = size_of::<DonationState>();
    let withdraw_account_size = size_of::<WithdrawState>();
    let campaign_account_rent = rent.minimum_balance(campaign_account_size);
    let withdraw_account_rent = rent.minimum_balance(withdraw_account_size);

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
            lamports: 10_000_000_000,
            data: vec![0; donation_account_size],
            owner: program_id,
            ..Account::default()
        },
    );
    program_test.add_account(
        withdraw_keypair.pubkey(),
        Account {
            lamports: withdraw_account_rent,
            data: vec![0; withdraw_account_size],
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
            goal: 40_000_000_000,
            amount_raised: 0,
        },
        vec![
            AccountMeta::new(campaign_keypair.pubkey(), false),
            AccountMeta::new(donation_keypair.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    );

    let mut create_tx = Transaction::new_with_payer(&[create_instruction], Some(&payer.pubkey()));
    create_tx.sign(&[&payer, &donation_keypair], recent_blockhash);
    banks_client.process_transaction(create_tx).await.expect("Create campaign failed");

    // donate funds to created campaign
    let instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::DonateFunds {
            campaign: campaign_keypair.pubkey(),
            donor: donation_keypair.pubkey(),
            amount: 5_000_000_000,
        },
        vec![
            AccountMeta::new(campaign_keypair.pubkey(), false),
            AccountMeta::new(donation_keypair.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &donation_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.expect("Donation failed");

    // withdraw funds
    let instruction = Instruction::new_with_borsh(
        program_id,
        &ProgramInstruction::WithdrawFunds {
            campaign: campaign_keypair.pubkey(),
            creator: withdraw_keypair.pubkey(),
            recipient: recipient_keypair.pubkey(),
            amount: 1_000_000_000,
        },
        vec![
            AccountMeta::new(campaign_keypair.pubkey(), false),
            AccountMeta::new(withdraw_keypair.pubkey(), true),
            AccountMeta::new(recipient_keypair.pubkey(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &withdraw_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.expect("Withdrawal failed");

    Ok(())
}
