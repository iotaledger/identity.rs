

pub struct VerifiableCredential {
    pub name: String
}

impl VerifiableCredential {
    /// Creates a new DID. `params` and `fragment` are both optional.
    pub fn new(name: String) -> Self {
        VerifiableCredential {
            name: name
        }
    }
}