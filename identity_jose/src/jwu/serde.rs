// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str;

use crate::error::Error;
use crate::error::Result;
use crate::jose::JoseHeader;
use crate::jws::JwsHeader;

// The default value of the "b64" header parameter
const DEFAULT_B64: bool = true;

// Claims defined in the base JWE/JWS RFCs
const PREDEFINED: &[&str] = &[
  "alg", "jku", "jwk", "kid", "x5u", "x5c", "x5t", "x5t#s256", "typ", "cty", "crit", "enc", "zip", "epk", "apu", "apv",
  "iv", "tag", "p2s", "p2c",
];

pub fn parse_utf8(slice: &(impl AsRef<[u8]> + ?Sized)) -> Result<&str> {
  str::from_utf8(slice.as_ref()).map_err(Error::InvalidUtf8)
}

pub fn check_slice_param<T>(name: &'static str, slice: Option<&[T]>, value: &T) -> Result<()>
where
  T: PartialEq,
{
  if slice.map(|slice| slice.contains(value)).unwrap_or(true) {
    Ok(())
  } else {
    Err(Error::InvalidParam(name))
  }
}

pub fn filter_non_empty_bytes<'a, T, U: 'a>(value: T) -> Option<&'a [u8]>
where
  T: Into<Option<&'a U>>,
  U: AsRef<[u8]> + ?Sized,
{
  value.into().map(AsRef::as_ref).filter(|value| !value.is_empty())
}

pub fn create_message(header: &[u8], claims: &[u8]) -> Vec<u8> {
  let capacity: usize = header.len() + 1 + claims.len();
  let mut message: Vec<u8> = Vec::with_capacity(capacity);

  message.extend(header);
  message.push(b'.');
  message.extend(claims);
  message
}

pub fn extract_b64(header: Option<&JwsHeader>) -> bool {
  header.and_then(JwsHeader::b64).unwrap_or(DEFAULT_B64)
}

pub fn validate_jws_headers(
  protected: Option<&JwsHeader>,
  unprotected: Option<&JwsHeader>,
  permitted: Option<&[String]>,
) -> Result<()> {
  validate_disjoint(protected, unprotected)?;
  validate_crit(protected, unprotected, permitted)?;
  validate_b64(protected, unprotected)?;

  Ok(())
}

pub fn validate_crit<T>(protected: Option<&T>, unprotected: Option<&T>, permitted: Option<&[String]>) -> Result<()>
where
  T: JoseHeader,
{
  // The "crit" parameter MUST be integrity protected
  if unprotected.map(|header| header.has_claim("crit")).unwrap_or_default() {
    return Err(Error::InvalidParam("unprotected crit"));
  }

  let values: Option<&[String]> = protected.and_then(|header| header.common().crit());

  // The "crit" parameter MUST NOT be an empty list
  if values.map(|values| values.is_empty()).unwrap_or_default() {
    return Err(Error::InvalidParam("empty crit"));
  }

  let permitted: &[String] = permitted.unwrap_or_default();
  let values: &[String] = values.unwrap_or_default();

  for value in values {
    // The "crit" parameter MUST NOT contain any header parameters defined by
    // the JOSE JWS/JWA specifications.
    if PREDEFINED.contains(&&**value) {
      return Err(Error::InvalidParam("crit contains pre-defined parameters"));
    }

    // The "crit" parameter MUST be understood by the application.
    if !permitted.contains(value) {
      return Err(Error::InvalidParam("unpermitted crit"));
    }

    let exists: bool = protected
      .map(|header| header.has_claim(value))
      .or_else(|| unprotected.map(|header| header.has_claim(value)))
      .unwrap_or_default();

    if !exists {
      return Err(Error::InvalidParam("crit"));
    }
  }

  Ok(())
}

pub fn validate_b64(protected: Option<&JwsHeader>, unprotected: Option<&JwsHeader>) -> Result<()> {
  // The "b64" parameter MUST be integrity protected
  if unprotected.and_then(JwsHeader::b64).is_some() {
    return Err(Error::InvalidParam("unprotected `b64` parameter"));
  }

  let b64: Option<bool> = protected.and_then(|header| header.b64());
  let crit: Option<&[String]> = protected.and_then(|header| header.crit());

  // The "b64" parameter MUST be included in the "crit" parameter values
  match (b64, crit) {
    (Some(_), Some(values)) if values.iter().any(|value| value == "b64") => Ok(()),
    (Some(_), None) => Err(Error::InvalidParam(
      "`b64` param must be included in the crit parameter values",
    )),
    _ => Ok(()),
  }
}

pub fn validate_disjoint(protected: Option<&JwsHeader>, unprotected: Option<&JwsHeader>) -> Result<()> {
  let is_disjoint: bool = match (protected, unprotected) {
    (Some(protected), Some(unprotected)) => protected.is_disjoint(unprotected),
    _ => true,
  };

  if is_disjoint {
    Ok(())
  } else {
    Err(Error::InvalidContent(
      "protected and unprotected headers are not disjoint",
    ))
  }
}
