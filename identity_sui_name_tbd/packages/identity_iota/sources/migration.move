module identity_iota::migration {
    use identity_iota::{migration_registry::MigrationRegistry, document, controller::ControllerCap};
    use stardust::alias_output::{AliasOutput, extract_assets};

    const ENotADidOutput: u64 = 1;

    /// Creates a new `Document` from an Iota 1.0 legacy `AliasOutput`.
    public fun migrate_alias_output(alias_output: AliasOutput, migration_registry: &mut MigrationRegistry, ctx: &mut TxContext): ControllerCap {
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
        assert!(document::is_did_output(state_metadata.borrow()), ENotADidOutput);
        let legacy_id = alias_id.to_inner();
        // Destroy alias.
        object::delete(alias_id);

        let (document, controller_capability) = document::new(
            state_metadata.extract(),
            iota,
            native_tokens,
            ctx,
        );

        // Add a migration record.
        migration_registry.add(legacy_id, document);

        // Transfer the capability to the governor.
        controller_capability
    }
}