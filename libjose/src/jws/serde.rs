use core::fmt::Display;
use core::str::from_utf8;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_slice;
use serde_json::to_value;
use serde_json::to_vec;
use serde_json::Map;
use serde_json::Value;

use crate::error::Error;
use crate::error::Result;
use crate::jws::JwsHeader;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::utils::decode_b64_into;
use crate::utils::encode_b64_into;
use crate::utils::Empty;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct JwsRawToken<T = Empty> {
  pub header: JwsHeader<T>,
  pub claims: Vec<u8>,
}

/// The JWS Compact Serialization represents digitally signed or MACed
/// content as a compact, URL-safe string. This string is:
///
///    BASE64URL(UTF8(JWS Protected Header)) || '.' ||
///    BASE64URL(JWS Payload) || '.' ||
///    BASE64URL(JWS Signature)
///
/// Only one signature/MAC is supported by the JWS Compact Serialization and
/// it provides no syntax to represent a JWS Unprotected Header value.
///
/// [RFC7515#3.1](https://tools.ietf.org/html/rfc7515#section-3.1)
///
/// [RFC7515#5.1](https://tools.ietf.org/html/rfc7515#section-5.1)
///
/// [RFC7515#7.1](https://tools.ietf.org/html/rfc7515#section-7.1)
pub fn serialize_compact<T>(
  data: impl AsRef<[u8]>,
  header: &JwsHeader<T>,
  signer: &dyn JwsSigner,
) -> Result<String>
where
  T: Serialize,
{
  serialize_components(data, header, signer).map(Components::into_compact)
}

/// Serialize the token according to the JWS detached content specification.
///
/// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
pub fn serialize_detached<T>(
  data: impl AsRef<[u8]>,
  header: &JwsHeader<T>,
  signer: &dyn JwsSigner,
) -> Result<String>
where
  T: Serialize,
{
  serialize_components(data, header, signer).map(Components::into_detached)
}

pub fn deserialize_compact<T>(
  data: impl AsRef<[u8]>,
  verifier: &dyn JwsVerifier,
) -> Result<JwsRawToken<T>>
where
  T: DeserializeOwned,
{
  let config = DeserializeConfig::new(verifier);
  let (header, components) = deserialize_compact_components(data, config)?;

  Ok(JwsRawToken {
    header,
    claims: components.payload,
  })
}

pub fn deserialize_detached<T>(
  data: impl AsRef<[u8]>,
  payload: impl AsRef<[u8]>,
  verifier: &dyn JwsVerifier,
) -> Result<JwsHeader<T>>
where
  T: DeserializeOwned,
{
  let config = DeserializeConfig::with_payload(verifier, payload.as_ref());
  let (header, _) = deserialize_compact_components(data, config)?;

  Ok(header)
}

// =============================================================================
//
// =============================================================================

type RawComponents = Components<Vec<u8>>;
type B64Components = Components<String>;

#[derive(Debug)]
struct Components<T> {
  header: T,
  payload: T,
  signature: T,
}

impl<T> Components<T> {
  fn new() -> Self
  where
    T: Default,
  {
    Self {
      header: T::default(),
      payload: T::default(),
      signature: T::default(),
    }
  }

  fn create_message(&self) -> Vec<u8>
  where
    T: AsRef<[u8]>,
  {
    create_jws_message(&self.header, &self.payload)
  }

  fn into_compact(self) -> String
  where
    T: Display,
  {
    format!("{}.{}.{}", self.header, self.payload, self.signature)
  }

  fn into_detached(self) -> String
  where
    T: Display,
  {
    format!("{}..{}", self.header, self.signature)
  }
}

struct DeserializeConfig<'a, 'b> {
  verifier: &'a dyn JwsVerifier,
  payload: Option<&'b [u8]>,
}

impl<'a, 'b> DeserializeConfig<'a, 'b> {
  pub fn new(verifier: &'a dyn JwsVerifier) -> Self {
    Self {
      verifier,
      payload: None,
    }
  }

  pub fn with_payload(verifier: &'a dyn JwsVerifier, payload: &'b [u8]) -> Self {
    Self {
      verifier,
      payload: Some(payload),
    }
  }

  pub fn detached(&self) -> bool {
    self.payload.is_some()
  }
}

// =============================================================================
//
// =============================================================================

fn serialize_components<T>(
  data: impl AsRef<[u8]>,
  header: &JwsHeader<T>,
  signer: &dyn JwsSigner,
) -> Result<B64Components>
where
  T: Serialize,
{
  let payload: &[u8] = data.as_ref();

  // Extract the "b64" which header which determines the content encoding method.
  let b64: bool = extract_b64(header);

  // Extract the JOSE header, agumented with claims from the signer.
  let header: Map<String, Value> = extract_header(header, signer)?;

  let mut components: B64Components = Components::new();

  // Encode and add the header.
  encode_b64_into(&to_vec(&header)?, &mut components.header);

  // Encode and add the payload; the payload MAY be base64-encoded.
  if b64 {
    // Serialize and include the encoded payload.
    encode_b64_into(payload, &mut components.payload);
  } else {
    // Add the payload as a raw string.
    components
      .payload
      .push_str(extract_unencoded_payload(payload)?);
  }

  // Crate and sign the message
  let message: Vec<u8> = components.create_message();
  let signature: Vec<u8> = signer.sign(&message)?;

  // Encode and add the signature
  encode_b64_into(&signature, &mut components.signature);

  // Return the raw JSON web signature components
  Ok(components)
}

fn deserialize_compact_components<T>(
  data: impl AsRef<[u8]>,
  config: DeserializeConfig,
) -> Result<(JwsHeader<T>, RawComponents)>
where
  T: DeserializeOwned,
{
  // Extract the components of the compact JWS token
  let split: Vec<&[u8]> = data.as_ref().split(|byte| *byte == b'.').collect();

  // Make sure the JWS token has the expected number of components
  if split.len() != 3 {
    return Err(Error::InvalidJwsFormat(anyhow!("Bad Content")));
  }

  // The middle segment should be empty when using detached content
  if config.detached() && !split[1].is_empty() {
    return Err(Error::InvalidJwsFormat(anyhow!("Bad Content")));
  }

  let mut data: RawComponents = Components::new();

  // Extract the base64-encoded components of the token for convenience
  let header_raw: &[u8] = split[0];
  let payload_raw: &[u8] = config.payload.unwrap_or(split[1]);
  let signature_raw: &[u8] = split[2];

  // Add the base64-decoded header to the components
  decode_b64_into(header_raw, &mut data.header)?;

  // Deserialize to a JOSE header
  let jose: JwsHeader<T> = from_slice(&data.header)?;

  // Validate the claims in the JOSE header
  check_header_claims(&jose, config.verifier)?;

  // Parse the signature
  decode_b64_into(&signature_raw, &mut data.signature)?;

  // Re-build the message
  let message: Vec<u8> = create_jws_message(header_raw, payload_raw);

  // Verify the signature
  config.verifier.verify(&message, &data.signature)?;

  // Only extract and decode the payload if NOT using the detached serialization
  if !config.detached() {
    // Check if the payload is base64url-encoded and decode if necessary
    if extract_b64(&jose) {
      decode_b64_into(payload_raw, &mut data.payload)?;
    } else {
      data.payload.extend_from_slice(payload_raw);
    }
  }

  Ok((jose, data))
}

// =============================================================================
//
// =============================================================================

// Handle the "b64" header parameter.
//
// See [RFC7797](https://tools.ietf.org/html/rfc7797#section-3) for more info.
//
// The following table shows the JWS Signing Input computation, depending
// upon the value of this parameter:
//
// +-------+-----------------------------------------------------------+
// | "b64" | JWS Signing Input Formula                                 |
// +-------+-----------------------------------------------------------+
// | true  | ASCII(BASE64URL(UTF8(JWS Protected Header)) || '.' ||     |
// |       | BASE64URL(JWS Payload))                                   |
// |       |                                                           |
// | false | ASCII(BASE64URL(UTF8(JWS Protected Header)) || '.') ||    |
// |       | JWS Payload                                               |
// +-------+-----------------------------------------------------------+
fn extract_b64<T>(header: &JwsHeader<T>) -> bool {
  header
    .crit()
    .filter(|crit| crit.iter().any(|crit| crit == "b64"))
    .and_then(|_| header.b64())
    .unwrap_or(true)
}

// Extract the JOSE header as a JSON object with signer-specific claims.
fn extract_header<T>(header: &JwsHeader<T>, signer: &dyn JwsSigner) -> Result<Map<String, Value>>
where
  T: Serialize,
{
  let mut object = match to_value(header)? {
    Value::Object(object) => object,
    _ => unreachable!(),
  };

  // This MUST be present in the signed JOSE header.
  object.insert("alg".into(), signer.alg().name().into());

  // Use the "kid" claim from the signer, if present.
  if let Some(kid) = signer.kid() {
    object.insert("kid".into(), kid.to_string().into());
  }

  Ok(object)
}

fn extract_unencoded_payload(data: &[u8]) -> Result<&str> {
  let payload: &str = from_utf8(data).map_err(|_| Error::InvalidClaims(anyhow!("Bad Content")))?;

  // Validate the payload
  //
  // [More Info](https://tools.ietf.org/html/rfc7797#section-5.2)
  //
  // TODO: Provide a more flexible API for this validaton
  if payload.contains('.') {
    return Err(Error::InvalidClaims(anyhow!("Bad Content")));
  }

  Ok(payload)
}

fn create_jws_message(header: impl AsRef<[u8]>, payload: impl AsRef<[u8]>) -> Vec<u8> {
  let header: &[u8] = header.as_ref();
  let payload: &[u8] = payload.as_ref();
  let capacity: usize = header.len() + 1 + payload.len();
  let mut message: Vec<u8> = Vec::with_capacity(capacity);

  message.extend_from_slice(header);
  message.push(b'.');
  message.extend_from_slice(payload);
  message
}

fn check_header_claims<T>(header: &JwsHeader<T>, verifier: &dyn JwsVerifier) -> Result<()> {
  check_header_alg(header, verifier)?;
  check_header_kid(header, verifier)?;
  Ok(())
}

/// Ensure the header algorithm matches the verifier algorithm.
fn check_header_alg<T>(header: &JwsHeader<T>, verifier: &dyn JwsVerifier) -> Result<()> {
  if header.alg() == verifier.alg() {
    Ok(())
  } else {
    Err(Error::InvalidJwsFormat(anyhow!("Invalid Claim (alg)")))
  }
}

/// Ensure the header key ID matches the verifier key ID if one is set.
fn check_header_kid<T>(header: &JwsHeader<T>, verifier: &dyn JwsVerifier) -> Result<()> {
  match (verifier.kid(), header.kid()) {
    (Some(kid), Some(value)) if value == kid => Ok(()),
    (Some(_), Some(_)) => Err(Error::InvalidJwsFormat(anyhow!("Invalid Claim (kid)"))),
    (Some(_), None) => Err(Error::InvalidJwsFormat(anyhow!("Missing Claim (kid)"))),
    (None, _) => Ok(()),
  }
}
