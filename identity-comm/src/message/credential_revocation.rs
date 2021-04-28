use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Revocation {
  context: String,
  thread: Uuid,
  credential_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Revocation {
  pub fn new(context: String, thread: Uuid, credential_id: String) -> Self {
    Self {
      context,
      thread,
      credential_id,
      callback_url: None,
      response_requested: None,
      id: None,
      comment: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the revocation's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the revocation's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the revocation's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the revocation's thread.
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the revocation's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the revocation's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
    self.thread = thread;
  }

  /// Get a mutable reference to the revocation's credential id.
  pub fn credential_id_mut(&mut self) -> &mut String {
    &mut self.credential_id
  }

  /// Get a reference to the revocation's credential id.
  pub fn credential_id(&self) -> &String {
    &self.credential_id
  }

  /// Set the revocation's credential id.
  pub fn set_credential_id(&mut self, credential_id: String) {
    self.credential_id = credential_id;
  }

  /// Get a mutable reference to the revocation's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Get a reference to the revocation's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Set the revocation's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the revocation's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the revocation's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the revocation's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the revocation's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the revocation's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the revocation's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the revocation's comment.
  pub fn comment_mut(&mut self) -> &mut Option<String> {
    &mut self.comment
  }

  /// Get a reference to the revocation's comment.
  pub fn comment(&self) -> &Option<String> {
    &self.comment
  }

  /// Set the revocation's comment.
  pub fn set_comment(&mut self, comment: Option<String>) {
    self.comment = comment;
  }

  /// Get a mutable reference to the revocation's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the revocation's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the revocation's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
