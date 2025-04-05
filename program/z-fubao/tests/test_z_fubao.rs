#[cfg(test)]
mod tests {
    mod constants {
        use lazy_static::lazy_static;
        use solana_program::pubkey::Pubkey;
        use solana_sdk::{signature::Keypair, signer::Signer};
        use spl_associated_token_account::get_associated_token_address;
        use std::str::FromStr;

        lazy_static! {
            pub static ref PROGRAM_ID: Pubkey =
                Pubkey::from_str("6tYirs3md3GB2z3VarvjCYCTcPfUKDHC6W19ahRpasdf").unwrap();
            pub static ref DEPLOYER: Keypair = Keypair::new();
            pub static ref AUTHORITY: Pubkey =
                Pubkey::find_program_address(&[b"authority"], &PROGRAM_ID).0;
            pub static ref ZBTC_MINT_KEYPAIR: Keypair = Keypair::new();
            pub static ref ZBTC_MINT: Pubkey = ZBTC_MINT_KEYPAIR.pubkey();
            pub static ref ZBTC_VAULT: Pubkey =
                get_associated_token_address(&AUTHORITY, &ZBTC_MINT);
            pub static ref ZUSD_MINT_KEYPAIR: Keypair = Keypair::new();
            pub static ref ZUSD_MINT: Pubkey = ZUSD_MINT_KEYPAIR.pubkey();
            pub static ref LENDING_STATE: Pubkey =
                Pubkey::find_program_address(&[b"lending_state"], &PROGRAM_ID).0;
        }
    }
    mod utils {
        use solana_sdk::pubkey::Pubkey;

        pub fn find_obligation_pda(user: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
            let seeds = &[b"obligation", user.as_ref()];
            Pubkey::find_program_address(seeds, program_id)
        }
    }
    mod encoder {
        use crate::tests::constants::*;
        use crate::tests::utils::*;
        use solana_sdk::{
            instruction::{AccountMeta, Instruction},
            pubkey::Pubkey,
            system_program,
        };
        use spl_associated_token_account::get_associated_token_address;

        pub async fn create_init_lending_state_instruction(
            program_id: &Pubkey,
            owner: &Pubkey,
        ) -> Instruction {
            Instruction::new_with_bytes(
                *program_id,
                &[0], // Initialize instruction
                vec![
                    AccountMeta::new(*owner, true), // 0. Owner account (signer, writable)
                    AccountMeta::new(*AUTHORITY, false), // 1. Authority account (writable)
                    AccountMeta::new(*LENDING_STATE, false), // 2. Lending state account (writable)
                    AccountMeta::new_readonly(*ZBTC_MINT, false),    // 3. ZBTC mint
                    AccountMeta::new(*ZUSD_MINT, false),             // 4. ZUSD mint
                    AccountMeta::new(*ZBTC_VAULT, false), // 5. ZBTC vault account (writable)
                    AccountMeta::new_readonly(solana_program::system_program::id(), false), // 6. System program
                    AccountMeta::new_readonly(spl_token::id(), false), // 7. Token program
                    AccountMeta::new_readonly(spl_associated_token_account::id(), false), // 8. Associated token program
                ],
            )
        }

        pub async fn create_init_obligation_instruction(
            program_id: &Pubkey,
            user: &Pubkey,
        ) -> Instruction {
            Instruction::new_with_bytes(
                *program_id,
                &[1], // InitObligation instruction
                vec![
                    AccountMeta::new(*user, true), // 0. User account (signer, writable)
                    AccountMeta::new(*AUTHORITY, false), // 1. Authority account (writable)
                    AccountMeta::new(find_obligation_pda(user, program_id).0, false), // 1. Obligation account (PDA, writable)
                    AccountMeta::new_readonly(system_program::id(), false), // 2. System program
                ],
            )
        }

        pub async fn create_deposit_zbtc_instruction(
            program_id: &Pubkey,
            user: &Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = vec![2]; // DepositZBTC instruction
            data.extend_from_slice(&amount.to_le_bytes());

            Instruction::new_with_bytes(
                *program_id,
                &data,
                vec![
                    AccountMeta::new(*user, true), // 0. User account (signer, writable)
                    AccountMeta::new(*AUTHORITY, false), // 1. Authority account (writable)
                    AccountMeta::new(find_obligation_pda(user, program_id).0, false), // 1. Obligation account (PDA, writable)
                    AccountMeta::new(get_associated_token_address(user, &ZBTC_MINT), false), // 2. User's ZBTC token account (writable)
                    AccountMeta::new(*ZBTC_VAULT, false), // 3. ZBTC vault token account (writable)
                    AccountMeta::new_readonly(spl_token::id(), false), // 4. Token program id
                ],
            )
        }

        pub async fn create_withdraw_zbtc_instruction(
            program_id: &Pubkey,
            user: &Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = vec![3]; // WithdrawZBTC instruction
            data.extend_from_slice(&amount.to_le_bytes());

            Instruction::new_with_bytes(
                *program_id,
                &data,
                vec![
                    AccountMeta::new(*user, true), // 0. User account (signer, writable)
                    AccountMeta::new(*AUTHORITY, false), // 1. Authority account (writable)
                    AccountMeta::new(find_obligation_pda(user, program_id).0, false), // 1. Obligation account (PDA, writable)
                    AccountMeta::new(get_associated_token_address(user, &ZBTC_MINT), false), // 2. User's ZBTC token account (writable)
                    AccountMeta::new(*ZBTC_VAULT, false), // 3. ZBTC vault token account (writable)
                    AccountMeta::new_readonly(*LENDING_STATE, false), // 4. Lending state account
                    AccountMeta::new_readonly(spl_token::id(), false),         // 5. Token program id
                ],
            )
        }

        pub async fn create_borrow_zusd_instruction(
            program_id: &Pubkey,
            user: &Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = vec![4]; // BorrowZUSD instruction
            data.extend_from_slice(&amount.to_le_bytes());

            Instruction::new_with_bytes(
                *program_id,
                &data,
                vec![
                    AccountMeta::new(*user, true), // 0. User account (signer, writable)
                    AccountMeta::new_readonly(*AUTHORITY, false), // 1. Authority account
                    AccountMeta::new(find_obligation_pda(user, program_id).0, false), // 2. Obligation account (PDA, writable)
                    AccountMeta::new(get_associated_token_address(user, &ZUSD_MINT), false), // 3. User's ZUSD token account (writable)
                    AccountMeta::new(*ZUSD_MINT, false),         // 4. ZUSD mint
                    AccountMeta::new_readonly(*LENDING_STATE, false), // 5. Lending state account
                    AccountMeta::new_readonly(spl_token::id(), false),         // 6. Token program id
                ],
            )
        }

        pub async fn create_repay_zusd_instruction(
            program_id: &Pubkey,
            user: &Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = vec![5]; // RepayZUSD instruction
            data.extend_from_slice(&amount.to_le_bytes());

            Instruction::new_with_bytes(
                *program_id,
                &data,
                vec![
                    AccountMeta::new(*user, true), // 0. User account (signer, writable)
                    AccountMeta::new(*AUTHORITY, false), // 1. Authority account (writable)
                    AccountMeta::new(find_obligation_pda(user, program_id).0, false), // 1. Obligation account (PDA, writable)
                    AccountMeta::new(get_associated_token_address(user, &ZUSD_MINT), false), // 2. User's ZUSD token account (writable)
                    AccountMeta::new(*ZUSD_MINT, false),         // 3. ZUSD mint
                    AccountMeta::new_readonly(*LENDING_STATE, false), // 4. Lending state account
                    AccountMeta::new_readonly(spl_token::id(), false),         // 5. Token program id
                ],
            )
        }
    }

    use solana_program::sysvar::Sysvar;

    use {
        borsh::BorshDeserialize,
        constants::*,
        encoder::*,
        solana_client::nonblocking::rpc_client::RpcClient as AsyncRpcClient,
        solana_program::{
            program_pack::Pack,
            pubkey::Pubkey,
            system_instruction,
        },
        solana_program_test::*,
        solana_sdk::{
            signature::{Keypair, Signer},
            transaction::Transaction,
        },
        spl_associated_token_account::get_associated_token_address,
        std::str::FromStr,
        utils::*,
    };
    use z_fubao::processor::Processor;

    async fn fetch_and_init_devnet_accounts(program_test: &mut ProgramTest) {
        // Initialize async RPC client for devnet
        let rpc_client = AsyncRpcClient::new("https://api.devnet.solana.com".to_string());
        println!("Fetching devnet account data...");

        // Load token mints
        // load_account!(rpc_client, program_test, *ZBTC_MINT, owner: spl_token::id());
        // load_account!(rpc_client, program_test, *ZUSD_MINT, owner: spl_token::id());
    }

    // Helper function to verify obligation state
    async fn verify_obligation_state(
        banks_client: &mut BanksClient,
        obligation_pda: &Pubkey,
        expected_zbtc_deposit: u64,
        expected_zusd_borrowed: u64,
        test_case: &str,
    ) {
        let account = banks_client
            .get_account(*obligation_pda)
            .await
            .unwrap()
            .expect("Obligation account not found");

        let obligation = z_fubao::state::Obligation::try_from_slice(&account.data)
            .expect("Failed to deserialize obligation");

        println!(
            "Obligation state after {}: zbtc_deposit = {}, zusd_borrowed = {}",
            test_case, obligation.zbtc_deposit, obligation.zusd_borrowed
        );

        assert_eq!(
            obligation.zbtc_deposit, expected_zbtc_deposit,
            "Obligation zbtc_deposit should be {} after {}",
            expected_zbtc_deposit, test_case
        );

        assert_eq!(
            obligation.zusd_borrowed, expected_zusd_borrowed,
            "Obligation zusd_borrowed should be {} after {}",
            expected_zusd_borrowed, test_case
        );
    }

    async fn stat_obligation(banks_client: &mut BanksClient, payer: &Pubkey) {
        let (obligation_pda, _) = find_obligation_pda(payer, &PROGRAM_ID);
        let obligation = banks_client.get_account(obligation_pda).await.unwrap().unwrap();

        let obligation = z_fubao::state::Obligation::try_from_slice(&obligation.data)
            .expect("Failed to deserialize obligation");

        println!(
            r#"====================================================================================================================================
    Obligation state: zbtc_deposit = {}, zusd_borrowed = {}, max_borrowable = {}, max_withdrawable = {}
===================================================================================================================================="#,
            obligation.zbtc_deposit,
            obligation.zusd_borrowed,
            Processor::calculate_max_borrowable(&obligation, 50000, 70).unwrap(),
            Processor::calculate_max_withdrawable(&obligation, 50000, 70).unwrap(),
        );
    }

    #[tokio::test]
    async fn test_lending_protocol() {
        // Testing Scenario:
        // 1. Setup:
        //    - Initialize program test environment
        //    - Load lending program and accounts from mainnet
        //    - Create and fund a custom payer account with 10 SOL
        //    - Initialize lending state
        //    - Initialize obligation PDA for the user
        //
        // 2. Deposit ZBTC (request_id = 1):
        //    - Deposit 1 ZBTC as collateral
        //    - Verify obligation state updated correctly
        //
        // 3. Borrow ZUSD (request_id = 2):
        //    - Borrow 500 ZUSD
        //    - Verify obligation state updated correctly
        //    - Verify ZUSD tokens minted to user's account
        //
        // 4. Repay ZUSD (request_id = 3):
        //    - Repay 200 ZUSD
        //    - Verify obligation state updated correctly
        //    - Verify ZUSD tokens burned from user's account
        //
        // 5. Withdraw ZBTC (request_id = 4):
        //    - Withdraw 0.5 ZBTC
        //    - Verify obligation state updated correctly
        //
        // 6. Try to withdraw too much ZBTC (request_id = 5):
        //    - Attempt to withdraw more ZBTC than allowed by LTV ratio
        //    - Verify transaction fails
        //
        // 7. Try to borrow too much ZUSD (request_id = 6):
        //    - Attempt to borrow more ZUSD than allowed by LTV ratio
        //    - Verify transaction fails
        //
        // 8. Try to repay more than borrowed (request_id = 7):
        //    - Attempt to repay more ZUSD than borrowed
        //    - Verify transaction fails

        // ==================================================================================
        // Test Case 1: Setup
        // Purpose: Initialize the test environment and required accounts
        // Expected behavior:
        // - Program test environment initialized
        // - Lending program and accounts loaded from mainnet
        // - Custom payer funded with 10 SOL
        // - Lending state initialized
        // - Obligation PDA initialized
        // ==================================================================================
        // Initialize program test
        let mut program_test = ProgramTest::new("z_fubao", *PROGRAM_ID, None);

        // Initialize accounts from mainnet
        fetch_and_init_devnet_accounts(&mut program_test).await;

        // Start the test context with default payer
        let (mut banks_client, default_payer, recent_blockhash) = program_test.start().await;

        let payer = &Keypair::new();

        println!("Custom deployer public key: {}", DEPLOYER.pubkey());
        println!("Custom payer public key: {}", payer.pubkey());

        // Fund our payer account with SOL
        let fund_ix = system_instruction::transfer(
            &default_payer.pubkey(),
            &DEPLOYER.pubkey(),
            10_000_000_000, // 10 SOL
        );
        let fund_ix_2 = system_instruction::transfer(
            &default_payer.pubkey(),
            &payer.pubkey(),
            10_000_000_000, // 10 SOL
        );

        let fund_tx = Transaction::new_signed_with_payer(
            &[fund_ix, fund_ix_2],
            Some(&default_payer.pubkey()),
            &[&default_payer],
            recent_blockhash,
        );
        banks_client.process_transaction(fund_tx).await.unwrap();

        // Get payer balance
        let deployer_balance = banks_client.get_balance(DEPLOYER.pubkey()).await.unwrap();
        println!(
            "Custom deployer balance: {} SOL",
            deployer_balance as f64 / 1_000_000_000.0
        );
        let payer_balance = banks_client.get_balance(payer.pubkey()).await.unwrap();
        println!(
            "Custom payer balance: {} SOL",
            payer_balance as f64 / 1_000_000_000.0
        );

        let space = spl_token::state::Mint::LEN;

        let create_zbtc_account_ix = system_instruction::create_account(
            &DEPLOYER.pubkey(),
            &ZBTC_MINT,
            3000000,
            space as u64,
            &spl_token::id(),
        );

        let create_zusd_account_ix = system_instruction::create_account(
            &DEPLOYER.pubkey(),
            &ZUSD_MINT,
            3000000,
            space as u64,
            &spl_token::id(),
        );

        let create_zbtc_ix = spl_token::instruction::initialize_mint2(
            &spl_token::id(),
            &ZBTC_MINT,
            &DEPLOYER.pubkey(),
            None,
            9,
        )
        .unwrap();

        let create_zusd_ix = spl_token::instruction::initialize_mint2(
            &spl_token::id(),
            &ZUSD_MINT,
            &AUTHORITY,
            None,
            6,
        )
        .unwrap();

        let create_tokens_tx = Transaction::new_signed_with_payer(
            &[
                create_zbtc_account_ix,
                create_zbtc_ix,
                create_zusd_account_ix,
                create_zusd_ix,
            ],
            Some(&DEPLOYER.pubkey()),
            &[&*DEPLOYER, &ZBTC_MINT_KEYPAIR, &ZUSD_MINT_KEYPAIR],
            recent_blockhash,
        );

        banks_client
            .process_transaction(create_tokens_tx)
            .await
            .unwrap();

        let create_zbtc_vault_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &DEPLOYER.pubkey(),
                &AUTHORITY,
                &ZBTC_MINT,
                &spl_token::id(),
            );

        let create_zbtc_vault_tx = Transaction::new_signed_with_payer(
            &[create_zbtc_vault_ix],
            Some(&DEPLOYER.pubkey()),
            &[&DEPLOYER],
            recent_blockhash,
        );

        banks_client
            .process_transaction(create_zbtc_vault_tx)
            .await
            .unwrap();

        // Initialize lending state
        let init_lending_state_ix = create_init_lending_state_instruction(
            &PROGRAM_ID,
            &DEPLOYER.pubkey(),
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let init_lending_state_tx = Transaction::new_signed_with_payer(
            &[init_lending_state_ix],
            Some(&DEPLOYER.pubkey()),
            &[&DEPLOYER],
            recent_blockhash,
        );

        banks_client
            .process_transaction(init_lending_state_tx)
            .await
            .unwrap();

        // Initialize obligation PDA for the user
        let (obligation_pda, _) = find_obligation_pda(&payer.pubkey(), &PROGRAM_ID);
        let init_obligation_ix = create_init_obligation_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let init_obligation_tx = Transaction::new_signed_with_payer(
            &[init_obligation_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        banks_client
            .process_transaction(init_obligation_tx)
            .await
            .unwrap();
        println!("Initialized obligation PDA at: {}", obligation_pda);

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        // Create token accounts for the user
        let user_zbtc_account = get_associated_token_address(&payer.pubkey(), &ZBTC_MINT);
        let user_zusd_account = get_associated_token_address(&payer.pubkey(), &ZUSD_MINT);

        // ==================================================================================
        // Test Case 2: Deposit ZBTC
        // Purpose: Verify basic deposit functionality
        // Expected behavior:
        // - Successfully deposit 1 ZBTC as collateral
        // - Obligation state updated correctly
        // ==================================================================================
        println!("Testing deposit ZBTC instruction...");
        let deposit_amount: u64 = 1_000_000_000; // 1 ZBTC with 9 decimals

        // First, we need to create the user's ZBTC token account and fund it
        // This would normally be done by the user before interacting with our program
        // For testing purposes, we'll simulate this by creating the account and minting tokens

        // Create user's ZBTC token account
        let create_zbtc_account_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &payer.pubkey(),
                &payer.pubkey(),
                &ZBTC_MINT,
                &spl_token::id(),
            );

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let create_zbtc_account_tx = Transaction::new_signed_with_payer(
            &[create_zbtc_account_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        banks_client
            .process_transaction(create_zbtc_account_tx)
            .await
            .unwrap();
        println!(
            "Created user's ZBTC token account at: {}",
            user_zbtc_account
        );

        // Mint ZBTC tokens to user's account
        let mint_zbtc_ix = spl_token::instruction::mint_to(
            &spl_token::id(),
            &ZBTC_MINT,
            &user_zbtc_account,
            &DEPLOYER.pubkey(),
            &[],
            deposit_amount * 2, // Mint more than we need for testing
        )
        .expect("Failed to create mint to instruction");

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let mint_zbtc_tx = Transaction::new_signed_with_payer(
            &[mint_zbtc_ix],
            Some(&DEPLOYER.pubkey()),
            &[&DEPLOYER],
            recent_blockhash,
        );

        banks_client
            .process_transaction(mint_zbtc_tx)
            .await
            .unwrap();
        println!("Minted {} ZBTC to user's account", deposit_amount * 2);

        // Now deposit ZBTC to the lending protocol
        let deposit_zbtc_ix = create_deposit_zbtc_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
            deposit_amount,
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let deposit_zbtc_tx = Transaction::new_signed_with_payer(
            &[deposit_zbtc_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(deposit_zbtc_tx).await;
        match result {
            Ok(_) => {
                println!("Deposit ZBTC instruction executed successfully!");

                // Check obligation state
                verify_obligation_state(
                    &mut banks_client,
                    &obligation_pda,
                    deposit_amount,
                    0,
                    "deposit ZBTC",
                )
                .await;

                // Check user's ZBTC balance
                let token_account = banks_client
                    .get_account(user_zbtc_account)
                    .await
                    .unwrap()
                    .unwrap();
                let token_data = spl_token::state::Account::unpack(&token_account.data).unwrap();
                println!("User's ZBTC balance after deposit: {}", token_data.amount);
                assert_eq!(
                    token_data.amount, deposit_amount,
                    "User's ZBTC balance should be reduced by deposit amount"
                );

                // Check vault's ZBTC balance
                let vault_account = banks_client
                    .get_account(*ZBTC_VAULT)
                    .await
                    .unwrap()
                    .unwrap();
                let vault_data = spl_token::state::Account::unpack(&vault_account.data).unwrap();
                println!("Vault's ZBTC balance after deposit: {}", vault_data.amount);
                assert_eq!(
                    vault_data.amount, deposit_amount,
                    "Vault's ZBTC balance should be increased by deposit amount"
                );
            }
            Err(e) => {
                println!("Error executing deposit ZBTC instruction: {:?}", e);
                panic!("Deposit ZBTC instruction failed");
            }
        }

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        // ==================================================================================
        // Test Case 3: Borrow ZUSD
        // Purpose: Verify basic borrow functionality with token minting
        // Expected behavior:
        // - Successfully borrow 500 ZUSD
        // - Obligation state updated correctly
        // - ZUSD tokens minted to user's account
        // ==================================================================================
        println!("Testing borrow ZUSD instruction...");
        let borrow_amount: u64 = 500_000_000; // 500 ZUSD with 6 decimals

        // Create user's ZUSD token account
        let create_zusd_account_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &payer.pubkey(),
                &payer.pubkey(),
                &ZUSD_MINT,
                &spl_token::id(),
            );

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let create_zusd_account_tx = Transaction::new_signed_with_payer(
            &[create_zusd_account_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        banks_client
            .process_transaction(create_zusd_account_tx)
            .await
            .unwrap();
        println!(
            "Created user's ZUSD token account at: {}",
            user_zusd_account
        );

        // Now borrow ZUSD from the lending protocol
        let borrow_zusd_ix = create_borrow_zusd_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
            borrow_amount,
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let borrow_zusd_tx = Transaction::new_signed_with_payer(
            &[borrow_zusd_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(borrow_zusd_tx).await;
        match result {
            Ok(_) => {
                println!("Borrow ZUSD instruction executed successfully!");

                // Check obligation state
                verify_obligation_state(
                    &mut banks_client,
                    &obligation_pda,
                    deposit_amount,
                    borrow_amount,
                    "borrow ZUSD",
                )
                .await;

                // Check user's ZUSD balance
                let token_account = banks_client
                    .get_account(user_zusd_account)
                    .await
                    .unwrap()
                    .unwrap();
                let token_data = spl_token::state::Account::unpack(&token_account.data).unwrap();
                println!("User's ZUSD balance after borrow: {}", token_data.amount);
                assert_eq!(
                    token_data.amount, borrow_amount,
                    "User's ZUSD balance should be increased by borrow amount"
                );
            }
            Err(e) => {
                println!("Error executing borrow ZUSD instruction: {:?}", e);
                panic!("Borrow ZUSD instruction failed");
            }
        }

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        // ==================================================================================
        // Test Case 4: Repay ZUSD
        // Purpose: Verify basic repay functionality with token burning
        // Expected behavior:
        // - Successfully repay 200 ZUSD
        // - Obligation state updated correctly
        // - ZUSD tokens burned from user's account
        // ==================================================================================
        println!("Testing repay ZUSD instruction...");
        let repay_amount: u64 = 200_000_000; // 200 ZUSD with 6 decimals

        // First, we need to mint more ZUSD tokens to the user's account for repaying
        // This would normally be done by the user before interacting with our program
        // For testing purposes, we'll simulate this by minting tokens

        // Now repay ZUSD to the lending protocol
        let repay_zusd_ix = create_repay_zusd_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
            repay_amount,
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let repay_zusd_tx = Transaction::new_signed_with_payer(
            &[repay_zusd_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(repay_zusd_tx).await;
        match result {
            Ok(_) => {
                println!("Repay ZUSD instruction executed successfully!");

                // Check obligation state
                verify_obligation_state(
                    &mut banks_client,
                    &obligation_pda,
                    deposit_amount,
                    borrow_amount - repay_amount,
                    "repay ZUSD",
                )
                .await;

                // Check user's ZUSD balance
                let token_account = banks_client
                    .get_account(user_zusd_account)
                    .await
                    .unwrap()
                    .unwrap();
                let token_data = spl_token::state::Account::unpack(&token_account.data).unwrap();
                println!("User's ZUSD balance after repay: {}", token_data.amount);
                assert_eq!(
                    token_data.amount,
                    borrow_amount - repay_amount,
                    "User's ZUSD balance should remain the same after repaying"
                );
            }
            Err(e) => {
                println!("Error executing repay ZUSD instruction: {:?}", e);
                panic!("Repay ZUSD instruction failed");
            }
        }

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        // ==================================================================================
        // Test Case 5: Withdraw ZBTC
        // Purpose: Verify basic withdraw functionality
        // Expected behavior:
        // - Successfully withdraw 0.5 ZBTC
        // - Obligation state updated correctly
        // ==================================================================================
        println!("Testing withdraw ZBTC instruction...");
        let withdraw_amount: u64 = 500_000_000; // 0.5 ZBTC with 9 decimals

        // Now withdraw ZBTC from the lending protocol
        let withdraw_zbtc_ix = create_withdraw_zbtc_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
            withdraw_amount,
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let withdraw_zbtc_tx = Transaction::new_signed_with_payer(
            &[withdraw_zbtc_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(withdraw_zbtc_tx).await;
        match result {
            Ok(_) => {
                println!("Withdraw ZBTC instruction executed successfully!");

                // Check obligation state
                verify_obligation_state(
                    &mut banks_client,
                    &obligation_pda,
                    deposit_amount - withdraw_amount,
                    borrow_amount - repay_amount,
                    "withdraw ZBTC",
                )
                .await;

                // Check user's ZBTC balance
                let token_account = banks_client
                    .get_account(user_zbtc_account)
                    .await
                    .unwrap()
                    .unwrap();
                let token_data = spl_token::state::Account::unpack(&token_account.data).unwrap();
                println!("User's ZBTC balance after withdraw: {}", token_data.amount);
                assert_eq!(
                    token_data.amount,
                    deposit_amount + withdraw_amount,
                    "User's ZBTC balance should be increased by withdraw amount"
                );

                // Check vault's ZBTC balance
                let vault_account = banks_client
                    .get_account(*ZBTC_VAULT)
                    .await
                    .unwrap()
                    .unwrap();
                let vault_data = spl_token::state::Account::unpack(&vault_account.data).unwrap();
                println!("Vault's ZBTC balance after withdraw: {}", vault_data.amount);
                assert_eq!(
                    vault_data.amount,
                    deposit_amount - withdraw_amount,
                    "Vault's ZBTC balance should be decreased by withdraw amount"
                );
            }
            Err(e) => {
                println!("Error executing withdraw ZBTC instruction: {:?}", e);
                panic!("Withdraw ZBTC instruction failed");
            }
        }

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        // ==================================================================================
        // Test Case 6: Try to withdraw too much ZBTC
        // Purpose: Verify that withdrawing too much ZBTC fails due to LTV ratio
        // Expected behavior:
        // - Attempt to withdraw more ZBTC than allowed by LTV ratio fails
        // ==================================================================================
        println!("Testing withdraw too much ZBTC instruction...");
        let withdraw_too_much_amount: u64 = 1_000_000_000; // 1 ZBTC with 9 decimals (more than we have)

        // Now try to withdraw too much ZBTC from the lending protocol
        let withdraw_too_much_zbtc_ix = create_withdraw_zbtc_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
            withdraw_too_much_amount,
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let withdraw_too_much_zbtc_tx = Transaction::new_signed_with_payer(
            &[withdraw_too_much_zbtc_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let result = banks_client
            .process_transaction(withdraw_too_much_zbtc_tx)
            .await;
        match result {
            Ok(_) => {
                println!(
                    "Withdraw too much ZBTC instruction executed successfully, but it should have failed!"
                );
                panic!("Withdraw too much ZBTC instruction should have failed");
            }
            Err(e) => {
                println!(
                    "Withdraw too much ZBTC instruction failed as expected: {:?}",
                    e
                );
            }
        }

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        // ==================================================================================
        // Test Case 7: Try to borrow too much ZUSD
        // Purpose: Verify that borrowing too much ZUSD fails due to LTV ratio
        // Expected behavior:
        // - Attempt to borrow more ZUSD than allowed by LTV ratio fails
        // ==================================================================================
        println!("Testing borrow too much ZUSD instruction...");
        let borrow_too_much_amount: u64 = 24_701_000_000; // 24701 ZUSD with 6 decimals (more than allowed by LTV)

        // Now try to borrow too much ZUSD from the lending protocol
        let borrow_too_much_zusd_ix = create_borrow_zusd_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
            borrow_too_much_amount,
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let borrow_too_much_zusd_tx = Transaction::new_signed_with_payer(
            &[borrow_too_much_zusd_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let result = banks_client
            .process_transaction(borrow_too_much_zusd_tx)
            .await;
        match result {
            Ok(_) => {
                println!(
                    "Borrow too much ZUSD instruction executed successfully, but it should have failed!"
                );
                panic!("Borrow too much ZUSD instruction should have failed");
            }
            Err(e) => {
                println!(
                    "Borrow too much ZUSD instruction failed as expected: {:?}",
                    e
                );
            }
        }

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        // ==================================================================================
        // Test Case 8: Try to repay more than borrowed
        // Purpose: Verify that repaying more than borrowed fails
        // Expected behavior:
        // - Attempt to repay more ZUSD than borrowed fails
        // ==================================================================================
        println!("Testing repay more than borrowed instruction...");
        let repay_too_much_amount: u64 = 1_000_000_000; // 1000 ZUSD with 6 decimals (more than we borrowed)

        // Now try to repay more than borrowed ZUSD to the lending protocol
        let repay_too_much_zusd_ix = create_repay_zusd_instruction(
            &PROGRAM_ID,
            &payer.pubkey(),
            repay_too_much_amount,
        )
        .await;

        let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();
        let repay_too_much_zusd_tx = Transaction::new_signed_with_payer(
            &[repay_too_much_zusd_ix],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let result = banks_client
            .process_transaction(repay_too_much_zusd_tx)
            .await;
        match result {
            Ok(_) => {
                println!(
                    "Repay too much ZUSD instruction executed successfully, but it should have failed!"
                );
                panic!("Repay too much ZUSD instruction should have failed");
            }
            Err(e) => {
                println!(
                    "Repay too much ZUSD instruction failed as expected: {:?}",
                    e
                );
            }
        }

        stat_obligation(&mut banks_client, &payer.pubkey()).await;

        println!("All lending protocol tests completed successfully!");

    }

    #[tokio::test]
    async fn test_get_associated_token_address() {
        let a = get_associated_token_address(
            &Pubkey::from_str(&"69DPEf311TfFgHzgSukT8hVNtxAgxjMyxQXnUEbqCbeQ").unwrap(),
            &spl_token::native_mint::ID,
        );
        println!("{}", a);
    }
}
