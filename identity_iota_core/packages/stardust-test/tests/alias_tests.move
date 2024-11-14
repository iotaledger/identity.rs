module stardust::alias_tests {

    use std::type_name;

    use iota::bag;
    use iota::balance;
    use iota::coin;
    use iota::iota::IOTA;

    use stardust::alias;
    use stardust::alias_output;
    use stardust::utilities;

    const ENativeTokenBagNonEmpty: u64 = 1;

    // One Time Witness for coins used in the tests.
    public struct TEST_A has drop {}
    public struct TEST_B has drop {}

    // Demonstration on how to claim the assets from an `AliasOutput` with all unlock conditions inside one PTB.
    #[test]
    fun demonstrate_claiming_ptb() {
        let initial_iota_in_output = 10000;
        let initial_testA_in_output = 100;
        let initial_testB_in_output = 100;

        let owner = @0xA;
        let migrate_to = @0xD; 

        // Create a new tx context.
        let mut ctx = tx_context::new(
            // sender
            @0xA,
            // tx_hash
            x"3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
            // epoch
            1,
            // epoch ts in ms (10 in seconds)
            10000,
            // ids created
            0,
        );

        // Mint some tokens.
        let iota = balance::create_for_testing<IOTA>(initial_iota_in_output);

        let test_a_balance = balance::create_for_testing<TEST_A>(initial_testA_in_output);
        let test_b_balance = balance::create_for_testing<TEST_B>(initial_testB_in_output);

        // Add the native token balances to the bag.
        let mut native_tokens = bag::new(&mut ctx);

        native_tokens.add(type_name::get<TEST_A>().into_string(), test_a_balance);
        native_tokens.add(type_name::get<TEST_B>().into_string(), test_b_balance);

        // Create an `AliasOutput`.
        let mut alias_output = alias_output::create_for_testing(
            iota,
            native_tokens,
            &mut ctx,
        );

        let alias = alias::create_for_testing(
            // legacy state controller
            option::some(owner),
            // state index
            0,
            // state metadata
            option::some(b"state metadata content"),
            // sender feature
            option::some(owner),
            // metadata feature
            option::some(b"metadata content"),
            // issuer feature
            option::some(owner),
            // immutable metadata
            option::some(b"immutable metadata content"),
            &mut ctx,
        );    
        alias_output.attach_alias(alias);

        // Command 1: extract the base token and native tokens bag.
        let (extracted_base_token, mut extracted_native_tokens, extracted_alias) = alias_output.extract_assets();

        // Command 2: extract the asset A and send to the user.
        extracted_native_tokens = utilities::extract_and_send_to<TEST_A>(extracted_native_tokens, migrate_to, &mut ctx);

        // Command 3: extract the asset B and send to the user
        extracted_native_tokens = utilities::extract_and_send_to<TEST_B>(extracted_native_tokens, migrate_to, &mut ctx);
        assert!(extracted_native_tokens.is_empty(), ENativeTokenBagNonEmpty);

        // Command 4: delete the bag.
        bag::destroy_empty(extracted_native_tokens);

        // Command 5: create a coin from the extracted IOTA balance.
        let iota_coin = coin::from_balance(extracted_base_token, &mut ctx);

        // Command 6: send back the base token coin to the user.
        transfer::public_transfer(iota_coin, migrate_to);

        // Command 7: destroy the alias.
        alias::destroy(extracted_alias);

        // !!! migration complete !!! 
    }
}
