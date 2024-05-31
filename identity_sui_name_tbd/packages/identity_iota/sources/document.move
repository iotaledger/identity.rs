module identity_iota::document {
    use sui::{balance::{Self, Balance}, bag::{Self, Bag}, sui::SUI, vec_set::VecSet, vec_set};
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

    public fun empty(doc: vector<u8>, ctx: &mut TxContext): (Document, ControllerCap) {
        new(doc, balance::zero(), bag::new(ctx), ctx)
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

    // TESTS
    #[test_only] use sui::test_scenario;

    #[test]
    fun adding_a_controller_works() {
        let controller1 = @0x1;
        let controller2 = @0x2;
        let mut scenario = test_scenario::begin(controller1);

        // Create a DID document with no funds and 1 controller with a weight of 1 and a threshold of 1.
        // Share the document and send the controller capability to `controller1`.
        let (doc, cap) = empty(b"DID", scenario.ctx());
        transfer::public_share_object(doc);
        transfer::public_transfer(cap, controller1);

        scenario.next_tx(controller1);

        // Create a request to add a second controller.
        let mut doc = scenario.take_shared<Document>();
        let controller1_cap = scenario.take_from_address<ControllerCap>(controller1);
        doc.request_add_controller(&controller1_cap, 1, scenario.ctx());

        // Request is fullfilled, add a second controller and send the capability to `controller2`.
        scenario.next_tx(controller1);
        let req = scenario.take_shared<AddControllerRequest>();
        assert!(req.is_resolved(), 0);

        let cap = doc.add_controller(req, scenario.ctx());
        transfer::public_transfer(cap, controller2);

        scenario.next_tx(controller2);

        let controller2_cap = scenario.take_from_address<ControllerCap>(controller2);

        assert!(doc.controllers.contains(&controller2_cap.id().to_inner()), 0);

        // Cleanup
        test_scenario::return_to_address(controller1, controller1_cap);
        test_scenario::return_to_address(controller2, controller2_cap);
        test_scenario::return_shared(doc);

        let _ = scenario.end();
    }

    #[test]
    fun removing_a_controller_works() {
        let controller1 = @0x1;
        let controller2 = @0x2;
        let controller3 = @0x3;
        let mut scenario = test_scenario::begin(controller1);

        // Create a document shared by `controller1`, `controller2`, `controller3`.
        let (doc, mut caps) = new_with_controllers(
            b"DID",
            balance::zero(),
            bag::new(scenario.ctx()),
            2,
            vector[1, 1, 1],
            scenario.ctx()
        );
        let controller3_cap_id = caps[2].id().to_inner();
        // Transfer the capabilities to the owners.
        transfer::public_transfer(caps.remove(0), controller1);
        transfer::public_transfer(caps.remove(0), controller2);
        transfer::public_transfer(caps.remove(0), controller3);
        vector::destroy_empty(caps);
        transfer::public_share_object(doc);

        scenario.next_tx(controller1);

        // `controller1` creates a request to remove `controller3`.
        let mut doc = scenario.take_shared<Document>();
        let controller1_cap = scenario.take_from_address<ControllerCap>(controller1);

        doc.request_remove_controller(&controller1_cap, controller3_cap_id, scenario.ctx());

        scenario.next_tx(controller2);

        // `controller2` also approves the removal of `controller3`.
        let mut req = scenario.take_shared<RemoveControllerRequest>();
        let controller2_cap = scenario.take_from_address<ControllerCap>(controller2);

        req.approve(&controller2_cap);

        scenario.next_tx(controller2);

        // `controller3` is removed.
        doc.remove_controller(req);
        assert!(!doc.controllers.contains(&controller3_cap_id), 0);

        // cleanup.
        test_scenario::return_to_address(controller1, controller1_cap);
        test_scenario::return_to_address(controller2, controller2_cap);
        test_scenario::return_shared(doc);

        let _ = scenario.end();
    }
}