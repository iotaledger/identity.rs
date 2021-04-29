// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use uuid::Uuid;

/// A DIDComm  report message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/Standalone_Messages.md)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Report {
  context: String,
  thread: Uuid,
  reference: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Report {
  pub fn new(context: String, thread: Uuid, reference: String) -> Self {
    Self {
      context,
      thread,
      reference,
      comment: None,
      timing: None,
    }
  }
  pub fn default() -> Self {
    let default_context = "default-context".to_string();
    let default_thread = Uuid::new_v4();
    let default_reference = "default-reference".to_string();
    Self {
      context: default_context,
      thread: default_thread,
      reference: default_reference,
      comment: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the report's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the report's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the report's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the report's thread.
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the report's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the report's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
    self.thread = thread;
  }

  /// Get a mutable reference to the report's reference.
  pub fn reference_mut(&mut self) -> &mut String {
    &mut self.reference
  }

  /// Get a reference to the report's reference.
  pub fn reference(&self) -> &String {
    &self.reference
  }

  /// Set the report's reference.
  pub fn set_reference(&mut self, reference: String) {
    self.reference = reference;
  }

  /// Get a mutable reference to the report's comment.
  pub fn comment_mut(&mut self) -> &mut Option<String> {
    &mut self.comment
  }

  /// Get a reference to the report's comment.
  pub fn comment(&self) -> &Option<String> {
    &self.comment
  }

  /// Set the report's comment.
  pub fn set_comment(&mut self, comment: Option<String>) {
    self.comment = comment;
  }

  /// Get a mutable reference to the report's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the report's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the report's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
