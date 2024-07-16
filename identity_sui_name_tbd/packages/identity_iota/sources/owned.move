module identity_iota::owned {
    use iota::{vec_set::VecSet, transfer::Receiving};

    const EWrongObject: u64 = 0;
    const EUnretrievedObjects: u64 = 1;
    const EUnreturnedObjects: u64 = 2;

    public struct Withdraw has store {
        objects: VecSet<ID>,
    }

    public struct Borrow has store {
        withdraw: Withdraw,
        to_return: VecSet<ID>,
    }

    public(package) fun new_withdraw(objects: VecSet<ID>): Withdraw {
        Withdraw { objects }
    }

    public(package) fun withdraw<T: key + store>(
        action: &mut Withdraw,
        controllee: &mut UID,
        receiving: Receiving<T>,
    ): T {
        let id = {
            let vec = action.objects.keys();
            let last_id = vec[vec.length() - 1];
            let _ = vec;
            action.objects.remove(&last_id);
            last_id
        };
        let received = transfer::public_receive(controllee, receiving);
        let received_id = object::id(&received);
        assert!(received_id == id, EWrongObject);

        received
    }

    public(package) fun complete_withdraw(action: Withdraw) {
        let Withdraw { objects } = action;
        assert!(objects.is_empty(), EUnretrievedObjects);
    }

    public(package) fun new_borrow(objects: VecSet<ID>): Borrow {
        Borrow {
            withdraw: new_withdraw(objects),
            to_return: objects,
        }
    }

    public(package) fun borrow<T: key + store>(
        action: &mut Borrow,
        controllee: &mut UID,
        receiving: Receiving<T>,
    ): T {
        action.withdraw.withdraw(controllee, receiving)
    }

    public(package) fun put_back<T: key + store>(
        action: &mut Borrow,
        controllee: &UID,
        returned: T,
    ) {
        let id = object::id(&returned);
        assert!(action.to_return.contains(&id), EWrongObject);
        action.to_return.remove(&id);

        transfer::public_transfer(returned, controllee.to_address())
    }

    public(package) fun complete_borrow(action: Borrow) {
        let Borrow { withdraw, to_return } = action;
        complete_withdraw(withdraw);
        assert!(to_return.is_empty(), EUnreturnedObjects);
    }
}