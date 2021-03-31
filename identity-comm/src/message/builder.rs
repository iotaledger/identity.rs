// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use did_doc::{url::Url, Document, Signature};
use identity_iota::did::DID;

use crate::{error::Result, message::Message, message::Timing};


/// A `MessageBuilder` is used to generated a customized `Message`.
#[derive(Debug)]
pub struct MessageBuilder {
  pub(crate) callback_url: Option<Url>,
  pub(crate) response_requested: Option<bool>,
  pub(crate) context: Option<Url>,
  pub(crate) id: Option<DID>,
  pub(crate) did_document: Option<Document>,
  pub(crate) thread: Option<String>,
  pub(crate) challenge: Option<String>,
  pub(crate) signature: Option<Signature>,
  pub(crate) timing: Option<Timing>,
}



impl MessageBuilder {
  /// Creates a new `MessageBuilder`.
  pub fn new() -> Self {
    Self {
      callback_url: None,
      response_requested: None,
      context: None,
      id: None,
      did_document: None,
      thread: None,
      challenge: None,
      signature: None,
      timing: None,
    }
  }
  

  /*
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
  */
  /// Returns a new `Message` based on the `MessageBuilder` configuration.
  pub fn build(self) -> Result<Message> {
    Message::from_builder(self)
  }
}

impl Default for MessageBuilder {
  fn default() -> Self {
    Self::new()
  }
}
