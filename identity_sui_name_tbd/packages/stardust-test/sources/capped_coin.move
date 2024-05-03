// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module stardust::capped_coin {

    use std::ascii;
    use std::string;

    use sui::balance::{Supply, Balance};
    use sui::coin::{Self, Coin, TreasuryCap, CoinMetadata};

    /// The error returned when the maximum supply reached.
    const EMaximumSupplyReached: u64 = 0;

    /// The policy wrapper that ensures the supply of a `Coin` never exceeds the maximum supply.
    public struct MaxSupplyPolicy<phantom T> has key, store {
        id: UID,

        /// The maximum supply.
        maximum_supply: u64,

        /// The wrapped `TreasuryCap`.
        treasury_cap: TreasuryCap<T>,
    }

    /// Wrap a `TreasuryCap` into a `MaxSupplyPolicy` to prevent minting of tokens > max supply.
    /// Be careful, once you add a maximum supply you will not be able to change it or get rid of it!
    /// This gives coin holders a guarantee that the maximum supply of that specific coin will never change.
    public fun create_max_supply_policy<T>(
        treasury_cap: TreasuryCap<T>,
        maximum_supply: u64,
        ctx: &mut TxContext
    ): MaxSupplyPolicy<T> {
        MaxSupplyPolicy {
            id: object::new(ctx),
            maximum_supply,
            treasury_cap
        }
    }

    /// Return the total number of `T`'s in circulation.
    public fun total_supply<T>(policy: &MaxSupplyPolicy<T>): u64 {
        coin::total_supply(&policy.treasury_cap)
    }

    /// Get immutable reference to the treasury's `Supply`.
    public fun supply_immut<T>(policy: &MaxSupplyPolicy<T>): &Supply<T> {
        coin::supply_immut(&policy.treasury_cap)
    }
    
    /// Create a `Coin` worth `value` and increase the total supply in `cap` accordingly.
    public fun mint<T>(
        policy: &mut MaxSupplyPolicy<T>,
        value: u64,
        ctx: &mut TxContext
    ): Coin<T> {
        assert!(total_supply(policy) + value <= policy.maximum_supply, EMaximumSupplyReached);
        coin::mint(&mut policy.treasury_cap, value, ctx)
    }

    /// Mint some amount of `T` as a `Balance` and increase the total supply in `cap` accordingly.
    /// Aborts if `value` + `cap.total_supply` >= `U64_MAX`.
    public fun mint_balance<T>(
        policy: &mut MaxSupplyPolicy<T>,
        value: u64
    ): Balance<T> {
        assert!(total_supply(policy) + value <= policy.maximum_supply, EMaximumSupplyReached);
        coin::mint_balance(&mut policy.treasury_cap, value)
    }

    /// Destroy the coin `c` and decrease the total supply in `cap` accordingly.
    public entry fun burn<T>(policy: &mut MaxSupplyPolicy<T>, c: Coin<T>): u64 {
        coin::burn(&mut policy.treasury_cap, c)
    }

    /// Mint `amount` of `Coin` and send it to the `recipient`. Invokes `mint()`.
    public fun mint_and_transfer<T>(
        policy: &mut MaxSupplyPolicy<T>,
        amount: u64,
        recipient: address,
        ctx: &mut TxContext
    ) {
        assert!(total_supply(policy) + amount <= policy.maximum_supply, EMaximumSupplyReached);
        coin::mint_and_transfer(&mut policy.treasury_cap, amount, recipient, ctx)
    }

    // === Update coin metadata ===

    /// Update the `name` of the coin in the `CoinMetadata`.
    public fun update_name<T>(
        policy: &mut MaxSupplyPolicy<T>,
        metadata: &mut CoinMetadata<T>,
        name: string::String
    ) {
        coin::update_name(&policy.treasury_cap, metadata, name)
    }

    /// Update the `symbol` of the coin in the `CoinMetadata`.
    public fun update_symbol<T>(
        policy: &mut MaxSupplyPolicy<T>,
        metadata: &mut CoinMetadata<T>,
        symbol: ascii::String
    ) {
        coin::update_symbol(&policy.treasury_cap, metadata, symbol)
    }

    /// Update the `description` of the coin in the `CoinMetadata`.
    public fun update_description<T>(
        policy: &mut MaxSupplyPolicy<T>,
        metadata: &mut CoinMetadata<T>,
        description: string::String
    ) {
        coin::update_description(&policy.treasury_cap, metadata, description)
    }

    /// Update the `url` of the coin in the `CoinMetadata`
    public fun update_icon_url<T>(
        policy: &mut MaxSupplyPolicy<T>,
        metadata: &mut CoinMetadata<T>,
        url: ascii::String
    ) {
        coin::update_icon_url(&policy.treasury_cap, metadata, url)
    }
}
