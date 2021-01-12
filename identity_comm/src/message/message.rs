use did_url::DID;
use identity_core::{
    common::Object,
    convert::SerdeInto,
    crypto::{KeyPair, PublicKey},
};
use libjose::jwm::JwmAttributes;
use serde::Serialize;

use crate::{
    envelope::{Encrypted, EncryptionAlgorithm, Plaintext, SignatureAlgorithm, Signed},
    error::Result,
    message::MessageBuilder,
    utils::Timestamp,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message<T = ()> {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) from: Option<DID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to: Option<Vec<DID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) created_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expires_time: Option<Timestamp>,
    pub(crate) body: T,
}

impl<T> Message<T> {
    /// Creates a `MessageBuilder` to configure a new `Message`.
    ///
    /// This is the same as `MessageBuilder::new()`.
    pub fn builder(body: T) -> MessageBuilder<T> {
        MessageBuilder::new(body)
    }

    /// Returns a new `Message` based on the `MessageBuilder` configuration.
    pub fn from_builder(builder: MessageBuilder<T>) -> Result<Self> {
        let to: Option<Vec<DID>> = if builder.to.is_empty() { None } else { Some(builder.to) };

        let type_: String = builder
            .type_
            .unwrap_or_else(|| todo!("Error: Message Type is required"));

        Ok(Self {
            id: builder.id,
            type_,
            from: builder.from,
            to,
            created_time: builder.created_time,
            expires_time: builder.expires_time,
            body: builder.body,
        })
    }
}

impl<T> Message<T>
where
    T: Serialize,
{
    pub fn into_attributes(self) -> Result<JwmAttributes> {
        let mut attributes: JwmAttributes = JwmAttributes::new();

        attributes.set_id(self.id);
        attributes.set_type(self.type_);
        attributes.set_body(SerdeInto::serde_into::<Object>(&self.body)?);

        if let Some(created_time) = self.created_time {
            attributes.set_created_time(created_time.get());
        }

        if let Some(expires_time) = self.expires_time {
            attributes.set_expires_time(expires_time.get());
        }

        if let Some(from) = self.from {
            attributes.set_from(from);
        }

        if let Some(to) = self.to {
            attributes.set_to(to);
        }

        Ok(attributes)
    }

    pub fn pack_plain(&self) -> Result<Plaintext> {
        Plaintext::from_message(self)
    }

    pub fn pack_auth(
        &self,
        algorithm: EncryptionAlgorithm,
        recipients: &[PublicKey],
        sender: &KeyPair,
    ) -> Result<Encrypted> {
        Encrypted::from_message(self, algorithm, recipients, sender)
    }

    pub fn pack_auth_non_repudiable(
        &self,
        signature: SignatureAlgorithm,
        encryption: EncryptionAlgorithm,
        recipients: &[PublicKey],
        sender: &KeyPair,
    ) -> Result<Encrypted> {
        Self::pack_non_repudiable(self, signature, sender)
            .and_then(|signed| Encrypted::from_signed(&signed, encryption, recipients, sender))
    }

    pub fn pack_anon(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey]) -> Result<Encrypted> {
        Encrypted::anon_from_message(self, algorithm, recipients)
    }

    pub fn pack_non_repudiable(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed> {
        Signed::from_message(self, algorithm, sender)
    }
}
