use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use std::ops::Div;

use crate::{
    instructions::ZFubaoInstruction,
    state::{
        AUTHORITY_SEED, GLOBAL_CONFIG_SEED, OBLIGATION_SEED, Obligation, ZFubaoConfig,
        find_obligation_pda,
    },
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ZFubaoInstruction::unpack(instruction_data)?;

        match instruction {
            ZFubaoInstruction::Initialize { ltv_ratio, price } => {
                msg!("Instruction: Initialize");
                Self::process_initialize(program_id, accounts, ltv_ratio, price)
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
            ZFubaoInstruction::RefreshPrice => {
                msg!("Instruction: RefreshPrice");
                Self::process_refresh_price(program_id, accounts)
            }
            ZFubaoInstruction::Unstake { amount } => {
                msg!("Instruction: Unstake");
                Self::process_unstake(program_id, accounts, amount)
            }
        }
    }

    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        ltv_ratio: u8,
        price: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let owner = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let global_config_acount = next_account_info(account_info_iter)?;
        let zbtc_mint = next_account_info(account_info_iter)?;
        let zusd_mint = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // Check signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let (authority_pda, authority_bump) =
            Pubkey::find_program_address(&[AUTHORITY_SEED], program_id);
        if *authority_account.key != authority_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let (global_config_pda, global_config_bump) =
            Pubkey::find_program_address(&[GLOBAL_CONFIG_SEED], program_id);
        if *global_config_acount.key != global_config_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let rent = Rent::get()?;

        if authority_account.data_is_empty() {
            msg!("Create authority account");

            let space = 0;
            let lamports = rent.minimum_balance(space);
            invoke_signed(
                &system_instruction::create_account(
                    &owner.key,
                    &authority_account.key,
                    lamports,
                    space as u64,
                    program_id,
                ),
                &[owner.clone(), authority_account.clone()],
                &[&[AUTHORITY_SEED, &[authority_bump]]],
            )?;
        }

        // Check program ownership
        if global_config_acount.data_is_empty() {
            msg!("Create global config account");

            let space = ZFubaoConfig::LEN;
            let lamports = rent.minimum_balance(space);

            invoke_signed(
                &system_instruction::create_account(
                    &owner.key,
                    &global_config_acount.key,
                    lamports,
                    space as u64,
                    program_id,
                ),
                &[owner.clone(), global_config_acount.clone()],
                &[&[GLOBAL_CONFIG_SEED, &[global_config_bump]]],
            )?;
        }

        // Initialize lending state data
        let zfubao_config = ZFubaoConfig {
            authority: *authority_account.key,

            zbtc_mint: *zbtc_mint.key,
            zusd_mint: *zusd_mint.key,

            ltv_ratio,
            price,

            authority_bump,
            global_config_bump,

            start_time: Clock::get()?.unix_timestamp,
            szusd_price_ratio: 10000,
        };

        zfubao_config.serialize(&mut *global_config_acount.data.borrow_mut())?;

        msg!("Lending state initialized");
        Ok(())
    }

    fn process_init_obligation(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user = next_account_info(account_info_iter)?;
        let authority_account = next_account_info(account_info_iter)?;
        let global_config_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Derive PDA for obligation
        let (pda, bump_seed) =
            Pubkey::find_program_address(&[OBLIGATION_SEED, user.key.as_ref()], program_id);

        // Verify obligation account is the PDA
        if *obligation_account.key != pda {
            return Err(ProgramError::InvalidArgument);
        }

        if !obligation_account.data_is_empty() {
            return Err(ProgramError::InvalidAccountData);
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
                &[&[OBLIGATION_SEED, user.key.as_ref(), &[bump_seed]]],
            )?;
        }

        // Initialize obligation data
        let obligation = Obligation {
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
        let global_config_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zbtc_account = next_account_info(account_info_iter)?;
        let vault_zbtc_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if find_obligation_pda(&user.key, program_id).0 != *obligation_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
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
        let global_config_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zbtc_account = next_account_info(account_info_iter)?;
        let vault_zbtc_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if find_obligation_pda(&user.key, program_id).0 != *obligation_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Load lending state
        let global_config = ZFubaoConfig::try_from_slice(&global_config_account.data.borrow())?;

        // Verify lending state account
        if global_config_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if withdrawal would make the position under-collateralized
        let max_withdrawable = Self::calculate_max_withdrawable(&obligation, &global_config)?;

        if amount > max_withdrawable {
            return Err(ProgramError::InvalidArgument);
        }

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
            &[&[AUTHORITY_SEED, &[global_config.authority_bump]]],
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
        let global_config_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zusd_account = next_account_info(account_info_iter)?;
        let zusd_mint = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if find_obligation_pda(&user.key, program_id).0 != *obligation_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Load lending state
        let global_config = ZFubaoConfig::try_from_slice(&global_config_account.data.borrow())?;

        // Verify lending state account
        if global_config_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Calculate maximum borrowable amount
        let max_borrowable = Self::calculate_max_borrowable(&obligation, &global_config)?;

        msg!("max borrowable: {}", max_borrowable);

        // Check if borrow amount is within limits
        if amount > max_borrowable {
            return Err(ProgramError::InvalidArgument);
        }

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
            &[&[AUTHORITY_SEED, &[global_config.authority_bump]]],
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
        let global_config_account = next_account_info(account_info_iter)?;
        let obligation_account = next_account_info(account_info_iter)?;
        let user_zusd_account = next_account_info(account_info_iter)?;
        let zusd_mint = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;

        // Check signer
        if !user.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if find_obligation_pda(&user.key, program_id).0 != *obligation_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        // Load global config
        let global_config = ZFubaoConfig::try_from_slice(&global_config_account.data.borrow())?;

        // Load obligation data
        let mut obligation = Obligation::try_from_slice(&obligation_account.data.borrow())?;

        // Verify obligation account
        if obligation_account.owner != program_id {
            return Err(ProgramError::InvalidAccountData);
        }

        // Check if repay amount is valid
        if amount > obligation.zusd_borrowed {
            return Err(ProgramError::InvalidArgument);
        }

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
            &[&[AUTHORITY_SEED, &[global_config.authority_bump]]],
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
        let authority_account = next_account_info(accounts_iter)?;
        let global_config_account = next_account_info(accounts_iter)?;
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

        // Load global config
        let global_config = ZFubaoConfig::try_from_slice(&global_config_account.data.borrow())?;

        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                user_zusd_account.key,
                staking_vault.key,
                user_account.key,
                &[],
                amount,
            )?,
            &[
                token_program.clone(),
                user_zusd_account.clone(),
                staking_vault.clone(),
                user_account.clone(),
            ],
        )?;

        let adjusted_amount = amount
            .checked_mul(global_config.szusd_price_ratio)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(10000)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Only the program can mint SZUSD
        invoke_signed(
            &spl_token::instruction::mint_to(
                token_program.key,
                szusd_mint.key,
                user_szusd_account.key,
                &authority_account.key,
                &[],
                adjusted_amount,
            )?,
            &[
                szusd_mint.clone(),
                user_szusd_account.clone(),
                token_program.clone(),
                authority_account.clone(),
            ],
            &[&[AUTHORITY_SEED, &[global_config.authority_bump]]],
        )?;

        msg!(
            "Successfully staked {} ZUSD and minted {} SZUSD",
            amount,
            amount
        );
        Ok(())
    }

    fn process_refresh_price(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let authority_account = next_account_info(account_info_iter)?;
        let global_config_account = next_account_info(account_info_iter)?;

        let mut global_config = ZFubaoConfig::try_from_slice(&global_config_account.data.borrow())?;

        let current_time = Clock::get()?.unix_timestamp;
        let time_elapsed = current_time
            .checked_sub(global_config.start_time)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let seconds_elapsed = time_elapsed
            .checked_div(1000)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        let new_ratio = global_config
            .szusd_price_ratio
            .checked_add(seconds_elapsed as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        global_config.szusd_price_ratio = new_ratio;

        global_config.serialize(&mut *global_config_account.data.borrow_mut())?;

        msg!("Price refreshed");
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
        let authority_account = next_account_info(accounts_iter)?;
        let global_config_account = next_account_info(accounts_iter)?;
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

        // Load global config
        let global_config = ZFubaoConfig::try_from_slice(&global_config_account.data.borrow())?;

        invoke(
            &spl_token::instruction::burn(
                token_program.key,
                user_szusd_account.key,
                szusd_mint.key,
                user_account.key,
                &[],
                amount,
            )?,
            &[
                user_szusd_account.clone(),
                szusd_mint.clone(),
                user_account.clone(),
                token_program.clone(),
            ],
        )?;

        let current_szusd_price = global_config.get_current_szusd_price_in_zusd();
        let amount_in_zusd = amount
            .checked_mul(current_szusd_price)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        invoke_signed(
            &spl_token::instruction::transfer(
                token_program.key,
                staking_vault.key,
                user_zusd_account.key,
                &authority_account.key,
                &[],
                amount_in_zusd,
            )?,
            &[
                token_program.clone(),
                staking_vault.clone(),
                user_zusd_account.clone(),
                authority_account.clone(),
            ],
            &[&[AUTHORITY_SEED, &[global_config.authority_bump]]],
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
        global_config: &ZFubaoConfig,
    ) -> Result<u64, ProgramError> {
        // Calculate collateral value in USD
        let collateral_value = obligation
            .zbtc_deposit
            .checked_mul(global_config.price)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .div(1_000); // Decimal precision adjustment

        // Calculate maximum borrowable amount based on LTV ratio
        let max_borrowable = collateral_value
            .checked_mul(global_config.ltv_ratio as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(100)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(max_borrowable - obligation.zusd_borrowed)
    }

    // Helper function to calculate maximum withdrawable amount
    pub fn calculate_max_withdrawable(
        obligation: &Obligation,
        global_config: &ZFubaoConfig,
    ) -> Result<u64, ProgramError> {
        // Calculate collateral value in USD
        let collateral_value = obligation
            .zbtc_deposit
            .checked_mul(global_config.price)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Calculate minimum required collateral value based on borrowed amount and LTV ratio
        let min_collateral_value = obligation
            .zusd_borrowed
            .checked_mul(100)
            .ok_or(ProgramError::ArithmeticOverflow)?
            .checked_div(global_config.ltv_ratio as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Calculate maximum withdrawable collateral value
        let max_withdrawable_value = collateral_value
            .checked_sub(min_collateral_value)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Convert back to ZBTC
        let max_withdrawable = max_withdrawable_value
            .checked_div(global_config.price)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(max_withdrawable)
    }
}
