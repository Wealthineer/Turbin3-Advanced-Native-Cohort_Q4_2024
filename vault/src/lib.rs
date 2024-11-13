use solana_program::program_error::ProgramError;
// use instructions::VaultInstructions;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, pubkey};

const ID: Pubkey = pubkey!("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);

mod instructions;

fn process_instruction(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    instruction_data: &[u8]
) -> ProgramResult {
    //not necessary since pda would not calculate correctly
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // let (discriminator, data) = instruction_data
    //     .split_first()
    //     .ok_or(ProgramError::InvalidInstructionData)?;

    assert_eq!(instruction_data.len(), 8);

    instructions::withdraw::process(accounts, instruction_data)


//     let amount = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);

    //deposit can be directly done without the program - so we only need to cover the withdraw here
//     match VaultInstructions::try_from(*discriminator)? {
//         VaultInstructions::Deposit => instructions::deposit::process(accounts, amount),
//         VaultInstructions::Withdraw => instructions::withdraw::process(accounts, data),
//     }
}

