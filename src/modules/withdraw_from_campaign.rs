use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    native_token::lamports_to_sol, program_error::ProgramError, pubkey::Pubkey,
};

use crate::state::WithdrawState;

pub fn withdraw_from_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    campaign: Pubkey,
    creator: Pubkey,
    recipient: Pubkey,
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let campaign_account = next_account_info(account_info_iter)?;
    let withdrawal_account = next_account_info(account_info_iter)?;

    if campaign_account.owner != program_id {
        msg!("Campaign account must be owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !campaign_account.is_writable {
        msg!("Campaign account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if !withdrawal_account.is_signer {
        msg!("Withdrawal account must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !withdrawal_account.is_writable {
        msg!("Withdrawal account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut withdrawal_state = WithdrawState::try_from_slice(&withdrawal_account.data.borrow())
        .expect("Error deserializing WithdrawState");

    withdrawal_state.amount = amount;
    withdrawal_state.campaign = campaign;
    withdrawal_state.creator = creator;
    withdrawal_state.recipient = recipient;

    if withdrawal_state.creator != *withdrawal_account.key {
        msg!("Only the campaign creator can withdraw funds");
        return Err(ProgramError::InvalidAccountData);
    }

    let withdrawal_amount = withdrawal_state.amount;

    if **campaign_account.lamports.borrow() < withdrawal_amount {
        msg!("Insufficient funds in the campaign account");
        return Err(ProgramError::InsufficientFunds);
    }

    **campaign_account.try_borrow_mut_lamports()? -= withdrawal_amount;
    **withdrawal_account.try_borrow_mut_lamports()? += withdrawal_amount;

    withdrawal_state.serialize(&mut &mut withdrawal_account.data.borrow_mut()[..])?;

    msg!(
        "✅ Withdrawn {} SOL ({} Lamports) from {} to {} by {}",
        lamports_to_sol(withdrawal_amount),
        withdrawal_amount,
        campaign_account.key,
        withdrawal_state.recipient,
        withdrawal_account.key,
    );

    Ok(())
}
