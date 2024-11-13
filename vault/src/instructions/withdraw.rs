use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer};

pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let [_vault, signer, _system_program] = accounts else {
        return Err(ProgramError::InvalidArgument);
    };

    let (pda, bump) = Pubkey::find_program_address(&[signer.key.as_ref()], &crate::ID);

    invoke_signed(
        &transfer(
            &pda, 
            signer.key, 
            lamports
        ),
        accounts,
        &[&[signer.key.as_ref(), &[bump]]]   
    )
}

