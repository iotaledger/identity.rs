module identity_iota::migration {
    use identity_iota::{migration_registry::MigrationRegistry, identity};
    use stardust::{alias::Alias, alias_output::AliasOutput};
    use iota::{coin, iota::IOTA};

    const ENotADidOutput: u64 = 1;

    public fun migrate_alias(
        alias: Alias,
        migration_registry: &mut MigrationRegistry,
        ctx: &mut TxContext,
    ): address {
        // Extract needed data from `alias`.
        let alias_id = object::id(&alias);
        let mut state_metadata = *alias.state_metadata();
        // `alias` is not needed anymore, destroy it.
        alias.destroy();

        // Check if `state_metadata` contains a DID document.
        assert!(state_metadata.is_some() && identity::is_did_output(state_metadata.borrow()), ENotADidOutput);

        let identity = identity::new(state_metadata.extract(), ctx);
        let identity_addr = identity.id().to_address();

        // Add a migration record.
        migration_registry.add(alias_id, identity.id().to_inner());
        transfer::public_share_object(identity);

        identity_addr
    }

    /// Migrates a legacy `AliasOutput` containing a DID Document to an `Identity`.
    public fun migrate_alias_output(
        alias_output: AliasOutput<IOTA>, 
        migration_registry: &mut MigrationRegistry, 
        ctx: &mut TxContext
    ) {
        // Extract required data from output.
        let (iota, native_tokens, alias_data) = alias_output.extract_assets();

        let identity_addr = migrate_alias(alias_data, migration_registry, ctx);

        let coin = coin::from_balance(iota, ctx);
        transfer::public_transfer(coin, identity_addr);
        transfer::public_transfer(native_tokens, identity_addr);
    }
}


#[test_only]
module identity_iota::migration_tests {
    use iota::{test_scenario, balance, bag, iota::IOTA};
    use stardust::alias_output::{Self, AliasOutput};
    use identity_iota::identity::{Identity};
    use identity_iota::migration::migrate_alias_output;
    use stardust::alias::{Self, Alias};
    use identity_iota::migration_registry::{MigrationRegistry, init_testing};
    use identity_iota::multicontroller::ControllerCap;

    fun create_did_alias(ctx: &mut TxContext): Alias {
        let sender = ctx.sender();
        alias::create_for_testing(
            sender,
            1,
            option::some(b"DID"),
            option::some(sender),
            option::none(),
            option::none(),
            option::none(),
            ctx
        )
    } 
    
    fun create_empty_did_output(ctx: &mut TxContext): (AliasOutput<IOTA>, ID) {
        let mut alias_output = alias_output::create_for_testing(balance::zero(), bag::new(ctx), ctx);
        let alias = create_did_alias(ctx);
        let alias_id = object::id(&alias);
        alias_output.attach_alias(alias);

        (alias_output, alias_id)
    }

    #[test]
    fun test_migration_of_legacy_did_output() {
        let controller_a = @0x1;
        let mut scenario = test_scenario::begin(controller_a);
        
        let (did_output, alias_id) = create_empty_did_output(scenario.ctx());

        init_testing(scenario.ctx());

        scenario.next_tx(controller_a);
        let mut registry = scenario.take_shared<MigrationRegistry>();

        migrate_alias_output(did_output, &mut registry, scenario.ctx());

        scenario.next_tx(controller_a);
        let identity = scenario.take_shared<Identity>();
        let controller_a_cap = scenario.take_from_address<ControllerCap>(controller_a);

        // Assert correct binding in migration regitry
        assert!(registry.lookup(alias_id) == identity.id().to_inner(), 0);

        // Assert the sender is controller
        identity.did_doc().assert_is_member(&controller_a_cap);

        // assert the metadata is b"DID"
        let did = identity.did_doc().value();
        assert!(did == b"DID", 0);

        test_scenario::return_to_address(controller_a, controller_a_cap);
        test_scenario::return_shared(registry);
        test_scenario::return_shared(identity);
        let _ = scenario.end();
    }
}
