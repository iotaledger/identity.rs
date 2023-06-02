// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;
use identity_core::common::Url;

/// Options to be set in the JWT claims of a verifiable presentation.
#[derive(Clone, Debug)]
pub struct JwtPresentationOptions {
  /// Set the presentation's expiration date.
  /// Default: `None`.
  pub expiration_date: Option<Timestamp>,
  /// Set the issuance date.
  /// Default: current datetime.
  pub issuance_date: Option<Timestamp>,
  /// Sets the audience for presentation (`aud` property in JWT claims).
  /// Default: `None`.
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
