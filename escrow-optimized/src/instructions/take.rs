
use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};
use pinocchio_token::{instructions::{CloseAccount, Transfer}, state::TokenAccount};

use crate::Escrow;

pub fn take(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [taker, maker, mint_a, mint_b, escrow, maker_ta_b, taker_ta_b, taker_ta_a, vault, token_program, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(pinocchio_system::check_id(system_program.key()));
    assert!(pinocchio_token::check_id(token_program.key()));
    assert_eq!(&crate::ID, program_id);
    assert!(taker.is_signer());
    assert!(taker.is_writable());

    let escrow_data = Escrow::from_account_info(escrow);
    assert_eq!(&escrow_data.mint_a(), mint_a.key());
    assert_eq!(&escrow_data.mint_b(), mint_b.key());

    let bump_binding = [escrow_data.bump() as u8];
    let escrow_seeds = [Seed::from(b"escrow"), Seed::from(maker.key().as_ref()), Seed::from(&bump_binding)];
    let escrow_signer = Signer::from(&escrow_seeds);

    let amount = TokenAccount::from_account_info(vault)?.amount();

    //A: vault -> taker_ta_a
    Transfer {
        from: vault,
        to: taker_ta_a,
        authority: escrow,
        amount,
    }.invoke_signed(&[escrow_signer])?;

    //B: taker_ta_b -> maker_ta_b
    Transfer {
        from: taker_ta_b,
        to: maker_ta_b,
        authority: taker,
        amount: escrow_data.receive(),
    }.invoke()?;


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
