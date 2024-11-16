use mollusk_svm::Mollusk;

use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::AccountState;

#[cfg(test)]
pub mod make;

pub fn setup() -> (Mollusk, Pubkey) {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "22222222222222222222222222222222222222222222",
    ));

    let project_name = format!("target/deploy/{}", env!("CARGO_PKG_NAME"));
    let mut mollusk = Mollusk::new(&program_id, &project_name);

    mollusk_token::token::add_program(&mut mollusk);
    (mollusk, program_id)
}

//helper functions

pub fn create_account(lamports: u64, data_len: usize, owner: &Pubkey) -> AccountSharedData {
    AccountSharedData::new(lamports, data_len, owner)
}

pub fn create_mint_account(
    mollusk: &Mollusk,
    authority: Pubkey,
    supply: u64,
    decimals: u8,
    is_initialized: bool,
    token_program: Pubkey,
) -> AccountSharedData {
    let mut account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );

    spl_token::state::Mint {
        mint_authority: COption::Some(authority),
        supply,
        decimals,
        is_initialized,
        freeze_authority: COption::None,
    }
    .pack_into_slice(account.data_as_mut_slice());

    account
}

pub fn create_token_account(
    mollusk: &Mollusk,
    owner: &Pubkey,
    mint: &Pubkey,
    amount: u64,
) -> AccountSharedData {
    let lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(spl_token::state::Account::LEN);
    let mut account = create_account(lamports, spl_token::state::Account::LEN, &spl_token::id());
    spl_token::state::Account {
        mint: *mint,
        owner: *owner,
        amount,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    }
    .pack_into_slice(account.data_as_mut_slice());
    account
}
