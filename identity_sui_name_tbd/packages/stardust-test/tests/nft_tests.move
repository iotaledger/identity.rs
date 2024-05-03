// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::nft_tests {

    use std::ascii;
    use std::fixed_point32;
    use std::string;
    use std::type_name;

    use sui::bag;
    use sui::balance::{Self, Balance};
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use sui::table;
    use sui::test_scenario;
    use sui::url;
    use sui::vec_set;

    use stardust::irc27;
    use stardust::nft_output;
    use stardust::nft;

    use stardust::expiration_unlock_condition;
    use stardust::storage_deposit_return_unlock_condition;
    use stardust::timelock_unlock_condition;

    // One time witness for coins used in the tests.
    public struct TEST_A has drop {}
    public struct TEST_B has drop {}

    // Demonstration on how to claim the assets from an NFT output with all unlock conditions inside one PTB.
    #[test]
    fun nft_assets_extraction() {
        // Set up a test enviroment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Create an NftOutput object.
        let test_a_balance = balance::create_for_testing<TEST_A>(100);
        let test_b_balance = balance::create_for_testing<TEST_B>(200);

        let mut native_tokens = bag::new(scenario.ctx());
        native_tokens.add(type_name::get<TEST_A>().into_string(), test_a_balance);
        native_tokens.add(type_name::get<TEST_B>().into_string(), test_b_balance);

        let mut nft_output = nft_output::create_for_testing(
            balance::create_for_testing(10000),
            native_tokens,
            option::some(storage_deposit_return_unlock_condition::create_for_testing(@0xB, 1000)),
            option::some(timelock_unlock_condition::create_for_testing(5)),
            option::some(expiration_unlock_condition::create_for_testing(sender, @0xB, 20)),
            scenario.ctx(),
        );

        // Create an Nft object.
        let mut royalties = table::new(scenario.ctx());
        royalties.add(sender, fixed_point32::create_from_rational(1, 2));

        let mut attributes = vec_set::empty();
        attributes.insert(string::utf8(b"attribute"));

        let mut non_standard_fields = table::new(scenario.ctx());
        non_standard_fields.add(string::utf8(b"field"), string::utf8(b"value"));

        let nft = nft::create_for_testing(
            option::some(sender),
            option::some(b"metadata"),
            option::some(b"tag"),
            option::some(sender),
            irc27::create_for_testing(
                string::utf8(b"0.0.1"),
                string::utf8(b"image/png"),
                url::new_unsafe(ascii::string(b"www.best-nft.com/nft.png")),
                string::utf8(b"nft"),
                option::some(string::utf8(b"collection")),
                royalties,
                option::some(string::utf8(b"issuer")),
                option::some(string::utf8(b"description")),
                attributes,
                non_standard_fields,
            ),
            scenario.ctx(),
        );

        // Add the Nft as a dynamic field to the output.
        nft_output.attach_nft(nft);

        // Increment epoch timestamp.
        scenario.ctx().increment_epoch_timestamp(10000);

        // Extract assets.
        let (iota, mut native_tokens, nft) = nft_output.extract_assets(scenario.ctx());

        // Check the extracted IOTA balance.
        assert!(iota.value() == 9000, 0);

        // Check the extracted native tokens.
        assert!(native_tokens.borrow<ascii::String, Balance<TEST_A>>(type_name::get<TEST_A>().into_string()).value() == 100, 1);
        assert!(native_tokens.borrow<ascii::String, Balance<TEST_B>>(type_name::get<TEST_B>().into_string()).value() == 200, 2);

        // Check the extracted NFT.
        assert!(nft.legacy_sender().contains(&sender), 3);
        assert!(nft.metadata().contains(&b"metadata"), 4);
        assert!(nft.tag().contains(&b"tag"), 5);
        assert!(nft.immutable_issuer().contains(&sender), 6);

        assert!(nft.immutable_metadata().version() == string::utf8(b"0.0.1"), 7);
        assert!(nft.immutable_metadata().media_type() == string::utf8(b"image/png"), 8);
        assert!(nft.immutable_metadata().uri() == url::new_unsafe(ascii::string(b"www.best-nft.com/nft.png")), 9);
        assert!(nft.immutable_metadata().name() == string::utf8(b"nft"), 10);
        assert!(nft.immutable_metadata().collection_name().contains(&string::utf8(b"collection")), 11);
        assert!(nft.immutable_metadata().royalties().length() == 1, 12);
        assert!(nft.immutable_metadata().royalties()[sender] == fixed_point32::create_from_rational(1, 2), 13);
        assert!(nft.immutable_metadata().issuer_name().contains(&string::utf8(b"issuer")), 14);
        assert!(nft.immutable_metadata().description().contains(&string::utf8(b"description")), 15);
        assert!(nft.immutable_metadata().attributes().size() == 1, 16);
        assert!(nft.immutable_metadata().attributes().contains(&string::utf8(b"attribute")), 17);

        assert!(nft.immutable_metadata().non_standard_fields().length() == 1, 18);
        assert!(nft.immutable_metadata().non_standard_fields()[string::utf8(b"field")] == string::utf8(b"value"), 19);

        // Check the storage deposit return.
        scenario.next_tx(sender);
    
        let returned_storage_deposit = scenario.take_from_address<Coin<SUI>>(@0xB);

        assert!(returned_storage_deposit.value() == 1000, 18);

        test_scenario::return_to_address(@0xB, returned_storage_deposit);

        // Transfer the extracted assets.
        transfer::public_transfer(coin::from_balance(iota, scenario.ctx()), @0xC);

        let coin_a = coin::from_balance(native_tokens.remove<ascii::String, Balance<TEST_A>>(type_name::get<TEST_A>().into_string()), scenario.ctx());
        let coin_b = coin::from_balance(native_tokens.remove<ascii::String, Balance<TEST_B>>(type_name::get<TEST_B>().into_string()), scenario.ctx());
        
        transfer::public_transfer(coin_a, @0xC);
        transfer::public_transfer(coin_b, @0xC);

        transfer::public_transfer(nft, @0xC);

        // Cleanup.
        bag::destroy_empty(native_tokens);

        scenario.end();
    }
}
