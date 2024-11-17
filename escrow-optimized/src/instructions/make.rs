
use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, sysvars::{rent::Rent, Sysvar}, ProgramResult
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::{instructions::Transfer, state::Mint};

use crate::Escrow;

// IX DATA:
//     maker: Pubkey, 0
//     amount: u64, // 32
//     receive: u64, // 40
//     escrow_bump: u8, // 48

pub fn make(program_id: &Pubkey, accounts: &[AccountInfo], ix_data: &[u8]) -> ProgramResult {
    let [maker, mint_a, mint_b, escrow, maker_ta_a, vault, token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount: u64 = unsafe { *(ix_data.as_ptr().add(32) as *const u64) };
    let receive: u64 = unsafe { *(ix_data.as_ptr().add(40) as *const u64) };
    let escrow_bump: u8 = unsafe { *(ix_data.as_ptr().add(48) as *const u8) };

    assert!(pinocchio_system::check_id(system_program.key()));
    assert!(pinocchio_token::check_id(token_program.key()));
    assert_eq!(&crate::ID, program_id);
    assert!(maker.is_signer());
    assert!(maker.is_writable());
    assert_eq!(mint_a.owner(), token_program.key());
    assert_eq!(mint_b.owner(), token_program.key());
    assert_eq!(maker_ta_a.owner(), token_program.key());
    assert_eq!(vault.owner(), token_program.key());

    let _mint_unpacked = Mint::from_account_info(mint_a)?;

    assert!(escrow.is_writable() && escrow.data_is_empty());

    let bump_binding = [escrow_bump];
    let escrow_seeds = [Seed::from(b"escrow"), Seed::from(maker.key().as_ref()), Seed::from(&bump_binding)];
    let escrow_signer = Signer::from(&escrow_seeds);

    CreateAccount {
        from: maker,
        to: escrow,
        lamports: Rent::get()?.minimum_balance( Escrow::LEN), 
        space: Escrow::LEN as u64,
        owner: &crate::ID,

    }.invoke_signed(&[escrow_signer])?;

    unsafe {
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr() as *mut Pubkey) = *maker.key();
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(32) as *mut Pubkey) = *mint_a.key();
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(64) as *mut Pubkey) = *mint_b.key();
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(96) as *mut u64) = receive;
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().add(104) as *mut u8) = escrow_bump;
    }
    //we assume ata are already created
    Transfer {
        from: maker_ta_a,
        to: vault,
        authority: maker,
        amount,
    }.invoke()?;

    Ok(())
}
