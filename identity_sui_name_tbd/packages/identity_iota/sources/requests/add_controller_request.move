module identity_iota::add_controller_request {
    use identity_iota::controller::ControllerCap;
    use sui::{event, vec_set::{Self, VecSet}};

    const EInvalidCapability: u64 = 0;
    const EAlreadyVoted: u64 = 1;

    public struct RequestCreated has copy, drop {
        id: ID
    }

    public struct RequestResolved has copy, drop {
        id: ID
    }

    public struct AddControllerRequest has key {
        id: UID,
        did: ID,
        weight: u32,
        votes: u32,
        threshold: u32,
        voters: VecSet<ID>,
    }

    public fun weight(self: &AddControllerRequest): u32 {
        self.weight
    }

    public fun did(self: &AddControllerRequest): ID {
        self.did
    }

    public(package) fun new(
        cap: &ControllerCap,
        threshold: u32,
        weight: u32,
        ctx: &mut TxContext
    ) {
        let request = AddControllerRequest {
            id: object::new(ctx),
            did: cap.did(),
            weight,
            votes: cap.weight(),
            threshold,
            voters: vec_set::singleton(cap.id().to_inner()),
        };
        
        event::emit(RequestCreated { id: request.id.to_inner() });

        transfer::share_object(request);
    }
    
    public(package) fun destroy(self: AddControllerRequest) {
        let AddControllerRequest {
            id,
            did: _, 
            weight: _,
            votes: _,
            threshold: _,
            voters: _,
        } = self;
        object::delete(id)
    }

    public fun is_resolved(self: &AddControllerRequest): bool {
        self.votes >= self.threshold
    }

    /// Vote in favor for this request, possibly resolving it.
    public fun approve(
        self: &mut AddControllerRequest,
        cap: &ControllerCap,
    ) {
        // Make sure the received capability refers to the same DID document.
        assert!(cap.did() == self.did, EInvalidCapability);
        let cap_id = cap.id().to_inner();
        // Make sure the received capability hasn't already been used to vote.
        assert!(!self.voters.contains(&cap_id), EAlreadyVoted);

        // Vote for this change.
        self.votes = self.votes + cap.weight();
        self.voters.insert(cap_id);

        if (self.is_resolved()) {
            event::emit(RequestResolved { id: self.id.to_inner() })
        }
    }
}