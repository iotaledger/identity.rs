pub const PROPERTY_JWS: &str = "jws";
pub const PROPERTY_PROOF: &str = "proofValue";
pub const PROPERTY_SIGNATURE: &str = "signatureValue";

/// Represents one of the various proof values of a linked data signature suite
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SignatureValue {
    Jws(String),
    Proof(String),
    Signature(String),
}

impl SignatureValue {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Jws(_) => PROPERTY_JWS,
            Self::Proof(_) => PROPERTY_PROOF,
            Self::Signature(_) => PROPERTY_SIGNATURE,
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::Jws(ref inner) => inner,
            Self::Proof(ref inner) => inner,
            Self::Signature(ref inner) => inner,
        }
    }
}
