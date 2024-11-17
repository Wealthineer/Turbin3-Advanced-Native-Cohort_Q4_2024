use mollusk_svm::{program, result::Check};

use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::{
    tests::{create_mint_account, create_token_account, setup},
    Escrow,
};

#[test]
fn make() {
    let (mollusk, program_id) = setup();

    let token_admin = Pubkey::new_unique();
    let maker = Pubkey::new_unique();
    let mint_a = Pubkey::new_unique();
    let mint_b = Pubkey::new_unique();
    let (escrow, escrow_bump) =
        Pubkey::find_program_address(&[b"escrow", &maker.to_bytes()], &program_id);
    let maker_ta_a = Pubkey::new_unique();
    let vault = Pubkey::new_unique();

    let (token_program, token_program_account) = mollusk_token::token::keyed_account();
    let (system_program, system_program_account) = program::keyed_account_for_system_program();

    let mint_a_account =
        create_mint_account(&mollusk, token_admin, 1_000_000_000, 9, true, token_program);
    let mint_b_account =
        create_mint_account(&mollusk, token_admin, 1_000_000_000, 9, true, token_program);
    let vault_account = create_token_account(&mollusk, &escrow, &mint_a, 0);
    let maker_ta_a_account = create_token_account(&mollusk, &maker, &mint_a, 1_000_000);


    let mut instruction_data: Vec<u8> = Vec::new();
    instruction_data.push(0_u8); //discriminator
    instruction_data.extend_from_slice(&maker.to_bytes());
    instruction_data.extend_from_slice(&1_000_000u64.to_le_bytes());
    instruction_data.extend_from_slice(&1_000_000u64.to_le_bytes());
    instruction_data.push(escrow_bump);


    let instruction = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new_readonly(mint_a, false),
            AccountMeta::new_readonly(mint_b, false),
            AccountMeta::new(escrow, false),
            AccountMeta::new(maker_ta_a, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let result = mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (
                maker,
                AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
            ),
            (mint_a, mint_a_account),
            (mint_b, mint_b_account),
            (escrow, AccountSharedData::new(0, 0, &Pubkey::default())),
            (maker_ta_a, maker_ta_a_account),
            (vault, vault_account),
            (token_program, token_program_account),
            (system_program, system_program_account),
        ],
        &[
            Check::success(),
            Check::account(&escrow).owner(&program_id).build(),
            Check::account(&escrow).build(),
        ],
    );

    let escrow_result_account = result
        .get_account(&escrow)
        .expect("Escrow account not found");

    let escrow_result_data = &escrow_result_account.data();
    assert_eq!(escrow_result_data.len(), Escrow::LEN);

    let vault_result_account = result.get_account(&vault).expect("Vault account not found");

    let vault_result_data: spl_token::state::Account =
        solana_sdk::program_pack::Pack::unpack(&vault_result_account.data())
            .expect("Failed to unpack contributor token account");

    assert_eq!(vault_result_data.amount, 1_000_000);
}
