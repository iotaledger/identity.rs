use did_url::DID;
use identity_core::{common::Object, convert::SerdeInto};
use libjose::jwm::JwmAttributes;
use serde::Serialize;

use crate::{error::Result, message::MessageBuilder, utils::Timestamp};

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

    pub fn into_attributes(self) -> Result<JwmAttributes>
    where
        T: Serialize,
    {
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
}
