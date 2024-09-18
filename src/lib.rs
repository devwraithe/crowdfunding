use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use crate::state::CampaignState;
use instruction::ProgramInstruction;

pub mod instruction;
pub mod state;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::unpack(instruction_data)?;

    match instruction {
        ProgramInstruction::CreateCampaign {
            creator,
            goal,
            amount_raised,
            deadline,
        } => {
            create_campaign(program_id, accounts, creator, goal, amount_raised, deadline)
                .expect("TODO: panic message");
        }
    }

    Ok(())
}

fn create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    creator: Pubkey,
    goal: u64,
    amount_raised: u64,
    deadline: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let creator_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;

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
