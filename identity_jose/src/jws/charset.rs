// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::from_utf8;

use crate::error::Error;
use crate::error::Result;

/// The requirements towards the character set when encoding a JWS.
///
/// See the individual variants for more details.
#[derive(Clone, Copy, Debug)]
pub enum CharSet {
  /// The ASCII space character and all printable ASCII characters other
  /// than period ('.') (those characters in the ranges %x20-2D and %x2F-7E)
  /// MAY be included in a non-detached payload using the JWS Compact
  /// Serialization, provided that the application can transmit the
  /// resulting JWS without modification.
  Default,
  /// If a JWS using the JWS Compact Serialization and a non-detached
  /// payload is to be transmitted in a context that requires URL-safe
  /// characters, then the application MUST ensure that the payload
  /// contains only the URL-safe characters 'a'-'z', 'A'-'Z', '0'-'9',
  /// dash ('-'), underscore ('_'), and tilde ('~').
  UrlSafe,
}

impl CharSet {
  /// Validate unencoded content for used with the compact serialization
  /// format. The payload MUST NOT contain a period (`.`) and MAY be
  /// required to only contain URL-safe characters.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7797#section-5.2)
  pub fn validate<'data>(&self, data: &'data [u8]) -> Result<&'data str> {
    let payload: &'data str = from_utf8(data).map_err(|_| Error::InvalidContent("invalid UTF-8"))?;

    if payload.contains('.') {
      return Err(Error::InvalidContent("invalid character `.`"));
    }

    if !self.__validate(payload) {
      return Err(Error::InvalidContent("invalid character(s)"));
    }

    Ok(payload)
  }

  fn __validate(&self, data: &str) -> bool {
    match self {
      Self::Default => data.chars().all(|ch| matches!(ch, '\x20'..='\x2D' | '\x2F'..='\x7E')),
      Self::UrlSafe => data
        .chars()
        .all(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '~')),
    }
  }
}
