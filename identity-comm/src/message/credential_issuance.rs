use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialSelection {
  context: String,
  thread: Uuid,
  callback_url: Url,
  credential_types: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialSelection {
  pub fn new(context: String, thread: Uuid, callback_url: Url, credential_types: Vec<String>) -> Self {
    Self {
      context,
      thread,
      callback_url,
      credential_types,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the credential selection's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the credential selection's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the credential selection's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the credential selection's thread.
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the credential selection's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the credential selection's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
    self.thread = thread;
  }

  /// Get a mutable reference to the credential selection's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Set the credential selection's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a reference to the credential selection's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Get a mutable reference to the credential selection's credential types.
  pub fn credential_types_mut(&mut self) -> &mut Vec<String> {
    &mut self.credential_types
  }

  /// Get a reference to the credential selection's credential types.
  pub fn credential_types(&self) -> &Vec<String> {
    &self.credential_types
  }

  /// Set the credential selection's credential types.
  pub fn set_credential_types(&mut self, credential_types: Vec<String>) {
    self.credential_types = credential_types;
  }

  /// Get a mutable reference to the credential selection's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the credential selection's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the credential selection's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a reference to the credential selection's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Get a mutable reference to the credential selection's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Set the credential selection's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the credential selection's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the credential selection's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the credential selection's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialIssuance {
  context: String,
  thread: Uuid,
  credentials: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialIssuance {
  pub fn new(context: String, thread: Uuid, credentials: Vec<String>) -> Self {
    Self {
      context,
      thread,
      credentials,
      callback_url: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the credential issuance's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the credential issuance's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the credential issuance's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the credential issuance's thread.
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the credential issuance's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the credential issuance's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
    self.thread = thread;
  }

  /// Get a mutable reference to the credential issuance's credentials.
  pub fn credentials_mut(&mut self) -> &mut Vec<String> {
    &mut self.credentials
  }

  /// Get a reference to the credential issuance's credentials.
  pub fn credentials(&self) -> &Vec<String> {
    &self.credentials
  }

  /// Set the credential issuance's credentials.
  pub fn set_credentials(&mut self, credentials: Vec<String>) {
    self.credentials = credentials;
  }

  /// Get a mutable reference to the credential issuance's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Get a reference to the credential issuance's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Set the credential issuance's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the credential issuance's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the credential issuance's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the credential issuance's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the credential issuance's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the credential issuance's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the credential issuance's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the credential issuance's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the credential issuance's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the credential issuance's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
