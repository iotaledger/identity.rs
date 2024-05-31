module identity_iota::remove_controller_request {
    use identity_iota::{controller::ControllerCap, request_common::{Self, Request}};
    use sui::event;

    public struct RequestCreated has copy, drop {
        id: ID
    }

    public struct RequestResolved has copy, drop {
        id: ID
    }

    public struct RemoveControllerRequest has key {
        id: UID,
        id_to_remove: ID,
        inner: Request,
    }

    public fun id_to_remove(self: &RemoveControllerRequest): ID {
        self.id_to_remove
    }

    public fun did(self: &RemoveControllerRequest): ID {
        self.inner.did()
    }

    public(package) fun new(
        cap: &ControllerCap,
        threshold: u32,
        id_to_remove: ID,
        ctx: &mut TxContext
    ) {
        let request = RemoveControllerRequest {
            id: object::new(ctx),
            id_to_remove,
            inner: request_common::new(cap, threshold),
        };
        
        event::emit(RequestCreated { id: request.id.to_inner() });

        transfer::share_object(request);
    }
    
    public(package) fun destroy(self: RemoveControllerRequest) {
        let RemoveControllerRequest {
            id,
            id_to_remove: _,
            inner: _,
        } = self;
        object::delete(id)
    }

    public fun is_resolved(self: &RemoveControllerRequest): bool {
        self.inner.is_resolved()
    }

    /// Vote in favor for this request, possibly resolving it.
    public fun approve(
        self: &mut RemoveControllerRequest,
        cap: &ControllerCap,
    ) {
        self.inner.approve(cap);

        if (self.is_resolved()) {
            event::emit(RequestResolved { id: self.id.to_inner() })
        }
    }
}
