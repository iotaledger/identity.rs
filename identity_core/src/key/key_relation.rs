#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyRelation {
    VerificationMethod,
    Authentication,
    AssertionMethod,
    KeyAgreement,
    CapabilityInvocation,
    CapabilityDelegation,
}

impl Default for KeyRelation {
    fn default() -> Self {
        Self::VerificationMethod
    }
}
