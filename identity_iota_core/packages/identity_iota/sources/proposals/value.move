module identity_iota::update_value_proposal {
    use identity_iota::multicontroller::{Multicontroller, ControllerCap};

    public struct UpdateValue<V: store> has store {
        new_value: V,
    }

    public fun propose_update<V: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        new_value: V,
        expiration: Option<u64>,
        ctx: &mut TxContext,
    ): ID {
        let update_action = UpdateValue { new_value };
        multi.create_proposal(cap, update_action, expiration, ctx)
    } 

    public fun execute_update<V: store + drop>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        proposal_id: ID,
        ctx: &mut TxContext,
    ) {
        let action = multi.execute_proposal(cap, proposal_id, ctx);
        let UpdateValue { new_value } = action.unpack_action();

        multi.set_controlled_value(new_value)
    }
}