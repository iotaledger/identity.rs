module identity_iota::migration {
    use identity_iota::{migration_registry::MigrationRegistry, identity};
    use stardust::alias_output::{AliasOutput, extract_assets};
    use sui::coin;

    const ENotADidOutput: u64 = 1;

    /// Creates a new `Document` from an Iota 1.0 legacy `AliasOutput`.
    public fun migrate_alias_output(alias_output: AliasOutput, migration_registry: &mut MigrationRegistry, ctx: &mut TxContext) {
        // Extract required data from output.
        let (iota, native_tokens, alias_data) = extract_assets(alias_output);
        let (
            alias_id,
            _,
            _,
            mut state_metadata,
            _,
            _,
            _,
            _,
        ) = alias_data.destructure();
        // Check if `state_metadata` contains a DID document.
        assert!(identity::is_did_output(state_metadata.borrow()), ENotADidOutput);
        let legacy_id = alias_id.to_inner();
        // Destroy alias.
        object::delete(alias_id);

        let document = identity::new(state_metadata.extract(), ctx);
        let coin = coin::from_balance(iota, ctx);
        transfer::public_transfer(coin, document.id().to_address());
        transfer::public_transfer(native_tokens, document.id().to_address());

        // Add a migration record.
        migration_registry.add(legacy_id, document);        
    }
}