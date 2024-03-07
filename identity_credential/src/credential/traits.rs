use identity_core::common::Timestamp;

pub trait CredentialT {
  type Issuer;
  type Claim;
  type Id;

  fn id(&self) -> &Self::Id;
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

type ValidationError = ();

pub trait ValidableCredential<R, V, K>: CredentialT {
  async fn validate(&self, resolver: &R, verifier: &V) -> Result<(), ValidationError>;
}
