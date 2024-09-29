use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    native_token::lamports_to_sol, program_error::ProgramError, pubkey::Pubkey,
};

use crate::state::{CampaignState, DonationState};

pub fn donate_to_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    campaign: Pubkey,
    donor: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let campaign_account = next_account_info(account_info_iter)?;
    let donation_account = next_account_info(account_info_iter)?;

    if campaign_account.owner != program_id {
        msg!("Campaign account must be owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !campaign_account.is_writable {
        msg!("Campaign account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if !donation_account.is_signer {
        msg!("Donor must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // verify donor has enough lamports to donate
    if donation_account.lamports() < amount {
        msg!("Donor does not have enough lamports to donate");
        return Err(ProgramError::InsufficientFunds);
    }

    let mut donation_state = DonationState::try_from_slice(&donation_account.data.borrow())
        .map_err(|err| {
            msg!("Error deserializing DonationState: {}", err);
            ProgramError::InvalidAccountData
        })?;

    // update the donation state with the new data
    donation_state.amount = amount;
    donation_state.campaign = campaign;
    donation_state.donor = donor;

    let donation_amount = donation_state.amount;

    **donation_account.try_borrow_mut_lamports()? -= donation_amount;
    **campaign_account.try_borrow_mut_lamports()? += donation_amount;

    // serialize the updated state back into the donation account
    donation_state
        .serialize(&mut &mut donation_account.data.borrow_mut()[..])
        .map_err(|err| {
            msg!("Error serializing donation account: {}", err);
            ProgramError::InvalidAccountData
        })?;

    // update the campaign state with donation amount
    let mut campaign_state = CampaignState::try_from_slice(&campaign_account.data.borrow())
        .map_err(|err| {
            msg!("Error deserializing CampaignState: {}", err);
            ProgramError::InvalidAccountData
        })?;
    campaign_state.amount_raised += donation_amount;
    campaign_state
        .serialize(&mut &mut campaign_account.data.borrow_mut()[..])
        .map_err(|err| {
            msg!("Error serializing campaign account: {}", err);
            ProgramError::InvalidAccountData
        })?;

    msg!(
        "✅ Donated {} SOL ({} Lamports) to {}",
        lamports_to_sol(donation_amount),
        donation_amount,
        campaign_account.key
    );

    Ok(())
}
