module stardust::alias_output {

    use sui::bag::Bag;
    use sui::balance::Balance;
    use sui::dynamic_object_field;
    use sui::sui::SUI;
    use sui::transfer::Receiving;

    use stardust::alias::Alias;

    /// The Alias dynamic object field name.
    const ALIAS_NAME: vector<u8> = b"alias";

    /// Owned Object controlled by the Governor Address.
    public struct AliasOutput has key {
        /// This is a "random" UID, not the AliasID from Stardust.
        id: UID,

        /// The amount of IOTA coins held by the output.
        iota: Balance<SUI>,

        /// The `Bag` holds native tokens, key-ed by the stringified type of the asset.
        /// Example: key: "0xabcded::soon::SOON", value: Balance<0xabcded::soon::SOON>.
        native_tokens: Bag,
    }

    // === Public-Mutative Functions ===
    
    /// The function extracts assets from a legacy `AliasOutput`.
    ///    - returns the IOTA Balance,
    ///    - the native tokens Bag,
    ///    - and the `Alias` object that persists the AliasID=ObjectID from Stardust.
    public fun extract_assets(mut output: AliasOutput): (Balance<SUI>, Bag, Alias) {
        // Load the related alias object.
        let alias = load_alias(&mut output);

        // Unpack the output into its basic part.
        let AliasOutput {
            id,
            iota,
            native_tokens
        } = output;

        // Delete the output.
        object::delete(id);

        (iota, native_tokens, alias)
    }

    // === Public-Package Functions ===

    /// Utility function to receive an `AliasOutput` object in other Stardust modules.
    /// Other modules in the Stardust package can call this function to receive an `AliasOutput` object (nft).
    public(package) fun receive(parent: &mut UID, output: Receiving<AliasOutput>) : AliasOutput {
        transfer::receive(parent, output)
    }

    // === Private Functions ===

    /// Loads the `Alias` object from the dynamic object field.
    fun load_alias(output: &mut AliasOutput): Alias {
        dynamic_object_field::remove(&mut output.id, ALIAS_NAME)
    }

    // === Test Functions ===

    #[test_only]
    public fun create_for_testing(
        iota: Balance<SUI>,
        native_tokens: Bag,
        ctx: &mut TxContext
    ): AliasOutput  {
        AliasOutput {
            id: object::new(ctx),
            iota,
            native_tokens,
        }
    }

    #[test_only]
    public fun attach_alias(output: &mut AliasOutput, alias: Alias) {
        dynamic_object_field::add(&mut output.id, ALIAS_NAME, alias)
    }
}
