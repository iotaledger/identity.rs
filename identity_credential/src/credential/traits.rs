use identity_core::common::{Timestamp, Url};

pub trait CredentialT {
  type Issuer;
  type Claim;

  fn id(&self) -> &Url;
  fn issuer(&self) -> &Self::Issuer;
  fn claim(&self) -> &Self::Claim;
  fn valid_from(&self) -> Timestamp;
  fn valid_until(&self) -> Option<Timestamp>;
  fn is_valid_at(&self, timestamp: &Timestamp) -> bool {
    self.valid_from() <= *timestamp && self.valid_until().map(|t| t > *timestamp).unwrap_or(true)
  }
  fn check_validity_time_frame(&self) -> bool {
    self.is_valid_at(&Timestamp::now_utc())
  }
}

pub trait VerifiableCredentialT<'c>: CredentialT {
  type Proof;

  fn proof(&'c self) -> Self::Proof;
}

pub trait ProofT {
  fn algorithm(&self) -> &str;
  fn signature(&self) -> &[u8];
  fn signing_input(&self) -> &[u8];
  fn verification_method(&self) -> Option<Url>;
}
