module z_fubao::lending {
    // Error codes
    use z_fubao::config::ZFubaoConfig;
    use sui::coin::{Self, Coin};
    use sui::event::emit;
    use sui::balance::{Self};
    use z_fubao::lbtc::LBTC;
    use z_fubao::zusd::ZUSD;
    use z_fubao::vault::Vault;
    
    const EINSUFFICIENT_COLLATERAL: u64 = 1;
    const EINSUFFICIENT_BALANCE: u64 = 2;
    const EINVALID_AMOUNT: u64 = 3;
    const EINSUFFICIENT_LBTC_IN_VAULT: u64 = 4;

    public struct Obligation has key {
        id: UID,
        user: address,
        lbtc_deposit: u64,
        zusd_borrowed: u64,
    }

    // Events
    public struct DepositEvent has copy, drop {
        user: address,
        amount: u64,
    }

    public struct WithdrawEvent has copy, drop {
        user: address,
        amount: u64,
    }

    public struct BorrowEvent has copy, drop {
        user: address,
        amount: u64,
    }

    public struct RepayEvent has copy, drop {
        user: address,
        amount: u64,
    }
    
    // Lending functions
    public fun deposit_collateral(
        _config: &ZFubaoConfig,
        vault: &mut Vault,
        obligation: &mut Obligation,
        lbtc_coin: Coin<LBTC>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        assert!(amount > 0, EINVALID_AMOUNT);
        assert!(coin::value(&lbtc_coin) >= amount, EINSUFFICIENT_BALANCE);

        let vault_lbtc_balance = vault.get_mut_lbtc_balance();
        balance::join(vault_lbtc_balance, lbtc_coin.into_balance());

        obligation.lbtc_deposit = obligation.lbtc_deposit + amount;
        
        emit(DepositEvent {
            user: tx_context::sender(ctx),
            amount,
        });
    }

    #[allow(lint(self_transfer))]
    public fun withdraw_collateral(
        config: &ZFubaoConfig,
        vault: &mut Vault,
        obligation: &mut Obligation,
        amount: u64,
        ctx: &mut TxContext
    ) {
        assert!(amount > 0, EINVALID_AMOUNT);
        assert!(obligation.lbtc_deposit >= amount, EINSUFFICIENT_COLLATERAL);

        let max_withdrawable = calculate_max_withdrawable(config, obligation);
        assert!(amount <= max_withdrawable, EINSUFFICIENT_COLLATERAL);


        let vault_lbtc_balance = vault.get_mut_lbtc_balance();
        assert!(balance::value(vault_lbtc_balance) >= amount, EINSUFFICIENT_LBTC_IN_VAULT);
        
        let lbtc_coin = balance::split(vault_lbtc_balance, amount);
        transfer::public_transfer(lbtc_coin.into_coin(ctx), tx_context::sender(ctx));

        obligation.lbtc_deposit = obligation.lbtc_deposit - amount;

        emit(WithdrawEvent {
            user: tx_context::sender(ctx),
            amount,
        });
    }

    public fun borrow(
        config: &ZFubaoConfig,
        vault: &mut Vault,
        obligation: &mut Obligation,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(amount > 0, EINVALID_AMOUNT);
        
        let max_borrowable = calculate_max_borrowable(config, obligation);
        assert!(amount <= max_borrowable, EINSUFFICIENT_COLLATERAL);

        obligation.zusd_borrowed = obligation.zusd_borrowed + amount;
        coin::mint_and_transfer(vault.get_zusd_treasury_cap(), amount, sender, ctx);
        
        emit(BorrowEvent {
            user: sender,
            amount,
        });
    }

    public fun repay(
        config: &ZFubaoConfig,
        vault: &mut Vault,
        obligation: &mut Obligation,
        zusd_coin: Coin<ZUSD>,
        amount: u64,
        ctx: &mut TxContext
    ) {
        assert!(amount > 0, EINVALID_AMOUNT);
        assert!(coin::value(&zusd_coin) >= amount, EINSUFFICIENT_BALANCE);
        assert!(obligation.zusd_borrowed >= amount, EINSUFFICIENT_BALANCE);


        obligation.zusd_borrowed = obligation.zusd_borrowed - amount;
        coin::burn(vault.get_zusd_treasury_cap(), zusd_coin);
        
        emit(RepayEvent {
            user: tx_context::sender(ctx),
            amount,
        });
    }
    // Helper functions
    fun calculate_max_borrowable(
        config: &ZFubaoConfig,
        obligation: &Obligation
    ): u64 {
        let collateral_value = obligation.lbtc_deposit * z_fubao::config::get_zbtc_price(config);
        let max_borrowable = (collateral_value * (z_fubao::config::get_ltv_ratio(config) as u64)) / 100;
        max_borrowable - obligation.zusd_borrowed
    }

    fun calculate_max_withdrawable(
        config: &ZFubaoConfig,
        obligation: &Obligation
    ): u64 {
        let required_collateral = (obligation.zusd_borrowed * 100) / (z_fubao::config::get_ltv_ratio(config) as u64);
        let max_withdrawable = obligation.lbtc_deposit - required_collateral;
        max_withdrawable
    }
}