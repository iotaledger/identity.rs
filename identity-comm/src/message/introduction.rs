// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct IntroductionProposal {
  context: String,
  thread: String,
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl IntroductionProposal {
  pub fn new(context: String, thread: String, callback_url: Url) -> Self {
    Self {
      context,
      thread,
      callback_url,
      response_requested: None,
      id: None,
      comment: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the introduction proposal's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the introduction proposal's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the introduction proposal's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the introduction proposal's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the introduction proposal's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the introduction proposal's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the introduction proposal's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Get a reference to the introduction proposal's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Set the introduction proposal's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the introduction proposal's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the introduction proposal's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the introduction proposal's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the introduction proposal's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the introduction proposal's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the introduction proposal's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the introduction proposal's comment.
  pub fn comment_mut(&mut self) -> &mut Option<String> {
    &mut self.comment
  }

  /// Get a reference to the introduction proposal's comment.
  pub fn comment(&self) -> &Option<String> {
    &self.comment
  }

  /// Set the introduction proposal's comment.
  pub fn set_comment(&mut self, comment: Option<String>) {
    self.comment = comment;
  }

  /// Get a mutable reference to the introduction proposal's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the introduction proposal's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the introduction proposal's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct IntroductionResponse {
  context: String,
  thread: String,
  consent: bool,
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

impl IntroductionResponse {
  pub fn new(context: String, thread: String, consent: bool) -> Self {
    Self {
      context,
      thread,
      consent,
      callback_url: None,
      response_requested: None,
      id: None,
      comment: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the introduction response's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the introduction response's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the introduction response's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the introduction response's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the introduction response's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the introduction response's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the introduction response's consent.
  pub fn consent_mut(&mut self) -> &mut bool {
    &mut self.consent
  }

  /// Get a reference to the introduction response's consent.
  pub fn consent(&self) -> &bool {
    &self.consent
  }

  /// Set the introduction response's consent.
  pub fn set_consent(&mut self, consent: bool) {
    self.consent = consent;
  }

  /// Get a mutable reference to the introduction response's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Get a reference to the introduction response's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Set the introduction response's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the introduction response's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the introduction response's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the introduction response's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the introduction response's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the introduction response's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the introduction response's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the introduction response's comment.
  pub fn comment_mut(&mut self) -> &mut Option<String> {
    &mut self.comment
  }

  /// Get a reference to the introduction response's comment.
  pub fn comment(&self) -> &Option<String> {
    &self.comment
  }

  /// Set the introduction response's comment.
  pub fn set_comment(&mut self, comment: Option<String>) {
    self.comment = comment;
  }

  /// Get a mutable reference to the introduction response's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the introduction response's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the introduction response's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
