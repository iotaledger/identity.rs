module identity_iota::migration_registry {
    use sui::{table::Table, table, transfer::share_object};

    /// One time witness needed to construct a singleton migration registry.
    public struct MIGRATION_REGISTRY has drop {}

    /// Object that tracks migrated alias outputs to their corresponding object IDs.
    public struct MigrationRegistry has key {
        id: UID,
        /// A mapping from Iota 1.0 Alias IDs to Iota 3.0 Object IDs.
        aliasIdMappings: Table<ID, ID>,
    }

    /// Creates a singleton instance of `MigrationRegistry` when publishing this package.
    fun init(_otw: MIGRATION_REGISTRY, ctx: &mut TxContext) {
        let registry = MigrationRegistry {
            id: object::new(ctx),
            aliasIdMappings: table::new<ID, ID>(ctx),
        };
        share_object(registry);
    }

    /// Lookup an alias ID into the migration registry.
    public fun lookup(self: &MigrationRegistry, alias_id: ID): Option<ID> {
        if (self.aliasIdMappings.contains(alias_id)) {
            option::some(*self.aliasIdMappings.borrow(alias_id))
        } else {
            option::none()
        }
    }

    /// Adds a new Alias ID -> Object ID binding to the regitry.
    public(package) fun add(self: &mut MigrationRegistry, alias_id: ID, object_id: ID) {
        self.add(alias_id, object_id);
    }
}