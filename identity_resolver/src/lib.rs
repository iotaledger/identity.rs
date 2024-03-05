// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![doc = include_str!("./../README.md")]
#![warn(
  rust_2018_idioms,
  unreachable_pub,
  missing_docs,
  rustdoc::missing_crate_level_docs,
  rustdoc::broken_intra_doc_links,
  rustdoc::private_intra_doc_links,
  rustdoc::private_doc_tests,
  clippy::missing_safety_doc
)]

mod error;
mod resolution;

use std::ops::Deref;

pub use self::error::Error;
pub use self::error::ErrorCause;
pub use self::error::Result;
use identity_core::ResolverT;
pub use resolution::*;

use identity_did::DIDUrl;
use identity_iota_core::Error as IotaError;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaIdentityClientExt;
use identity_verification::MethodData;
use iota_sdk::client::Client;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct IotaResolver(Client);

impl Deref for IotaResolver {
  type Target = Client;
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

impl ResolverT<MethodData> for IotaResolver {
  type Input = DIDUrl;
  type Error = IotaError;

  async fn fetch(&self, input: &Self::Input) -> Result<MethodData, Self::Error> {
    let did = IotaDID::try_from_core(input.did().clone()).map_err(IotaError::DIDSyntaxError)?;
    let doc = self.resolve_did(&did).await?;

    let key = doc
      .resolve_method(input, None)
      .map(|method| method.data())
      .cloned()
      .ok_or(todo!("an error for verification method not found"))?;

    Ok(key)
  }
}
