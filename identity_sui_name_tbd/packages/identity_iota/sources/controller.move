module identity_iota::controller {

    const EWeightCannotBeZero: u64 = 0; 

    const DEFAULT_WEIGHT: u32 = 1;

    public struct ControllerCap has key, store {
        id: UID,
        /// The DID this capability has control over.
        did: ID,
        /// Voting weight of this capability.
        weight: u32,
    }

    public(package) fun new(did: ID, ctx: &mut TxContext): ControllerCap {
        ControllerCap {
            id: object::new(ctx),
            did,
            weight: DEFAULT_WEIGHT,
        }
    }

    /// Creates a new capability for document `did`, with a voting weight of `weight`.
    public(package) fun new_with_weight(did: ID, weight: u32, ctx: &mut TxContext): ControllerCap {
        assert!(weight >= 1, EWeightCannotBeZero);

        ControllerCap {
            id: object::new(ctx),
            did,
            weight
        }
    }

    public fun id(self: &ControllerCap): &UID {
        &self.id
    }

    public fun did(self: &ControllerCap): ID {
        self.did
    }

    public fun weight(self: &ControllerCap): u32 {
        self.weight
    }
}