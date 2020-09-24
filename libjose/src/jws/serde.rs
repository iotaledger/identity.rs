use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::fmt::Error as FmtError;
use core::str::from_utf8;
use serde::de::Error as _;
use serde_json::Error;
use serde::Serialize;
use serde_json::to_vec;
use serde_json::to_value;
use serde_json::to_string;
use serde_json::Map;
use serde_json::Value;
use core::convert::TryFrom;
use serde::de::DeserializeOwned;
use core::iter::FromIterator;
use serde_json::from_slice;

use crate::alloc::BTreeSet;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsRawToken;
use crate::jws::JwsToken;
use crate::alloc::String;
use crate::alloc::Vec;
use crate::jws::JwsSigner;
use crate::jws::JwsHeader;
use crate::error::Result;
use crate::utils::Empty;
use crate::error::DecodeError;
use crate::error::EncodeError;
use crate::utils::encode_b64_into;
use crate::utils::decode_b64_into;

// =============================================================================
// General
// =============================================================================

fn take_str(string: &mut String) -> String {
  core::mem::replace(string, String::new())
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JwsFormat {
  Compact,
  General,
  Flatten,
}

pub struct JwsSignatureScope<'a, T = Empty, U = Empty> {
  /// The signature algorithm implementation.
  signer: &'a dyn JwsSigner,
  /// The protected JOSE header.
  header_p: Option<JwsHeader<T>>,
  /// The unprotected JOSE header.
  header_u: Option<JwsHeader<U>>,
}

impl<'a, S, T, U> From<(&'a S, JwsHeader<T>)> for JwsSignatureScope<'a, T, U> where S: JwsSigner {
  fn from(other: (&'a S, JwsHeader<T>)) -> Self {
    Self {
      signer: other.0,
      header_p: Some(other.1),
      header_u: None,
    }
  }
}

impl<'a, S, T, U> From<(&'a S, JwsHeader<T>, JwsHeader<U>)> for JwsSignatureScope<'a, T, U> where S: JwsSigner {
  fn from(other: (&'a S, JwsHeader<T>, JwsHeader<U>)) -> Self {
    Self {
      signer: other.0,
      header_p: Some(other.1),
      header_u: Some(other.2),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwsSignature {
  signature: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  protected: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  header: Option<Map<String, Value>>,
}

/// An encoded JSON Web Signature in general JSON format.
#[derive(Debug, Deserialize, Serialize)]
pub struct JwsEncodedGeneral {
  #[serde(skip_serializing_if = "Option::is_none")]
  payload: Option<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  signatures: Vec<JwsSignature>,
}

/// An encoded JSON Web Signature in flattened JSON format.
#[derive(Debug, Deserialize, Serialize)]
pub struct JwsEncodedFlatten {
  #[serde(skip_serializing_if = "Option::is_none")]
  payload: Option<String>,
  #[serde(flatten)]
  signature: JwsSignature,
}

#[derive(Debug)]
pub enum JwsEncoded {
  /// The JWS Compact Serialization represents digitally signed or MACed content
  /// as a compact, URL-safe string.
  Compact(String),
  /// The JWS JSON Serialization represents digitally signed or MACed content as
  /// a JSON object. This representation is neither optimized for compactness
  /// nor URL-safe.
  General(JwsEncodedGeneral),
  /// The flattened JWS JSON Serialization syntax is based upon the general
  /// syntax but flattens it, optimizing it for the single digital signature/MAC
  /// case.
  Flatten(JwsEncodedFlatten),
}

impl JwsEncoded {
  pub fn to_string(&self) -> Result<String> {
    match self {
      Self::Compact(inner) => Ok(inner.clone()),
      Self::General(ref inner) => to_string(inner).map_err(Into::into),
      Self::Flatten(ref inner) => to_string(inner).map_err(Into::into),
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

pub struct JwsEncoder<'a, T = Empty, U = Empty> {
  /// The serialization format of the encoded token.
  format: JwsFormat,
  /// The signature scope used to create cryptographic signatures.
  scopes: Vec<JwsSignatureScope<'a, T, U>>,
  /// Encode the token with detached content.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  detached: bool,
}

impl<'a, T, U> JwsEncoder<'a, T, U> {
  /// Creates a new `JwsEncoder` with the default format (Compact).
  pub const fn new() -> Self {
    Self::with_format(JwsFormat::Compact)
  }

  /// Creates a new `JwsEncoder` with the given `format`.
  pub const fn with_format(format: JwsFormat) -> Self {
    Self {
      format,
      scopes: Vec::new(),
      detached: false,
    }
  }

  pub fn format(mut self, value: impl Into<JwsFormat>) -> Self {
    self.format = value.into();
    self
  }

  pub fn attach(&mut self) {
    self.detached = false;
  }

  pub fn detach(&mut self) {
    self.detached = true;
  }

  pub fn scope(mut self, value: impl Into<JwsSignatureScope<'a, T, U>>) -> Self {
    self.scopes.push(value.into());
    self
  }

  pub fn scopes(mut self, value: impl IntoIterator<Item = impl Into<JwsSignatureScope<'a, T, U>>>) -> Self {
    self.scopes.extend(value.into_iter().map(Into::into));
    self
  }

  pub fn encode<D>(&self, data: &D) -> Result<JwsEncoded>
  where
    T: Serialize + Clone,
    U: Serialize,
    D: Serialize,
  {
    to_vec(data).map_err(Into::into).and_then(|data| self.encode_slice(&data))
  }

  pub fn encode_slice(&self, data: impl AsRef<[u8]>) -> Result<JwsEncoded>
  where
    T: Serialize + Clone,
    U: Serialize
  {
    self.check_fmt()?;

    // A helper for local state and signature composition
    let mut components: B64Components = Components::new();

    // The signatures that will be created
    let mut signatures: Vec<JwsSignature> = Vec::with_capacity(self.scopes.len());

    // Indicates if we are using base64-encoded content
    let mut b64: Option<bool> = None;

    // 1. Create the content to be used as the JWS Payload.
    let payload: &[u8] = data.as_ref();

    for scope in self.scopes.iter() {
      let header_p: Option<&JwsHeader<T>> = scope.header_p.as_ref();
      let header_u: Option<&JwsHeader<U>> = scope.header_u.as_ref();

      // If this scope has an unprotected header, make sure it doesn't have any
      // restricted parameters.
      if let Some(header) = header_u {
        self.check_restricted_params(header)?;
      }

      // Create an "enhanced" version of the protected header where the "alg" and
      // "kid" properties are set from values given by the signer. This helps
      // avoid security issues that may arise from mismatched algorithms in the
      // JOSE header.
      //
      // Note: This is only done if an integrity-protected header was provided.
      //
      let header_p: Option<JwsHeader<T>> = header_p
        .cloned()
        .map(|header| self.enhance_protected_header(header, scope.signer));

      // 3. Create the JSON object(s) containing the desired set of Header
      //    Parameters, which together comprise the JOSE Header (the JWS
      //    Protected Header and/or the JWS Unprotected Header).
      let jose: Map<String, Value> = create_jose_header(header_p.as_ref(), header_u)?;

      // Validate the combined JOSE header parameters
      self.check_alg(&jose, scope.signer)?;
      self.check_crit(&jose)?;

      //
      // 2. Compute the encoded payload value BASE64URL(JWS Payload).
      //

      // Extract the "b64" header parameter and encode the payload as required.
      // Also take the time to make sure all headers have the same value for the
      // "b64" header parameter - this is a requirement AND as a result we only
      // need to encode the payload once.
      //
      // See: https://tools.ietf.org/html/rfc7797#section-3
      match (b64, extract_b64(&jose)?) {
        (Some(true), true) => {}
        (Some(false), false) => {}
        (Some(_), _) => {
          return Err(EncodeError::InvalidJoseHeader("Invalid `b64` parameter").into());
        }
        (None, true) => {
          // Add the payload as a base64-encoded string.
          encode_b64_into(payload, &mut components.payload);
          b64 = Some(true);
        }
        (None, false) => {
          // Add the payload as a UTF-8 string.
          components.payload.push_str(self.create_unencoded_payload(payload)?);
          b64 = Some(false);
        }
      }

      //
      // 3. We already created the JOSE header as a previous step.
      //

      // 4. Compute the encoded header BASE64URL(UTF8(JWS Protected Header)).
      if let Some(header) = header_p.as_ref() {
        encode_b64_into(&to_vec(header)?, &mut components.header);
      }

      // 5. Compute the JWS Signature in the manner defined for the
      //    particular algorithm being used over the JWS Signing Input
      //    ASCII(BASE64URL(UTF8(JWS Protected Header)) || '.' ||
      //    BASE64URL(JWS Payload)). The "alg" (algorithm) Header Parameter
      //    MUST be present in the JOSE Header, with the algorithm value
      //    accurately representing the algorithm used to construct the JWS
      //    Signature.
      let signature: Vec<u8> = scope.signer.sign(&components.create_message())?;

      // 6. Compute the encoded signature value BASE64URL(JWS Signature).
      encode_b64_into(&signature, &mut components.signature);

      // Extract the protected header
      let header_p: Option<String> = Some(&mut components.header)
        .filter(|header| !header.is_empty())
        .map(take_str);

      // Serialize the unprotected header
      let header_u: Option<Map<String, Value>> = header_u
        .map(to_object)
        .transpose()?;

      // Add the signature components for this scope
      signatures.push(JwsSignature {
        signature: take_str(&mut components.signature),
        protected: header_p,
        header: header_u,
      });

      // 7. If the JWS JSON Serialization is being used, repeat this process
      //    (steps 3-6) for each digital signature or MAC operation being
      //    performed.
    }

    // Sanity check
    assert_eq!(self.scopes.len(), signatures.len());

    //
    // 8. Create the desired serialized output.
    //

    let payload: Option<String> = if self.detached {
      None
    } else {
      Some(components.payload)
    };

    match (self.format, signatures.as_slice()) {
      (JwsFormat::Compact, [signature]) => match signature {
        JwsSignature { signature, protected: Some(header), header: None } => {
          let data: String = if let Some(payload) = payload {
            create_jws_compact(header, payload, signature)
          } else {
            create_jws_compact_detached(header, signature)
          };

          Ok(JwsEncoded::Compact(data))
        }
        _ => {
          unreachable!("Invalid Signature")
        }
      }
      (JwsFormat::General, _) => {
        Ok(JwsEncoded::General(JwsEncodedGeneral {
          payload,
          signatures,
        }))
      }
      (JwsFormat::Flatten, [_]) => {
        Ok(JwsEncoded::Flatten(JwsEncodedFlatten {
          payload,
          signature: signatures.pop().expect("infallible")
        }))
      }
      (JwsFormat::Flatten, [..]) | (JwsFormat::Compact, [..]) => {
        unreachable!("Too Many Signatures")
      }
    }
  }

  // Ensure the format/signer configuration is properly aligned
  fn check_fmt(&self) -> Result<()> {
    match (self.format, self.scopes.len()) {
      (_, 0) => {
        Err(EncodeError::InvalidConfiguration("Not Enough Signer Scopes").into())
      }
      (JwsFormat::General, _) | (JwsFormat::Compact, 1) | (JwsFormat::Flatten, 1) => {
        Ok(())
      }
      (JwsFormat::Compact, _) | (JwsFormat::Flatten, _) => {
        Err(EncodeError::InvalidConfiguration("Too Many Signer Scopes").into())
      }
    }
  }

  fn check_restricted_params<C>(&self, header: &JwsHeader<C>) -> Result<()> {
    // The "b64" header parameter MUST be protected if provided
    //
    // See: https://tools.ietf.org/html/rfc7797#section-3
    if header.b64().is_some() {
      return Err(EncodeError::InvalidJoseHeader("Unprotected `b64`").into());
    }

    // The "crit" header parameter MUST be protected if provided
    //
    // See: https://tools.ietf.org/html/rfc7515#section-4.1.11
    if header.crit().is_some() {
      return Err(EncodeError::InvalidJoseHeader("Unprotected `crit`").into());
    }

    Ok(())
  }

  fn enhance_protected_header(&self, mut header: JwsHeader<T>, signer: &dyn JwsSigner) -> JwsHeader<T> {
    // This MUST be present in the signed JOSE header and security is
    // slightly enhanced if this is a protected header property.
    //
    // See: https://tools.ietf.org/html/rfc7515#section-10.7
    header.set_alg(signer.alg());

    // Use the "kid" claim from the signer, if present.
    if let Some(kid) = signer.kid() {
      header.set_kid(kid);
    }

    header
  }

  fn check_alg(&self, header: &Map<String, Value>, signer: &dyn JwsSigner) -> Result<()> {
    // The "alg" parameter MUST be present in the JOSE header; ensure it
    // matches the algorithm of the signer.
    //
    // See: https://tools.ietf.org/html/rfc7515#section-10.7
    match header.get("alg") {
      Some(Value::String(alg)) if alg == signer.alg().name() => {
        Ok(())
      }
      Some(_) => {
        Err(EncodeError::InvalidJoseHeader("Invalid `alg` parameter (type)").into())
      }
      None => {
        Err(EncodeError::InvalidJoseHeader("Missing `alg` parameter").into())
      }
    }
  }

  fn check_crit(&self, header: &Map<String, Value>) -> Result<()> {
    // The "crit" parameter MUST NOT be an empty list
    //
    // See: https://tools.ietf.org/html/rfc7515#section-4.1.11
    match header.get("crit") {
      Some(Value::Array(crit)) if crit.is_empty() => {
        Err(EncodeError::InvalidJoseHeader("Invalid `crit` parameter (empty)").into())
      }
      Some(Value::Array(_)) => {
        Ok(())
      }
      Some(_) => {
        Err(EncodeError::InvalidJoseHeader("Invalid `crit` parameter (type)").into())
      }
      None => {
        Ok(())
      }
    }
  }

  fn create_unencoded_payload<'p>(&self, data: &'p [u8]) -> Result<&'p str> {
    let payload: &str = match from_utf8(data) {
      Ok(payload) => payload,
      Err(error) => return Err(EncodeError::InvalidContent(error).into()),
    };

    // Validate the payload
    //
    // See: https://tools.ietf.org/html/rfc7797#section-5.2
    //
    // TODO: Provide a more flexible API for this validaton
    if payload.contains('.') {
      return Err(EncodeError::InvalidContentChar('.').into());
    }

    Ok(payload)
  }
}

// =============================================================================
// Misc Helpers
// =============================================================================

fn create_jose_header<T, U>(
  header_p: Option<&T>,
  header_u: Option<&U>,
) -> Result<Map<String, Value>> where T: Serialize, U: Serialize {
  // Convert the protected header to a JSON object
  let mut header_p_map: Map<String, Value> = match header_p {
    Some(header) => to_object(header)?,
    None => Map::new(),
  };

  // Convert the unprotected header to a JSON object
  let mut header_u_map: Map<String, Value> = match header_u {
    Some(header) => to_object(header)?,
    None => Map::new(),
  };

  // Both headers CANNOT be empty - Something MUST be present
  if header_p_map.is_empty() && header_u_map.is_empty() {
    return Err(EncodeError::InvalidJoseHeader("Cannot be empty").into());
  }

  // Merge the unprotected values into the protected header
  //
  // The headers CANNOT contain duplicate properties values
  for (key, value) in header_u_map {
    if header_p_map.insert(key, value).is_some() {
      return Err(EncodeError::InvalidJoseHeader("Duplicate Property").into());
    }
  }

  Ok(header_p_map)
}

fn to_object<T>(data: &T) -> Result<Map<String, Value>> where T: Serialize {
  match to_value(data)? {
    Value::Object(object) => {
      Ok(object)
    }
    _ => {
      Err(Error::custom("Invalid Object").into())
    }
  }
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
  match (header.get("b64"), header.get("crit")) {
    (Some(Value::Bool(b64)), Some(Value::Array(crit))) => {
      // // The "crit" param MUST be included and contain "b64".
      // // More Info: https://tools.ietf.org/html/rfc7797#section-6
      // if !crit.iter().any(|crit| crit == "b64") {
      //   return Err(EncodeError::MissingCritB64.into());
      // }

      Ok(*b64)
    }
    (Some(_), Some(_)) => {
      Err(EncodeError::InvalidJoseHeader("Bad Type").into())
    }
    (Some(_), None) => {
      Err(EncodeError::InvalidJoseHeader("Missing `crit` parameter").into())
    }
    (None, _) => {
      // The default behaviour is to use base64url-encoded payloads
      Ok(true)
    }
  }
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
}

// =============================================================================
// String/Byte Helpers
// =============================================================================

fn create_jws_message(
  header: impl AsRef<[u8]>,
  payload: impl AsRef<[u8]>,
) -> Vec<u8> {
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

fn create_jws_compact_detached(
  header: impl AsRef<str>,
  signature: impl AsRef<str>,
) -> String {
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
