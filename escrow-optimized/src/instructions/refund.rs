use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};
use pinocchio_token::{instructions::{CloseAccount, Transfer}, state::TokenAccount};

use crate::Escrow;

pub fn refund(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [maker, _mint_a, escrow, maker_ta_a, vault, token_program, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(pinocchio_system::check_id(system_program.key()));
    assert!(pinocchio_token::check_id(token_program.key()));
    assert_eq!(&crate::ID, program_id);
    assert!(maker.is_signer());
    assert!(maker.is_writable());

    assert!(escrow.is_writable());

    let escrow_data = Escrow::from_account_info(escrow);

    let bump_binding = [escrow_data.bump() as u8];
    let escrow_seeds = [Seed::from(b"escrow"), Seed::from(maker.key().as_ref()), Seed::from(&bump_binding)];
    let escrow_signer = Signer::from(&escrow_seeds);
    //we assume ata are already created

    let amount = TokenAccount::from_account_info(vault)?.amount();

    //A: vault -> maker_ta_a
    Transfer {
        from: vault,
        to: maker_ta_a,
        authority: escrow,
        amount,
    }.invoke_signed(&[escrow_signer])?;


    let escrow_signer = Signer::from(&escrow_seeds); //do this again since first time was moved
    //close vault
    CloseAccount {
        account: vault,
        authority: escrow,
        destination: maker,
    }.invoke_signed(&[escrow_signer])?;

    //close escrow by draining lamports and setting data length to 0
    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
