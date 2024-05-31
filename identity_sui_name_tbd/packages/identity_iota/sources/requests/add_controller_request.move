module identity_iota::add_controller_request {
    use identity_iota::{controller::ControllerCap, request_common::{Self, Request}};
    use sui::event;

    public struct RequestCreated has copy, drop {
        id: ID
    }

    public struct RequestResolved has copy, drop {
        id: ID
    }

    public struct AddControllerRequest has key {
        id: UID,
        weight: u32,
        inner: Request,
    }

    public fun weight(self: &AddControllerRequest): u32 {
        self.weight
    }

    public fun did(self: &AddControllerRequest): ID {
        self.inner.did()
    }

    public(package) fun new(
        cap: &ControllerCap,
        threshold: u32,
        weight: u32,
        ctx: &mut TxContext
    ) {
        let request = AddControllerRequest {
            id: object::new(ctx),
            weight,
            inner: request_common::new(cap, threshold),
        };
        
        event::emit(RequestCreated { id: request.id.to_inner() });

        transfer::share_object(request);
    }
    
    public(package) fun destroy(self: AddControllerRequest) {
        let AddControllerRequest {
            id,
            weight: _,
            inner: _,
        } = self;
        object::delete(id)
    }

    public fun is_resolved(self: &AddControllerRequest): bool {
        self.inner.is_resolved()
    }

    /// Vote in favor for this request, possibly resolving it.
    public fun approve(
        self: &mut AddControllerRequest,
        cap: &ControllerCap,
    ) {
        self.inner.approve(cap);

        if (self.is_resolved()) {
            event::emit(RequestResolved { id: self.id.to_inner() })
        }
    }
}
