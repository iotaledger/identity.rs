// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

use crate::error::Error;
use crate::error::Result;
use crate::revocation::RevocationBitmap;

/// A parsed data url.
#[derive(Clone, Debug, PartialEq)]
pub struct BitmapRevocationEndpoint(pub(crate) String);

impl BitmapRevocationEndpoint {
  /// Parses an [`BitmapRevocationEndpoint`] from the given input string.
  pub fn parse(input: &str) -> Result<Self> {
    data_url::DataUrl::process(input).map_err(|_| Error::InvalidBitmapEndpoint("not a valid data url"))?;
    Ok(BitmapRevocationEndpoint(input.to_owned()))
  }

  /// Consumes the [`BitmapRevocationEndpoint`] and returns the value as a `String`.
  pub fn into_string(self) -> String {
    self.0
  }

  /// Returns the data from the [`BitmapRevocationEndpoint`].
  pub fn data(&self) -> &str {
    self.0.split_at(6).1
  }
}

impl TryFrom<Url> for BitmapRevocationEndpoint {
  type Error = Error;

  fn try_from(other: Url) -> Result<Self> {
    Self::parse(&other.into_string())
  }
}

impl TryFrom<BitmapRevocationEndpoint> for Url {
  type Error = Error;

  fn try_from(other: BitmapRevocationEndpoint) -> Result<Self> {
    Self::parse(&other.0).map_err(|_| Error::InvalidBitmapEndpoint("not a valid data url"))
  }
}

impl TryFrom<&BitmapRevocationEndpoint> for RevocationBitmap {
  type Error = Error;

  fn try_from(endpoint: &BitmapRevocationEndpoint) -> Result<Self> {
    RevocationBitmap::deserialize_compressed_b64(endpoint.data())
  }
}

impl TryFrom<RevocationBitmap> for BitmapRevocationEndpoint {
  type Error = Error;

  fn try_from(revocation_bitmap: RevocationBitmap) -> Result<Self> {
    let b64_encoded_map: String = revocation_bitmap.serialize_compressed_b64()?;
    BitmapRevocationEndpoint::parse(&format!("data:,{}", b64_encoded_map))
  }
}
