module identity_iota::add_controller_request {
    use identity_iota::{controller::ControllerCap, request_common::{Self, Request}};

    public struct AddControllerRequest has key {
        id: UID,
        weight: u64,
        recipient: address,
        inner: Request,
    }

    public fun recipient(self: &AddControllerRequest): address {
        self.recipient
    }

    public fun weight(self: &AddControllerRequest): u64 {
        self.weight
    }

    public fun did(self: &AddControllerRequest): ID {
        self.inner.did()
    }

    public(package) fun new(
        cap: &ControllerCap,
        weight: u64,
        recipient: address,
        ctx: &mut TxContext
    ) {
        let request = AddControllerRequest {
            id: object::new(ctx),
            weight,
            recipient,
            inner: request_common::new(cap),
        };
        
        transfer::share_object(request);
    }
    
    public(package) fun destroy(self: AddControllerRequest) {
        let AddControllerRequest {
            id,
            weight: _,
            inner: _,
            recipient: _,
        } = self;
        object::delete(id)
    }

    public fun is_resolved(self: &AddControllerRequest, threshold: u64): bool {
        self.inner.is_resolved(threshold)
    }

    /// Vote in favor for this request, possibly resolving it.
    public fun approve(
        self: &mut AddControllerRequest,
        cap: &ControllerCap,
    ) {
        self.inner.approve(cap);
    }
}
