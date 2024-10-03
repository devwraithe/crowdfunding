use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    native_token::lamports_to_sol, program_error::ProgramError, pubkey::Pubkey,
};

use crate::state::{CampaignState, WithdrawState};

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
    let creator_account = next_account_info(account_info_iter)?;

    if campaign_account.owner != program_id {
        msg!("Campaign account must be owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !campaign_account.is_writable || !creator_account.is_writable {
        msg!("Both accounts must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if !creator_account.is_signer {
        msg!("Creator account must sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let campaign_state = CampaignState::try_from_slice(&campaign_account.data.borrow())
        .map_err(|err| {
            msg!("Error deserializing CampaignState: {}", err);
            ProgramError::InvalidAccountData
        })?;

    // Verify that the withdrawal account is actually the creator of the campaign
    if campaign_state.creator != *creator_account.key {
        msg!("Only the campaign creator can withdraw funds");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut withdrawal_state = WithdrawState::try_from_slice(&creator_account.data.borrow())
        .map_err(|err| {
            msg!("Error deserializing WithdrawState: {}", err);
            ProgramError::InvalidAccountData
        })?;

    withdrawal_state.amount = amount;
    withdrawal_state.campaign = campaign;
    withdrawal_state.creator = creator;
    withdrawal_state.recipient = recipient;

    let withdrawal_amount = withdrawal_state.amount;

    if **campaign_account.lamports.borrow() < withdrawal_amount {
        msg!("Insufficient funds in the campaign account");
        return Err(ProgramError::InsufficientFunds);
    }

    **campaign_account.try_borrow_mut_lamports()? -= withdrawal_amount;
    **creator_account.try_borrow_mut_lamports()? += withdrawal_amount;

    withdrawal_state
        .serialize(&mut &mut creator_account.data.borrow_mut()[..])
        .map_err(|err| {
            msg!("Error serializing WithdrawalState: {}", err);
            ProgramError::InvalidAccountData
        })?;

    msg!(
        "✅ Withdrawn {} SOL ({} Lamports) from {} to {} by {}",
        lamports_to_sol(withdrawal_amount),
        withdrawal_amount,
        campaign_account.key,
        withdrawal_state.recipient,
        creator_account.key,
    );

    Ok(())
}
