module identity_iota::migration_registry {
    use sui::{dynamic_object_field as ofield, transfer::share_object};

    /// One time witness needed to construct a singleton migration registry.
    public struct MIGRATION_REGISTRY has drop {}

    /// A wrapper `key` type for `ID`.
    public struct Entry has key, store {
        id: UID,
        target: ID,
    }

    /// Object that tracks migrated alias outputs to their corresponding object IDs.
    public struct MigrationRegistry has key {
        id: UID,
    }

    /// Creates a singleton instance of `MigrationRegistry` when publishing this package.
    fun init(_otw: MIGRATION_REGISTRY, ctx: &mut TxContext) {
        let registry = MigrationRegistry {
            id: object::new(ctx),
        };
        share_object(registry);
    }

    /// Lookup an alias ID into the migration registry.
    public fun lookup(self: &MigrationRegistry, alias_id: ID): Option<ID> {
        if (ofield::exists_(&self.id, alias_id)) {
            let entry = ofield::borrow<ID, Entry>(&self.id, alias_id);
            option::some(entry.target)
        } else {
            option::none()
        }
    }

    /// Adds a new Alias ID -> Object ID binding to the regitry.
    public(package) fun add(self: &mut MigrationRegistry, alias_id: ID, object_id: ID, ctx: &mut TxContext) {
        let entry = Entry {
            id: object::new(ctx),
            target: object_id,
        };
        ofield::add(&mut self.id, alias_id, entry);
    }
}