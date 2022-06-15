// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Functionality for creating [DIDComm plaintext messages](https://identity.foundation/didcomm-messaging/spec/#didcomm-plaintext-messages)

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;

use crate::envelope::EnvelopeExt;
use crate::error::Result;

/// A DIDComm Plaintext Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-plaintext-messages)
///
/// # Layout
///
///   `JWM(Content)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Plaintext(pub(crate) String);

impl Plaintext {
  pub fn pack<T: ToJson>(message: &T) -> Result<Self> {
    message.to_json().map_err(Into::into).map(Self)
  }

  pub fn unpack<T: FromJson>(&self) -> Result<T> {
    T::from_json(&self.0).map_err(Into::into)
  }
}

impl EnvelopeExt for Plaintext {
  const FEXT: &'static str = "dcpm";
  const MIME: &'static str = "application/didcomm-plain+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}
