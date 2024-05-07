module identity_iota::controller {

    public struct ControllerCap has key, store {
        id: UID,
        /// The DID this capability has control over.
        did: ID,
    }

    public(package) fun new(did: ID, ctx: &mut TxContext): ControllerCap {
        ControllerCap {
            id: object::new(ctx),
            did,
        }
    }

    public fun did(self: &ControllerCap): ID {
        self.did
    }
}