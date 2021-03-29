use std::ops::Deref;

use did_doc::{url::Url, Document, Signature};
use did_url::DID;
use identity_core::crypto::{KeyPair, PublicKey};
use serde::Serialize;

use crate::{
  envelope::{Encrypted, EncryptionAlgorithm, Plaintext, SignatureAlgorithm, Signed},
  error::Result,
  message::MessageBuilder,
};

use super::timing::Timing;

macro_rules! impl_accessors {
  ($fn:ident, $ty:ty) => {
    pub fn $fn(&self) -> Option<$ty> {
      self.$fn.as_ref()
    }
  };
}
macro_rules! impl_accessors_mut {
  ($fn:ident,$vn:ident, $ty:ty) => {
    pub fn $fn(&mut self) -> Option<$ty> {
      self.$vn.as_mut()
    }
  };
}
macro_rules! impl_setter {
  ($fn:ident,$vn:ident, $ty:ty) => {
    pub fn $fn(&mut self, value: Option<$ty>) {
      self.$vn = value
    }
  };
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Trustping(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TrustpingResponse(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DidRequest(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DidResponse(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResolutionRequest(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResolutionResult(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthenticationRequest(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthenticationResponse(Message);

impl Deref for AuthenticationResponse {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl Deref for AuthenticationRequest {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for ResolutionResult {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for ResolutionRequest {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl Deref for DidResponse {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl Deref for DidRequest {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for TrustpingResponse {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for Trustping {
  type Target = Message;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Message {
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  context: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  did_document: Option<Document>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  challenge: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  signature: Option<Signature>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Message {
  /// Creates a `MessageBuilder` to configure a new `Message`.
  ///
  /// This is the same as `MessageBuilder::new()`.
  pub fn builder() -> MessageBuilder {
    MessageBuilder::new()
  }

  /// Returns a new `Message` based on the `MessageBuilder` configuration.
  pub fn from_builder(builder: MessageBuilder) -> Result<Self> {
    /*
        let to: Option<Vec<DID>> = if builder.to.is_empty() { None } else { Some(builder.to) };

        let type_: String = builder
          .type_
          .unwrap_or_else(|| todo!("Error: Message Type is required"));
    */
    Ok(Self {
      callback_url: builder.callback_url,
      response_requested: builder.response_requested,
      context: builder.context,
      id: builder.id,
      did_document: builder.did_document,
      thread: builder.thread,
      challenge: builder.challenge,
      signature: builder.signature,
      timing: builder.timing,
    })
  }
  impl_accessors!(callback_url, &Url);
  impl_accessors_mut!(callback_url_mut, callback_url, &mut Url);
  impl_setter!(set_callback_url, callback_url, Url);

  impl_accessors!(response_requested, &bool);
  impl_accessors_mut!(response_requested_mut, response_requested, &mut bool);
  impl_setter!(set_response_requested, response_requested, bool);

  impl_accessors!(context, &Url);
  impl_accessors_mut!(context_mut, context, &mut Url);
  impl_setter!(set_context, context, Url);

  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);

  impl_accessors!(did_document, &Document);
  impl_accessors_mut!(did_document_mut, did_document, &mut Document);
  impl_setter!(set_did_document, did_document, Document);

  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(challenge, &String);
  impl_accessors_mut!(challenge_mut, challenge, &mut String);
  impl_setter!(set_challenge, challenge, String);

  impl_accessors!(signature, &Signature);
  impl_accessors_mut!(signature_mut, signature, &mut Signature);
  impl_setter!(set_signature, signature, Signature);

  impl_accessors!(timing, &Timing);
  impl_accessors_mut!(timing_mut, timing, &mut Timing);
  impl_setter!(set_timing, timing, Timing);
}

pub trait Placeholder {
  fn pack_plain(&self) -> Result<Plaintext>;
  fn pack_auth(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey], sender: &KeyPair) -> Result<Encrypted>;
  fn pack_auth_non_repudiable(
    &self,
    signature: SignatureAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Encrypted>;
  fn pack_anon(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey]) -> Result<Encrypted>;
  fn pack_non_repudiable(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed>;
}

impl<T: Serialize> Placeholder for T {
  fn pack_plain(&self) -> Result<Plaintext> {
    Plaintext::from_message(self)
  }

  fn pack_auth(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey], sender: &KeyPair) -> Result<Encrypted> {
    Encrypted::from_message(self, algorithm, recipients, sender)
  }

  fn pack_auth_non_repudiable(
    &self,
    signature: SignatureAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Encrypted> {
    Self::pack_non_repudiable(self, signature, sender)
      .and_then(|signed| Encrypted::from_signed(&signed, encryption, recipients, sender))
  }

  fn pack_anon(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey]) -> Result<Encrypted> {
    Encrypted::anon_from_message(self, algorithm, recipients)
  }

  fn pack_non_repudiable(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed> {
    Signed::from_message(self, algorithm, sender)
  }
}

mod tests {
  use super::*;
  #[test]
  pub fn test_setter() {
    let mut message = Message::default();
    message.set_response_requested(Some(true));
    dbg!(message.pack_plain());
  }
  pub fn test_envelope() {
    let message = Trustping::default();
    dbg!(message.pack_plain());
    let keypair = KeyPair::new_ed25519().unwrap();
    let result = message.pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair);
    dbg!(result);
  }
}
