module identity_iota::transfer_proposal {
    use identity_iota::{multicontroller::{Multicontroller, Action, ControllerCap}, owned::{Self, Withdraw}};
    use iota::{transfer::Receiving, vec_set::VecSet};

    const EDifferentLength: u64 = 0;
    const EUnsentAssets: u64 = 0;

    public struct Send has store {
        withdraw: Withdraw,
        recipients: vector<address>,
    }

    public fun propose_send<V>(
        multi: &mut Multicontroller<V>,
        cap: &ControllerCap,
        expiration: Option<u64>,
        objects: VecSet<ID>,
        recipients: vector<address>,
        ctx: &mut TxContext,
    ) {
        assert!(objects.size() == recipients.length(), EDifferentLength);
        let withdraw = owned::new_withdraw(objects);
        let action = Send { withdraw, recipients };

        multi.create_proposal(cap, action,expiration, ctx);
    }

    public fun send<T: key + store>(
        action: &mut Action<Send>,
        controllee: &mut UID,
        received: Receiving<T>,
    ) {
        let send_action = action.borrow_mut();
        let object = send_action.withdraw.withdraw(controllee, received);
        transfer::public_transfer(object, send_action.recipients.pop_back());
    }

    public fun complete_send(action: Action<Send>) {
        let Send { withdraw, recipients } = action.unpack_action();
        assert!(recipients.is_empty(), EUnsentAssets);

        recipients.destroy_empty();
        withdraw.complete_withdraw();
    }
}