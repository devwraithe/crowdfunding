use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use instruction::ProgramInstruction;
use modules::create_campaign::create_campaign;
use modules::donate_funds::donate_to_campaign;

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
            deadline,
        } => {
            create_campaign(program_id, accounts, creator, goal, amount_raised, deadline)
                .expect("TODO: panic message");
        }
        ProgramInstruction::DonateFunds {
            campaign,
            donor,
            amount,
        } => {
            donate_to_campaign(program_id, accounts, campaign, donor, amount)
                .expect("TODO: panic message");
        }
    }

    Ok(())
}
