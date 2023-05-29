// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::{Object, Timestamp, Url};
use identity_verification::jws::JwsHeader;

use crate::{presentation::JwtPresentation, validator::vc_jwt_validation::DecodedJwtCredential};

/// Decoded [`JwtPresentation`] from a cryptographically verified JWS.
/// Note that having an instance of this type only means the JWS it was constructed from was verified.
/// It does not imply anything about a potentially present proof property on the presentation itself.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct DecodedJwtPresentation<T = Object, U = Object> {
  /// The decoded presentation parsed to the [Verifiable Credentials Data model](https://www.w3.org/TR/vc-data-model/).
  pub presentation: JwtPresentation<T>,
  /// The protected header parsed from the JWS.
  pub header: Box<JwsHeader>,
  /// The expiration dated parsed from the JWT claims.
  pub expiration_date: Option<Timestamp>,
  /// The `aud` property parsed from the JWT claims.
  pub aud: Option<Url>,
  /// The credentials included in the presentation (decoded).
  pub credentials: Vec<DecodedJwtCredential<U>>,
}
