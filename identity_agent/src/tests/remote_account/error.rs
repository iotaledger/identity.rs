// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// The error type for the [`RemoteAccount`].
#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub(crate) enum RemoteAccountError {
  #[error("identity not found")]
  IdentityNotFound,
  #[error("placeholder DIDs cannot be managed")]
  PlaceholderDID,
}
