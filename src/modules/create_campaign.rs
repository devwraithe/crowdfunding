use crate::state::CampaignState;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    creator: Pubkey,
    goal: u64,
    amount_raised: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let campaign_account = next_account_info(account_info_iter)?;
    let creator_account = next_account_info(account_info_iter)?;

    if campaign_account.owner != program_id {
        msg!("Campaign account must be owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !creator_account.is_signer {
        msg!("Creator must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = Rent::get()?;
    if !rent.is_exempt(campaign_account.lamports(), campaign_account.data_len()) {
        msg!("Campaign account must be rent exempt");
        return Err(ProgramError::AccountNotRentExempt);
    }

    let campaign_state = CampaignState {
        creator,
        goal,
        amount_raised,
    };

    campaign_state
        .serialize(&mut &mut campaign_account.data.borrow_mut()[..])
        .map_err(|err| {
            msg!("Error serializing CampaignState: {}", err);
            ProgramError::InvalidAccountData
        })?;

    msg!("✅ New campaign created!");

    Ok(())
}
