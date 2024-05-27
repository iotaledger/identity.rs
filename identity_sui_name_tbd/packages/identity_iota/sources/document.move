module identity_iota::document {
    use sui::{balance::Balance, bag::Bag, sui::SUI};
    use identity_iota::{controller::ControllerCap, controller};

    const ENotADidOutput: u64 = 1;
    const EInvalidCapability: u64 = 2;

    /// DID document.
    public struct Document has key, store {
        id: UID,
        doc: vector<u8>,
        iota: Balance<SUI>,
        native_tokens: Bag,
    }

    /// Creates a new DID Document.
    public fun new(doc: vector<u8>, iota: Balance<SUI>, native_tokens: Bag, ctx: &mut TxContext): (Document, ControllerCap) {
        let doc = Document {
            id: object::new(ctx),
            doc,
            iota,
            native_tokens
        };
        let doc_id = doc.id.to_inner();
        
        (doc, controller::new(doc_id, ctx))
    }

    /// Updates the DID document.
    public fun update(self: &mut Document, data: vector<u8>, controller_capability: &ControllerCap) {
        // Check the provided capability is for this document.
        assert!(self.id.to_inner() == controller_capability.did(), EInvalidCapability);
        // Check `data` is a DID document.
        assert!(is_did_output(&data), ENotADidOutput);
        self.doc = data;
    }

    /// Checks if `data` is a state matadata representing a DID.
    /// i.e. starts with the bytes b"DID".
    public(package) fun is_did_output(data: &vector<u8>): bool {
        data[0] == 0x44 &&      // b'D'
            data[1] == 0x49 &&  // b'I'
            data[2] == 0x44     // b'D'
    }
}