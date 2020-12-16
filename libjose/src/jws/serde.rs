use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::iter;
use core::str;
use serde::de::DeserializeOwned;
use serde::de::Error as _;
use serde::Serialize;
use serde_json::from_slice;
use serde_json::from_value;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::to_vec;
use serde_json::Map;
use serde_json::Value;

use crate::error::Error;
use crate::error::Result;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::JwsSigner;
use crate::jws::JwsVerifier;
use crate::jwt::JwtClaims;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::encode_b64;

const PARAM_ALG: &str = "alg";
const PARAM_B64: &str = "b64";
const PARAM_CRIT: &str = "crit";
const PARAM_KID: &str = "kid";

type Object = Map<String, Value>;

// =============================================================================
// JWS Encoding Format
// =============================================================================

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

// =============================================================================
// JWS Encoded Data
// =============================================================================

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
// Character Set
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

// =============================================================================
// JWS Encoder
// =============================================================================

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

  pub fn encode<T>(
    &self,
    claims: &JwtClaims<T>,
    header: &JwsHeader,
    signer: &dyn JwsSigner,
  ) -> Result<JwsEncoded>
  where
    T: Serialize,
  {
    self.encode_slice(&to_vec(claims)?, header, signer)
  }

  pub fn encode_slice<T>(
    &self,
    claims: &T,
    header: &JwsHeader,
    signer: &dyn JwsSigner,
  ) -> Result<JwsEncoded>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    // 1. Create the content to be used as the JWS Payload.

    let claims: &[u8] = claims.as_ref();

    // 3. Create the JSON object(s) containing the desired set of Header
    //    Parameters, which together comprise the JOSE Header (the JWS
    //    Protected Header and/or the JWS Unprotected Header).
    let jose: Object = create_jose_header(header, signer)?;

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
      JwsFormat::General => Ok(JwsEncoded::General(JwsEncodedGeneral {
        payload,
        signatures: vec![JwsSignature {
          signature: signature_b64,
          protected: header_b64,
        }],
      })),
      JwsFormat::Flatten => Ok(JwsEncoded::Flatten(JwsEncodedFlatten {
        payload,
        signature: JwsSignature {
          signature: signature_b64,
          protected: header_b64,
        },
      })),
    }
  }

  fn check_alg(&self, header: &Object, signer: &dyn JwsSigner) -> Result<()> {
    // The "alg" parameter MUST be present in the JOSE header; ensure it
    // matches the algorithm of the signer.
    //
    // See: https://tools.ietf.org/html/rfc7515#section-10.7
    match header.get(PARAM_ALG) {
      Some(Value::String(alg)) if alg == signer.alg().name() => Ok(()),
      Some(_) => Err(Error::InvalidParam(PARAM_ALG)),
      None => Err(Error::MissingParam(PARAM_ALG)),
    }
  }

  fn check_crit(&self, header: &Object) -> Result<()> {
    // The "crit" parameter MUST NOT be an empty list
    //
    // See: https://tools.ietf.org/html/rfc7515#section-4.1.11
    match header.get(PARAM_CRIT) {
      Some(Value::Array(crit)) if crit.is_empty() => Err(Error::InvalidParam(PARAM_CRIT)),
      Some(Value::Array(_)) => Ok(()),
      Some(_) => Err(Error::InvalidParam(PARAM_CRIT)),
      None => Ok(()),
    }
  }

  fn create_unencoded_payload<'p>(&self, data: &'p [u8]) -> Result<&'p str> {
    let payload: &str = match str::from_utf8(data) {
      Ok(payload) => payload,
      Err(_) => return Err(Error::InvalidContent("UTF-8")),
    };

    // Perform additional validations for unencoded content using the compact
    // serialization format. The payload MUST NOT contain a period (`.`) and
    // it MAY be required to only contain URL-safe characters.
    //
    // See: https://tools.ietf.org/html/rfc7797#section-5.2
    if self.format == JwsFormat::Compact {
      if payload.contains('.') {
        return Err(Error::InvalidContent("Invalid Character: `.`"));
      }

      if !self.charset.check(payload) {
        // TODO: Improve this error
        return Err(Error::InvalidContent("Invalid Character(s)"));
      }
    }

    Ok(payload)
  }
}

// =============================================================================
// JWS Decoder
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

  pub fn decode<T, U>(&self, data: &T, verifier: &dyn JwsVerifier) -> Result<(JwsHeader, Option<U>)>
  where
    T: AsRef<[u8]> + ?Sized,
    U: DeserializeOwned,
  {
    let (header, claims): _ = self.decode_token(data, verifier)?;
    let claims: Option<U> = claims.as_deref().map(from_slice).transpose()?;

    Ok((header, claims))
  }

  pub fn decode_token<T>(
    &self,
    data: &T,
    verifier: &dyn JwsVerifier,
  ) -> Result<(JwsHeader, Option<Vec<u8>>)>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    let segments: Segments<'_> = match self.format {
      JwsFormat::Compact => extract_compact(data.as_ref(), self.payload)?,
      JwsFormat::General => extract_general(data.as_ref(), self.payload)?,
      JwsFormat::Flatten => extract_flatten(data.as_ref(), self.payload)?,
    };

    let header: Object = from_slice(&decode_b64(&segments.protected)?)?;

    self.check_header_alg(&header, verifier)?;
    self.check_header_kid(&header, verifier)?;
    self.check_header_crit(&header)?;

    let payload: Cow<[u8]> = if extract_b64(&header)? {
      Cow::Owned(decode_b64(&segments.payload)?)
    } else {
      Cow::Borrowed(segments.payload.as_ref())
    };

    let signature: Vec<u8> = decode_b64(&segments.signature)?;
    let message: Vec<u8> = create_jws_message(&segments.protected, &payload);

    verifier.verify(&message, &signature)?;

    if self.payload.is_some() {
      to_header(header).map(|header| (header, None))
    } else {
      to_header(header).map(|header| (header, Some(payload.to_vec())))
    }
  }

  // Ensure the header algorithm matches the verifier algorithm.
  fn check_header_alg(&self, header: &Object, verifier: &dyn JwsVerifier) -> Result<()> {
    match (header.get(PARAM_ALG), verifier.alg().name()) {
      (Some(Value::String(received)), expected)
        if received == expected && self.permit_algorithm(received) =>
      {
        Ok(())
      }
      (None, _) => Err(Error::MissingParam(PARAM_ALG)),
      (Some(_), _) => Err(Error::InvalidParam(PARAM_ALG)),
    }
  }

  // Ensure the header key ID matches the verifier key ID if one is set.
  fn check_header_kid(&self, header: &Object, verifier: &dyn JwsVerifier) -> Result<()> {
    match (header.get(PARAM_KID), verifier.kid()) {
      (Some(Value::String(received)), Some(expected)) if received == expected => Ok(()),
      (Some(_), Some(_)) => Err(Error::InvalidParam(PARAM_KID)),
      (None, Some(_)) => Err(Error::MissingParam(PARAM_KID)),
      (_, None) => Ok(()),
    }
  }

  // If a "crit" parameter was provided, ensure all specified parameters are in
  // the JOSE header.
  fn check_header_crit(&self, header: &Object) -> Result<()> {
    match header.get(PARAM_CRIT) {
      Some(Value::Array(ref crit)) => match crit.as_slice() {
        [head @ Value::String(_), tail @ ..] => {
          for key in iter::once(head).chain(tail) {
            let key: &str = match key {
              Value::String(value) => &*value,
              _ => unreachable!(),
            };

            if !header.contains_key(key) {
              return Err(Error::InvalidParam(PARAM_CRIT));
            }
          }

          Ok(())
        }
        [..] => Err(Error::InvalidParam(PARAM_CRIT)),
      },
      Some(_) => Err(Error::InvalidParam(PARAM_CRIT)),
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

fn create_jose_header(header: &JwsHeader, signer: &dyn JwsSigner) -> Result<Object> {
  let mut jose: Object = to_object(header)?;

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
fn extract_b64(header: &Object) -> Result<bool> {
  match (header.get(PARAM_B64), header.get(PARAM_CRIT)) {
    (Some(Value::Bool(b64)), Some(Value::Array(crit))) => {
      // The "crit" param MUST be included and contain "b64".
      //
      // See: https://tools.ietf.org/html/rfc7797#section-6
      if !crit.iter().any(|crit| crit == PARAM_B64) {
        return Err(Error::InvalidParam(PARAM_CRIT));
      }

      Ok(*b64)
    }
    (Some(_), Some(_)) => Err(Error::InvalidParam("b64/crit")),
    (Some(_), None) => Err(Error::MissingParam(PARAM_CRIT)),
    (None, _) => {
      // The default behaviour is to use base64url-encoded payloads
      Ok(true)
    }
  }
}

fn to_object<T>(data: &T) -> Result<Object>
where
  T: Serialize,
{
  match to_value(data)? {
    Value::Object(object) => Ok(object),
    _ => Err(serde_json::Error::custom("invalid object").into()),
  }
}

fn to_header(object: Object) -> Result<JwsHeader> {
  from_value(Value::Object(object)).map_err(Into::into)
}

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

// =============================================================================
// Decoding Helper
// =============================================================================

#[derive(Debug)]
struct Segments<'a> {
  signature: Cow<'a, [u8]>,
  protected: Cow<'a, [u8]>,
  payload: Cow<'a, [u8]>,
}

impl Segments<'_> {
  const COUNT: usize = 3;
}

fn extract_compact<'a>(data: &'a [u8], payload: Option<&'a [u8]>) -> Result<Segments<'a>> {
  // Split the token into individual segments
  let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

  // Ensure the token has the expected number of segments
  if split.len() != Segments::COUNT {
    return Err(Error::InvalidContent("Invalid Segments"));
  }

  if let Some(payload) = payload {
    if !split[1].is_empty() {
      return Err(Error::InvalidContent("Unexpected Payload"));
    }

    Ok(Segments {
      protected: Cow::Borrowed(split[0]),
      signature: Cow::Borrowed(split[2]),
      payload: Cow::Borrowed(payload),
    })
  } else {
    Ok(Segments {
      protected: Cow::Borrowed(split[0]),
      signature: Cow::Borrowed(split[2]),
      payload: Cow::Borrowed(split[1]),
    })
  }
}

fn extract_general<'a>(data: &'a [u8], payload: Option<&'a [u8]>) -> Result<Segments<'a>> {
  let mut data: JwsEncodedGeneral = from_slice(data)?;

  let signature: JwsSignature = data
    .signatures
    .pop()
    .ok_or(Error::InvalidContent("Multiple Signatures"))?;

  if let Some(payload) = payload {
    if data.payload.is_some() {
      return Err(Error::InvalidContent("Unexpected Payload"));
    }

    Ok(Segments {
      protected: Cow::Owned(signature.protected.into_bytes()),
      signature: Cow::Owned(signature.signature.into_bytes()),
      payload: Cow::Borrowed(payload),
    })
  } else if let Some(payload) = data.payload.map(String::into_bytes) {
    Ok(Segments {
      protected: Cow::Owned(signature.protected.into_bytes()),
      signature: Cow::Owned(signature.signature.into_bytes()),
      payload: Cow::Owned(payload),
    })
  } else {
    Err(Error::InvalidContent("Missing Payload"))
  }
}

fn extract_flatten<'a>(data: &'a [u8], payload: Option<&'a [u8]>) -> Result<Segments<'a>> {
  let data: JwsEncodedFlatten = from_slice(data)?;

  if let Some(payload) = payload {
    if data.payload.is_some() {
      return Err(Error::InvalidContent("Unexpected Payload"));
    }

    Ok(Segments {
      protected: Cow::Owned(data.signature.protected.into_bytes()),
      signature: Cow::Owned(data.signature.signature.into_bytes()),
      payload: Cow::Borrowed(payload),
    })
  } else if let Some(payload) = data.payload.map(String::into_bytes) {
    Ok(Segments {
      protected: Cow::Owned(data.signature.protected.into_bytes()),
      signature: Cow::Owned(data.signature.signature.into_bytes()),
      payload: Cow::Owned(payload),
    })
  } else {
    Err(Error::InvalidContent("Missing Payload"))
  }
}
