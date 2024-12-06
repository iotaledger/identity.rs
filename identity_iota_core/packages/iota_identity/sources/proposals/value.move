// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_identity::update_value_proposal {
    use iota_identity::multicontroller::Multicontroller;
    use iota_identity::controller::DelegationToken;

    public struct UpdateValue<V: store> has store {
        new_value: V,
    }

    public fun propose_update<V: store>(
        multi: &mut Multicontroller<V>,
        cap: &DelegationToken,
        new_value: V,
        expiration: Option<u64>,
        ctx: &mut TxContext,
    ): ID {
        let update_action = UpdateValue { new_value };
        multi.create_proposal(cap, update_action, expiration, ctx)
    } 

    public fun execute_update<V: store + drop>(
        multi: &mut Multicontroller<V>,
        cap: &DelegationToken,
        proposal_id: ID,
        ctx: &mut TxContext,
    ) {


        let action = multi.execute_proposal(cap, proposal_id, ctx);
        let UpdateValue { new_value } = action.unpack_action();

        multi.set_controlled_value(new_value)
    }
}