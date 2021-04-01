// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::ToJson as _;
use serde::Serialize;
use serde::Deserialize;

use crate::{envelope::EnvelopeExt, error::Result, error::Error};

/// A DIDComm Plaintext Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-plaintext-messages)
///
/// # Layout
///
///   `JWM(Content)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope(String);

impl Envelope {
  pub fn from_message<T: Serialize>(message: &T) -> Result<Self> {
    message.to_json().map_err(Into::into).map(Self)
  }
  pub fn to_message<'a, T>(&'a self) -> Result<T> 
  where 
  T: Deserialize<'a> {
    serde_json::from_str(&self.0).map_err(Error::from)
  }
  
}

impl EnvelopeExt for Envelope {
  const FEXT: &'static str = "dcpm";
  const MIME: &'static str = "application/didcomm-plain+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}
