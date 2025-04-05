use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ZFubaoInstruction {
    /// Initialize a new vault
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account of the person initializing the vault
    /// 1. `[]` The authority account
    /// 2. `[writable]` The lending state account
    /// 3. `[]` The ZBTC mint
    /// 4. `[]` The ZUSD mint
    /// 5. `[writable]` The ZBTC vault account
    /// 6. `[]` The system program id
    /// 7. `[]` The token program id
    /// 8. `[]` The associated token program id
    Initialize,

    /// Initialize a new obligation for a user
    ///
    /// Accounts expected:
    /// 0. `[signer]` The user account
    /// 1. `[]` Authority account
    /// 2. `[writable]` The obligation account (PDA)
    /// 3. `[]` The system program
    InitObligation,

    /// Deposit ZBTC as collateral
    ///
    /// Accounts expected:
    /// 0. `[signer]` The user account
    /// 1. `[]` Authority account
    /// 2. `[writable]` The obligation account (PDA)
    /// 3. `[writable]` User's ZBTC token account
    /// 4. `[writable]` ZBTC vault token account
    /// 5. `[]` Token program id
    DepositZBTC { amount: u64 },

    /// Withdraw ZBTC collateral
    ///
    /// Accounts expected:
    /// 0. `[signer]` The user account
    /// 1. `[]` Authority account
    /// 2. `[writable]` The obligation account (PDA)
    /// 3. `[writable]` User's ZBTC token account
    /// 4. `[writable]` ZBTC vault token account
    /// 5. `[]` Lending state account
    /// 6. `[]` Token program id
    WithdrawZBTC { amount: u64 },

    /// Borrow ZUSD
    ///
    /// Accounts expected:
    /// 0. `[signer]` The user account
    /// 1. `[]` Authority account
    /// 2. `[writable]` The obligation account (PDA)
    /// 3. `[writable]` User's ZUSD token account
    /// 4. `[]` ZUSD mint
    /// 5. `[]` Lending state account
    /// 6. `[]` Token program id
    BorrowZUSD { amount: u64 },

    /// Repay ZUSD
    ///
    /// Accounts expected:
    /// 0. `[signer]` The user account
    /// 1. `[]` Authority account
    /// 2. `[writable]` The obligation account (PDA)
    /// 3. `[writable]` User's ZUSD token account
    /// 4. `[]` ZUSD mint
    /// 5. `[]` Lending state account
    /// 6. `[]` Token program id
    RepayZUSD { amount: u64 },

    /// Stake ZUSD tokens and mint SZUSD tokens
    ///
    /// Accounts expected:
    /// 0. `[signer]` User's main account
    /// 1. `[writable]` User's ZUSD token account
    /// 2. `[writable]` User's SZUSD token account
    /// 3. `[]` ZUSD mint
    /// 4. `[writable]` SZUSD mint
    /// 5. `[writable]` Staking vault - where ZUSD is stored
    /// 6. `[]` Token program
    /// 7. `[]` System program
    Stake { amount: u64 },

    /// Unstake SZUSD tokens and get back ZUSD tokens
    ///
    /// Accounts expected:
    /// 0. `[signer]` User's main account
    /// 1. `[writable]` User's ZUSD token account
    /// 2. `[writable]` User's SZUSD token account
    /// 3. `[]` ZUSD mint
    /// 4. `[writable]` SZUSD mint
    /// 5. `[writable]` Staking vault - where ZUSD is stored
    /// 6. `[]` Token program
    /// 7. `[]` System program
    Unstake { amount: u64 },
}

impl ZFubaoInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(9);
        match self {
            Self::Initialize => {
                buf.extend_from_slice(&[0]);
            }
            Self::InitObligation => {
                buf.extend_from_slice(&[1]);
            }
            Self::DepositZBTC { amount } => {
                buf.extend_from_slice(&[2]);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::WithdrawZBTC { amount } => {
                buf.extend_from_slice(&[3]);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::BorrowZUSD { amount } => {
                buf.extend_from_slice(&[4]);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::RepayZUSD { amount } => {
                buf.extend_from_slice(&[5]);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Stake { amount } => {
                buf.extend_from_slice(&[6]);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Unstake { amount } => {
                buf.extend_from_slice(&[7]);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
        }
        buf
    }
}
