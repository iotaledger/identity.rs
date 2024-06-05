module identity_iota::config_proposal {
    use identity_iota::multicontroller::{ControllerCap, Multicontroller};
    use std::string::String;
    use sui::vec_map::VecMap;

    const ENotMember: u64 = 0;
    const EInvalidThreshold: u64 = 1;

    public struct Modify has store {
        threshold: Option<u64>,
        controllers_to_add: VecMap<address, u64>,
        controllers_to_remove: vector<ID>,
    }

    public fun propose_modify<V>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        key: String,
        mut threshold: Option<u64>,
        controllers_to_add: VecMap<address, u64>,
        controllers_to_remove: vector<ID>,
        ctx: &mut TxContext,
    ) {
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
            voting_power_decrease = voting_power_decrease + multi.voting_power(controller_id);
            i = i + 1;
        };
        let new_max_votes = multi.max_votes() + voting_power_increase - voting_power_decrease;

        let threshold = if (threshold.is_some()) {
            let threshold = threshold.extract();
            assert!(threshold > 0 && threshold <= new_max_votes, EInvalidThreshold);
            threshold
        } else {
            multi.threshold()
        };

        let action = Modify {
            threshold: option::some(threshold),
            controllers_to_add,
            controllers_to_remove,
        };

        multi.create_proposal(cap, action, key, ctx);
    }

    public fun execute_modify<V>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        key: String,
        ctx: &mut TxContext,
    ) {
        let action = multi.execute_proposal(cap, key);
        let Modify { mut threshold, controllers_to_add, controllers_to_remove } = action.unpack_action();

        if (threshold.is_some()) multi.set_threshold(threshold.extract());
        multi.add_members(controllers_to_add, ctx);
        multi.remove_members(controllers_to_remove);
    }
}