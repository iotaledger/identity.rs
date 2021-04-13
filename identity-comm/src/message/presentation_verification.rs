use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;

#[derive(Debug, Deserialize, Serialize)]
pub struct PresentationRequest {
  context: String,
  thread: String,
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  credential_types: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  supported_issuers: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl PresentationRequest {
  pub fn new(context: String, thread: String, callback_url: Url) -> Self {
    Self {
      context,
      thread,
      callback_url,
      credential_types: None,
      supported_issuers: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the presentation request's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the presentation request's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the presentation request's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the presentation request's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the presentation request's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the presentation request's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the presentation request's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Get a reference to the presentation request's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Set the presentation request's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the presentation request's credential types.
  pub fn credential_types_mut(&mut self) -> &mut Option<Vec<String>> {
    &mut self.credential_types
  }

  /// Get a reference to the presentation request's credential types.
  pub fn credential_types(&self) -> &Option<Vec<String>> {
    &self.credential_types
  }

  /// Set the presentation request's credential types.
  pub fn set_credential_types(&mut self, credential_types: Option<Vec<String>>) {
    self.credential_types = credential_types;
  }

  /// Get a mutable reference to the presentation request's supported issuers.
  pub fn supported_issuers_mut(&mut self) -> &mut Option<Vec<String>> {
    &mut self.supported_issuers
  }

  /// Get a reference to the presentation request's supported issuers.
  pub fn supported_issuers(&self) -> &Option<Vec<String>> {
    &self.supported_issuers
  }

  /// Set the presentation request's supported issuers.
  pub fn set_supported_issuers(&mut self, supported_issuers: Option<Vec<String>>) {
    self.supported_issuers = supported_issuers;
  }

  /// Get a mutable reference to the presentation request's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the presentation request's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the presentation request's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the presentation request's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the presentation request's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the presentation request's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the presentation request's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the presentation request's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the presentation request's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PresentationResponse {
  context: String,
  thread: String,
  verifiable_presentation: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl PresentationResponse {
  pub fn new(context: String, thread: String, verifiable_presentation: Vec<String>) -> Self {
    Self {
      context,
      thread,
      verifiable_presentation,
      callback_url: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the presentation response's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the presentation response's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the presentation response's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the presentation response's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the presentation response's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the presentation response's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the presentation response's verifiable presentation.
  pub fn verifiable_presentation_mut(&mut self) -> &mut Vec<String> {
    &mut self.verifiable_presentation
  }

  /// Get a reference to the presentation response's verifiable presentation.
  pub fn verifiable_presentation(&self) -> &Vec<String> {
    &self.verifiable_presentation
  }

  /// Set the presentation response's verifiable presentation.
  pub fn set_verifiable_presentation(&mut self, verifiable_presentation: Vec<String>) {
    self.verifiable_presentation = verifiable_presentation;
  }

  /// Get a mutable reference to the presentation response's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Get a reference to the presentation response's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Set the presentation response's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the presentation response's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the presentation response's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the presentation response's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the presentation response's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the presentation response's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the presentation response's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the presentation response's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the presentation response's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the presentation response's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
