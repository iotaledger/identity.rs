module stardust::address_unlock_condition_tests {

    use iota::bag;
    use iota::balance;
    use iota::coin;
    use iota::iota::IOTA;

    use stardust::alias;
    use stardust::alias_output;
    use stardust::basic_output;
    use stardust::expiration_unlock_condition;
    use stardust::storage_deposit_return_unlock_condition;
    use stardust::timelock_unlock_condition;
        
    const ENativeTokenBagNonEmpty: u64 = 1;
    const EIotaBlanceMismatch: u64 = 3;
    
    // One Time Witness for coins used in the tests.
    public struct TEST_A has drop {}
    public struct TEST_B has drop {}

    // Demonstration on how to claim the assets from a basic alias_output with all unlock conditions inside one PTB.
    #[test]
    fun demonstrate_alias_address_unlocking() {
        let initial_iota_in_output = 10000;

        let owner = @0xA;
        let migrate_to = @0xD; 

        // Create a new tx context.
        let mut ctx = tx_context::new(
            // sender
            @0xA,
            // tx)hash
            x"3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
            // epoch
            1,
            // epoch ts in ms (10 in seconds)
            10000,
            // ids created
            0,
        );

        let mut alias_output = alias_output::create_for_testing(
            // iota
            balance::create_for_testing<IOTA>(initial_iota_in_output),
            // tokens
            bag::new(&mut ctx),
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

        // `BasicOutput` owned by the alias.
        let basic_iota_balance = balance::create_for_testing<IOTA>(initial_iota_in_output);
        let timelocked_until = 5;
        let expiration_after = 20;
        let sdruc_return_address = @0xB;
        let sdruc_return_amount = 1000;
        let expiration_return_address = @0xC;

        let basic_output = basic_output::create_for_testing(
            basic_iota_balance,
            bag::new(&mut ctx),
            option::some(storage_deposit_return_unlock_condition::create_for_testing(sdruc_return_address, sdruc_return_amount)),
            option::some(timelock_unlock_condition::create_for_testing(timelocked_until)),
            option::some(expiration_unlock_condition::create_for_testing(owner, expiration_return_address, expiration_after)),
            // metadata feature
            option::some(b"metadata content"),
            // tag feature
            option::some(b"tag content"),
            // sender feature
            option::some(owner),
            &mut ctx,
        );

        // Command 1: unlock the basic token.
        // TODO: is it possible to create a Receiving object?
        // transfer::transfer(basic_output, alias.id().uid_to_address());
        // let basic_output = unlock_alias_address_owned_basic(&mut alias, basic_output);

        // Command 2: extract the base token and native token bag.
        let (extracted_base_token, extracted_native_tokens) = basic_output.extract_assets(&mut ctx);

        // Command 3: delete the bag.
        extracted_native_tokens.destroy_empty();

        // Command 4: create a coin from the extracted IOTA balance.
        let iota_coin = coin::from_balance(extracted_base_token, &mut ctx);
        // We should have `initial_iota_in_output` - `sdruc_return_amount` left in the coin.
        assert!(iota_coin.value() == (initial_iota_in_output - sdruc_return_amount), EIotaBlanceMismatch);

        // Command 6: send back the base token coin to the user.
        transfer::public_transfer(iota_coin, migrate_to);

        // Command 7: extract the base token and native tokens bag.
        let (extracted_base_token, native_token_bag, extracted_alias) = alias_output.extract_assets();

        // Command 8: delete the bag.
        assert!(native_token_bag.is_empty(), ENativeTokenBagNonEmpty);
        bag::destroy_empty(native_token_bag);

        // Command 9: create a coin from the extracted iota balance.
        let iota_coin = coin::from_balance(extracted_base_token, &mut ctx);

        // Command 11: send back the base token coin to the user.
        transfer::public_transfer(iota_coin, migrate_to);

        // Command 12: destroy the alias.
        alias::destroy(extracted_alias);

        // !!! migration complete !!! 
    }
}
