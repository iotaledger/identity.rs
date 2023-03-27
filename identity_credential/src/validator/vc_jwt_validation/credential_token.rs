// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use crate::credential::Credential;
use identity_core::common::Object;
use identity_verification::jose::jws::JwsHeader;

/// Decoded [`Credential`] from a cryptographically verified JWS.
// TODO: Currently this can only be obtained with `T = Object`. Either generalize
// the functions returning this type to be generic over `T`, or remove the generic parameter.
#[non_exhaustive]
pub struct CredentialToken<T = Object> {
  /// The decoded Credential
  pub credential: Credential<T>,
  /// The protected header parsed from the JWS.
  pub header: Box<JwsHeader>,
}
