#[cfg(test)]
mod tests {

    use borsh::BorshSerialize;
    use mollusk_svm::{program, result::Check, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, WritableAccount, ReadableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };
    use spl_token::state::AccountState;

    use crate::state::Escrow;

    use crate::processor::{EscrowArgs, EscrowInstruction};


    


    #[test]
    fn make() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(&program_id, "target/deploy/escrow");
        mollusk_token::token::add_program(&mut mollusk);

        let token_admin = Pubkey::new_unique();
        let maker = Pubkey::new_unique();
        let mint_a = Pubkey::new_unique();
        let mint_b = Pubkey::new_unique();
        let (escrow, escrow_bump) = Pubkey::find_program_address(&[b"escrow", &maker.to_bytes()], &program_id);
        let maker_ta_a = Pubkey::new_unique();
        let vault = Pubkey::new_unique();

        let (token_program, token_program_account) = mollusk_token::token::keyed_account();
        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let mint_a_account = create_mint_account(&mollusk, token_admin, 1_000_000_000, 9, true, token_program);
        let mint_b_account = create_mint_account(&mollusk, token_admin, 1_000_000_000, 9, true, token_program);
        let vault_account = pack_token_account(&mollusk, &escrow, &mint_a,0);
        let maker_ta_a_account = pack_token_account(&mollusk, &maker, &mint_a, 1_000_000);

        let escrow_instruction = EscrowInstruction::Make(EscrowArgs {
            maker,
            amount: 1_000_000u64,
            receive: 1_000_000u64,
            escrow_bump: escrow_bump,
        });

        let mut instruction_data: Vec<u8> = Vec::new();
        escrow_instruction.serialize(&mut instruction_data).expect("Failed to serialize instruction");


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
            &[Check::success()],
        );

        let escrow_result_account = result
        .get_account(&escrow)
        .expect("Escrow account not found");

        let data = escrow_result_account.data();
        println!("data: {:?}", data);

        assert!(!result.program_result.is_err());
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
        token_program: Pubkey
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
    
    pub fn pack_token_account(mollusk: &Mollusk, owner: &Pubkey, mint: &Pubkey, amount: u64) -> AccountSharedData {
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

}