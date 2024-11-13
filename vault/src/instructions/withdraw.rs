use solana_nostd_sha256::hashv;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError, program_memory::sol_memcmp};

pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [vault, signer, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // let (pda, bump) = Pubkey::find_program_address(&[signer.key.as_ref()], &crate::ID);

    let lamports = u64::from_le_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]);
    let bump = data[8];

    //not even this is necessary since we check that the signer is the owner of the pda when calculating the signer seeds
    // let pda = Pubkey::new_from_array(hashv(&[signer.key.as_ref(), &[bump]]));

    //CPIs have 1000CU base cost - so lets get rid of them.
    // invoke_signed(
    //     &transfer(
    //         vault.key, 
    //         signer.key, 
    //         lamports
    //     ),
    //     accounts,
    //     &[&[signer.key.as_ref(), &[bump]]]   
    // )

    let pda = hashv(&[signer.key.as_ref(), &[bump]]);
    assert_eq!(sol_memcmp(&pda, vault.key.as_ref(), 32), 0);

    **vault.try_borrow_mut_lamports()? -= lamports; //since the program owns the pda it can withdraw lamports from the vault
    **signer.try_borrow_mut_lamports()? += lamports; //you can always add lamports to an account even if you dont own it

    Ok(())
}

