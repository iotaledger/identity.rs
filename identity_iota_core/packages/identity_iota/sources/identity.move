module identity_iota::identity {
    use iota::{vec_map::{Self, VecMap}, transfer::Receiving};
    use identity_iota::{
        multicontroller::{Self, ControllerCap, Multicontroller, Action},
        update_value_proposal,
        config_proposal,
        transfer_proposal::{Self, Send},
        borrow_proposal::{Self, Borrow},
        did_deactivation_proposal::{Self, DidDeactivation},
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

    public fun approve_proposal<T: store>(
        self: &mut Identity,
        cap: &ControllerCap,
        proposal_id: ID,
    ) {
        self.did_doc.approve_proposal<vector<u8>, T>(cap, proposal_id);
    }

    public fun propose_deactivation(
        self: &mut Identity,
        cap: &ControllerCap,
        expiration: Option<u64>,
        ctx: &mut TxContext,
    ): Option<ID> {
        let proposal_id = self.did_doc.create_proposal(
            cap,
            did_deactivation_proposal::new(),
            expiration,
            ctx,
        );
        let is_approved = self
            .did_doc
            .is_proposal_approved<_, did_deactivation_proposal::DidDeactivation>(proposal_id);
        if (is_approved) {
            self.execute_deactivation(cap, proposal_id, ctx);
            option::none()
        } else {
            option::some(proposal_id)
        }
    }

    public fun execute_deactivation(
        self: &mut Identity,
        cap: &ControllerCap,
        proposal_id: ID,
        ctx: &mut TxContext,
    ) {
        let _ = self.did_doc.execute_proposal<vector<u8>, DidDeactivation>(
            cap,
            proposal_id,
            ctx,
        ).unwrap();
        self.did_doc.set_controlled_value(vector[]);
    }

    public fun propose_update(
        self: &mut Identity,
        cap: &ControllerCap,
        updated_doc: vector<u8>,
        expiration: Option<u64>,
        ctx: &mut TxContext,
    ): Option<ID> {
        assert!(is_did_output(&updated_doc), ENotADidDocument);
        let proposal_id = update_value_proposal::propose_update(
            &mut self.did_doc,
            cap,
            updated_doc,
            expiration,
            ctx,
        );

        let is_approved = self
            .did_doc
            .is_proposal_approved<_, update_value_proposal::UpdateValue<vector<u8>>>(proposal_id);
        if (is_approved) {
            self.execute_update(cap, proposal_id, ctx);
            option::none()
        } else {
            option::some(proposal_id)
        }
    }

    public fun execute_update(
        self: &mut Identity,
        cap: &ControllerCap,
        proposal_id: ID,
        ctx: &mut TxContext,
    ) {
        update_value_proposal::execute_update(
            &mut self.did_doc,
            cap,
            proposal_id,
            ctx,
        );
    }

    public fun propose_config_change(
        self: &mut Identity,
        cap: &ControllerCap,
        expiration: Option<u64>,
        threshold: Option<u64>,
        controllers_to_add: VecMap<address, u64>,
        controllers_to_remove: vector<ID>,
        controllers_to_update: VecMap<ID, u64>,
        ctx: &mut TxContext,
    ): Option<ID> {
        let proposal_id = config_proposal::propose_modify(
            &mut self.did_doc,
            cap,
            expiration,
            threshold,
            controllers_to_add,
            controllers_to_remove,
            controllers_to_update,
            ctx
        );

        let is_approved = self
            .did_doc
            .is_proposal_approved<_, config_proposal::Modify>(proposal_id);
        if (is_approved) {
            self.execute_config_change(cap, proposal_id, ctx);
            option::none()
        } else {
            option::some(proposal_id)
        }
    }

    public fun execute_config_change(
        self: &mut Identity,
        cap: &ControllerCap,
        proposal_id: ID,
        ctx: &mut TxContext
    ) {
        config_proposal::execute_modify(
            &mut self.did_doc,
            cap,
            proposal_id,
            ctx,
        )
    }

    public fun propose_send(
        self: &mut Identity,
        cap: &ControllerCap,
        expiration: Option<u64>,
        objects: vector<ID>,
        recipients: vector<address>,
        ctx: &mut TxContext,
    ) {
        transfer_proposal::propose_send(
            &mut self.did_doc,
            cap,
            expiration,
            objects,
            recipients,
            ctx
        );
    }

    public fun execute_send<T: key + store>(
        self: &mut Identity,
        send_action: &mut Action<Send>,
        receiving: Receiving<T>,
    ) {
        transfer_proposal::send(send_action, &mut self.id, receiving);
    }

    public fun propose_borrow(
        self: &mut Identity,
        cap: &ControllerCap,
        expiration: Option<u64>,
        objects: vector<ID>,
        ctx: &mut TxContext,
    ) {
      let identity_address = self.id().to_address();
      borrow_proposal::propose_borrow(
        &mut self.did_doc,
        cap,
        expiration,
        objects,
        identity_address,
        ctx,
      );
    }

    public fun execute_borrow<T: key + store>(
        self: &mut Identity,
        borrow_action: &mut Action<Borrow>,
        receiving: Receiving<T>,
    ): T {
        borrow_proposal::borrow(borrow_action, &mut self.id, receiving)
    }

    public fun propose_new_controller(
        self: &mut Identity,
        cap: &ControllerCap,
        expiration: Option<u64>,
        new_controller_addr: address,
        voting_power: u64,
        ctx: &mut TxContext, 
    ): Option<ID> {
        let mut new_controllers = vec_map::empty();
        new_controllers.insert(new_controller_addr, voting_power);

        self.propose_config_change(cap, expiration, option::none(), new_controllers, vector[], vec_map::empty(), ctx)
    }

    public fun execute_proposal<T: store>(
        self: &mut Identity,
        cap: &ControllerCap,
        proposal_id: ID,
        ctx: &mut TxContext,
    ): Action<T> {
        self.did_doc.execute_proposal(cap, proposal_id, ctx)
    }

    /// Checks if `data` is a state matadata representing a DID.
    /// i.e. starts with the bytes b"DID".
    public(package) fun is_did_output(data: &vector<u8>): bool {
        data[0] == 0x44 &&      // b'D'
            data[1] == 0x49 &&  // b'I'
            data[2] == 0x44     // b'D'
    }

    public(package) fun did_doc(self: &Identity): &Multicontroller<vector<u8>> {
        &self.did_doc
    }

    #[test_only]
    public(package) fun to_address(self: &Identity): address {
        self.id().to_inner().id_to_address()
    }
}


#[test_only]
module identity_iota::identity_tests {
    use iota::test_scenario;
    use identity_iota::identity::{new, ENotADidDocument, Identity, new_with_controllers};
    use identity_iota::config_proposal::Modify;
    use identity_iota::multicontroller::{ControllerCap, EExpiredProposal, EThresholdNotReached};
    use iota::vec_map;

    #[test]
    fun adding_a_controller_works() {
        let controller1 = @0x1;
        let controller2 = @0x2;
        let mut scenario = test_scenario::begin(controller1);

        // Create a DID document with no funds and 1 controller with a weight of 1 and a threshold of 1.
        // Share the document and send the controller capability to `controller1`.
        let identity = new(b"DID", scenario.ctx());
        transfer::public_share_object(identity);

        scenario.next_tx(controller1);

        // Create a request to add a second controller.
        let mut identity = scenario.take_shared<Identity>();
        let controller1_cap = scenario.take_from_address<ControllerCap>(controller1);
        // This is carried out immediately.
        identity.propose_new_controller(&controller1_cap, option::none(), controller2, 1, scenario.ctx());

        scenario.next_tx(controller2);

        let controller2_cap = scenario.take_from_address<ControllerCap>(controller2);

        identity.did_doc().assert_is_member(&controller2_cap);

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

        let proposal_id = identity.propose_config_change(
            &controller1_cap,
            option::none(),
            option::none(),
            vec_map::empty(),
            vector[controller3_cap.id().to_inner()],
            vec_map::empty(),
            scenario.ctx()
        ).destroy_some();

        scenario.next_tx(controller2);

        // `controller2` also approves the removal of `controller3`.
        let controller2_cap = scenario.take_from_address<ControllerCap>(controller2);
        identity.approve_proposal<Modify>(&controller2_cap, proposal_id);

        scenario.next_tx(controller2);

        // `controller3` is removed.
        identity.execute_config_change(&controller2_cap, proposal_id, scenario.ctx());
        assert!(!identity.did_doc().controllers().contains(&controller3_cap.id().to_inner()), 0);

        // cleanup.
        test_scenario::return_to_address(controller1, controller1_cap);
        test_scenario::return_to_address(controller2, controller2_cap);
        test_scenario::return_to_address(controller3, controller3_cap);
        test_scenario::return_shared(identity);

        let _ = scenario.end();
    }

    #[test, expected_failure(abort_code = EThresholdNotReached)]
    fun test_controller_addition_fails_when_threshold_not_met() {
        let controller_a = @0x1;
        let controller_b = @0x2;
        let controller_c = @0x3;

        // The controller that is not part of the ACL.
        let controller_d = @0x4;

        let mut scenario = test_scenario::begin(controller_a);

        let mut controllers = vec_map::empty();
        controllers.insert(controller_a, 10);
        controllers.insert(controller_b, 5);
        controllers.insert(controller_c, 5);

        // === First transaction ===
        // Controller A can execute config changes
        {
            let identity = new_with_controllers(
                b"DID",
                controllers,
                10,
                scenario.ctx(),
            );
            transfer::public_share_object(identity);
            scenario.next_tx(controller_a);

            // Controller A alone should be able to do anything.
            let mut identity = scenario.take_shared<Identity>();
            let controller_a_cap = scenario.take_from_address<ControllerCap>(controller_a);

            // Create a request to add a new controller. This is carried out immediately as controller_a has enough voting power
            identity.propose_new_controller(&controller_a_cap, option::none(), controller_d, 1, scenario.ctx());

            scenario.next_tx(controller_d);

            let controller_d_cap = scenario.take_from_address<ControllerCap>(controller_d);

            identity.did_doc().assert_is_member(&controller_d_cap);

            test_scenario::return_shared(identity);
            test_scenario::return_to_address(controller_a, controller_a_cap);
            test_scenario::return_to_address(controller_d, controller_d_cap);
        };


        // Controller B alone should not be able to make changes.
        {
            let identity = new_with_controllers(
            b"DID",
            controllers,
            10,
            scenario.ctx(),
            );
            transfer::public_share_object(identity);
            scenario.next_tx(controller_a);

            let mut identity = scenario.take_shared<Identity>();
            let controller_b_cap = scenario.take_from_address<ControllerCap>(controller_b);

            let proposal_id = identity.propose_new_controller(&controller_b_cap, option::none(), controller_d, 1, scenario.ctx()).destroy_some();

            scenario.next_tx(controller_b);
            identity.execute_config_change(&controller_b_cap, proposal_id, scenario.ctx());
            scenario.next_tx(controller_d);

            let controller_d_cap = scenario.take_from_address<ControllerCap>(controller_d);
            assert!(!identity.did_doc().controllers().contains(&controller_d_cap.id().to_inner()), 0);

            test_scenario::return_to_address(controller_b, controller_b_cap);
            test_scenario::return_to_address(controller_d, controller_d_cap);
            test_scenario::return_shared(identity);
        };
        let _ = scenario.end();
    }

    #[test]
    fun test_controller_addition_works_when_threshold_met() {
        let controller_a = @0x1;
        let controller_b = @0x2;
        let controller_c = @0x3;

        // The controller that is not part of the ACL.
        let controller_d = @0x4;

        let mut scenario = test_scenario::begin(controller_b);

        let mut controllers = vec_map::empty();
        controllers.insert(controller_a, 10);
        controllers.insert(controller_b, 5);
        controllers.insert(controller_c, 5);

        // === First transaction ===
        // Controller B & C can execute config changes
        let identity = new_with_controllers(
            b"DID",
            controllers,
            10,
            scenario.ctx(),
        );
        transfer::public_share_object(identity);
        scenario.next_tx(controller_b);

        let mut identity = scenario.take_shared<Identity>();
        let controller_b_cap = scenario.take_from_address<ControllerCap>(controller_b);

        // Create a request to add a new controller.
        let proposal_id = identity.propose_new_controller(&controller_b_cap, option::none(), controller_d, 10, scenario.ctx()).destroy_some();

        scenario.next_tx(controller_b);
        let controller_c_cap = scenario.take_from_address<ControllerCap>(controller_c);
        identity.approve_proposal<Modify>(&controller_c_cap, proposal_id);

        scenario.next_tx(controller_a);
        identity.execute_config_change(&controller_c_cap, proposal_id, scenario.ctx());

        scenario.next_tx(controller_d);

        let controller_d_cap = scenario.take_from_address<ControllerCap>(controller_d);

        identity.did_doc().assert_is_member(&controller_d_cap);

        test_scenario::return_shared(identity);
        test_scenario::return_to_address(controller_b, controller_b_cap);
        test_scenario::return_to_address(controller_c, controller_c_cap);
        test_scenario::return_to_address(controller_d, controller_d_cap);

        let _ = scenario.end();

    }

    #[test]
    fun check_identity_can_own_another_identity() {
        let controller_a = @0x1;
        let mut scenario = test_scenario::begin(controller_a);

        let first_identity = new(b"DID", scenario.ctx());
        transfer::public_share_object(first_identity);

        scenario.next_tx(controller_a);
        let first_identity = scenario.take_shared<Identity>();

        let mut controllers = vec_map::empty();
        controllers.insert(first_identity.to_address(), 10);

        // Create a second identity.
        let second_identity = new_with_controllers(
            b"DID",
            controllers,
            10,
            scenario.ctx(),
        );

        transfer::public_share_object(second_identity);

        scenario.next_tx(first_identity.to_address());
        let first_identity_cap = scenario.take_from_address<ControllerCap>(first_identity.to_address());

        let mut second_identity = scenario.take_shared<Identity>();

        assert!(second_identity.did_doc().controllers().contains(&first_identity_cap.id().to_inner()), 0);
        
        second_identity.propose_new_controller(&first_identity_cap, option::none(), controller_a, 10, scenario.ctx()).destroy_none();

        scenario.next_tx(controller_a);
        let controller_a_cap = scenario.take_from_address<ControllerCap>(controller_a);

        second_identity.did_doc().assert_is_member(&controller_a_cap);

        test_scenario::return_shared(second_identity);
        test_scenario::return_to_address(controller_a, controller_a_cap);
        test_scenario::return_to_address(first_identity.to_address(), first_identity_cap);
        test_scenario::return_shared(first_identity);

        let _ = scenario.end();
    }

    #[test, expected_failure(abort_code = ENotADidDocument)]
    fun test_update_proposal_cannot_propose_non_did_doc() {
        let controller = @0x1;
        let mut scenario = test_scenario::begin(controller);

        let identity = new(b"DID", scenario.ctx());
        transfer::public_share_object(identity);

        scenario.next_tx(controller);

        // Propose a change for updating the did document
        let mut identity = scenario.take_shared<Identity>();
        let cap = scenario.take_from_address<ControllerCap>(controller);

        let _proposal_id = identity.propose_update(&cap, b"NOT DID", option::none(), scenario.ctx());

        test_scenario::return_to_address(controller, cap);
        test_scenario::return_shared(identity);

        scenario.end();
    }

    #[test, expected_failure(abort_code = EExpiredProposal)]
    fun expired_proposals_cannot_be_executed() {
        let controller_a = @0x1;
        let controller_b = @0x2;
        let new_controller = @0x3;
        let mut scenario = test_scenario::begin(controller_a);
        let expiration_epoch = scenario.ctx().epoch();
        
        let mut controllers = vec_map::empty();
        controllers.insert(controller_a, 1);
        controllers.insert(controller_b, 1);

        let identity = new_with_controllers(b"DID", controllers, 2, scenario.ctx());
        transfer::public_share_object(identity);

        scenario.next_tx(controller_a);

        let mut identity = scenario.take_shared<Identity>();
        let cap = scenario.take_from_address<ControllerCap>(controller_a);
        let proposal_id = identity.propose_new_controller(&cap, option::some(expiration_epoch), new_controller, 1, scenario.ctx()).destroy_some();

        scenario.next_tx(controller_b);
        let cap_b = scenario.take_from_address<ControllerCap>(controller_b);
        identity.approve_proposal<Modify>(&cap_b, proposal_id);

        scenario.later_epoch(100, controller_a);
        // this should fail!
        identity.execute_config_change(&cap, proposal_id, scenario.ctx());

        test_scenario::return_to_address(controller_a, cap);
        test_scenario::return_to_address(controller_b, cap_b);
        test_scenario::return_shared(identity);

        scenario.end();
    }
}
