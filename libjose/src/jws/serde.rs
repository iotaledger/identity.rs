use alloc::vec::Vec;
use alloc::vec;
use alloc::borrow::Cow;
use alloc::collections::BTreeSet;
use alloc::string::String;
use core::convert::TryFrom;
use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter::once;
use core::iter::FromIterator;
use core::str::from_utf8;
use serde::de::DeserializeOwned;
use serde::de::Error as _;
use serde::Serialize;
use serde_json::from_slice;
use serde_json::from_value;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::to_vec;
use serde_json::Error;
use serde_json::Map;
use serde_json::Value;

use crate::error::DecodeError;
use crate::error::EncodeError;
use crate::error::Result;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::JwsRawToken;
use crate::jws::JwsSigner;
use crate::jws::JwsToken;
use crate::jws::JwsVerifier;
use crate::jwt::JwtClaims;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

const PARAM_ALG: &str = "alg";
const PARAM_B64: &str = "b64";
const PARAM_CRIT: &str = "crit";
const PARAM_KID: &str = "kid";

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JwsFormat {
  Compact,
  General,
  Flatten,
}

impl Default for JwsFormat {
  fn default() -> Self {
    Self::Compact
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwsSignature {
  pub signature: String,
  // The spec says this is optional but we don't support unprotected headers so
  // we require it.
  pub protected: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwsEncodedGeneral {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub payload: Option<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub signatures: Vec<JwsSignature>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JwsEncodedFlatten {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub payload: Option<String>,
  #[serde(flatten)]
  pub signature: JwsSignature,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JwsEncoded {
  Compact(String),
  General(JwsEncodedGeneral),
  Flatten(JwsEncodedFlatten),
}

impl JwsEncoded {
  pub fn to_string(&self) -> Result<String> {
    match self {
      Self::Compact(inner) => Ok(inner.clone()),
      Self::General(inner) => to_string(&inner).map_err(Into::into),
      Self::Flatten(inner) => to_string(&inner).map_err(Into::into),
    }
  }
}

impl Display for JwsEncoded {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.to_string().map_err(|_| FmtError)?)
  }
}

// =============================================================================
// Encoding
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharSet {
  Default,
  UrlSafe,
}

impl CharSet {
  pub fn check(&self, data: impl AsRef<str>) -> bool {
    match self {
      Self::Default => {
        // The ASCII space character and all printable ASCII characters other
        // than period ('.') (those characters in the ranges %x20-2D and %x2F-7E)
        // MAY be included in a non-detached payload using the JWS Compact
        // Serialization, provided that the application can transmit the
        // resulting JWS without modification.
        data
          .as_ref()
          .chars()
          .all(|ch| matches!(ch, '\x20'..='\x2D' | '\x2F'..='\x7E'))
      }
      Self::UrlSafe => {
        // If a JWS using the JWS Compact Serialization and a non-detached
        // payload is to be transmitted in a context that requires URL-safe
        // characters, then the application MUST ensure that the payload
        // contains only the URL-safe characters 'a'-'z', 'A'-'Z', '0'-'9',
        // dash ('-'), underscore ('_'), and tilde ('~').
        data
          .as_ref()
          .chars()
          .all(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '~'))
      }
    }
  }
}

impl Default for CharSet {
  fn default() -> Self {
    Self::Default
  }
}

#[derive(Clone, Debug, Default)]
pub struct JwsEncoder {
  /// The serialization format of the encoded token.
  format: JwsFormat,
  /// The validation rules for unencoded content using compact serialization
  charset: CharSet,
  /// Encode the token with detached content.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  detach: bool,
}

impl JwsEncoder {
  /// Creates a new `JwsEncoder` with the default format (Compact).
  pub const fn new() -> Self {
    Self::with_format(JwsFormat::Compact)
  }

  /// Creates a new `JwsEncoder` with the given `format`.
  pub const fn with_format(format: JwsFormat) -> Self {
    Self {
      format,
      charset: CharSet::Default,
      detach: false,
    }
  }

  pub fn format(mut self, value: impl Into<JwsFormat>) -> Self {
    self.format = value.into();
    self
  }

  pub fn charset(mut self, value: impl Into<CharSet>) -> Self {
    self.charset = value.into();
    self
  }

  pub fn attach(mut self) -> Self {
    self.detach = false;
    self
  }

  pub fn detach(mut self) -> Self {
    self.detach = true;
    self
  }

  pub fn encode<T, U>(
    &self,
    claims: &JwtClaims<T>,
    header: &JwsHeader<U>,
    signer: &dyn JwsSigner,
  ) -> Result<JwsEncoded>
  where
    T: Serialize,
    U: Serialize,
  {
    self.encode_slice(&to_vec(claims)?, header, signer)
  }

  pub fn encode_slice<T, U>(
    &self,
    claims: &T,
    header: &JwsHeader<U>,
    signer: &dyn JwsSigner,
  ) -> Result<JwsEncoded>
  where
    T: AsRef<[u8]> + ?Sized,
    U: Serialize,
  {
    // 1. Create the content to be used as the JWS Payload.

    let claims: &[u8] = claims.as_ref();

    // 3. Create the JSON object(s) containing the desired set of Header
    //    Parameters, which together comprise the JOSE Header (the JWS
    //    Protected Header and/or the JWS Unprotected Header).
    let jose: Map<String, Value> = create_jose_header(header, signer)?;

    // Sanity check the JOSE header parameters
    self.check_alg(&jose, signer)?;
    self.check_crit(&jose)?;

    // 2. Compute the encoded payload value BASE64URL(JWS Payload).

    let mut payload_b64: Vec<u8> = Vec::new();

    // Extract the "b64" header parameter and encode the payload as required.
    //
    // See: https://tools.ietf.org/html/rfc7797#section-3
    if extract_b64(&jose)? {
      // Add the payload as a base64-encoded string.
      payload_b64 = encode_b64(claims).into_bytes();
    } else if !self.detach {
      // Add the payload as a UTF-8 string.
      payload_b64.extend_from_slice(self.create_unencoded_payload(claims)?.as_bytes());
    } else {
      // Add the payload as-is.
      payload_b64.extend_from_slice(claims);
    }

    // 3. We already created the JOSE header as a previous step.

    // 4. Compute the encoded header BASE64URL(UTF8(JWS Protected Header)).

    let header_b64: String = encode_b64(&to_vec(&jose)?);

    // 5. Compute the JWS Signature in the manner defined for the
    //    particular algorithm being used over the JWS Signing Input
    //    ASCII(BASE64URL(UTF8(JWS Protected Header)) || '.' ||
    //    BASE64URL(JWS Payload)). The "alg" (algorithm) Header Parameter
    //    MUST be present in the JOSE Header, with the algorithm value
    //    accurately representing the algorithm used to construct the JWS
    //    Signature.
    let message: Vec<u8> = create_jws_message(&header_b64, &payload_b64);
    let signature: Vec<u8> = signer.sign(&message)?;

    // 6. Compute the encoded signature value BASE64URL(JWS Signature).
    let signature_b64: String = encode_b64(&signature);

    // 7. We don't support multiple JWS signatures

    // 8. Create the desired serialized output.

    let payload: Option<String> = if self.detach {
      None
    } else {
      // This should not fail because we validated the non-detached payload
      // above and ensured the content was valid UTF-8.
      Some(String::from_utf8(payload_b64).expect("infallible"))
    };

    match self.format {
      JwsFormat::Compact => {
        let data: String = if let Some(payload) = payload {
          create_jws_compact(header_b64, payload, signature_b64)
        } else {
          create_jws_compact_detached(header_b64, signature_b64)
        };

        Ok(JwsEncoded::Compact(data))
      }
      JwsFormat::General => {
        let data: JwsEncodedGeneral = JwsEncodedGeneral {
          payload,
          signatures: vec![JwsSignature {
            signature: signature_b64,
            protected: header_b64,
          }],
        };

        Ok(JwsEncoded::General(data))
      }
      JwsFormat::Flatten => {
        let data: JwsEncodedFlatten = JwsEncodedFlatten {
          payload,
          signature: JwsSignature {
            signature: signature_b64,
            protected: header_b64,
          },
        };

        Ok(JwsEncoded::Flatten(data))
      }
    }
  }

  fn check_alg(&self, header: &Map<String, Value>, signer: &dyn JwsSigner) -> Result<()> {
    // The "alg" parameter MUST be present in the JOSE header; ensure it
    // matches the algorithm of the signer.
    //
    // See: https://tools.ietf.org/html/rfc7515#section-10.7
    match header.get(PARAM_ALG) {
      Some(Value::String(alg)) if alg == signer.alg().name() => Ok(()),
      Some(_) => Err(EncodeError::InvalidParam(PARAM_ALG).into()),
      None => Err(EncodeError::MissingParam(PARAM_ALG).into()),
    }
  }

  fn check_crit(&self, header: &Map<String, Value>) -> Result<()> {
    // The "crit" parameter MUST NOT be an empty list
    //
    // See: https://tools.ietf.org/html/rfc7515#section-4.1.11
    match header.get(PARAM_CRIT) {
      Some(Value::Array(crit)) if crit.is_empty() => {
        Err(EncodeError::InvalidParam(PARAM_CRIT).into())
      }
      Some(Value::Array(_)) => Ok(()),
      Some(_) => Err(EncodeError::InvalidParam(PARAM_CRIT).into()),
      None => Ok(()),
    }
  }

  fn create_unencoded_payload<'p>(&self, data: &'p [u8]) -> Result<&'p str> {
    let payload: &str = match from_utf8(data) {
      Ok(payload) => payload,
      Err(_) => return Err(EncodeError::InvalidContent("UTF-8").into()),
    };

    // Perform additional validations for unencoded content using the compact
    // serialization format. The payload MUST NOT contain a period (`.`) and
    // it MAY be required to only contain URL-safe characters.
    //
    // See: https://tools.ietf.org/html/rfc7797#section-5.2
    if self.format == JwsFormat::Compact {
      if payload.contains('.') {
        return Err(EncodeError::InvalidContent("Invalid Character: `.`").into());
      }

      if !self.charset.check(payload) {
        // TODO: Improve this error
        return Err(EncodeError::InvalidContent("Invalid Character(s)").into());
      }
    }

    Ok(payload)
  }
}

// =============================================================================
// Decoding
// =============================================================================

#[derive(Clone, Debug, Default)]
pub struct JwsDecoder<'a> {
  /// The serialization format of the encoded data
  format: JwsFormat,
  /// A set of acceptable algorithms
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8725#section-3.1)
  algs: BTreeSet<JwsAlgorithm>,
  /// A set of required extension parameters
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.11)
  crit: BTreeSet<String>,
  /// The detached payload, if using detached content
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  payload: Option<&'a [u8]>,
}

impl<'a> JwsDecoder<'a> {
  /// Creates a new `JwsDecoder` with the default format (Compact).
  pub fn new() -> Self {
    Self::with_format(JwsFormat::Compact)
  }

  /// Creates a new `JwsDecoder` with the given `format`.
  pub fn with_format(format: JwsFormat) -> Self {
    Self {
      format,
      algs: BTreeSet::new(),
      crit: BTreeSet::new(),
      payload: None,
    }
  }

  pub fn with_algs(algs: impl IntoIterator<Item = impl Into<JwsAlgorithm>>) -> Self {
    Self {
      format: JwsFormat::Compact,
      algs: BTreeSet::from_iter(algs.into_iter().map(Into::into)),
      crit: BTreeSet::new(),
      payload: None,
    }
  }

  pub fn with_crit(crit: impl IntoIterator<Item = impl Into<String>>) -> Self {
    Self {
      format: JwsFormat::Compact,
      algs: BTreeSet::new(),
      crit: BTreeSet::from_iter(crit.into_iter().map(Into::into)),
      payload: None,
    }
  }

  pub fn format(mut self, value: impl Into<JwsFormat>) -> Self {
    self.format = value.into();
    self
  }

  pub fn alg(mut self, value: impl Into<JwsAlgorithm>) -> Self {
    self.algs.insert(value.into());
    self
  }

  pub fn crit(mut self, value: impl Into<String>) -> Self {
    self.crit.insert(value.into());
    self
  }

  pub fn payload<T>(mut self, value: &'a T) -> Self
  where
    T: AsRef<[u8]> + ?Sized,
  {
    self.payload = Some(value.as_ref());
    self
  }

  pub fn decode<T, U, V>(&self, data: &T, verifier: &dyn JwsVerifier) -> Result<JwsToken<U, V>>
  where
    T: AsRef<[u8]> + ?Sized,
    U: DeserializeOwned,
    V: DeserializeOwned,
  {
    // TODO: Better separate decoder errors from the TryFrom error as this can
    // lead to confusion regarding signature validation.
    self
      .decode_token(data, verifier)
      .and_then(TryFrom::try_from)
  }

  pub fn decode_token<T, U>(&self, data: &T, verifier: &dyn JwsVerifier) -> Result<JwsRawToken<U>>
  where
    T: AsRef<[u8]> + ?Sized,
    U: DeserializeOwned,
  {
    let detached: bool = self.payload.is_some();

    let signature_b64: Cow<[u8]>;
    let header_b64: Cow<[u8]>;
    let mut payload_b64: Cow<[u8]>;

    // 1. Parse the JWS representation to extract the serialized values for the
    //    components of the JWS. When using the JWS Compact Serialization, these
    //    components are the base64url-encoded representations of the JWS
    //    Protected Header, the JWS Payload, and the JWS Signature, and when
    //    using the JWS JSON Serialization, these components also include the
    //    unencoded JWS Unprotected Header value. When using the JWS Compact
    //    Serialization, the JWS Protected Header, the JWS Payload, and the JWS
    //    Signature are represented as base64url-encoded values in that order,
    //    with each value being separated from the next by a single period ('.')
    //    character, resulting in exactly two delimiting period characters being
    //    used. The JWS JSON Serialization is described in Section 7.2.
    match self.format {
      JwsFormat::Compact => {
        // Split the token into individual segments
        let split: Vec<&[u8]> = data.as_ref().split(|byte| *byte == b'.').collect();

        // Ensure the token has the expect number of segments
        if split.len() != 3 {
          return Err(DecodeError::InvalidInput.into());
        }

        // Ensure the mid segment is empty when using detached content
        if detached && !split[1].is_empty() {
          return Err(DecodeError::InvalidInput.into());
        }

        header_b64 = Cow::Borrowed(split[0]);
        payload_b64 = Cow::Borrowed(split[1]);
        signature_b64 = Cow::Borrowed(split[2]);
      }
      JwsFormat::General => {
        let mut data: JwsEncodedGeneral = from_slice(data.as_ref())?;
        let signature: JwsSignature = data.signatures.pop().ok_or(DecodeError::InvalidInput)?;

        header_b64 = Cow::Owned(signature.protected.into_bytes());
        payload_b64 = Cow::Owned(data.payload.ok_or(DecodeError::InvalidInput)?.into_bytes());
        signature_b64 = Cow::Owned(signature.signature.into_bytes());
      }
      JwsFormat::Flatten => {
        let data: JwsEncodedFlatten = from_slice(data.as_ref())?;

        header_b64 = Cow::Owned(data.signature.protected.into_bytes());
        payload_b64 = Cow::Owned(data.payload.ok_or(DecodeError::InvalidInput)?.into_bytes());
        signature_b64 = Cow::Owned(data.signature.signature.into_bytes());
      }
    }

    if detached {
      payload_b64 = Cow::Borrowed(self.payload.expect("infallible"));
    }

    // 2. Base64url-decode the encoded representation of the JWS Protected
    //    Header, following the restriction that no line breaks, whitespace,
    //    or other additional characters have been used.
    let header: Vec<u8> = decode_b64(&header_b64)?;

    // 3. Verify that the resulting octet sequence is a UTF-8-encoded
    //    representation of a completely valid JSON object conforming to
    //    RFC7159; let the JWS Protected Header be this JSON object.
    // 4. If using the JWS Compact Serialization, let the JOSE Header be the JWS
    //    Protected Header. Otherwise, when using the JWS JSON Serialization,
    //    let the JOSE Header be the union of the members of the corresponding
    //    JWS Protected Header and JWS Unprotected Header, all of which must be
    //    completely valid JSON objects. During this step, verify that the
    //    resulting JOSE Header does not contain duplicate Header Parameter
    //    names. When using the JWS JSON Serialization, this restriction
    //    includes that the same Header Parameter name also MUST NOT occur in
    //    distinct JSON object values that together comprise the JOSE Header.
    let jose: Map<String, Value> = from_slice(&header)?;

    // 5. Verify that the implementation understands and can process all
    //    fields that it is required to support, whether required by this
    //    specification, by the algorithm being used, or by the "crit" Header
    //    Parameter value, and that the values of those parameters are also
    //    understood and supported.
    self.check_header_claims(&jose, verifier)?;

    // 6. Base64url-decode the encoded representation of the JWS Payload,
    //    following the restriction that no line breaks, whitespace, or other
    //    additional characters have been used.
    let payload: Vec<u8> = if extract_b64(&jose)? {
      decode_b64(&payload_b64)?
    } else {
      payload_b64.to_vec()
    };

    // 7. Base64url-decode the encoded representation of the JWS Signature,
    //    following the restriction that no line breaks, whitespace, or other
    //    additional characters have been used.
    let signature: Vec<u8> = decode_b64(signature_b64)?;

    // 8. Validate the JWS Signature against the JWS Signing Input
    //    ASCII(BASE64URL(UTF8(JWS Protected Header)) || '.' || BASE64URL(JWS
    //    Payload)) in the manner defined for the algorithm being used, which
    //    MUST be accurately represented by the value of the "alg" (algorithm)
    //    Header Parameter, which MUST be present. See Section 10.6 for
    //    security considerations on algorithm validation. Record whether the
    //    validation succeeded or not.
    verifier.verify(&create_jws_message(header_b64, payload_b64), &signature)?;

    // 9. No need to repeat as we only support a single signature

    // 10. If none of the validations in step 9 succeeded, then the JWS MUST be
    //    considered invalid. Otherwise, in the JWS JSON Serialization case,
    //    return a result to the application indicating which of the validations
    //    succeeded and failed. In the JWS Compact Serialization case, the
    //    result can simply indicate whether or not the JWS was successfully
    //    validated.
    let header: JwsHeader<U> = from_value(Value::Object(jose))?;

    if detached {
      Ok(JwsRawToken {
        header,
        claims: Vec::new(),
      })
    } else {
      Ok(JwsRawToken {
        header,
        claims: payload,
      })
    }
  }

  fn check_header_claims(
    &self,
    header: &Map<String, Value>,
    verifier: &dyn JwsVerifier,
  ) -> Result<()> {
    self.check_header_alg(header, verifier)?;
    self.check_header_kid(header, verifier)?;
    self.check_header_crit(header)?;

    Ok(())
  }

  // Ensure the header algorithm matches the verifier algorithm.
  fn check_header_alg(
    &self,
    header: &Map<String, Value>,
    verifier: &dyn JwsVerifier,
  ) -> Result<()> {
    match (header.get(PARAM_ALG), verifier.alg().name()) {
      (Some(Value::String(received)), expected)
        if received == expected && self.permit_algorithm(received) =>
      {
        Ok(())
      }
      (None, _) => Err(DecodeError::MissingParam(PARAM_ALG).into()),
      (Some(_), _) => Err(DecodeError::InvalidParam(PARAM_ALG).into()),
    }
  }

  // Ensure the header key ID matches the verifier key ID if one is set.
  fn check_header_kid(
    &self,
    header: &Map<String, Value>,
    verifier: &dyn JwsVerifier,
  ) -> Result<()> {
    match (header.get(PARAM_KID), verifier.kid()) {
      (Some(Value::String(received)), Some(expected)) if received == expected => Ok(()),
      (Some(_), Some(_)) => Err(DecodeError::InvalidParam(PARAM_KID).into()),
      (None, Some(_)) => Err(DecodeError::MissingParam(PARAM_KID).into()),
      (_, None) => Ok(()),
    }
  }

  // If a "crit" parameter was provided, ensure all specified parameters are in
  // the JOSE header.
  fn check_header_crit(&self, header: &Map<String, Value>) -> Result<()> {
    match header.get(PARAM_CRIT) {
      Some(Value::Array(ref crit)) => match crit.as_slice() {
        [head @ Value::String(_), tail @ ..] => {
          for key in once(head).chain(tail) {
            let key: &str = match key {
              Value::String(value) => value.as_str(),
              _ => unreachable!(),
            };

            if !header.contains_key(key) {
              return Err(DecodeError::InvalidParam(PARAM_CRIT).into());
            }
          }

          Ok(())
        }
        [..] => Err(DecodeError::InvalidParam(PARAM_CRIT).into()),
      },
      Some(_) => Err(DecodeError::InvalidParam(PARAM_CRIT).into()),
      None => Ok(()),
    }
  }

  fn permit_algorithm(&self, alg: &str) -> bool {
    for allowed in self.algs.iter() {
      if allowed.name() == alg {
        return true;
      }
    }

    false
  }
}

// =============================================================================
// Misc Helpers
// =============================================================================

fn create_jose_header<T>(
  header: &JwsHeader<T>,
  signer: &dyn JwsSigner,
) -> Result<Map<String, Value>>
where
  T: Serialize,
{
  let mut jose: Map<String, Value> = to_object(header)?;

  // The JOSE header MUST contain an algorithm and SHOULD be included in the
  // integrity-protected header. We don't support an unprotected header BUT we
  // do want the "alg" parameter to line up with the signer implementation.
  //
  // See: https://tools.ietf.org/html/rfc7515#section-10.7
  jose.insert(PARAM_ALG.into(), signer.alg().name().into());

  // Use the "kid" parameter from the signer.
  if let Some(kid) = signer.kid() {
    jose.insert(PARAM_KID.into(), kid.into());
  }

  Ok(jose)
}

// Extract the "b64" header parameter. See [RFC7797](https://tools.ietf.org/html/rfc7797#section-3) for more info.
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
fn extract_b64(header: &Map<String, Value>) -> Result<bool> {
  match (header.get(PARAM_B64), header.get(PARAM_CRIT)) {
    (Some(Value::Bool(b64)), Some(Value::Array(crit))) => {
      // The "crit" param MUST be included and contain "b64".
      //
      // See: https://tools.ietf.org/html/rfc7797#section-6
      if !crit.iter().any(|crit| crit == PARAM_B64) {
        return Err(EncodeError::InvalidParam(PARAM_CRIT).into());
      }

      Ok(*b64)
    }
    (Some(_), Some(_)) => Err(EncodeError::InvalidParam("b64/crit").into()),
    (Some(_), None) => Err(EncodeError::MissingParam(PARAM_CRIT).into()),
    (None, _) => {
      // The default behaviour is to use base64url-encoded payloads
      Ok(true)
    }
  }
}

fn to_object<T>(data: &T) -> Result<Map<String, Value>>
where
  T: Serialize,
{
  match to_value(data)? {
    Value::Object(object) => Ok(object),
    _ => Err(Error::custom("Invalid Object").into()),
  }
}

// =============================================================================
// String/Byte Helpers
// =============================================================================

fn create_jws_message(header: impl AsRef<[u8]>, payload: impl AsRef<[u8]>) -> Vec<u8> {
  let header: &[u8] = header.as_ref();
  let payload: &[u8] = payload.as_ref();
  let capacity: usize = header.len() + 1 + payload.len();

  let mut output: Vec<u8> = Vec::with_capacity(capacity);

  output.extend_from_slice(header);
  output.push(b'.');
  output.extend_from_slice(payload);
  output
}

fn create_jws_compact(
  header: impl AsRef<str>,
  payload: impl AsRef<str>,
  signature: impl AsRef<str>,
) -> String {
  let header: &str = header.as_ref();
  let payload: &str = payload.as_ref();
  let signature: &str = signature.as_ref();
  let capacity: usize = header.len() + 1 + payload.len() + 1 + signature.len();

  let mut output: String = String::with_capacity(capacity);

  output.push_str(header.as_ref());
  output.push('.');
  output.push_str(payload.as_ref());
  output.push('.');
  output.push_str(signature.as_ref());
  output
}

fn create_jws_compact_detached(header: impl AsRef<str>, signature: impl AsRef<str>) -> String {
  let header: &str = header.as_ref();
  let signature: &str = signature.as_ref();
  let capacity: usize = header.len() + 2 + signature.len();

  let mut output: String = String::with_capacity(capacity);

  output.push_str(header.as_ref());
  output.push('.');
  output.push('.');
  output.push_str(signature.as_ref());
  output
}
