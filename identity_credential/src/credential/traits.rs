use identity_core::common::{Timestamp, Url};

pub trait CredentialT {
    type Issuer;
    type Claim;
    
    fn id(&self) -> &Url;
    fn issuer(&self) -> &Self::Issuer;
    fn claim(&self) -> &Self::Claim;
    fn is_valid_at(&self, timestamp: &Timestamp) -> bool;
    fn check_validity_time_frame(&self) -> bool {
        self.is_valid_at(&Timestamp::now_utc())
    }
}

pub trait VerifiableCredentialT<'c, P>: CredentialT {
    fn proof(&'c self) -> P;
}

pub trait ProofT {
    fn signature(&self) -> impl AsRef<[u8]>;
    fn verification_method(&self) -> Option<&impl Into<Url>>;
}

pub trait StatusT {
    type State;
    fn type_(&self) -> &str;
}