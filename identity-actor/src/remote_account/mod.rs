// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod handler;
mod requests;

pub use handler::RemoteAccount;
pub use requests::IdentityCreate;
pub use requests::IdentityGet;
pub use requests::IdentityList;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum RemoteAccountError {
  IdentityNotFound,
  AccountError(String),
}

impl From<identity_account::Error> for RemoteAccountError {
  fn from(err: identity_account::Error) -> Self {
    Self::AccountError(err.to_string())
  }
}
