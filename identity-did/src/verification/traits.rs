// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;

/// Represents all possible verification method types
///
/// see [W3C DID-core spec](https://www.w3.org/TR/did-core/#relative-did-urls)
pub enum MethodUriType {
  Absolute,
  Relative,
}

/// Used to return absolute or relative method URI.
///
/// This trait is used to determine whether absolute or relative method URIs
/// should be used to signing data.
///
/// [More Info](https://www.w3.org/TR/did-core/#relative-did-urls)
pub trait TryMethod {
  /// Flag that determines whether absolute or rleative URI
  const TYPE: MethodUriType;

  /// Returns String representation of absolute or relative method URI, if any.
  fn method(method_id: &DID) -> Option<String> {
    let fragment = method_id.fragment()?;

    Some(match Self::TYPE {
      MethodUriType::Absolute => method_id.to_string(),
      MethodUriType::Relative => core::iter::once('#').chain(fragment.chars()).collect(),
    })
  }

  /// Returns String representation of absolute or relative method URI.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used.
  fn try_method(method_id: &DID) -> Result<String> {
    Self::method(method_id).ok_or(Error::InvalidMethodFragment)
  }
}
