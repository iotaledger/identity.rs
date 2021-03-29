use identity_core::crypto::{KeyPair, PublicKey};
use libjose::jwe::{Encoder, JweAlgorithm, JweEncryption, JweFormat, JweHeader};
use serde::Serialize;

use crate::{
  envelope::{EnvelopeExt, Plaintext, Signed},
  error::Result,
};

/// Supported content encryption algorithms
///
/// [Reference (auth)](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption)
/// [Reference (anon)](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption)
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
  A256GCM,
  XC20P,
}

impl From<Algorithm> for JweEncryption {
  fn from(other: Algorithm) -> Self {
    match other {
      Algorithm::A256GCM => Self::A256GCM,
      Algorithm::XC20P => Self::XC20P,
    }
  }
}

/// A DIDComm Encrypted Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-encrypted-message)
///
/// # Layout
///
///   `JWE(Plaintext | Signed)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope(pub String);

impl Envelope {
  pub fn from_message<T: Serialize>(
    message: &T,
    algorithm: Algorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Plaintext::from_message(message)
      .and_then(|plaintext| Self::from_plaintext(&plaintext, algorithm, recipients, sender))
  }

  pub fn from_plaintext(
    envelope: &Plaintext,
    algorithm: Algorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::from_envelope(envelope, algorithm, recipients, Some(sender))
  }

  pub fn from_signed(
    envelope: &Signed,
    algorithm: Algorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::from_envelope(envelope, algorithm, recipients, Some(sender))
  }

  pub fn anon_from_message<T: Serialize>(message: &T, algorithm: Algorithm, recipients: &[PublicKey]) -> Result<Self> {
    Plaintext::from_message(message).and_then(|plaintext| Self::anon_from_plaintext(&plaintext, algorithm, recipients))
  }

  pub fn anon_from_plaintext(envelope: &Plaintext, algorithm: Algorithm, recipients: &[PublicKey]) -> Result<Self> {
    Self::from_envelope(envelope, algorithm, recipients, None)
  }

  pub fn anon_from_signed(envelope: &Signed, algorithm: Algorithm, recipients: &[PublicKey]) -> Result<Self> {
    Self::from_envelope(envelope, algorithm, recipients, None)
  }

  fn from_envelope<T>(
    envelope: &T,
    algorithm: Algorithm,
    recipients: &[PublicKey],
    sender: Option<&KeyPair>,
  ) -> Result<Self>
  where
    T: EnvelopeExt,
  {
    let header: JweHeader = if sender.is_some() {
      JweHeader::new(JweAlgorithm::ECDH_1PU, algorithm.into())
    } else {
      JweHeader::new(JweAlgorithm::ECDH_ES, algorithm.into())
    };

    let mut encoder: Encoder = Encoder::new().format(JweFormat::General).protected(&header);

    if let Some(sender) = sender {
      encoder = encoder.secret(sender.secret().as_ref());
    }

    recipients
      .iter()
      .fold(encoder, |encoder, recipient| encoder.recipient(recipient.as_ref()))
      .encode(envelope.as_bytes())
      .map_err(Into::into)
      .map(Self)
  }
}

impl EnvelopeExt for Envelope {
  const FEXT: &'static str = "dcem";
  const MIME: &'static str = "application/didcomm-encrypted+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}
