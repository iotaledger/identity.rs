// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module stardust::capped_coin_tests {

    use iota::coin::{Self, Coin};
    use iota::test_scenario;

    use stardust::capped_coin;

    public struct CAPPED_COIN_TESTS has drop {}

    // Show how Capped Coin works and test if it acts according to specs.
    #[test]
    fun create_and_mint_capped_coin() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);
        let witness = CAPPED_COIN_TESTS{};

        // Create a Coin.
        let (cap, meta) = coin::create_currency(
            witness,
            0, 
            b"TEST",
            b"TEST",
            b"TEST",
            option::none(),
            scenario.ctx(),
        );

        // Create the policy and consume the Cap, only 100 allowed of this coin after this!
        let mut policy = capped_coin::create_max_supply_policy(cap, 100, scenario.ctx());

        // We should start out with a Supply of 0.
        assert!(capped_coin::total_supply(&policy) == 0, 0);

        capped_coin::mint_and_transfer(&mut policy, 10, sender, scenario.ctx());

        scenario.next_tx(sender);

        // We should start out with a Supply of 0.
        assert!(capped_coin::total_supply(&policy) == 10, 0);

        transfer::public_transfer(policy, scenario.ctx().sender());
        transfer::public_freeze_object(meta);

        scenario.end();
    }

    // Show how burning works.
    #[test]
    fun create_and_burn_capped_coin() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);
        let witness = CAPPED_COIN_TESTS{};

        // Create a Coin.
        let (cap, meta) = coin::create_currency(
            witness,
            0, 
            b"TEST",
            b"TEST",
            b"TEST",
            option::none(),
            scenario.ctx(),
        );

        // Create the policy and consume the Cap, only 100 allowed of this coin after this!
        let mut policy = capped_coin::create_max_supply_policy(cap, 100, scenario.ctx());

        // We should start out with a Supply of 0.
        assert!(capped_coin::total_supply(&policy) == 0, 0);

        capped_coin::mint_and_transfer(&mut policy, 10, sender, scenario.ctx());

        scenario.next_tx(sender);

        // We should start out with a Supply of 0.
        assert!(capped_coin::total_supply(&policy) == 10, 0);

        let coin = scenario.take_from_address<Coin<CAPPED_COIN_TESTS>>(sender);
        capped_coin::burn(&mut policy, coin);

        assert!(capped_coin::total_supply(&policy) == 0, 0);

        transfer::public_transfer(policy, scenario.ctx().sender());
        transfer::public_freeze_object(meta);

        scenario.end();
    }

    // Demonstrate cap limitations.
    #[test]
    #[expected_failure(abort_code = capped_coin::EMaximumSupplyReached)]
    fun create_and_mint_too_many_capped_coins() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);
        let witness = CAPPED_COIN_TESTS{};

        // Create a Coin.
        let (cap, meta) = coin::create_currency(
            witness,
            0, 
            b"TEST",
            b"TEST",
            b"TEST",
            option::none(),
            scenario.ctx(),
        );

        // Create the policy and consume the Cap, only 100 allowed of this coin after this!
        let mut policy = capped_coin::create_max_supply_policy(cap, 100, scenario.ctx());

        // We should start out with a Supply of 0.
        assert!(capped_coin::total_supply(&policy) == 0, 0);

        capped_coin::mint_and_transfer(&mut policy, 10, sender, scenario.ctx());

        scenario.next_tx(sender);

        // We should start out with a Supply of 0.
        assert!(capped_coin::total_supply(&policy) == 10, 0);

        capped_coin::mint_and_transfer(&mut policy, 1000, sender, scenario.ctx());

        transfer::public_transfer(policy, scenario.ctx().sender());
        transfer::public_freeze_object(meta);

        scenario.end();
    }
}
