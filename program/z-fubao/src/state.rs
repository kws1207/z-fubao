use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Obligation {
    pub owner: Pubkey,
    pub zbtc_deposit: u64,
    pub zusd_borrowed: u64,
}

impl Obligation {
    pub const LEN: usize = 32 + // owner
        8 + // zbtc_deposit
        8; // zusd_borrowed
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LendingState {
    pub authority: Pubkey,
    pub zbtc_mint: Pubkey,
    pub zusd_mint: Pubkey,
    pub zbtc_vault: Pubkey,
    pub ltv_ratio: u8, // in basis points (e.g., 7500 = 75%)
    pub price: u64,    // zBTC price in USD (e.g., 50000 = $50,000)
    pub bump: u8,
}

impl LendingState {
    pub const LEN: usize = 32 + // authority
        32 + // zbtc_mint
        32 + // zusd_mint
        32 + // zbtc_vault
        1 + // ltv_ratio
        8 + // price
        1; // bump
}
