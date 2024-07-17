module identity_iota::migration_registry {
    use iota::{dynamic_object_field as field, transfer::share_object, event};
    use identity_iota::identity::Identity;

    /// One time witness needed to construct a singleton migration registry.
    public struct MIGRATION_REGISTRY has drop {}


    /// Event type that is fired upon creation of a `MigrationRegistry`.
    public struct MigrationRegistryCreated has copy, drop {
        id: ID
    }

    /// Object that tracks migrated alias outputs to their corresponding object IDs.
    public struct MigrationRegistry has key {
        id: UID,
    }

    /// Creates a singleton instance of `MigrationRegistry` when publishing this package.
    fun init(_otw: MIGRATION_REGISTRY, ctx: &mut TxContext) {
        let id = object::new(ctx);
        let registry_id = id.to_inner();
        let registry = MigrationRegistry {
            id
        };
        share_object(registry);
        // Signal the creation of a migration registry.
        event::emit(MigrationRegistryCreated { id: registry_id });
    }

    /// Checks whether the given alias ID exists in the migration registry.
    public fun exists(self: &MigrationRegistry, alias_id: ID): bool {
        field::exists_(&self.id, alias_id)
    }

    /// Lookup an alias ID into the migration registry.
    public fun borrow(self: &MigrationRegistry, alias_id: ID): &Identity {
        field::borrow<ID, Identity>(&self.id, alias_id)
    }

    /// Mutably borrow the migrated document `Document` corresponding
    /// to the provided `alias_id`, if any.
    public fun borrow_mut(self: &mut MigrationRegistry, alias_id: ID): &mut Identity {
        field::borrow_mut(&mut self.id, alias_id)
    }

    /// Adds a new Alias ID -> Object ID binding to the regitry.
    public(package) fun add(self: &mut MigrationRegistry, alias_id: ID, doc: Identity) {
        field::add(&mut self.id, alias_id, doc);
    }

    //= Test Functions
    #[test_only]
    public fun init_testing(ctx: &mut TxContext) {
        init(MIGRATION_REGISTRY {}, ctx);
    }
}
