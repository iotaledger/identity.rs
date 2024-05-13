module identity_iota::migration_registry {
    use sui::{dynamic_field as field, transfer::share_object, event};

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

    /// Lookup an alias ID into the migration registry.
    public fun lookup(self: &MigrationRegistry, alias_id: ID): Option<ID> {
        if (field::exists_(&self.id, alias_id)) {
            let entry = field::borrow<ID, ID>(&self.id, alias_id);
            option::some(*entry)
        } else {
            option::none()
        }
    }

    /// Adds a new Alias ID -> Object ID binding to the regitry.
    public(package) fun add(self: &mut MigrationRegistry, alias_id: ID, object_id: ID) {
        field::add(&mut self.id, alias_id, object_id);
    }
}