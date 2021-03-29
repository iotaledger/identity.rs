// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Copy, Debug)]
pub enum ResourceType {
  Noop,
  IdentityMetadata,
  IdentityDocument,
  AuthData,
  DiffData,
  CredentialMetadata,
  CredentialDocument,
}

impl ResourceType {
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Noop => "",
      Self::IdentityMetadata => "IdentityMetadata",
      Self::IdentityDocument => "IdentityDocument",
      Self::AuthData => "AuthData",
      Self::DiffData => "DiffData",
      Self::CredentialMetadata => "CredentialMetadata",
      Self::CredentialDocument => "CredentialDocument",
    }
  }
}
