use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CampaignState {
    pub creator: Pubkey,
    pub goal: u64,
    pub amount_raised: u64,
    pub deadline: u64,
}
