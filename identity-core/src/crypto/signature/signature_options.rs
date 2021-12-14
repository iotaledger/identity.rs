use crate::common::Timestamp;

/// Convenience struct for holding attributes to pass to a new
/// [`Signature`](crate::crypto::Signature).
#[derive(Clone, Debug, Default)]
pub struct SignatureOptions {
  /// [`Signature::created`](crate::crypto::Signature::created)
  pub created: Option<Timestamp>,
  /// [`Signature::expires`](crate::crypto::Signature::expires)
  pub expires: Option<Timestamp>,
  /// [`Signature::challenge`](crate::crypto::Signature::challenge)
  pub challenge: Option<String>,
  /// [`Signature::domain`](crate::crypto::Signature::domain)
  pub domain: Option<String>,
  /// [`Signature::purpose`](crate::crypto::Signature::purpose)
  pub purpose: Option<String>,
}

impl SignatureOptions {
  /// Creates a new `SignatureOptions` with all options unset.
  pub fn new() -> Self {
    Self {
      created: None,
      expires: None,
      challenge: None,
      domain: None,
      purpose: None,
    }
  }

  /// Sets the [`Signature::created`](crate::crypto::Signature::created) field.
  pub fn created(mut self, created: Timestamp) -> Self {
    self.created = Some(created);
    self
  }

  /// Sets the [`Signature::expires`](crate::crypto::Signature::expires) field.
  /// The signature will fail validation after the specified datetime.
  pub fn expires(mut self, expires: Timestamp) -> Self {
    self.expires = Some(expires);
    self
  }

  /// Sets the [`Signature::challenge`](crate::crypto::Signature::challenge) field.
  pub fn challenge(mut self, challenge: String) -> Self {
    self.challenge = Some(challenge);
    self
  }

  /// Sets the [`Signature::domain`](crate::crypto::Signature::domain) field.
  pub fn domain(mut self, domain: String) -> Self {
    self.domain = Some(domain);
    self
  }

  /// Sets the [`Signature::purpose`](crate::crypto::Signature::purpose) field.
  pub fn purpose(mut self, purpose: String) -> Self {
    self.purpose = Some(purpose);
    self
  }
}
