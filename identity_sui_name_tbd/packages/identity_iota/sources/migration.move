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

#[test_only]
module identity_iota::migration_tests {
    use sui::test_scenario;
    use stardust::alias_output::{create_for_testing, create_empty_for_testing, extract_assets, AliasOutput, load_alias, attach_alias};
    use sui::{balance, bag, sui::SUI};
    use identity_iota::migration::migrate_alias_output;
    use std::debug;
    use sui::dynamic_object_field;
    use stardust::alias::{Alias, create_with_state_metadata_for_testing};
    use identity_iota::migration_registry::{MigrationRegistry, init_testing};
    // migration of legacy did_output - create a did_output (look at the script and replicate it in a move test), migrate it and make sure MigrationRegistry::lookup(old_id) resolves to an Identity
    #[test]
    fun test_migration_of_legacy_did_output() {
        let controller_a = @0x1;
        let mut scenario = test_scenario::begin(controller_a);

        let alias_output = create_empty_for_testing(scenario.ctx());
        transfer::public_share_object(alias_output);

        scenario.next_tx(controller_a);

        let mut alias_output = scenario.take_shared<AliasOutput>();

        let alias: Alias = create_with_state_metadata_for_testing(
            option::none(),
            1,
            b"DID",
            option::none(),
            option::none(),
            option::none(),
            option::none(),
            scenario.ctx()
        );
        transfer::public_share_object(alias);

        scenario.next_tx(controller_a);
        let alias = scenario.take_shared<Alias>();
        let id = alias.to_id();

        scenario.next_tx(controller_a);
        init_testing(scenario.ctx());

        scenario.next_tx(controller_a);
        let mut registry = scenario.take_shared<MigrationRegistry>();

        alias_output.attach_alias(alias);
        migrate_alias_output(alias_output, &mut registry, scenario.ctx());

        assert!(registry.exists(id.to_inner()), 1);

        test_scenario::return_shared(registry);
        let _ = scenario.end();
    }
}