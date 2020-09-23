use core::convert::TryFrom;
use core::iter::FromIterator;
use core::str::from_utf8;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_slice;
use serde_json::to_value;
use serde_json::to_vec;
use serde_json::Map;
use serde_json::Value;

use crate::alloc::String;
use crate::alloc::ToString;
use crate::alloc::Vec;
use crate::error::DecodeError;
use crate::error::EncodeError;
use crate::error::Result;
use crate::jws::JwsHeader;
use crate::jws::JwsRawToken;
use crate::jws::JwsSigner;
use crate::jws::JwsToken;
use crate::jws::JwsVerifier;
use crate::utils::decode_b64_into;
use crate::utils::encode_b64_into;

// The default encoding behaviour is to use base64url-encoded payloads
const B64_DEFAULT: bool = true;

// =============================================================================
// Encoding
// =============================================================================

// TODO: Encode General
// TODO: Encode Flattened
pub struct Encoder;

impl Encoder {
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
  pub fn encode_compact<T>(
    data: impl AsRef<[u8]>,
    header: &JwsHeader<T>,
    signer: &dyn JwsSigner,
  ) -> Result<String>
  where
    T: Serialize,
  {
    encode_components(data, header, signer).map(Components::into_compact)
  }

  /// Serialize the token with compact serialization and detached content.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  pub fn encode_compact_detached<T>(
    data: impl AsRef<[u8]>,
    header: &JwsHeader<T>,
    signer: &dyn JwsSigner,
  ) -> Result<String>
  where
    T: Serialize,
  {
    encode_components(data, header, signer).map(Components::into_compact_detached)
  }
}

fn encode_components<T>(
  data: impl AsRef<[u8]>,
  header: &JwsHeader<T>,
  signer: &dyn JwsSigner,
) -> Result<B64Components>
where
  T: Serialize,
{
  let payload: &[u8] = data.as_ref();
  let mut components: B64Components = Components::new();

  // Extract the "b64" which header which determines the content encoding method.
  let b64: bool = extract_b64(header)?;

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

  // Extract the JOSE header, agumented with claims from the signer.
  let header: Map<String, Value> = extract_header(header, signer)?;

  // Encode and add the header.
  encode_b64_into(&to_vec(&header)?, &mut components.header);

  // Create and sign the message
  let message: Vec<u8> = components.create_message();
  let signature: Vec<u8> = signer.sign(&message)?;

  // Encode and add the signature
  encode_b64_into(&signature, &mut components.signature);

  // Return the raw signature components
  Ok(components)
}

// =============================================================================
// Decoding
// =============================================================================

use crate::alloc::BTreeSet;
use crate::jws::JwsAlgorithm;

// TODO: Decode JSON
pub struct Decoder {
  algorithms: BTreeSet<JwsAlgorithm>,
}

impl Decoder {
  pub fn new() -> Self {
    Self {
      algorithms: BTreeSet::new(),
    }
  }

  pub fn with_algorithms(algorithms: impl IntoIterator<Item = impl Into<JwsAlgorithm>>) -> Self {
    Self {
      algorithms: BTreeSet::from_iter(algorithms.into_iter().map(Into::into)),
    }
  }

  pub fn decode_compact_token<T, U>(
    &self,
    data: impl AsRef<[u8]>,
    verifier: &dyn JwsVerifier,
  ) -> Result<JwsToken<T, U>>
  where
    T: DeserializeOwned,
    U: DeserializeOwned,
  {
    self
      .decode_compact(data, verifier)
      .and_then(JwsToken::try_from)
  }

  pub fn decode_compact<T>(
    &self,
    data: impl AsRef<[u8]>,
    verifier: &dyn JwsVerifier,
  ) -> Result<JwsRawToken<T>>
  where
    T: DeserializeOwned,
  {
    let (header, components) = self.decode_compact_components(data, verifier, None)?;

    Ok(JwsRawToken {
      header,
      claims: components.payload,
    })
  }

  pub fn decode_compact_detached<T>(
    &self,
    data: impl AsRef<[u8]>,
    payload: impl AsRef<[u8]>,
    verifier: &dyn JwsVerifier,
  ) -> Result<JwsHeader<T>>
  where
    T: DeserializeOwned,
  {
    let (header, _) = self.decode_compact_components(data, verifier, Some(payload.as_ref()))?;

    Ok(header)
  }

  fn decode_compact_components<T>(
    &self,
    data: impl AsRef<[u8]>,
    verifier: &dyn JwsVerifier,
    payload: Option<&[u8]>,
  ) -> Result<(JwsHeader<T>, RawComponents)>
  where
    T: DeserializeOwned,
  {
    // Extract the components of the compact JWS token
    let split: Vec<&[u8]> = data.as_ref().split(|byte| *byte == b'.').collect();
    let detached: bool = payload.is_some();
    let mut components: RawComponents = Components::new();

    // Make sure the JWS token has the expected number of components
    if split.len() != 3 {
      return Err(DecodeError::InvalidSegments.into());
    }

    // The middle segment should be empty when using detached content
    if detached && !split[1].is_empty() {
      return Err(DecodeError::InvalidSegments.into());
    }

    // Extract the base64-encoded components of the token for convenience
    let header_raw: &[u8] = split[0];
    let payload_raw: &[u8] = payload.unwrap_or(split[1]);
    let signature_raw: &[u8] = split[2];

    // Add the base64-decoded header to the components
    decode_b64_into(header_raw, &mut components.header)?;

    // Deserialize to a JOSE header
    let jose: JwsHeader<T> = from_slice(&components.header)?;

    // Validate the claims in the JOSE header
    self.check_header_claims(&jose, verifier)?;

    // Parse the signature
    decode_b64_into(&signature_raw, &mut components.signature)?;

    // Re-build the message
    let message: Vec<u8> = create_jws_message(header_raw, payload_raw);

    // Verify the signature
    verifier.verify(&message, &components.signature)?;

    // Only extract and decode the payload if NOT using the detached serialization
    if !detached {
      // Check if the payload is base64url-encoded and decode if necessary
      if extract_b64(&jose)? {
        decode_b64_into(payload_raw, &mut components.payload)?;
      } else {
        components.payload.extend_from_slice(payload_raw);
      }
    }

    Ok((jose, components))
  }

  fn check_header_claims<T>(
    &self,
    header: &JwsHeader<T>,
    verifier: &dyn JwsVerifier,
  ) -> Result<()> {
    self.check_header_alg(header, verifier)?;
    self.check_header_kid(header, verifier)?;

    let valid: bool = self.algorithms.contains(&header.alg());

    Ok(())
  }

  /// Ensure the header algorithm matches the verifier algorithm.
  fn check_header_alg<T>(&self, header: &JwsHeader<T>, verifier: &dyn JwsVerifier) -> Result<()> {
    if header.alg() == verifier.alg() {
      Ok(())
    } else {
      Err(DecodeError::InvalidClaim("alg").into())
    }
  }

  /// Ensure the header key ID matches the verifier key ID if one is set.
  fn check_header_kid<T>(&self, header: &JwsHeader<T>, verifier: &dyn JwsVerifier) -> Result<()> {
    match (verifier.kid(), header.kid()) {
      (Some(kid), Some(value)) if value == kid => Ok(()),
      (Some(_), Some(_)) => Err(DecodeError::InvalidClaim("kid").into()),
      (Some(_), None) => Err(DecodeError::MissingClaim("kid").into()),
      (None, _) => Ok(()),
    }
  }
}

// =============================================================================
// Helpers
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
fn extract_b64<T>(header: &JwsHeader<T>) -> Result<bool> {
  match (header.b64(), header.crit()) {
    (Some(b64), Some(crit)) => {
      // The "crit" param MUST be included and contain "b64".
      // More Info: https://tools.ietf.org/html/rfc7797#section-6
      if !crit.iter().any(|crit| crit == "b64") {
        return Err(EncodeError::MissingCritB64.into());
      }

      Ok(b64)
    }
    (Some(_), None) => Err(EncodeError::MissingCrit.into()),
    (None, _) => Ok(B64_DEFAULT),
  }
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
  let payload: &str = match from_utf8(data) {
    Ok(payload) => payload,
    Err(error) => return Err(EncodeError::InvalidContent(error).into()),
  };

  // Validate the payload
  // More Info: https://tools.ietf.org/html/rfc7797#section-5.2
  //
  // TODO: Provide a more flexible API for this validaton
  if payload.contains('.') {
    return Err(EncodeError::InvalidContentChar('.').into());
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

// =============================================================================
// Component Helpers
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
    T: AsRef<str>,
  {
    let mut capacity: usize = 2; // initially 2 for delimiters
    capacity += self.header.as_ref().len();
    capacity += self.payload.as_ref().len();
    capacity += self.signature.as_ref().len();

    let mut output: String = String::with_capacity(capacity);
    output.push_str(self.header.as_ref());
    output.push('.');
    output.push_str(self.payload.as_ref());
    output.push('.');
    output.push_str(self.signature.as_ref());
    output
  }

  fn into_compact_detached(self) -> String
  where
    T: AsRef<str>,
  {
    let mut capacity: usize = 2; // initially 2 for delimiters
    capacity += self.header.as_ref().len();
    capacity += self.signature.as_ref().len();

    let mut output: String = String::with_capacity(capacity);
    output.push_str(self.header.as_ref());
    output.push('.');
    output.push('.');
    output.push_str(self.signature.as_ref());
    output
  }
}
