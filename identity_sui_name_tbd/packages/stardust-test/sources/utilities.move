module stardust::utilities {
    // === Imports ===
    use std::type_name;

    use sui::bag::Bag;
    use sui::balance::Balance;
    use sui::coin;

    // === Errors ===

    /// Returned when trying to extract a `Balance<T>` from a `Bag` and the balance is zero.
    const EZeroNativeTokenBalance: u64 = 0;

    // === Public-Mutative Functions ===

    /// Extract a `Balance<T>` from a `Bag`, create a `Coin` out of it and send it to the address.
    /// NOTE: We return the `Bag` by value so the function can be called repeatedly in a PTB.
    public fun extract_and_send_to<T>(mut bag: Bag, to: address, ctx: &mut TxContext): Bag {
        let coin = coin::from_balance(extract_<T>(&mut bag), ctx);
        transfer::public_transfer(coin, to);
        bag
    }

    /// Extract a `Balance<T>` from a `Bag` and return it. Caller can decide what to do with it.
    /// NOTE: We return the `Bag` by value so the function can be called repeatedly in a PTB.
    public fun extract<T>(mut bag: Bag): (Bag, Balance<T>) {
        let balance = extract_<T>(&mut bag);
        (bag, balance)
    }

    // === Private Functions ===

    /// Get a `Balance<T>` from a `Bag`.
    /// Aborts if the balance is zero or if there is no balance for the type `T`.
    fun extract_<T>(bag: &mut Bag): Balance<T> {
        let key = type_name::get<T>().into_string();

        // This call aborts if the key doesn't exist.
        let balance : Balance<T> = bag.remove(key);

        assert!(balance.value() != 0, EZeroNativeTokenBalance);

        balance
    }
}
