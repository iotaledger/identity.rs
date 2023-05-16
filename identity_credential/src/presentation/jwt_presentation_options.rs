// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::common::Url;

#[derive(Clone, Debug)]
pub struct JwtPresentationOptions {
  pub expiration_date: Option<Timestamp>,
  pub issuance_date: Option<Timestamp>,
  pub audience: Option<Url>,
}

impl Default for JwtPresentationOptions {
  fn default() -> Self {
    Self {
      expiration_date: None,
      issuance_date: Some(Timestamp::now_utc()),
      audience: None,
    }
  }
}
