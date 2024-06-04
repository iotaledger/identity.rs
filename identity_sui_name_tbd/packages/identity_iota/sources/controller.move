module identity_iota::controller {

    const EWeightCannotBeZero: u64 = 0; 

    const DEFAULT_WEIGHT: u64 = 1;

    public struct ControllerCap has key {
        id: UID,
        /// The DID this capability has control over.
        did: ID,
        /// Voting weight of this capability.
        weight: u64,
    }

    public(package) fun new(did: ID, ctx: &mut TxContext): ControllerCap {
        ControllerCap {
            id: object::new(ctx),
            did,
            weight: DEFAULT_WEIGHT,
        }
    }

    public(package) fun transfer(self: ControllerCap, recipient: address) {
        transfer::transfer(self, recipient);
    }

    /// Creates a new capability for document `did`, with a voting weight of `weight`.
    public(package) fun new_with_weight(did: ID, weight: u64, ctx: &mut TxContext): ControllerCap {
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

    public fun weight(self: &ControllerCap): u64 {
        self.weight
    }
}