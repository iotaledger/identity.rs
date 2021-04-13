use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialOptionRequest {
  context: String,
  thread: String,
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl CredentialOptionRequest {
  pub fn new(
    context: String,
    thread: String,
    callback_url: Url,
  ) -> Self {
    Self {
      context,
      thread,
      callback_url,
      response_requested: None,
      id: None,
      timing: None,
    }
  }
}
