// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::de;
use serde::Deserialize;

use crate::did::DIDUrl;
use crate::did::DID;
use crate::Error;
use crate::Result;

/// Deserializes an [`DIDUrl`] while enforcing that its fragment is non-empty.
pub fn deserialize_did_url_with_fragment<'de, D, T>(deserializer: D) -> Result<DIDUrl<T>, D::Error>
where
  D: de::Deserializer<'de>,
  T: DID + serde::Deserialize<'de>,
{
  let did_url: DIDUrl<T> = DIDUrl::deserialize(deserializer)?;
  check_fragment_non_empty(&did_url).map_err(de::Error::custom)?;
  Ok(did_url)
}

/// Validates whether the given [`DIDUrl`] has an identifying fragment for a verification method.
///
/// # Errors
/// [`Error::InvalidMethodFragment`] if the fragment is missing.
pub fn check_fragment_non_empty<D>(id: &DIDUrl<D>) -> Result<()>
where
  D: DID,
{
  if id.fragment().unwrap_or_default().is_empty() {
    return Err(Error::InvalidMethodFragment);
  }
  Ok(())
}
