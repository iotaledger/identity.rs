module identity_iota::config_proposal {
    use identity_iota::multicontroller::{ControllerCap, Multicontroller};
    use iota::vec_map::VecMap;

    const ENotMember: u64 = 0;
    const EInvalidThreshold: u64 = 1;

    public struct Modify has store {
        threshold: Option<u64>,
        controllers_to_add: VecMap<address, u64>,
        controllers_to_remove: vector<ID>,
        controllers_to_update: VecMap<ID, u64>,
    }

    public fun propose_modify<V>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        expiration: Option<u64>,
        mut threshold: Option<u64>,
        controllers_to_add: VecMap<address, u64>,
        controllers_to_remove: vector<ID>,
        controllers_to_update: VecMap<ID, u64>,
        ctx: &mut TxContext,
    ): ID {
        let mut max_votes = 0;
        let (mut cs, mut vps) = controllers_to_update.into_keys_values();
        while (!cs.is_empty()) {
            let c = cs.pop_back();
            let vp = vps.pop_back();
            assert!(multi.controllers().contains(&c), ENotMember);
            max_votes = max_votes + vp;
        };
        let (_, mut voting_powers) = controllers_to_add.into_keys_values();
        let mut voting_power_increase = 0;
        while (!voting_powers.is_empty()) {
            let voting_power = voting_powers.pop_back();

            voting_power_increase = voting_power_increase + voting_power;
        };
        voting_powers.destroy_empty();

        let mut i = 0;
        let mut voting_power_decrease = 0;
        while (i < controllers_to_remove.length()) {
            let controller_id = controllers_to_remove[i];
            assert!(multi.controllers().contains(&controller_id), ENotMember);
            let mut vp = multi.voting_power(controller_id);
            if (controllers_to_update.contains(&controller_id)) {
                vp = *controllers_to_update.get(&controller_id);
            };
            voting_power_decrease = voting_power_decrease + vp;
            i = i + 1;
        };

        let mut i = 0;
        while (i < multi.controllers().length()) {
            let controller_id = multi.controllers()[i];
            if (!controllers_to_update.contains(&controller_id)) {
                max_votes = max_votes + multi.voting_power(controller_id);
            };
            i = i + 1;
        };

        let new_max_votes = max_votes + voting_power_increase - voting_power_decrease;

        let threshold = if (threshold.is_some()) {
            let threshold = threshold.extract();
            threshold
        } else {
            multi.threshold()
        };

        assert!(threshold > 0 && threshold <= new_max_votes, EInvalidThreshold);

        let action = Modify {
            threshold: option::some(threshold),
            controllers_to_add,
            controllers_to_remove,
            controllers_to_update,
        };

        multi.create_proposal(cap, action, expiration, ctx)
    }

    public fun execute_modify<V>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        proposal_id: ID,
        ctx: &mut TxContext,
    ) {
        let action = multi.execute_proposal(cap, proposal_id, ctx);
        let Modify {
            mut threshold,
            controllers_to_add,
            controllers_to_remove,
            controllers_to_update
        } = action.unpack_action();

        if (threshold.is_some()) multi.set_threshold(threshold.extract());
        multi.update_members(controllers_to_update);
        multi.add_members(controllers_to_add, ctx);
        multi.remove_members(controllers_to_remove);
    }
}