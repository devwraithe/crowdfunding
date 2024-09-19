use crate::state::CampaignState;
use borsh::BorshSerialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;

pub fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    creator: Pubkey,
    goal: u64,
    amount_raised: u64,
    deadline: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let campaign_account = next_account_info(account_info_iter)?;
    let creator_account = next_account_info(account_info_iter)?;

    if !creator_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if campaign_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let rent = Rent::get()?;
    if !rent.is_exempt(campaign_account.lamports(), campaign_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    let campaign_state = CampaignState {
        creator,
        goal,
        amount_raised,
        deadline,
    };

    campaign_state.serialize(&mut &mut campaign_account.data.borrow_mut()[..])?;

    msg!("✅ New campaign created!");

    Ok(())
}
