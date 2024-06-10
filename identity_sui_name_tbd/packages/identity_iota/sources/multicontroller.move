module identity_iota::multicontroller {
    use sui::{dynamic_field as df, vec_map::{Self, VecMap}, vec_set::{Self, VecSet}};
    use std::string::String;

    const EInvalidController: u64 = 0;
    const EControllerAlreadyVoted: u64 = 1;    
    const EThresholdNotReached: u64 = 2;
    const EInvalidThreshold: u64 = 3;
    const EExpiredProposal: u64 = 4;
    const ENotVotedYet: u64 = 5;

    public struct ControllerCap has key {
        id: UID,
    }

    public fun id(self: &ControllerCap): &UID {
        &self.id
    }

    public struct Multicontroller<V> has store {
        threshold: u64,
        controllers: VecMap<ID, u64>,
        proposals: VecMap<String, Proposal>,
        controlled_value: V,
    }

    public fun new<V>(controlled_value: V, ctx: &mut TxContext): Multicontroller<V> {
        new_with_controller(controlled_value, ctx.sender(), ctx)
    }

    public fun new_with_controller<V>(
        controlled_value: V,
        controller: address,
        ctx: &mut TxContext
    ): Multicontroller<V> {
        let mut controllers = vec_map::empty();
        controllers.insert(controller, 1);

        new_with_controllers(controlled_value, controllers, 1, ctx)
    }

    public fun new_with_controllers<V>(
        controlled_value: V,
        controllers: VecMap<address, u64>,
        threshold: u64,
        ctx: &mut TxContext,
    ): Multicontroller<V> {
        let (mut addrs, mut vps) = controllers.into_keys_values();
        let mut controllers = vec_map::empty();
        while(!addrs.is_empty()) {
            let addr = addrs.pop_back();
            let vp = vps.pop_back();

            let cap = ControllerCap { id: object::new(ctx) };
            controllers.insert(cap.id.to_inner(), vp);

            transfer::transfer(cap, addr);
        };

        let mut multi = Multicontroller {
            controlled_value,
            controllers,
            threshold,
            proposals: vec_map::empty(),
        };
        multi.set_threshold(threshold);

        multi
    }

    public struct Proposal has key, store {
        id: UID,
        votes: u64,
        voters: VecSet<ID>,
        expiration_epoch: Option<u64>,
    }

    public fun is_expired(self: &Proposal, ctx: &mut TxContext): bool {
        if (self.expiration_epoch.is_some()) {
            let expiration = *self.expiration_epoch.borrow();
            expiration < ctx.epoch()
        } else {
            false
        }
    }

    public struct Action<T: store> {
        inner: T,
    }

    public(package) fun action_mut<T: store>(action: &mut Action<T>): &mut T {
        &mut action.inner
    }

    public struct ActionKey has copy, store, drop {}

    public(package) fun assert_is_member<V>(multi: &Multicontroller<V>, cap: &ControllerCap) {
        assert!(multi.controllers.contains(&cap.id.to_inner()), EInvalidController);
    }

    public fun create_proposal<V, T: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        action: T,
        key: String,
        expiration_epoch: Option<u64>,
        ctx: &mut TxContext,
    ) {
        multi.assert_is_member(cap);
        let cap_id = cap.id.to_inner();
        let voting_power = multi.voting_power(cap_id);

        let mut proposal = Proposal {
            id: object::new(ctx),
            votes: voting_power,
            voters: vec_set::singleton(cap.id.to_inner()),
            expiration_epoch
        };

        df::add(&mut proposal.id, ActionKey {}, action);

        multi.proposals.insert(key, proposal);
    }

    public fun approve_proposal<V>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        key: String,
    ) {
        multi.assert_is_member(cap);
        let cap_id = cap.id.to_inner();
        let voting_power = multi.voting_power(cap_id);

        let proposal = multi.proposals.get_mut(&key); 
        assert!(!proposal.voters.contains(&cap_id), EControllerAlreadyVoted);

        proposal.votes = proposal.votes + voting_power;
        proposal.voters.insert(cap_id);
    }

    public fun execute_proposal<V, T: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        key: String,
        ctx: &mut TxContext,
    ): Action<T> {
        multi.assert_is_member(cap);

        let (_, proposal) = multi.proposals.remove(&key);
        assert!(proposal.votes >= multi.threshold, EThresholdNotReached);
        assert!(!proposal.is_expired(ctx), EExpiredProposal);

        let Proposal {
            mut id,
            votes: _,
            voters: _,
            expiration_epoch: _,
        } = proposal;

        let inner = df::remove(&mut id, ActionKey {});
        id.delete();

        Action { inner }
    }

    public fun remove_approval<V>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        key: String,
    ) {
        let cap_id = cap.id.to_inner();
        let vp = multi.voting_power(cap_id);

        let proposal = multi.proposals.get_mut(&key);
        assert!(proposal.voters.contains(&cap_id), ENotVotedYet);

        proposal.voters.remove(&cap_id);
        proposal.votes = proposal.votes - vp;
    }

    public fun controllers<V>(multi: &Multicontroller<V>): vector<ID> {
        multi.controllers.keys()
    }

    public fun threshold<V>(multi: &Multicontroller<V>): u64 {
        multi.threshold
    }

    public fun voting_power<V>(multi: &Multicontroller<V>, controller_id: ID): u64 {
        *multi.controllers.get(&controller_id)
    }

    public fun max_votes<V>(multi: &Multicontroller<V>): u64 {
        let (_, mut values) = multi.controllers.into_keys_values();
        let mut sum = 0;
        while (!values.is_empty()) {
            sum = sum + values.pop_back();
        };

        sum
    }

    public(package) fun unpack_action<T: store>(action: Action<T>): T {
        let Action { inner } = action;
        inner
    }

    public(package) fun add_members<V>(multi: &mut Multicontroller<V>, to_add: VecMap<address, u64>, ctx: &mut TxContext) {
        let mut i = 0;
        while (i < to_add.size()) {
            let (addr, vp) = to_add.get_entry_by_idx(i);
            let new_cap = ControllerCap { id: object::new(ctx) };
            multi.controllers.insert(new_cap.id.to_inner(), *vp);
            transfer::transfer(new_cap, *addr);
            i = i + 1;
        }
    }

    public(package) fun remove_members<V>(multi: &mut Multicontroller<V>, mut to_remove: vector<ID>) {
        while (!to_remove.is_empty()) {
            let id = to_remove.pop_back();
            multi.controllers.remove(&id);
        }
    }

    public(package) fun set_threshold<V>(multi: &mut Multicontroller<V>, threshold: u64) {
        assert!(threshold <= multi.max_votes(), EInvalidThreshold);
        multi.threshold = threshold;
    }

    public(package) fun set_controlled_value<V: store + drop>(multi: &mut Multicontroller<V>, controlled_value: V) {
        multi.controlled_value = controlled_value;
    }
    public fun get_value<V: store>(multi: &Multicontroller<V>): &V {
        &multi.controlled_value
    }
}