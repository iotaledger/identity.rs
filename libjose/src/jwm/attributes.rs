// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde_json::Map;
use serde_json::Value;

use crate::jwt::JwtClaims;
use crate::lib::*;

/// JSON Web Message Attributes Set
///
/// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#rfc.section.3)
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct JwmAttributes<T = ()> {
  /// A unique identifier for the JWM.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-id)
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<String>, // Message ID
  /// The type of the message.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-type)
  #[serde(skip_serializing_if = "Option::is_none")]
  type_: Option<String>, // Message Type
  /// Application-level message content.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-body)
  #[serde(skip_serializing_if = "Option::is_none")]
  body: Option<Map<String, Value>>, // Message Body
  /// The intended recipients of the JWM.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-to)
  #[serde(skip_serializing_if = "Option::is_none")]
  to: Option<Vec<String>>, // Recipients
  /// The sender of the JWM
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-from)
  #[serde(skip_serializing_if = "Option::is_none")]
  from: Option<String>, // Message From
  /// Associates the JWM to a group of related messages.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-thread-id)
  #[serde(skip_serializing_if = "Option::is_none")]
  thread_id: Option<String>, // Message Thread ID
  /// The time in which the message was created.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-created-time)
  #[serde(skip_serializing_if = "Option::is_none")]
  created_time: Option<i64>, // Message Created Time
  /// The lifespan or lifetime of the JWM.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-expires-time)
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_time: Option<i64>, // Message Expiry Time
  /// A url to which a response to the message can be sent.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-reply-url)
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_url: Option<String>, // Message Reply URL
  /// Who a response to the message should be sent to.
  ///
  /// [More Info](https://tools.ietf.org/id/draft-looker-jwm-01.html#attributes-reply-to)
  #[serde(skip_serializing_if = "Option::is_none")]
  reply_to: Option<Vec<String>>, // Message Reply To
  /// Default JWT claims.
  #[serde(flatten)]
  inner: JwtClaims<T>,
}

impl<T> JwmAttributes<T> {
  /// Create a new `JwmAttributes` set.
  pub const fn new() -> Self {
    Self {
      id: None,
      type_: None,
      body: None,
      to: None,
      from: None,
      thread_id: None,
      created_time: None,
      expires_time: None,
      reply_url: None,
      reply_to: None,
      inner: JwtClaims::new(),
    }
  }

  /// Returns the value for the ID atribute (id).
  pub fn id(&self) -> Option<&str> {
    self.id.as_deref()
  }

  /// Sets a value for the ID atribute (id).
  pub fn set_id(&mut self, value: impl Into<String>) {
    self.id = Some(value.into());
  }

  /// Returns the value for the type atribute (type).
  pub fn type_(&self) -> Option<&str> {
    self.type_.as_deref()
  }

  /// Sets a value for the type atribute (type).
  pub fn set_type(&mut self, value: impl Into<String>) {
    self.type_ = Some(value.into());
  }

  /// Returns the value for the body atribute (body).
  pub fn body(&self) -> Option<&Map<String, Value>> {
    self.body.as_ref()
  }

  /// Sets a value for the body atribute (body).
  pub fn set_body(&mut self, value: impl IntoIterator<Item = impl Into<(String, Value)>>) {
    self.body = Some(value.into_iter().map(Into::into).collect())
  }

  /// Returns the value for the to atribute (to).
  pub fn to(&self) -> Option<&[String]> {
    self.to.as_deref()
  }

  /// Sets a value for the to atribute (to).
  pub fn set_to(&mut self, value: impl IntoIterator<Item = impl Into<String>>) {
    self.to = Some(value.into_iter().map(Into::into).collect());
  }

  /// Returns the value for the from atribute (from).
  pub fn from(&self) -> Option<&str> {
    self.from.as_deref()
  }

  /// Sets a value for the from atribute (from).
  pub fn set_from(&mut self, value: impl Into<String>) {
    self.from = Some(value.into());
  }

  /// Returns the value for the thread ID atribute (thread_id).
  pub fn thread_id(&self) -> Option<&str> {
    self.thread_id.as_deref()
  }

  /// Sets a value for the thread ID atribute (thread_id).
  pub fn set_thread_id(&mut self, value: impl Into<String>) {
    self.thread_id = Some(value.into());
  }

  /// Returns the value for the created time atribute (created_time).
  pub fn created_time(&self) -> Option<i64> {
    self.created_time
  }

  /// Sets a value for the created time atribute (created_time).
  pub fn set_created_time(&mut self, value: impl Into<i64>) {
    self.created_time = Some(value.into());
  }

  /// Returns the value for the expires time atribute (expires_time).
  pub fn expires_time(&self) -> Option<i64> {
    self.expires_time
  }

  /// Sets a value for the expires time atribute (expires_time).
  pub fn set_expires_time(&mut self, value: impl Into<i64>) {
    self.expires_time = Some(value.into());
  }

  /// Returns the value for the reply URL atribute (reply_url).
  pub fn reply_url(&self) -> Option<&str> {
    self.reply_url.as_deref()
  }

  /// Sets a value for the reply URL atribute (reply_url).
  pub fn set_reply_url(&mut self, value: impl Into<String>) {
    self.reply_url = Some(value.into());
  }

  /// Returns the value for the reply to atribute (reply_to).
  pub fn reply_to(&self) -> Option<&[String]> {
    self.reply_to.as_deref()
  }

  /// Sets a value for the reply to atribute (reply_to).
  pub fn set_reply_to(&mut self, value: impl IntoIterator<Item = impl Into<String>>) {
    self.reply_to = Some(value.into_iter().map(Into::into).collect());
  }

  /// Returns a reference to the inner `JwtClaims` object.
  pub fn claims(&self) -> &JwtClaims<T> {
    &self.inner
  }

  /// Returns a mutable reference to the inner `JwtClaims` object.
  pub fn claims_mut(&mut self) -> &mut JwtClaims<T> {
    &mut self.inner
  }
}
