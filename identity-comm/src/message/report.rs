use crate::message::Timing;
use identity_core::common::Url;

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
  context: Url,
  //todo: needs to be an uuid -> https://github.com/uuid-rs/uuid
  thread: String,
  reference: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Report {
  pub fn new(context: Url, thread: String, reference: String) -> Self {
    Self {
      context,
      thread,
      reference,
      comment:None,
      timing:None,
    }
  }

    /// Get a mutable reference to the report's context.
    pub fn context_mut(&mut self) -> &mut Url {
        &mut self.context
    }

    /// Get a reference to the report's context.
    pub fn context(&self) -> &Url {
        &self.context
    }

    /// Set the report's context.
    pub fn set_context(&mut self, context: Url) {
        self.context = context;
    }

    /// Get a mutable reference to the report's thread.
    pub fn thread_mut(&mut self) -> &mut String {
        &mut self.thread
    }

    /// Get a reference to the report's thread.
    pub fn thread(&self) -> &String {
        &self.thread
    }

    /// Set the report's thread.
    pub fn set_thread(&mut self, thread: String) {
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
