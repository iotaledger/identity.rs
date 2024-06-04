module identity_iota::transfer_control_request {
    use identity_iota::{controller::ControllerCap, request_common::{Self, Request}};

    public struct TransferControlRequest has key {
        id: UID,
        controller_cap_id: ID,
        current_controller: address,
        recipient: address,
        inner: Request,
    }

    public fun recipient(self: &TransferControlRequest): address {
        self.recipient
    }

    public fun capability_id(self: &TransferControlRequest): ID {
        self.controller_cap_id
    }

    public fun did(self: &TransferControlRequest): ID {
        self.inner.did()
    }

    public(package) fun new(
        cap: &ControllerCap,
        recipient: address,
        ctx: &mut TxContext
    ) {
        let request = TransferControlRequest {
            id: object::new(ctx),
            inner: request_common::new(cap),
            controller_cap_id: cap.id().to_inner(),
            current_controller: ctx.sender(),
            recipient,
        };
        
        transfer::share_object(request);
    }
    
    public(package) fun destroy(self: TransferControlRequest) {
        let TransferControlRequest {
            id,
            current_controller: _,
            controller_cap_id: _,
            inner: _,
            recipient: _,
        } = self;
        object::delete(id)
    }

    public fun is_resolved(self: &TransferControlRequest, threshold: u64): bool {
        self.inner.is_resolved(threshold)
    }

    /// Vote in favor for this request, possibly resolving it.
    public fun approve(
        self: &mut TransferControlRequest,
        cap: &ControllerCap,
    ) {
        self.inner.approve(cap);
    }
}
