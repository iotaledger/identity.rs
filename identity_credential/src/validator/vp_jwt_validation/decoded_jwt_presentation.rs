// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::{Object, Timestamp, Url};
use identity_verification::jws::JwsHeader;

use crate::{presentation::JwtPresentation, validator::vc_jwt_validation::DecodedJwtCredential};

/// Decoded [`Credential`] from a cryptographically verified JWS.
/// Note that having an instance of this type only means the JWS it was constructed from was verified.
/// It does not imply anything about a potentially present proof property on the credential itself.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct DecodedJwtPresentation<T = Object, U = Object> {
  /// The decoded credential parsed to the [Verifiable Credentials Data model](https://www.w3.org/TR/vc-data-model/).
  pub presentation: JwtPresentation<T>,
  /// The protected header parsed from the JWS.
  pub header: Box<JwsHeader>,

  pub expiration_date: Option<Timestamp>,

  pub aud: Option<Url>,

  pub credentials: Vec<DecodedJwtCredential<U>>,
}
