use std::borrow::BorrowMut;

use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};
use pinocchio_token::{instructions::{CloseAccount, TransferChecked}, state::{Mint, TokenAccount}};

use crate::Escrow;

pub fn refund(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [maker, mint_a, escrow, maker_ta_a, vault, token_program, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(pinocchio_system::check_id(system_program.key()));
    assert!(pinocchio_token::check_id(token_program.key()));
    assert_eq!(&crate::ID, program_id);
    assert!(maker.is_signer());
    assert!(maker.is_writable());

    let mint_a_decimals = Mint::from_account_info(mint_a)?.decimals();
    assert!(escrow.is_writable());

    let escrow_data = Escrow::from_account_info(escrow);

    let bump_binding = [escrow_data.bump() as u8];
    let escrow_seeds = [Seed::from(b"escrow"), Seed::from(maker.key().as_ref()), Seed::from(&bump_binding)];
    let escrow_signer = Signer::from(&escrow_seeds);
    //we assume ata are already created

    let amount = TokenAccount::from_account_info(vault)?.amount();

    //A: vault -> maker_ta_a
    TransferChecked {
        from: vault,
        to: maker_ta_a,
        authority: escrow,
        mint: mint_a,
        decimals: mint_a_decimals,
        amount,
    }.invoke_signed(&[escrow_signer])?;


    let escrow_signer = Signer::from(&escrow_seeds); //do this again since first time was moved
    //close vault
    CloseAccount {
        account: vault,
        authority: escrow,
        destination: maker,
    }.invoke_signed(&[escrow_signer])?;

    //close escrow

    let maker_orig_lamports = maker.lamports();
    *maker.lamports().borrow_mut() = maker_orig_lamports
        .checked_add(escrow.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;
    *escrow.lamports().borrow_mut() = 0;
    escrow.realloc(0, false)?;

    Ok(())
}
