module identity_iota::document {
    use sui::{balance::Balance, bag::Bag, sui::SUI, vec_set::VecSet, vec_set};
    use identity_iota::{
        controller::{Self, ControllerCap},
        add_controller_request::{Self, AddControllerRequest},
        remove_controller_request::{Self, RemoveControllerRequest},
    };

    const EInvalidCapability: u64 = 1;
    /// The threshold specified upon document creation was not valid.
    /// Threshold must be greater than or equal to 1.
    const EInvalidThreshold: u64 = 2;
    /// The controller list must contain at least 1 element.
    const EInvalidControllersList: u64 = 3;
    const EInvalidWeight: u64 = 4;
    const EInvalidRequest: u64 = 5;

    /// DID document.
    public struct Document has key, store {
        id: UID,
        doc: vector<u8>,
        iota: Balance<SUI>,
        native_tokens: Bag,
        /// Minimum amount of votes required to perform a change.
        threshold: u32,
        /// Set of capability's IDs tied to this DID document.
        controllers: VecSet<ID>,
    }

    /// Creates a new DID Document with a single controller.
    public fun new(doc: vector<u8>, iota: Balance<SUI>, native_tokens: Bag, ctx: &mut TxContext): (Document, ControllerCap) {
        let doc_id = object::new(ctx);
        let controller = controller::new(doc_id.to_inner(), ctx);
        let doc = Document {
            id: doc_id,
            doc,
            iota,
            native_tokens,
            threshold: 1,
            controllers: vec_set::singleton(controller.id().to_inner()),
        };
        
        (doc, controller)
    }

    /// Creates a new DID Document controlled by multiple controllers.
    /// The `weights` vectors is used to create a vector of `ControllerCap`s `controller_caps`,
    /// where `controller_caps[i].weight = weights[i]` for all `i`s in `[0, weights.length())`.
    public fun new_with_controllers(
        doc: vector<u8>,
        iota: Balance<SUI>,
        native_tokens: Bag,
        threshold: u32,
        mut weights: vector<u32>,
        ctx: &mut TxContext,
    ): (Document, vector<ControllerCap>) {
        assert!(threshold >= 1, EInvalidThreshold);
        assert!(weights.length() >= 1, EInvalidControllersList);

        let doc_uid = object::new(ctx);
        let doc_id = doc_uid.to_inner();

        let mut controllers = vec_set::empty();
        let mut controller_caps = vector::empty();
        while (!weights.is_empty()) {
            let weight = weights.pop_back();
            let cap = controller::new_with_weight(doc_id, weight, ctx);
            controllers.insert(cap.id().to_inner());
            controller_caps.push_back(cap);
        };

        let document = Document {
            id: doc_uid,
            doc,
            iota,
            native_tokens,
            threshold,
            controllers
        };

        (document, controller_caps)
    }

    public fun threshold(self: &Document): u32 {
        self.threshold
    }

    public fun is_capability_valid(self: &Document, cap: &ControllerCap): bool {
        self.id.to_inner() == cap.did() && self.controllers.contains(&cap.id().to_inner())
    }    

    /// Creates a request for adding a new controller to `self`.
    /// `weight` is the voting weight for the new controller.
    public fun request_add_controller(
        self: &mut Document,
        cap: &ControllerCap,
        weight: u32,
        ctx: &mut TxContext
    ) {
        // Check the provided capability is for this document.
        assert!(self.is_capability_valid(cap), EInvalidCapability);
        assert!(weight >= 1, EInvalidWeight);

        add_controller_request::new(cap, self.threshold, weight, ctx);
    }

    /// Creates a request for adding a new controller to `self`.
    /// `controller_to_remove` is the ID of the controller that will be removed.
    public fun request_remove_controller(
        self: &mut Document,
        cap: &ControllerCap,
        controller_to_remove: ID,
        ctx: &mut TxContext,
    ) {
        // Check the provided capability is for this document.
        assert!(self.is_capability_valid(cap), EInvalidCapability);
        assert!(self.controllers.contains(&controller_to_remove), EInvalidWeight);

        remove_controller_request::new(cap, self.threshold, controller_to_remove, ctx);
    }

    /// Consume an approved request for adding a controller, creating a new controller.
    public fun add_controller(
        self: &mut Document,
        req: AddControllerRequest,
        ctx: &mut TxContext
    ): ControllerCap {
        assert!(self.id.to_inner() == req.did() && req.is_resolved(), EInvalidRequest);

        let controller_cap = controller::new_with_weight(self.id.to_inner(), req.weight(), ctx);
        self.controllers.insert(controller_cap.id().to_inner());

        req.destroy();

        controller_cap
    }

    /// Consume an approved request for removing a controller.
    public fun remove_controller(
        self: &mut Document,
        req: RemoveControllerRequest,
    ) {
        assert!(self.id.to_inner() == req.did() && req.is_resolved(), EInvalidRequest);

        self.controllers.remove(&req.id_to_remove());

        req.destroy();
    }


    /// Checks if `data` is a state matadata representing a DID.
    /// i.e. starts with the bytes b"DID".
    public(package) fun is_did_output(data: &vector<u8>): bool {
        data[0] == 0x44 &&      // b'D'
            data[1] == 0x49 &&  // b'I'
            data[2] == 0x44     // b'D'
    }
}
