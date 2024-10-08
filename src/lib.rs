use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use instruction::ProgramInstruction;
use modules::create_campaign::create_campaign;
use modules::donate_to_campaign::donate_to_campaign;
use modules::withdraw_from_campaign::withdraw_from_campaign;

pub mod instruction;
mod modules;
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
        } => {
            create_campaign(program_id, accounts, creator, goal, amount_raised)?;
        }
        ProgramInstruction::DonateFunds {
            campaign,
            donor,
            amount,
        } => {
            donate_to_campaign(program_id, accounts, campaign, donor, amount)?;
        }
        ProgramInstruction::WithdrawFunds {
            campaign,
            creator,
            recipient,
            amount,
        } => {
            withdraw_from_campaign(program_id, accounts, campaign, creator, recipient, amount)?;
        }
    }

    Ok(())
}
