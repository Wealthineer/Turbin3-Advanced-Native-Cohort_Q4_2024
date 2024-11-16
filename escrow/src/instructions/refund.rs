use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
    program_error::ProgramError, program_pack::Pack, pubkey::Pubkey, system_program,
};
use spl_token::instruction::{close_account, transfer_checked};
use spl_token::state::Mint;

use crate::Escrow;

pub fn refund(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [maker, mint_a, escrow, maker_ta_a, vault, token_program, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(system_program::check_id(system_program.key));
    assert!(spl_token::check_id(token_program.key));
    assert!(crate::check_id(program_id));
    assert!(maker.is_signer);
    assert!(maker.is_writable);

    let mint_a_unpacked = Mint::unpack(&mint_a.try_borrow_data()?)?;
    assert!(escrow.is_writable);

    let escrow_data = *bytemuck::try_from_bytes_mut::<Escrow>(&mut *escrow.data.borrow_mut())
        .map_err(|_| ProgramError::AccountBorrowFailed)?;

    let escrow_seeds = &[
        b"escrow".as_ref(),
        &maker.key.to_bytes(),
        &[escrow_data.bump as u8],
    ];
    //we assume ata are already created

    let amount = spl_token::state::Account::unpack(&vault.try_borrow_data()?)?.amount;

    //A: vault -> maker_ta_a
    invoke_signed(
        &transfer_checked(
            token_program.key,
            vault.key,
            mint_a.key,
            maker_ta_a.key,
            escrow.key,
            &[],
            amount,
            mint_a_unpacked.decimals,
        )?,
        accounts,
        &[escrow_seeds],
    )?;

    //close vault
    invoke_signed(
        &close_account(token_program.key, vault.key, maker.key, escrow.key, &[])?,
        accounts,
        &[escrow_seeds],
    )?;

    //close escrow
    let mut escrow_data = escrow.data.borrow_mut();
    escrow_data.fill(0);
    let maker_orig_lamports = maker.lamports();
    **maker.lamports.borrow_mut() = maker_orig_lamports
        .checked_add(escrow.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **escrow.lamports.borrow_mut() = 0;

    Ok(())
}
