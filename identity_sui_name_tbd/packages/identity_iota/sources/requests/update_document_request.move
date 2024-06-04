module identity_iota::update_document_request {
    use identity_iota::{controller::ControllerCap, request_common::{Self, Request}};

    public struct UpdateDocumentRequest has key {
        id: UID,
        doc: vector<u8>,
        inner: Request,
    }

    public fun did(self: &UpdateDocumentRequest): ID {
        self.inner.did()
    }

    public fun doc(self: &UpdateDocumentRequest): &vector<u8> {
        &self.doc
    }

    public(package) fun new(
        cap: &ControllerCap,
        doc: vector<u8>,
        ctx: &mut TxContext
    ) {
        let request = UpdateDocumentRequest {
            id: object::new(ctx),
            doc,
            inner: request_common::new(cap),
        };
        
        transfer::share_object(request);
    }
    
    public(package) fun destroy(self: UpdateDocumentRequest) {
        let UpdateDocumentRequest {
            id,
            inner: _,
            doc: _,
        } = self;
        object::delete(id)
    }

    public fun is_resolved(self: &UpdateDocumentRequest, threshold: u64): bool {
        self.inner.is_resolved(threshold)
    }

    /// Vote in favor for this request, possibly resolving it.
    public fun approve(
        self: &mut UpdateDocumentRequest,
        cap: &ControllerCap,
    ) {
        self.inner.approve(cap);
    }

    public(package) fun take_doc(self: &mut UpdateDocumentRequest): vector<u8> {
        let doc = self.doc;

        doc
    }
}