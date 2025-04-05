use std::ops::Div;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token;

use crate::{
    instructions::ZFubaoInstruction,
    state::{LendingState, Obligation},
};

pub struct Processor;

impl Processor {
    fn unpack_token_account(
        account_info: &AccountInfo,
        token_program_id: &Pubkey,
    ) -> Result<spl_token::state::Account, ProgramError> {
        if account_info.owner != token_program_id {
            Err(ProgramError::InvalidAccountData)
        } else {
            spl_token::state::Account::unpack(&account_info.data.borrow())
                .map_err(|_| ProgramError::InvalidAccountData)
        }
    }

    fn validate_ata_address<'a>(
        user_info: &AccountInfo<'a>,
        token_mint_info: &AccountInfo<'a>,
        user_token_account_info: &AccountInfo<'a>,
    ) -> ProgramResult {
        let expected_token_account =
            get_associated_token_address(user_info.key, token_mint_info.key);
        msg!(
            "Expected token account: {}, Actual token account: {}",
            expected_token_account,
            user_token_account_info.key
        );
        if expected_token_account != *user_token_account_info.key {
            return Err(ProgramError::InvalidArgument);
        }

        Ok(())
    }

    fn validate_and_create_ata_if_not_exists<'a>(
        user_info: &AccountInfo<'a>,
        token_mint_info: &AccountInfo<'a>,
        user_token_account_info: &AccountInfo<'a>,
        system_program_info: &AccountInfo<'a>,
        token_program_info: &AccountInfo<'a>,
        associated_token_program_info: &AccountInfo<'a>,
    ) -> Result<bool, ProgramError> {
        Self::validate_ata_address(user_info, token_mint_info, user_token_account_info)?;
        let didnt_exist = user_token_account_info.data_is_empty();
        if didnt_exist {
            invoke(
                &spl_associated_token_account::instruction::create_associated_token_account(
                    user_info.key,
                    user_info.key,
                    token_mint_info.key,
                    &spl_token::id(),
                ),
                &[
                    user_info.clone(),
                    user_token_account_info.clone(),
                    token_mint_info.clone(),
                    system_program_info.clone(),
                    token_program_info.clone(),
                    associated_token_program_info.clone(),
                ],
            )?;
        }
        Ok(didnt_exist) // created
    }

    fn prepare_wsol_account_with_balance<'a>(
        user_info: &AccountInfo<'a>,
        wsol_mint_info: &AccountInfo<'a>,
        user_wsol_account_info: &AccountInfo<'a>,
        sol_amount: u64,
        system_program_info: &AccountInfo<'a>,
        token_program_info: &AccountInfo<'a>,
        associated_token_program_info: &AccountInfo<'a>,
    ) -> ProgramResult {
        let created = Self::validate_and_create_ata_if_not_exists(
            user_info,
            wsol_mint_info,
            user_wsol_account_info,
            system_program_info,
            token_program_info,
            associated_token_program_info,
        )?;

        let transfer_amount = if created {
            sol_amount
        } else {
            sol_amount.saturating_sub(
                Self::unpack_token_account(user_wsol_account_info, &spl_token::id())?.amount,
            )
        };

        if transfer_amount > 0 {
            invoke(
                &system_instruction::transfer(
                    user_info.key,
                    user_wsol_account_info.key,
                    transfer_amount,
                ),
                &[user_info.clone(), user_wsol_account_info.clone()],
            )?;
            invoke(
                &spl_token::instruction::sync_native(&spl_token::id(), user_wsol_account_info.key)?,
                &[user_wsol_account_info.clone()],
            )?;
        }

        Ok(())
    }

    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ZFubaoInstruction::unpack(instruction_data)?;

        match instruction {
            ZFubaoInstruction::Initialize => {
                msg!("Instruction: Initialize");
                Self::process_initialize(program_id, accounts)
            }
            ZFubaoInstruction::InitObligation => {
                msg!("Instruction: InitObligation");
                Self::process_init_obligation(program_id, accounts)
            }
            ZFubaoInstruction::DepositZBTC { amount } => {
                msg!("Instruction: DepositZBTC");
                Self::process_deposit_zbtc(program_id, accounts, amount)
            }
            ZFubaoInstruction::WithdrawZBTC { amount } => {
                msg!("Instruction: WithdrawZBTC");
                Self::process_withdraw_zbtc(program_id, accounts, amount)
            }
            ZFubaoInstruction::BorrowZUSD { amount } => {
                msg!("Instruction: BorrowZUSD");
                Self::process_borrow_zusd(program_id, accounts, amount)
            }
            ZFubaoInstruction::RepayZUSD { amount } => {
                msg!("Instruction: RepayZUSD");
                Self::process_repay_zusd(program_id, accounts, amount)
            }
            ZFubaoInstruction::Stake { amount } => {
                msg!("Instruction: Stake");
                Self::process_stake(program_id, accounts, amount)
            }
            ZFubaoInstruction::Unstake { amount } => {
                msg!("Instruction: Unstake");
                Self::process_unstake(program_id, accounts, amount)
            }
        }
    }

    fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let owner = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let lending_state_account = next_account_info(account_info_iter)?;
        let zbtc_mint = next_account_info(account_info_iter)?;
        let zusd_mint = next_account_info(account_info_iter)?;
        let zbtc_vault = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let associated_token_program = next_account_info(account_info_iter)?;
        // Check signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if authority_account.data_is_empty() {
            msg!("Create authority account");

            let rent = Rent::get()?;
            let space = 0;
            let lamports = rent.minimum_balance(space);

            let (pda, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);

            if *authority_account.key != pda {
                return Err(ProgramError::InvalidAccountData);
            }

            invoke_signed(
                &system_instruction::create_account(
                    &owner.key,
                    &authority_account.key,
                    lamports,
                    space as u64,
                    program_id,
                ),
                &[owner.clone(), authority_account.clone()],
                &[&[b"authority", &[bump_seed]]],
            )?;
        }

        // Check program ownership
        if lending_state_account.data_is_empty() {
            msg!("Create lending state account");

            let rent = Rent::get()?;
            let space = LendingState::LEN;
            let lamports = rent.minimum_balance(space);

            let (pda, bump_seed) = Pubkey::find_program_address(&[b"lending_state"], program_id);

            if *lending_state_account.key != pda {
                return Err(ProgramError::InvalidAccountData);
            }

            invoke_signed(
                &system_instruction::create_account(
                    &owner.key,
                    &lending_state_account.key,
                    lamports,
                    space as u64,
                    program_id,
                ),
                &[owner.clone(), lending_state_account.clone()],
                &[&[b"lending_state", &[bump_seed]]],
            )?;
        }

        // Initialize lending state data
        let lending_state = LendingState {
            authority: *authority_account.key,
            zbtc_mint: *zbtc_mint.key,
            zusd_mint: *zusd_mint.key,
            zbtc_vault: *zbtc_vault.key,
            ltv_ratio: 70, // 70% LTV
            price: 50000,  // $50,000 per BTC
            bump: 0,       // Will be set when PDA is derived
        };

        lending_state.serialize(&mut *lending_state_account.data.borrow_mut())?;

        msg!("Lending state initialized");
        Ok(())
    }

    fn process_init_obligation(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Derive PDA for obligation
        let (pda, bump_seed) =
            Pubkey::find_program_address(&[b"obligation", user.key.as_ref()], program_id);

        // Verify obligation account is the PDA
        if *obligation_account.key != pda {
            return Err(ProgramError::InvalidArgument);
        }

        // Check program ownership
        if obligation_account.owner != program_id {
            msg!("Create obligation account");

            let rent = Rent::get()?;
            let space = Obligation::LEN;
            let lamports = rent.minimum_balance(space);

            invoke_signed(
                &system_instruction::create_account(
                    user.key,
                    obligation_account.key,
                    lamports,
                    space as u64,
                    program_id,
                ),
                &[
                    user.clone(),
                    obligation_account.clone(),
                    system_program.clone(),
                ],
                &[&[b"obligation", user.key.as_ref(), &[bump_seed]]],
            )?;
        }

        // Initialize obligation data
        let obligation = Obligation {
            owner: *user.key,
            zbtc_deposit: 0,
            zusd_borrowed: 0,
        };

        obligation.serialize(&mut *obligation_account.data.borrow_mut())?;

        msg!("Obligation initialized for user {}", user.key);
        Ok(())
    }

    fn process_deposit_zbtc(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zbtc_account = next_account_info(account_info_iter)?;
        let vault_zbtc_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Verify obligation owner
        if obligation.owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        // Transfer ZBTC from user to vault
        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                user_zbtc_account.key,
                vault_zbtc_account.key,
                user.key,
                &[],
                amount,
            )?,
            &[
                user_zbtc_account.clone(),
                vault_zbtc_account.clone(),
                user.clone(),
                token_program.clone(),
            ],
        )?;

        // Update obligation state
        obligation.zbtc_deposit = obligation
            .zbtc_deposit
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Save updated obligation data
        obligation.serialize(&mut *obligation_account.data.borrow_mut())?;

        msg!("Deposited {} ZBTC", amount);
        Ok(())
    }

    fn process_withdraw_zbtc(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zbtc_account = next_account_info(account_info_iter)?;
        let vault_zbtc_account = next_account_info(account_info_iter)?;
        let lending_state_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Verify obligation owner
        if obligation.owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        // Load lending state
        let lending_state = LendingState::try_from_slice(&lending_state_account.data.borrow())?;

        // Verify lending state account
        if lending_state_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if withdrawal would make the position under-collateralized
        let max_withdrawable = Self::calculate_max_withdrawable(
            &obligation,
            lending_state.price,
            lending_state.ltv_ratio,
        )?;

        if amount > max_withdrawable {
            return Err(ProgramError::InvalidArgument);
        }

        let (_, auth_bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);
        // Transfer ZBTC from vault to user
        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                vault_zbtc_account.key,
                user_zbtc_account.key,
                &authority_account.key,
                &[],
                amount,
            )?,
            &[
                vault_zbtc_account.clone(),
                user_zbtc_account.clone(),
                authority_account.clone(),
                token_program.clone(),
            ],
            &[&[b"authority", &[auth_bump_seed]]],
        )?;

        // Update obligation state
        obligation.zbtc_deposit = obligation
            .zbtc_deposit
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Save updated obligation data
        obligation.serialize(&mut *obligation_account.data.borrow_mut())?;

        msg!("Withdrawn {} ZBTC", amount);
        Ok(())
    }

    fn process_borrow_zusd(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zusd_account = next_account_info(account_info_iter)?;
        let zusd_mint = next_account_info(account_info_iter)?;
        let lending_state_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Verify obligation owner
        if obligation.owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        // Load lending state
        let lending_state = LendingState::try_from_slice(&lending_state_account.data.borrow())?;

        // Verify lending state account
        if lending_state_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Calculate maximum borrowable amount
        let max_borrowable = Self::calculate_max_borrowable(
            &obligation,
            lending_state.price,
            lending_state.ltv_ratio,
        )?;

        msg!("max borrowable: {}", max_borrowable);

        // Check if borrow amount is within limits
        if amount > max_borrowable {
            return Err(ProgramError::InvalidArgument);
        }

        let (pda, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);

        // Mint ZUSD tokens to user's account
        invoke_signed(
            &spl_token::instruction::mint_to(
                token_program.key,
                zusd_mint.key,
                user_zusd_account.key,
                &authority_account.key,
                &[],
                amount,
            )?,
            &[
                zusd_mint.clone(),
                user_zusd_account.clone(),
                token_program.clone(),
                authority_account.clone(),
            ],
            &[&[b"authority", &[bump_seed]]],
        )?;

        // Update obligation state
        obligation.zusd_borrowed = obligation
            .zusd_borrowed
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Save updated obligation data
        obligation.serialize(&mut *obligation_account.data.borrow_mut())?;

        msg!("Borrowed {} ZUSD", amount);
        Ok(())
    }

    fn process_repay_zusd(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zusd_account = next_account_info(account_info_iter)?;
        let zusd_mint = next_account_info(account_info_iter)?;
        let lending_state_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        msg!("user: {}", user.key);
        msg!("authority_account: {}", authority_account.key);
        msg!("obligation_account: {}", obligation_account.key);
        msg!("user_zusd_account: {}", user_zusd_account.key);
        msg!("zusd_mint: {}", zusd_mint.key);
        msg!("lending_state_account: {}", lending_state_account.key);
        msg!("token_program: {}", token_program.key);

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Verify obligation owner
        if obligation.owner != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        // Load lending state
        let lending_state = LendingState::try_from_slice(&lending_state_account.data.borrow())?;

        // Verify lending state account
        if lending_state_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if repay amount is valid
        if amount > obligation.zusd_borrowed {
            return Err(ProgramError::InvalidArgument);
        }

        let (_, auth_bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);

        // Burn the ZUSD tokens
        invoke_signed(
            &spl_token::instruction::burn(
                token_program.key,
                user_zusd_account.key,
                zusd_mint.key,
                &user.key,
                &[],
                amount,
            )?,
            &[
                user_zusd_account.clone(),
                zusd_mint.clone(),
                user.clone(),
                token_program.clone(),
            ],
            &[&[b"authority", &[auth_bump_seed]]],
        )?;

        // Update obligation state
        obligation.zusd_borrowed = obligation
            .zusd_borrowed
            .checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Save updated obligation data
        obligation.serialize(&mut *obligation_account.data.borrow_mut())?;

        msg!("Repaid {} ZUSD", amount);
        Ok(())
    }

    fn process_stake(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_account = next_account_info(accounts_iter)?;
        let user_zusd_account = next_account_info(accounts_iter)?;
        let user_szusd_account = next_account_info(accounts_iter)?;
        let zusd_mint = next_account_info(accounts_iter)?;
        let szusd_mint = next_account_info(accounts_iter)?;
        let staking_vault = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Transfer ZUSD from user to staking vault
        let transfer_instruction = spl_token::instruction::transfer(
            token_program.key,
            user_zusd_account.key,
            staking_vault.key,
            user_account.key,
            &[],
            amount,
        )?;

        invoke(
            &transfer_instruction,
            &[
                token_program.clone(),
                user_zusd_account.clone(),
                staking_vault.clone(),
                user_account.clone(),
            ],
        )?;

        // Mint SZUSD to the user
        let mint_instruction = spl_token::instruction::mint_to(
            token_program.key,
            szusd_mint.key,
            user_szusd_account.key,
            &program_id, // Authority is the program
            &[],
            amount,
        )?;

        // To mint tokens, we need to sign with the program
        let (pda, bump_seed) = Pubkey::find_program_address(&[b"mint_authority"], program_id);

        // Only the program can mint SZUSD
        invoke_signed(
            &mint_instruction,
            &[
                token_program.clone(),
                szusd_mint.clone(),
                user_szusd_account.clone(),
                user_account.clone(),
            ],
            &[&[b"mint_authority", &[bump_seed]]],
        )?;

        msg!(
            "Successfully staked {} ZUSD and minted {} SZUSD",
            amount,
            amount
        );
        Ok(())
    }

    // Process unstake instruction
    fn process_unstake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_account = next_account_info(accounts_iter)?;
        let user_zusd_account = next_account_info(accounts_iter)?;
        let user_szusd_account = next_account_info(accounts_iter)?;
        let zusd_mint = next_account_info(accounts_iter)?;
        let szusd_mint = next_account_info(accounts_iter)?;
        let staking_vault = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Burn SZUSD tokens from user
        let burn_instruction = spl_token::instruction::burn(
            token_program.key,
            user_szusd_account.key,
            szusd_mint.key,
            user_account.key,
            &[],
            amount,
        )?;

        invoke(
            &burn_instruction,
            &[
                token_program.clone(),
                user_szusd_account.clone(),
                szusd_mint.clone(),
                user_account.clone(),
            ],
        )?;

        // Transfer ZUSD from staking vault to user
        let (pda, bump_seed) = Pubkey::find_program_address(&[b"vault_authority"], program_id);

        let transfer_instruction = spl_token::instruction::transfer(
            token_program.key,
            staking_vault.key,
            user_zusd_account.key,
            &pda, // Authority is the program PDA
            &[],
            amount,
        )?;

        invoke_signed(
            &transfer_instruction,
            &[
                token_program.clone(),
                staking_vault.clone(),
                user_zusd_account.clone(),
            ],
            &[&[b"vault_authority", &[bump_seed]]],
        )?;

        msg!(
            "Successfully unstaked {} SZUSD and returned {} ZUSD",
            amount,
            amount
        );
        Ok(())
    }

    // Helper function to calculate maximum borrowable amount
    pub fn calculate_max_borrowable(
        obligation: &Obligation,
        price: u64,
        ltv_ratio: u8,
    ) -> Result<u64, ProgramError> {
        // Calculate collateral value in USD
        let collateral_value = obligation.zbtc_deposit
            .checked_mul(price)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .div(1_000); // Decimal precision adjustment

        // Calculate maximum borrowable amount based on LTV ratio
        let max_borrowable = collateral_value
            .checked_mul(ltv_ratio as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(max_borrowable - obligation.zusd_borrowed)
    }

    // Helper function to calculate maximum withdrawable amount
    pub fn calculate_max_withdrawable(
        obligation: &Obligation,
        price: u64,
        ltv_ratio: u8,
    ) -> Result<u64, ProgramError> {
        // Calculate collateral value in USD
        let collateral_value = obligation.zbtc_deposit
            .checked_mul(price)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Calculate minimum required collateral value based on borrowed amount and LTV ratio
        let min_collateral_value = obligation.zusd_borrowed
            .checked_mul(100)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(ltv_ratio as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Calculate maximum withdrawable collateral value
        let max_withdrawable_value = collateral_value
            .checked_sub(min_collateral_value)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Convert back to ZBTC
        let max_withdrawable = max_withdrawable_value
            .checked_div(price)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(max_withdrawable)
    }
}
