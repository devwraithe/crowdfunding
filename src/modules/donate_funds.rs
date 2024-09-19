use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    native_token::lamports_to_sol, program_error::ProgramError, pubkey::Pubkey,
};

use crate::state::DonationState;

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

    if !campaign_account.is_writable {
        msg!("Campaign account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if !donation_account.is_signer {
        msg!("Donor must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if campaign_account.owner != program_id {
        msg!("Campaign account must be owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut donation_state = DonationState::try_from_slice(&donation_account.data.borrow())
        .expect("Error deserializing DonationState");

    // update the donation state with the new data
    donation_state.amount = amount;
    donation_state.campaign = campaign;
    donation_state.donor = donor;

    let donation_amount = donation_state.amount;

    **donation_account.try_borrow_mut_lamports()? -= donation_amount;
    **campaign_account.try_borrow_mut_lamports()? += donation_amount;

    // serialize the updated state back into the donation account
    donation_state.serialize(&mut &mut donation_account.data.borrow_mut()[..])?;

    msg!(
        "✅ Donated {} SOL ({} Lamports) to {}",
        lamports_to_sol(donation_amount),
        donation_amount,
        campaign_account.key
    );

    Ok(())
}
