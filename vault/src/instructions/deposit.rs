use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer};

pub fn process(accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (pda, _) = Pubkey::try_find_program_address(
        &[signer.key.as_ref()], 
        &crate::ID
    ).ok_or(ProgramError::InvalidSeeds)?; //this helps catching user errors but is not strictly necessary and eats cus

    assert_eq!(&pda, vault.key);

    invoke(
        &transfer(
            signer.key, 
            vault.key, 
            lamports
        ),
        accounts
    )
}

