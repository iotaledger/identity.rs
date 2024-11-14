module stardust::basic_tests {

    use std::type_name;

    use iota::bag;
    use iota::balance;
    use iota::coin;
    use iota::iota::IOTA;

    use stardust::basic_output;
    use stardust::expiration_unlock_condition;
    use stardust::storage_deposit_return_unlock_condition;
    use stardust::timelock_unlock_condition;
    use stardust::utilities;

    const ENoBaseTokenBalance: u64 = 1;
    const ENativeTokenBagNonEmpty: u64 = 2;
    const EIotaBalanceMismatch: u64 = 3;

    // One Time Witness for coins used in the tests.
    public struct TEST_A has drop {}
    public struct TEST_B has drop {}

    // Demonstration on how to claim the assets from a `BasicOutput` with all unlock conditions inside one PTB.
    #[test]
    fun demonstrate_claiming_ptb() {
        let initial_iota_in_output = 10000;
        let initial_testA_in_output = 100;
        let initial_testB_in_output = 100;
        let sdruc_return_amount = 1000;

        let timelocked_until = 5;
        let expiration_after = 20;
        let owner = @0xA;
        let sdruc_return_address = @0xB;
        let expiration_return_address = @0xC;
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

        let output = basic_output::create_for_testing(
            iota,
            native_tokens,
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

        // Ready with the basic output, now we can claim the assets.
        // The task is to assemble a PTB-like transaction in move that demonstrates how to claim.
        // PTB inputs: basic output ID (`basic`) and address to migrate to (`migrate_to`)

        // Command 1: extract the base token and native tokens bag.
        let (extracted_base_token, mut extracted_native_tokens) = output.extract_assets(&mut ctx);
        assert!(extracted_base_token.value() == 9000, ENoBaseTokenBalance);

        // Command 2: extract the asset A and send to the user.
        extracted_native_tokens = utilities::extract_and_send_to<TEST_A>(extracted_native_tokens, migrate_to, &mut ctx);

        // Command 3: extract the asset B and send to the user.
        extracted_native_tokens = utilities::extract_and_send_to<TEST_B>(extracted_native_tokens, migrate_to, &mut ctx);
        assert!(extracted_native_tokens.is_empty(), ENativeTokenBagNonEmpty);

        // Command 4: delete the bag.
        extracted_native_tokens.destroy_empty();

        // Command 5: create a coin from the extracted IOTA balance.
        let iota_coin = coin::from_balance(extracted_base_token, &mut ctx);
        // We should have `initial_iota_in_output` - `sdruc_return_amount` left in the coin.
        assert!(iota_coin.value() == (initial_iota_in_output - sdruc_return_amount), EIotaBalanceMismatch);

        // Command 6: send back the base token coin to the user.
        // If we sponsored the transaction with our own coins, now is the time to detuct it from the user by taking from `iota_coin` and merging it into the gas token
        // since we can dry run the tx before submission, we know how much to charge the user, or we charge the whole gas budget.
        transfer::public_transfer(iota_coin, migrate_to);

        // !!! migration complete !!! 
    }
}
