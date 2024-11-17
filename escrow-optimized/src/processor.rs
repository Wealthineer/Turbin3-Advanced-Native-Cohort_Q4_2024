use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

use crate::instructions;

// DATA:
//     discriminator: u8, // 0 - Make, 1 - Take, 2 - Refund
//     maker: Pubkey,
//     amount: u64,
//     receive: u64,
//     escrow_bump: u8,


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let discriminator = data[0] as u8;
    let ix_data = &data[1..];

    match discriminator {
        0 => {
            instructions::make(program_id, accounts, ix_data)?
        }
        1 => {
            instructions::take(program_id, accounts)?
        }
        2 => {
            instructions::refund(program_id, accounts)?
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
