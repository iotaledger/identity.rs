use identity_core::convert::ToJson as _;
use serde::Serialize;

use crate::{envelope::EnvelopeExt, error::Result};

/// A DIDComm Plaintext Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-plaintext-messages)
///
/// # Layout
///
///   `JWM(Content)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope(String);

impl Envelope {
  pub fn from_message<T: Serialize>(message: &T) -> Result<Self> {
    message.to_json().map_err(Into::into).map(Self)
  }
}

impl EnvelopeExt for Envelope {
  const FEXT: &'static str = "dcpm";
  const MIME: &'static str = "application/didcomm-plain+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}
