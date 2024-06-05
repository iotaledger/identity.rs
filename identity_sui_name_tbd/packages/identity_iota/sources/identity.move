module identity_iota::identity {
    use sui::{transfer::Receiving, vec_map::{Self, VecMap}, vec_set::VecSet};
    use std::string::String;
    use identity_iota::{
        multicontroller::{Self, Action, ControllerCap, Multicontroller},
        update_value_proposal,
        config_proposal,
        transfer_proposal::{Self, Send},
    };

    const ENotADidDocument: u64 = 0;
    /// The threshold specified upon document creation was not valid.
    /// Threshold must be greater than or equal to 1.
    const EInvalidThreshold: u64 = 2;
    /// The controller list must contain at least 1 element.
    const EInvalidControllersList: u64 = 3;

    /// On-chain Identity.
    public struct Identity has key, store {
        id: UID,
        /// same as stardust `state_metadata`.
        did_doc: Multicontroller<vector<u8>>,
    }

    /// Creates a new DID Document with a single controller.
    public fun new(doc: vector<u8>, ctx: &mut TxContext): Identity {
        new_with_controller(doc, ctx.sender(), ctx)
    }

    public fun new_with_controller(
        doc: vector<u8>,
        controller: address,
        ctx: &mut TxContext,
    ): Identity {
        Identity {
            id: object::new(ctx),
            did_doc: multicontroller::new_with_controller(doc, controller, ctx)
        }
    }

    /// Creates a new DID Document controlled by multiple controllers.
    /// The `weights` vectors is used to create a vector of `ControllerCap`s `controller_caps`,
    /// where `controller_caps[i].weight = weights[i]` for all `i`s in `[0, weights.length())`.
    public fun new_with_controllers(
        doc: vector<u8>,
        controllers: VecMap<address, u64>,
        threshold: u64, 
        ctx: &mut TxContext,
    ): Identity {
        assert!(is_did_output(&doc), ENotADidDocument);
        assert!(threshold >= 1, EInvalidThreshold);
        assert!(controllers.size() > 0, EInvalidControllersList);

        Identity {
            id: object::new(ctx),
            did_doc: multicontroller::new_with_controllers(doc, controllers, threshold, ctx),
        }
    }

    public fun id(self: &Identity): &UID {
        &self.id
    }

    public fun threshold(self: &Identity): u64 {
        self.did_doc.threshold()
    }

    public fun approve_proposal(
        self: &mut Identity,
        cap: &ControllerCap,
        name: String
    ) {
        self.did_doc.approve_proposal(cap, name);
    }

    public fun propose_update(
        self: &mut Identity,
        cap: &ControllerCap,
        name: String,
        updated_doc: vector<u8>,
        ctx: &mut TxContext,
    ) {
        update_value_proposal::propose_update(
            &mut self.did_doc,
            cap,
            name,
            updated_doc,
            ctx,
        )
    }

    public fun execute_update(
        self: &mut Identity,
        cap: &ControllerCap,
        name: String,
    ) {
        update_value_proposal::execute_update(
            &mut self.did_doc,
            cap,
            name
        );
    }

    public fun propose_config_change(
        self: &mut Identity,
        cap: &ControllerCap,
        name: String,
        threshold: Option<u64>,
        controllers_to_add: VecMap<address, u64>,
        controllers_to_remove: vector<ID>,
        ctx: &mut TxContext,
    ) {
        config_proposal::propose_modify(
            &mut self.did_doc,
            cap, 
            name, 
            threshold,
            controllers_to_add,
            controllers_to_remove,
            ctx
        )
    }

    public fun execute_config_change(
        self: &mut Identity,
        cap: &ControllerCap,
        name: String,
        ctx: &mut TxContext, 
    ) {
        config_proposal::execute_modify(
            &mut self.did_doc,
            cap,
            name,
            ctx,
        )
    }

    public fun propose_send(
        self: &mut Identity,
        cap: &ControllerCap,
        name: String,
        objects: VecSet<ID>,
        recipients: vector<address>,
        ctx: &mut TxContext,
    ) {
        transfer_proposal::propose_send(
            &mut self.did_doc,
            cap,
            name,
            objects,
            recipients,
            ctx
        );
    }

    public fun send<T: key + store>(
        self: &mut Identity,
        send_action: &mut Action<Send>,
        received: Receiving<T>, 
    ) {
        transfer_proposal::send(send_action, &mut self.id, received);
    }

    public fun propose_new_controller(
        self: &mut Identity,
        cap: &ControllerCap,
        name: String,
        new_controller_addr: address,
        voting_power: u64,
        ctx: &mut TxContext, 
    ) {
        let mut new_controllers = vec_map::empty();
        new_controllers.insert(new_controller_addr, voting_power);

        self.propose_config_change(cap, name, option::none(), new_controllers, vector[], ctx);
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
    #[test_only] use std::string;

    #[test]
    fun adding_a_controller_works() {
        let controller1 = @0x1;
        let controller2 = @0x2;
        let proposal_name = string::utf8(b"add controller2");
        let mut scenario = test_scenario::begin(controller1);

        // Create a DID document with no funds and 1 controller with a weight of 1 and a threshold of 1.
        // Share the document and send the controller capability to `controller1`.
        let identity = new(b"DID", scenario.ctx());
        transfer::public_share_object(identity);

        scenario.next_tx(controller1);

        // Create a request to add a second controller.
        let mut identity = scenario.take_shared<Identity>();
        let controller1_cap = scenario.take_from_address<ControllerCap>(controller1);
        identity.propose_new_controller(&controller1_cap, proposal_name, controller2, 1, scenario.ctx());

        // Request is fullfilled, add a second controller and send the capability to `controller2`.
        scenario.next_tx(controller1);

        identity.execute_config_change(&controller1_cap, proposal_name, scenario.ctx());

        scenario.next_tx(controller2);

        let controller2_cap = scenario.take_from_address<ControllerCap>(controller2);

        identity.did_doc.assert_is_member(&controller2_cap);

        // Cleanup
        test_scenario::return_to_address(controller1, controller1_cap);
        test_scenario::return_to_address(controller2, controller2_cap);
        test_scenario::return_shared(identity);

        let _ = scenario.end();
    }

    #[test]
    fun removing_a_controller_works() {
        let controller1 = @0x1;
        let controller2 = @0x2;
        let controller3 = @0x3;
        let proposal_name = string::utf8(b"remove controller3");
        let mut scenario = test_scenario::begin(controller1);

        let mut controllers = vec_map::empty();
        controllers.insert(controller1, 1);
        controllers.insert(controller2, 1);
        controllers.insert(controller3, 1);

        // Create an identity shared by `controller1`, `controller2`, `controller3`.
        let identity = new_with_controllers(
            b"DID",
            controllers,
            2,
            scenario.ctx(),
        );
        transfer::public_share_object(identity);

        scenario.next_tx(controller1);

        // `controller1` creates a request to remove `controller3`.
        let mut identity = scenario.take_shared<Identity>();
        let controller1_cap = scenario.take_from_address<ControllerCap>(controller1);
        let controller3_cap = scenario.take_from_address<ControllerCap>(controller3);

        identity.propose_config_change(
            &controller1_cap,
            proposal_name,
            option::none(),
            vec_map::empty(),
            vector[controller3_cap.id().to_inner()],
            scenario.ctx()
        );

        scenario.next_tx(controller2);

        // `controller2` also approves the removal of `controller3`.
        let controller2_cap = scenario.take_from_address<ControllerCap>(controller2);
        identity.approve_proposal(&controller2_cap, proposal_name);

        scenario.next_tx(controller2);

        // `controller3` is removed.
        identity.execute_config_change(&controller2_cap, proposal_name, scenario.ctx());
        assert!(!identity.did_doc.controllers().contains(&controller3_cap.id().to_inner()), 0);

        // cleanup.
        test_scenario::return_to_address(controller1, controller1_cap);
        test_scenario::return_to_address(controller2, controller2_cap);
        test_scenario::return_to_address(controller3, controller3_cap); 
        test_scenario::return_shared(identity);

        let _ = scenario.end();
    }
}