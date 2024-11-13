use instructions::VaultInstructions;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError};
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, pubkey};

const ID: Pubkey = pubkey!("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);

mod instructions;

fn process_instruction(
    _program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8]
) -> ProgramResult {
    //not necessary since pda would not calculate correctly
    // if program_id.ne(&crate::ID) {
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    // assert_eq!(data.len(), 8);

    let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);

    match VaultInstructions::try_from(*discriminator)? {
        VaultInstructions::Deposit => instructions::deposit::process(accounts, amount),
        VaultInstructions::Withdraw => instructions::withdraw::process(accounts, amount),
    }
}

