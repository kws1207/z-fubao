use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const AUTHORITY_SEED: &[u8] = b"authority";

pub const GLOBAL_CONFIG_SEED: &[u8] = b"global_config";
pub const OBLIGATION_SEED: &[u8] = b"obligation";
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ZFubaoConfig {
    // general
    //// account
    pub authority: Pubkey,

    //// mint
    pub zbtc_mint: Pubkey,
    pub zusd_mint: Pubkey,

    //// bump seed
    pub authority_bump: u8,
    pub global_config_bump: u8,

    // lending
    pub ltv_ratio: u8, // in basis points (e.g., 7500 = 75%)
    pub price: u64,    // zBTC price in USD (e.g., 50000 = $50,000)

    // staking
    pub start_time: i64,
    pub szusd_price_ratio: u64, // 1 -> 0.01%
}

impl ZFubaoConfig {
    pub const LEN: usize = 32 + // authority
        32 + // zbtc_mint
        32 + // zusd_mint
        1 + // authority_bump
        1 + // global_config_bump
        1 + // ltv_ratio
        8 + // price
        8 + // start_time
        8; // szusd_price_ratio

    pub fn get_current_szusd_price_in_zusd(&self) -> u64 {
        self.szusd_price_ratio / 10000
    }
}

// Lending
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Obligation {
    pub zbtc_deposit: u64,
    pub zusd_borrowed: u64,
}

impl Obligation {
    pub const LEN: usize =
        8 + // zbtc_deposit
        8; // zusd_borrowed
}

pub fn find_obligation_pda(user: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    let seeds = &[OBLIGATION_SEED, user.as_ref()];
    Pubkey::find_program_address(seeds, program_id)
}
