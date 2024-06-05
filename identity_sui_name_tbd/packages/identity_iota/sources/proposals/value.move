module identity_iota::update_value_proposal {
    use identity_iota::multicontroller::{Multicontroller, ControllerCap};
    use std::string::String;

    public struct UpdateValue<V: store> has store {
        new_value: V,
    }

    public fun propose_update<V: store>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        key: String,
        new_value: V,
        ctx: &mut TxContext,
    ) {
        let update_action = UpdateValue { new_value };
        multi.create_proposal(cap, update_action, key, ctx);
    } 

    public fun execute_update<V: store + drop>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        key: String,
    ) {
        let action = multi.execute_proposal(cap, key);
        let UpdateValue { new_value } = action.unpack_action();

        multi.set_value(new_value)
    }
}