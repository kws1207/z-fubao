module z_fubao::z_fubao {
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use sui::event::emit;
    use z_fubao::config::{Self, ZFubaoConfig};
    use sui::balance::Balance;
    use sui::balance;
    use z_fubao::zusd::ZUSD;
    use z_fubao::vault::Vault;

    const EINSUFFICIENT_BALANCE: u64 = 1;
    const EINVALID_AMOUNT: u64 = 2;
    const EUNAUTHORIZED: u64 = 3;
    const ESTAKING_POSITION_NOT_FOUND: u64 = 4;
    const EINSUFFICIENT_ZUSD_IN_VAULT: u64 = 5;

    public struct StakingPosition has key {
        id: UID,
        user: address,
        staked_amount: u64,
    }

    public struct OpenStakingPositionEvent has copy, drop {
        user: address,
    }

    public struct CloseStakingPositionEvent has copy, drop {
        user: address,
    }

    public struct StakeEvent has copy, drop {
        user: address,
        amount: u64,
    }

    public struct UnstakeEvent has copy, drop {
        user: address,
        amount: u64,
    }

    public fun open_staking_position(
        config: &ZFubaoConfig,
        staking_position: &mut StakingPosition,
        ctx: &mut TxContext
    ) {
        let staking_position = StakingPosition {
            id: object::new(ctx),
            user: tx_context::sender(ctx),
            staked_amount: 0,
        };

        transfer::transfer(staking_position, tx_context::sender(ctx));

        emit(OpenStakingPositionEvent {
            user: tx_context::sender(ctx),
        });
    }

    public fun close_staking_position(
        config: &ZFubaoConfig,
        vault: &mut Vault,
        staking_position: &mut StakingPosition,
        ctx: &mut TxContext
    ) {
        assert!(ctx.sender() == staking_position.user, EUNAUTHORIZED);

        user_withdraw_zusd(vault, staking_position, ctx);

        emit(CloseStakingPositionEvent {
            user: tx_context::sender(ctx),
        });
    }

    // Staking functions
    public fun stake(
        config: &ZFubaoConfig,
        vault: &mut Vault,
        staking_position: &mut StakingPosition,
        zusd_coin: Coin<ZUSD>,
        ctx: &mut TxContext
    ) {
        assert!(ctx.sender() == staking_position.user, EUNAUTHORIZED);

        let amount = coin::value(&zusd_coin);
        assert!(amount > 0, EINVALID_AMOUNT);

        let zusd_balance = vault.get_mut_zusd_balance();
        balance::join(zusd_balance, zusd_coin.into_balance());

        staking_position.staked_amount = staking_position.staked_amount + amount;

        emit(StakeEvent {
            user: tx_context::sender(ctx),
            amount,
        });
    }

    public fun unstake(
        config: &ZFubaoConfig,
        vault: &mut Vault,
        staking_position: &mut StakingPosition,
        amount: u64,
        ctx: &mut TxContext
    ) {
        assert!(ctx.sender() == staking_position.user, EUNAUTHORIZED);
        assert!(amount > 0, EINVALID_AMOUNT);
        assert!(amount <= staking_position.staked_amount, EINSUFFICIENT_BALANCE);

        user_withdraw_zusd(vault, staking_position, ctx);

        emit(UnstakeEvent {
            user: tx_context::sender(ctx),
            amount,
        });
    }

    fun user_withdraw_zusd(
        vault: &mut Vault,
        staking_position: &mut StakingPosition,
        ctx: &mut TxContext
    ) {
        let zusd_balance = vault.get_mut_zusd_balance();
        assert!(balance::value(zusd_balance) >= staking_position.staked_amount, EINSUFFICIENT_ZUSD_IN_VAULT);

        let zusd_coin = balance::split(zusd_balance, staking_position.staked_amount).into_coin(ctx);
        transfer::public_transfer(zusd_coin, staking_position.user);
    }

}