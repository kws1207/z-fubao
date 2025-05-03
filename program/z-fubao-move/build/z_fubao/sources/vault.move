module z_fubao::vault {
    use sui::event::emit;
    use z_fubao::lbtc::LBTC;
    use sui::coin::TreasuryCap;
    use z_fubao::szusd::SZUSD;
    use z_fubao::zusd::ZUSD;
    use sui::balance::Balance;
    use sui::balance;

    const EINVALID_AMOUNT: u64 = 1;
    const EUNAUTHORIZED: u64 = 2;

    public struct VaultLBTCBalanceUpdatedEvent has copy, drop {
        old_balance: u64,
        new_balance: u64,
    }

    public struct VaultZUSDBalanceUpdatedEvent has copy, drop {
        old_balance: u64,
        new_balance: u64,
    }

    public struct Vault has key {
        id: UID,
        authority: address,
        
        lbtc_balance: Balance<LBTC>,
        zusd_balance: Balance<ZUSD>,

        zusd_treasury_cap: TreasuryCap<ZUSD>,
        szusd_treasury_cap: TreasuryCap<SZUSD>,
    }

    public struct VaultCap has key {
        id: UID
    }

    public struct VAULT has drop {}


    fun init(otw: VAULT, ctx: &mut TxContext) {
        // Creating and sending the Publisher object to the sender.
        sui::package::claim_and_keep(otw, ctx);

        // Creating and sending the HouseCap object to the sender.
        let vault_cap = VaultCap {
        id: object::new(ctx)
        };

        transfer::transfer(vault_cap, ctx.sender());
    }

    /// Initialize the configuration with a one-time witness
    public fun initialize(
        vault_cap: VaultCap,
        zusd_treasury_cap: TreasuryCap<ZUSD>,
        szusd_treasury_cap: TreasuryCap<SZUSD>,
        ctx: &mut TxContext
    ) {
        let vault = Vault {
            id: object::new(ctx),
            authority: ctx.sender(),
            lbtc_balance: balance::zero(),
            zusd_balance: balance::zero(),
            zusd_treasury_cap,
            szusd_treasury_cap,
        };

        let VaultCap { id } = vault_cap;
        object::delete(id);

        transfer::share_object(vault);
    }

    public fun deposit_lbtc(vault: &mut Vault, lbtc: Balance<LBTC>, ctx: &mut TxContext) {
        assert!(balance::value(&lbtc) > 0, EINVALID_AMOUNT);
        assert!(ctx.sender() == vault.authority, EUNAUTHORIZED);

        balance::join(&mut vault.lbtc_balance, lbtc);
        emit(VaultLBTCBalanceUpdatedEvent {
            old_balance: balance::value(&vault.lbtc_balance),
            new_balance: balance::value(&vault.lbtc_balance),
        });
    }

    #[allow(lint(self_transfer))]
    public fun withdraw_lbtc(vault: &mut Vault, amount: u64, ctx: &mut TxContext) {
        assert!(amount > 0 && amount <= balance::value(&vault.lbtc_balance), EINVALID_AMOUNT);
        assert!(ctx.sender() == vault.authority, EUNAUTHORIZED);

        let lbtc = balance::split(&mut vault.lbtc_balance, amount);
        transfer::public_transfer(lbtc.into_coin(ctx), ctx.sender());

        emit(VaultLBTCBalanceUpdatedEvent {
            old_balance: balance::value(&vault.lbtc_balance),
            new_balance: balance::value(&vault.lbtc_balance),
        });
    }

    public fun deposit_zusd(vault: &mut Vault, zusd: Balance<ZUSD>, ctx: &mut TxContext) {
        assert!(balance::value(&zusd) > 0, EINVALID_AMOUNT);
        assert!(ctx.sender() == vault.authority, EUNAUTHORIZED);

        balance::join(&mut vault.zusd_balance, zusd);
        emit(VaultZUSDBalanceUpdatedEvent {
            old_balance: balance::value(&vault.zusd_balance),
            new_balance: balance::value(&vault.zusd_balance),
        });
    }

    #[allow(lint(self_transfer))]
    public fun withdraw_zusd(vault: &mut Vault, amount: u64, ctx: &mut TxContext) {
        assert!(amount > 0 && amount <= balance::value(&vault.zusd_balance), EINVALID_AMOUNT);
        assert!(ctx.sender() == vault.authority, EUNAUTHORIZED);
        
        let zusd = balance::split(&mut vault.zusd_balance, amount);
        transfer::public_transfer(zusd.into_coin(ctx), ctx.sender());

        emit(VaultZUSDBalanceUpdatedEvent {
            old_balance: balance::value(&vault.zusd_balance),
            new_balance: balance::value(&vault.zusd_balance),
        });
    }

    public fun get_zusd_treasury_cap(vault: &mut Vault): &mut TreasuryCap<ZUSD> {
        &mut vault.zusd_treasury_cap
    }

    public fun get_szusd_treasury_cap(vault: &mut Vault): &mut TreasuryCap<SZUSD> {
        &mut vault.szusd_treasury_cap
    }

    public fun get_mut_lbtc_balance(vault: &mut Vault): &mut Balance<LBTC> {
        &mut vault.lbtc_balance
    }

    public fun get_mut_zusd_balance(vault: &mut Vault): &mut Balance<ZUSD> {
        &mut vault.zusd_balance
    }
    
    
}