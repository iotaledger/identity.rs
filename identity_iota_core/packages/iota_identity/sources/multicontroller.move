// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_identity::multicontroller {
    use iota::{object_bag::{Self, ObjectBag}, vec_map::{Self, VecMap}, vec_set::{Self, VecSet}};

    const EInvalidController: u64 = 0;
    const EControllerAlreadyVoted: u64 = 1;    
    const EThresholdNotReached: u64 = 2;
    const EInvalidThreshold: u64 = 3;
    const EExpiredProposal: u64 = 4;
    const ENotVotedYet: u64 = 5;
    const EProposalNotFound: u64 = 6;

    /// Capability that allows to access mutative APIs of a `Multicontroller`.
    public struct ControllerCap has key {
        id: UID,
    }

    public fun id(self: &ControllerCap): &UID {
        &self.id
    }

    /// Shares control of a value `V` with multiple entities called controllers.
    public struct Multicontroller<V> has store {
        threshold: u64,
        controllers: VecMap<ID, u64>,
        controlled_value: V,
        active_proposals: vector<ID>,
        proposals: ObjectBag,
    }

    /// Wraps a `V` in `Multicontroller`, making the tx's sender a controller with
    /// voting power 1.
    public fun new<V>(controlled_value: V, ctx: &mut TxContext): Multicontroller<V> {
        new_with_controller(controlled_value, ctx.sender(), ctx)
    }

    /// Wraps a `V` in `Multicontroller` and sends `controller` a `ControllerCap`.
    public fun new_with_controller<V>(
        controlled_value: V,
        controller: address,
        ctx: &mut TxContext
    ): Multicontroller<V> {
        let mut controllers = vec_map::empty();
        controllers.insert(controller, 1);

        new_with_controllers(controlled_value, controllers, 1, ctx)
    }

    /// Wraps a `V` in `Multicontroller`, settings `threshold` as the threshold,
    /// and using `controllers` to set controllers: i.e. each `(recipient, voting power)`
    /// in `controllers` results in `recipient` obtaining a `ControllerCap` with the
    /// specified voting power.
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
            active_proposals: vector[],
            proposals: object_bag::new(ctx),
        };
        multi.set_threshold(threshold);

        multi
    }

    /// Structure that encapsulates the logic required to make changes
    /// to a multicontrolled value.
    public struct Proposal<T: store> has key, store {
        id: UID,
        votes: u64,
        voters: VecSet<ID>,
        expiration_epoch: Option<u64>,
        action: T,
    }

    /// Returns `true` if `Proposal` `self` is expired.
    public fun is_expired<T: store>(self: &Proposal<T>, ctx: &mut TxContext): bool {
        if (self.expiration_epoch.is_some()) {
            let expiration = *self.expiration_epoch.borrow();
            expiration < ctx.epoch()
        } else {
            false
        }
    }

    /// Strucure that encapsulate the kind of change that will be performed
    /// when a proposal is carried out.
    public struct Action<T: store> {
        inner: T,
    }

    /// Consumes `Action` returning the inner value.
    public fun unwrap<T: store>(action: Action<T>): T {
        let Action { inner } = action;
        inner
    }

    /// Borrows the content of `action`.
    public fun borrow<T: store>(action: &Action<T>): &T {
        &action.inner
    }

    /// Mutably borrows the content of `action`.
    public fun borrow_mut<T: store>(action: &mut Action<T>): &mut T {
        &mut action.inner
    }

    public(package) fun assert_is_member<V>(multi: &Multicontroller<V>, cap: &ControllerCap) {
        assert!(multi.controllers.contains(&cap.id.to_inner()), EInvalidController);
    }

    /// Creates a new proposal for `Multicontroller` `multi`.
    public fun create_proposal<V, T: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        action: T,
        expiration_epoch: Option<u64>,
        ctx: &mut TxContext,
    ): ID {
        multi.assert_is_member(cap);
        let cap_id = cap.id.to_inner();
        let voting_power = multi.voting_power(cap_id);

        let proposal = Proposal {
            id: object::new(ctx),
            votes: voting_power,
            voters: vec_set::singleton(cap.id.to_inner()),
            expiration_epoch,
            action,
        };

        let proposal_id = object::id(&proposal);
        multi.proposals.add(proposal_id, proposal);
        multi.active_proposals.push_back(proposal_id);
        proposal_id
    }

    /// Approves an active `Proposal` in `multi`.
    public fun approve_proposal<V, T: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        proposal_id: ID,
    ) {
        multi.assert_is_member(cap);
        let cap_id = cap.id.to_inner();
        let voting_power = multi.voting_power(cap_id);

        let proposal = multi.proposals.borrow_mut<ID, Proposal<T>>(proposal_id);
        assert!(!proposal.voters.contains(&cap_id), EControllerAlreadyVoted);

        proposal.votes = proposal.votes + voting_power;
        proposal.voters.insert(cap_id);
    }

    /// Consumes the `multi`'s active `Proposal` with id `proposal_id`,
    /// returning its inner `Action`.
    /// This call fails if `multi`'s threshold has not been reached.
    public fun execute_proposal<V, T: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        proposal_id: ID,
        ctx: &mut TxContext,
    ): Action<T> {
        multi.assert_is_member(cap);

        let proposal = multi.proposals.remove<ID, Proposal<T>>(proposal_id);
        assert!(proposal.votes >= multi.threshold, EThresholdNotReached);
        assert!(!proposal.is_expired(ctx), EExpiredProposal);

        let Proposal {
            id,
            votes: _,
            voters: _,
            expiration_epoch: _,
            action: inner,
        } = proposal;

        id.delete();

        let (present, i) = multi.active_proposals.index_of(&proposal_id);
        assert!(present, EProposalNotFound);

        multi.active_proposals.remove(i);

        Action { inner }
    }

    /// Removes the approval given by the controller owning `cap` on `Proposal`
    /// `proposal_id`.
    public fun remove_approval<V, T: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        proposal_id: ID,
    ) {
        let cap_id = cap.id.to_inner();
        let vp = multi.voting_power(cap_id);

        let proposal = multi.proposals.borrow_mut<ID, Proposal<T>>(proposal_id);
        assert!(proposal.voters.contains(&cap_id), ENotVotedYet);

        proposal.voters.remove(&cap_id);
        proposal.votes = proposal.votes - vp;
    }

    /// Returns a reference to `multi`'s value.
    public fun value<V: store>(multi: &Multicontroller<V>): &V {
        &multi.controlled_value
    }

    /// Returns the list of `multi`'s controllers - i.e. the `ID` of its `ControllerCap`s.
    public fun controllers<V>(multi: &Multicontroller<V>): vector<ID> {
        multi.controllers.keys()
    }

    /// Returns `multi`'s threshold.
    public fun threshold<V>(multi: &Multicontroller<V>): u64 {
        multi.threshold
    }

    /// Returns the voting power of a given controller, identified by its `ID`.
    public fun voting_power<V>(multi: &Multicontroller<V>, controller_id: ID): u64 {
        *multi.controllers.get(&controller_id)
    }

    public(package) fun set_voting_power<V>(multi: &mut Multicontroller<V>, controller_id: ID, vp: u64) {
        assert!(multi.controllers().contains(&controller_id), EInvalidController);
        *multi.controllers.get_mut(&controller_id) = vp;
    }

    /// Returns the sum of all controllers voting powers.
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

    public(package) fun is_proposal_approved<V, A: store>(multi: &Multicontroller<V>, proposal_id: ID): bool {
        let proposal = multi.proposals.borrow<ID, Proposal<A>>(proposal_id);
        proposal.votes >= multi.threshold
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

    public(package) fun update_members<V>(multi: &mut Multicontroller<V>, mut to_update: VecMap<ID, u64>) {
        while (!to_update.is_empty()) {
            let (controller, vp) = to_update.pop();

            multi.set_voting_power(controller, vp);
        }
    }

    public(package) fun set_threshold<V>(multi: &mut Multicontroller<V>, threshold: u64) {
        assert!(threshold <= multi.max_votes(), EInvalidThreshold);
        multi.threshold = threshold;
    }

    public(package) fun set_controlled_value<V: store + drop>(multi: &mut Multicontroller<V>, controlled_value: V) {
        multi.controlled_value = controlled_value;
    }
}