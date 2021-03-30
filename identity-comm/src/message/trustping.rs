use crate::message::Message;

use crate::message::Timing;
use did_doc::{url::Url, Document, Signature};
use identity_iota::did::DID;
use serde::Serialize;

macro_rules! impl_accessors {
  ($fn:ident, $ty:ty) => {
    pub fn $fn(&self) -> Option<$ty> {
      self.0.$fn.as_ref()
    }
  };
}
macro_rules! impl_accessors_mut {
  ($fn:ident,$vn:ident, $ty:ty) => {
    pub fn $fn(&mut self) -> Option<$ty> {
      self.0.$vn.as_mut()
    }
  };
}
macro_rules! impl_setter {
  ($fn:ident,$vn:ident, $ty:ty) => {
    pub fn $fn(&mut self, value: Option<$ty>) {
      self.0.$vn = value
    }
  };
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TrustpingRequest(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TrustpingResponse(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DidRequest(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DidResponse(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResolutionRequest(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResolutionResponse(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthenticationRequest(Message);
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthenticationResponse(Message);

impl TrustpingRequest {
  impl_accessors!(callback_url, &Url);
  impl_accessors_mut!(callback_url_mut, callback_url, &mut Url);
  impl_setter!(set_callback_url, callback_url, Url);

  impl_accessors!(response_requested, &bool);
  impl_accessors_mut!(response_requested_mut, response_requested, &mut bool);
  impl_setter!(set_response_requested, response_requested, bool);

  impl_accessors!(context, &Url);
  impl_accessors_mut!(context_mut, context, &mut Url);
  impl_setter!(set_context, context, Url);

  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);

  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(timing, &Timing);
  impl_accessors_mut!(timing_mut, timing, &mut Timing);
  impl_setter!(set_timing, timing, Timing);
}
impl TrustpingResponse {
  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);

  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(timing, &Timing);
  impl_accessors_mut!(timing_mut, timing, &mut Timing);
  impl_setter!(set_timing, timing, Timing);
}
impl DidRequest {
  impl_accessors!(callback_url, &Url);
  impl_accessors_mut!(callback_url_mut, callback_url, &mut Url);
  impl_setter!(set_callback_url, callback_url, Url);

  impl_accessors!(context, &Url);
  impl_accessors_mut!(context_mut, context, &mut Url);
  impl_setter!(set_context, context, Url);

  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);

  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(timing, &Timing);
  impl_accessors_mut!(timing_mut, timing, &mut Timing);
  impl_setter!(set_timing, timing, Timing);
}

impl DidResponse {
  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);
}

impl ResolutionRequest {
  impl_accessors!(callback_url, &Url);
  impl_accessors_mut!(callback_url_mut, callback_url, &mut Url);
  impl_setter!(set_callback_url, callback_url, Url);

  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);

  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(timing, &Timing);
  impl_accessors_mut!(timing_mut, timing, &mut Timing);
  impl_setter!(set_timing, timing, Timing);
}

impl ResolutionResponse {
  impl_accessors!(did_document, &Document);
  impl_accessors_mut!(did_document_mut, did_document, &mut Document);
  impl_setter!(set_did_document, did_document, Document);

  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);

  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(timing, &Timing);
  impl_accessors_mut!(timing_mut, timing, &mut Timing);
  impl_setter!(set_timing, timing, Timing);
}

impl AuthenticationRequest {
  impl_accessors!(callback_url, &Url);
  impl_accessors_mut!(callback_url_mut, callback_url, &mut Url);
  impl_setter!(set_callback_url, callback_url, Url);

  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(challenge, &String);
  impl_accessors_mut!(challenge_mut, challenge, &mut String);
  impl_setter!(set_challenge, challenge, String);

  impl_accessors!(id, &DID);
  impl_accessors_mut!(id_mut, id, &mut DID);
  impl_setter!(set_id, id, DID);

  impl_accessors!(timing, &Timing);
  impl_accessors_mut!(timing_mut, timing, &mut Timing);
  impl_setter!(set_timing, timing, Timing);
}

impl AuthenticationResponse {
  impl_accessors!(thread, &String);
  impl_accessors_mut!(thread_mut, thread, &mut String);
  impl_setter!(set_thread_mut, thread, String);

  impl_accessors!(signature, &Signature);
  impl_accessors_mut!(signature_mut, signature, &mut Signature);
  impl_setter!(set_signature, signature, Signature);
}

mod tests {
  use super::*;
  use crate::message::message::Placeholder;
  #[test]
  pub fn test_setter() {
    let mut message = TrustpingRequest::default();
    message.set_response_requested(Some(true));
    dbg!(message.pack_plain());
  }
}
