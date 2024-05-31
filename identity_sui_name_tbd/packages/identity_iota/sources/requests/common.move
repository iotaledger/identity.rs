module identity_iota::request_common {
    use sui::vec_set::{Self, VecSet};
    use identity_iota::controller::ControllerCap;

    const EInvalidCapability: u64 = 0;
    const EAlreadyVoted: u64 = 1;

    public struct Request has store, drop {
        did: ID,
        votes: u32,
        threshold: u32,
        voters: VecSet<ID>,
    }

    public(package) fun new(controller: &ControllerCap, threshold: u32): Request {
        Request {
            threshold,
            did: controller.did(),
            votes: controller.weight(),
            voters: vec_set::singleton(controller.id().to_inner()),
        }
    }

    public(package) fun did(self: &Request): ID {
        self.did
    }

    public(package) fun is_resolved(self: &Request): bool {
        self.votes >= self.threshold
    }

    public(package) fun approve(self: &mut Request, cap: &ControllerCap) {
        // Make sure the received capability refers to the same DID document.
        assert!(cap.did() == self.did, EInvalidCapability);
        let cap_id = cap.id().to_inner();
        // Make sure the received capability hasn't already been used to vote.
        assert!(!self.voters.contains(&cap_id), EAlreadyVoted);

        // Vote for this change.
        self.votes = self.votes + cap.weight();
        self.voters.insert(cap_id);
    }
}