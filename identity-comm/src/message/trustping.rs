use crate::message::Message;

use crate::message::Timing;
use did_doc::url::Url;
use identity_iota::did::DID;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Trustping {
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  context: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Trustping {
  pub fn new(callback_url: Url) -> Self {
    Self {
      callback_url,
      response_requested: None,
      context: None,
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a reference to the trustping's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Get a mutable reference to the trustping's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Set the trustping's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the trustping's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the trustping's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the trustping's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the trustping's context.
  pub fn context_mut(&mut self) -> &mut Option<Url> {
    &mut self.context
  }

  /// Get a reference to the trustping's context.
  pub fn context(&self) -> &Option<Url> {
    &self.context
  }

  /// Set the trustping's context.
  pub fn set_context(&mut self, context: Option<Url>) {
    self.context = context;
  }

  /// Get a mutable reference to the trustping's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the trustping's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the trustping's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the trustping's thread.
  pub fn thread_mut(&mut self) -> &mut Option<String> {
    &mut self.thread
  }

  /// Get a reference to the trustping's thread.
  pub fn thread(&self) -> &Option<String> {
    &self.thread
  }

  /// Set the trustping's thread.
  pub fn set_thread(&mut self, thread: Option<String>) {
    self.thread = thread;
  }

  /// Get a mutable reference to the trustping's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the trustping's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the trustping's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TrustpingResponse {
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl TrustpingResponse {
  pub fn new() -> Self {
    Self {
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the trustping response's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the trustping response's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the trustping response's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the trustping response's thread.
  pub fn thread_mut(&mut self) -> &mut Option<String> {
    &mut self.thread
  }

  /// Get a reference to the trustping response's thread.
  pub fn thread(&self) -> &Option<String> {
    &self.thread
  }

  /// Set the trustping response's thread.
  pub fn set_thread(&mut self, thread: Option<String>) {
    self.thread = thread;
  }

  /// Get a mutable reference to the trustping response's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the trustping response's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the trustping response's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
impl Message for Trustping{}
impl Message for TrustpingResponse{}

mod tests {
  use std::str::FromStr;
  use super::*;
  use crate::message::message::AsEnvelope;
  use crate::envelope::EnvelopeExt;
  #[test]
  pub fn test_setter() {
    let mut message = Trustping::new(Url::from_str("https://example.com").unwrap());
    message.set_response_requested(Some(true));
    let plain_envelope = message.pack_plain().unwrap();
    let bytes = plain_envelope.as_bytes();
    dbg!(bytes);
  }
}
