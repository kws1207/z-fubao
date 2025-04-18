# Z-Fubao: Earn, Pay, and Live with your Bitcoin

Z-Fubao is a decentralized finance (DeFi) protocol built on Solana that enables users to:
1. Deposit ZBTC as collateral to mint ZUSD stablecoins
2. Stake ZUSD tokens to receive SZUSD which is yield-bearing stablecoin

## Protocol Overview

### Vault Program
The vault program allows users to:
- Deposit ZBTC as collateral and mint ZUSD stablecoins at a 70% loan-to-value ratio
- Repay ZUSD to unlock and withdraw their ZBTC collateral
- Maintain over-collateralization to prevent liquidation

### Staking Program
The staking program enables users to:
- Stake ZUSD tokens and receive SZUSD tokens 1:1
- Unstake by burning SZUSD tokens to receive back their ZUSD

### Client Application
A web-based interface for interacting with the protocol, built with:
- TypeScript
- Next.js
- React

## Getting Started

### Prerequisites
- Solana CLI tools
- Node.js 20.18.3
- npm package manager

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/z-fubao.git
cd z-fubao

# Build Solana programs
cd program/z-fubao
cargo build-sbf

# Deploy the program
solana config set --url https://api.devnet.solana.com # Set to devnet
solana airdrop 2 # Request some SOL for deployment
solana program deploy ../target/sbpf-solana-solana/release/z_fubao.so

# Set up the client application
cd ../../client
npm install
npm run dev
```

## Usage

### Minting ZUSD
1. Connect your wallet through the client interface
2. Navigate to the Vault section
3. Deposit ZBTC as collateral
4. Mint ZUSD tokens (up to 70% of your collateral value)

### Staking ZUSD
1. Navigate to the Staking section
2. Approve ZUSD for staking
3. Stake your ZUSD to receive SZUSD tokens
4. Unstake anytime by burning SZUSD tokens

## Architecture

### Smart Contracts
- `program/vault/src/lib.rs`: Handles collateral deposits, ZUSD minting, repayments
- `program/stake/src/main.rs`: Manages ZUSD staking and SZUSD token distribution

### Client
The client application provides a user-friendly interface to interact with the Solana programs, handling:
- Wallet connections
- Transaction building and signing
- Real-time updates of user positions
- Responsive UI for both desktop and mobile devices

## Development

### Program Deployment
```bash
solana program deploy --program-id <PROGRAM_ID> target/deploy/vault.so
solana program deploy --program-id <PROGRAM_ID> target/deploy/stake.so
```

### Client Development
```bash
cd client
npm run dev       # Start development server
npm run build     # Build for production
npm start         # Start production server
```

## Team
[<img src="https://github.com/user-attachments/assets/583a005a-d5f3-45c6-bab9-aa271f44fe6f" alt="Orakle@KAIST" width="200"/>](https://www.orakle-kaist.xyz/en)

