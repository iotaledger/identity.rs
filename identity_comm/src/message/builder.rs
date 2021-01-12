use did_url::DID;

use crate::{error::Result, message::Message, utils::Timestamp};

/// A `MessageBuilder` is used to generated a customized `Message`.
#[derive(Debug)]
pub struct MessageBuilder<T = ()> {
    pub(crate) id: String,
    pub(crate) type_: Option<String>,
    pub(crate) from: Option<DID>,
    pub(crate) to: Vec<DID>,
    pub(crate) created_time: Option<Timestamp>,
    pub(crate) expires_time: Option<Timestamp>,
    pub(crate) body: T,
}

impl<T> MessageBuilder<T> {
    /// Creates a new `MessageBuilder`.
    pub fn new(body: T) -> Self {
        Self {
            id: String::new(),
            type_: None,
            from: None,
            to: Vec::new(),
            created_time: None,
            expires_time: None,
            body,
        }
    }

    /// Sets the `id` value of the generated `Message`.
    #[must_use]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = value.into();
        self
    }

    /// Sets the `type` value of the generated `Message`.
    #[must_use]
    pub fn type_(mut self, value: impl Into<String>) -> Self {
        self.type_ = Some(value.into());
        self
    }

    /// Sets the `from` value of the generated `Message`.
    #[must_use]
    pub fn from(mut self, value: DID) -> Self {
        self.from = Some(value);
        self
    }

    /// Adds a value to the list of recipients for the generated `Message`.
    #[must_use]
    pub fn to(mut self, value: DID) -> Self {
        self.to.push(value);
        self
    }

    /// Sets the `created_time` value of the generated `Message`.
    #[must_use]
    pub fn created_time(mut self, value: impl Into<Timestamp>) -> Self {
        self.created_time = Some(value.into());
        self
    }

    /// Sets the `expires_time` value of the generated `Message`.
    #[must_use]
    pub fn expires_time(mut self, value: impl Into<Timestamp>) -> Self {
        self.expires_time = Some(value.into());
        self
    }

    /// Returns a new `Message` based on the `MessageBuilder` configuration.
    pub fn build(self) -> Result<Message<T>> {
        Message::from_builder(self)
    }
}

impl<T> Default for MessageBuilder<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
