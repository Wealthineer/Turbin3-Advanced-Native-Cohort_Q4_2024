use mollusk_svm::{program, result::Check};

use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{
    state::Escrow,
    tests::{create_account, create_mint_account, create_token_account, setup},
};


#[test]
fn take() {
    let (mollusk, program_id) = setup();

    let token_admin = Pubkey::new_unique();
    let maker = Pubkey::new_unique();
    let taker = Pubkey::new_unique();
    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();
    let (escrow, escrow_bump) =
        Pubkey::find_program_address(&[b"escrow", &maker.to_bytes()], &program_id);
    let taker_ta_a = Pubkey::new_unique();
    let taker_ta_b = Pubkey::new_unique();
    let maker_ta_b = Pubkey::new_unique();

    let vault = Pubkey::new_unique();

    let (token_program, token_program_account) = mollusk_token::token::keyed_account();
    let (system_program, system_program_account) = program::keyed_account_for_system_program();

    let mint_a_account =
        create_mint_account(&mollusk, token_admin, 1_000_000_000, 9, true, token_program);
    let mint_b_account =
        create_mint_account(&mollusk, token_admin, 1_000_000_000, 9, true, token_program);
    let vault_account = create_token_account(&mollusk, &escrow, &mint_a, 1_000_000);
    let taker_ta_a_account = create_token_account(&mollusk, &taker, &mint_a, 0);
    let taker_ta_b_account = create_token_account(&mollusk, &taker, &mint_b, 2_000_000);
    let maker_ta_b_account = create_token_account(&mollusk, &maker, &mint_b, 0);

    let lamports = mollusk.sysvars.rent.minimum_balance(Escrow::LEN);

    let mut escrow_account = create_account(lamports, Escrow::LEN, &program_id);

    escrow_account.set_data_from_slice(
        &[
            maker.to_bytes().to_vec(),
            mint_a.to_bytes().to_vec(),
            mint_b.to_bytes().to_vec(),
            2_000_000u64.to_le_bytes().to_vec(),
            (escrow_bump as u64).to_le_bytes().to_vec(),
        ]
        .concat(),
    );



    let mut instruction_data: Vec<u8> = Vec::new();
    instruction_data.push(1_u8); //discriminator

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(taker, true),
            AccountMeta::new(maker, false),
            AccountMeta::new_readonly(mint_a, false),
            AccountMeta::new_readonly(mint_b, false),
            AccountMeta::new(escrow, false),
            AccountMeta::new(maker_ta_b, false),
            AccountMeta::new(taker_ta_b, false),
            AccountMeta::new(taker_ta_a, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let result = mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (
                taker,
                AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
            ),
            (
                maker,
                AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
            ),
            (mint_a, mint_a_account),
            (mint_b, mint_b_account),
            (escrow, escrow_account),
            (maker_ta_b, maker_ta_b_account),
            (taker_ta_b, taker_ta_b_account),
            (taker_ta_a, taker_ta_a_account),
            (vault, vault_account),
            (token_program, token_program_account),
            (system_program, system_program_account),
        ],
        &[Check::success(), Check::account(&vault).closed().build()],
    );

    // Verify taker_ta_a received the tokens
    let taker_ta_a_result = result
        .get_account(&taker_ta_a)
        .expect("Taker token account A not found");

    let taker_ta_a_data: spl_token::state::Account =
        solana_sdk::program_pack::Pack::unpack(&taker_ta_a_result.data())
            .expect("Failed to unpack taker token account A");

    assert_eq!(taker_ta_a_data.amount, 1_000_000);

    // Verify maker_ta_b received the tokens
    let maker_ta_b_result = result
        .get_account(&maker_ta_b)
        .expect("Maker token account B not found");

    let maker_ta_b_data: spl_token::state::Account =
        solana_sdk::program_pack::Pack::unpack(&maker_ta_b_result.data())
            .expect("Failed to unpack maker token account B");

    assert_eq!(maker_ta_b_data.amount, 2_000_000);
}
