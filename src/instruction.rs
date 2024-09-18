use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramInstruction {
    CreateCampaign {
        creator: Pubkey,
        goal: u64,
        amount_raised: u64,
        deadline: u64,
    },
}

#[derive(BorshDeserialize)]
pub struct CreateCampaign {
    creator: Pubkey,
    goal: u64,
    amount_raised: u64,
    deadline: u64,
}

impl ProgramInstruction {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let payload = CreateCampaign::try_from_slice(rest).unwrap();
                Self::CreateCampaign {
                    creator: payload.creator,
                    goal: payload.goal,
                    amount_raised: payload.amount_raised,
                    deadline: payload.deadline,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
