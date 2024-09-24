use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CampaignState {
    pub creator: Pubkey,
    pub goal: u64,
    pub amount_raised: u64,
    pub deadline: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DonationState {
    pub campaign: Pubkey,
    pub donor: Pubkey,
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WithdrawState {
    pub campaign: Pubkey,
    pub creator: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
}
