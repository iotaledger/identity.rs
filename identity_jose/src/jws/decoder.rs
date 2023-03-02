// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str;
use std::borrow::Cow;

use serde::__private::de;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::Jwk;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsFormat;
use crate::jws::JwsHeader;
use crate::jws::JwsHeaderSet;
use crate::jwu::create_message;
use crate::jwu::decode_b64;
use crate::jwu::decode_b64_json;
use crate::jwu::filter_non_empty_bytes;
use crate::jwu::parse_utf8;
use crate::jwu::validate_jws_headers;

use super::decoder_config::DecodingConfig;
#[cfg(feature = "default-jws-signature-verifier")]
use super::DefaultJwsSignatureVerifier;
use super::JwsSignatureVerifier;
use super::SignatureVerificationError;
use super::VerificationInput;

const COMPACT_SEGMENTS: usize = 3;

/// A partially decoded token from a JWS.
///
/// Contains the decoded headers and the raw claims.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
  pub protected: Option<JwsHeader>,
  pub unprotected: Option<JwsHeader>,
  pub claims: Cow<'a, [u8]>,
}

// Todo: Consider boxing as this enum is very large
enum DecodedHeaders {
  Protected(JwsHeader),
  Unprotected(JwsHeader),
  Both {
    protected: JwsHeader,
    unprotected: JwsHeader,
  },
}

impl DecodedHeaders {
  fn new(protected: Option<JwsHeader>, unprotected: Option<JwsHeader>) -> Result<Self> {
    match (protected, unprotected) {
      (Some(protected), Some(unprotected)) => Ok(Self::Both { protected, unprotected }),
      (Some(protected), None) => Ok(Self::Protected(protected)),
      (None, Some(unprotected)) => Ok(Self::Unprotected(unprotected)),
      (None, None) => Err(Error::MissingHeader),
    }
  }
}

pub struct DecodedItem<'a> {
  headers: DecodedHeaders,
  signing_input: Box<[u8]>,
  decoded_signature: Box<[u8]>,
  claims: Cow<'a, [u8]>,
}
impl<'a> DecodedItem<'a> {
  /// Returns the decoded protected header if it exists.
  pub fn protected_header(&self) -> Option<&JwsHeader> {
    match self.headers {
      DecodedHeaders::Protected(ref header) => Some(header),
      DecodedHeaders::Both { ref protected, .. } => Some(protected),
      DecodedHeaders::Unprotected(_) => None,
    }
  }

  /// Returns the decoded unproteced header if it exists.
  pub fn unprotected_header(&self) -> Option<&JwsHeader> {
    match self.headers {
      DecodedHeaders::Unprotected(ref header) => Some(header),
      DecodedHeaders::Both { ref unprotected, .. } => Some(unprotected),
      DecodedHeaders::Protected(_) => None,
    }
  }

  /// The algorithm parsed from the protected header if it exists.
  pub fn alg(&self) -> Option<JwsAlgorithm> {
    self.protected_header().and_then(|protected| protected.alg())
  }

  /// Returns the JWS claims.
  pub fn claims(&self) -> &[u8] {
    &self.claims
  }

  /// Returns the signing input .
  ///
  /// See [RFC 7515: section 5.2 part 8.](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) and
  /// [RFC 7797 section 3](https://www.rfc-editor.org/rfc/rfc7797#section-3).
  pub fn signing_input(&self) -> &[u8] {
    &self.signing_input
  }

  /// Returns the decoded JWS signature.
  pub fn decoded_signature(&self) -> &[u8] {
    &self.decoded_signature
  }

  /// TODO: Document this
  pub fn verify<T>(self, verifier: &T) -> Result<Token<'a>> {
    todo!()
  }
}

// =============================================================================================
// Format dependent deserializable helper structs used by the decoder
// =============================================================================================
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct JwsSignature<'a> {
  header: Option<JwsHeader>,
  protected: Option<&'a str>,
  signature: &'a str,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct General<'a> {
  payload: Option<&'a str>,
  signatures: Vec<JwsSignature<'a>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Flatten<'a> {
  payload: Option<&'a str>,
  #[serde(flatten)]
  signature: JwsSignature<'a>,
}

// =============================================================================
// Decoder
// =============================================================================

/// The [`Decoder`] allows decoding a raw JWS into a [`Token`], verifying
/// the structure of the JWS and its signature.
#[derive(Default)]
pub struct Decoder {
  config: DecodingConfig,
}

pub struct DecodedSignaturesIter<'decoder, 'payload, 'signatures> 
{
  decoder: &'decoder Decoder, 
  signatures: std::vec::IntoIter<JwsSignature<'signatures>>, 
  payload: &'payload [u8]
}
impl<'decoder, 'payload, 'signatures> Iterator for DecodedSignaturesIter<'decoder, 'payload, 'signatures> 
{
  type Item = Result<DecodedItem<'payload>>; 

  fn next(&mut self) -> Option<Self::Item> {
      self.signatures.next().map(|signature| self.decoder.decode_signature(self.payload, signature))
  }
}
impl Decoder {
  /// Constructs a new [`Decoder`].
  pub fn new() -> Self {
    Self {
      config: DecodingConfig::default(),
    }
  }

  /// Append values to the list of permitted extension parameters.
  pub fn critical(mut self, value: impl Into<String>) -> Self {
    self.config.crits.get_or_insert_with(Vec::new).push(value.into());
    self
  }

  /// Defines whether a given [`Jwk`](crate::jwk::Jwk) used to verify a JWS,
  /// must have an `alg` parameter corresponding to the one extracted from the JWS header.
  /// This value is `true` by default.  
  pub fn jwk_must_have_alg(mut self, value: bool) -> Self {
    self.config.jwk_must_have_alg = value;
    self
  }

  /// When verifying a JWS encoded with the general JWS JSON serialization
  /// this value decides whether all signatures must be verified (the default behavior),
  /// otherwise only one signature needs to be verified in order for the entire JWS to be accepted.
  pub fn strict_signature_verification(mut self, value: bool) -> Self {
    self.config.strict_signature_verification = value;
    self
  }

  /// Specify the serialization format the [`Decoder`](crate::jws::Decoder) accepts. The default is
  /// [`JwsFormat::Compact`].
  pub fn format(mut self, value: JwsFormat) -> Self {
    self.config.format = value;
    self
  }

  /// Convenience method equivalent to [`Self::decode(data, &DefaultJwsSignatureVerifier::default(),<other
  /// parameters>)`](Self::decode()). This method is only available when the `default-jws-signature-verifier` feature
  /// is enabled.
  #[cfg(any(feature = "default-jws-signature-verifier", doc))]
  pub fn decode_default<'b, 'c, F>(
    &self,
    data: &'b [u8],
    jwk_provider: F,
    detached_payload: Option<&'b [u8]>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(&JwsHeaderSet<'_>, &mut bool) -> Option<&'c Jwk>,
  {
    self.decode(
      data,
      &DefaultJwsSignatureVerifier::default(),
      jwk_provider,
      detached_payload,
    )
  }

  pub fn decode_compact_serialization<'b>(
    &self,
    data: &'b [u8],
    detached_payload: Option<&'b [u8]>,
  ) -> Result<DecodedItem<'b>> {

    let mut segments = data.split(|byte| *byte == b'.');

    let (Some(protected), Some(payload), Some(signature), None) = (
      segments.next(),
      segments.next(),
      segments.next(),
      segments.next()
    ) else {
      return Err(Error::InvalidContent("invalid segments count"));
    }; 


    let signature: JwsSignature<'_> = JwsSignature {
      header: None,
      protected: Some(parse_utf8(protected)?),
      signature: parse_utf8(signature)?,
    };

    let payload = Self::expand_payload(detached_payload, Some(payload))?;

    self.decode_signature(payload, signature)
  }

  pub fn decode_flattened_serialization<'b>(
    &self, 
    data: &'b [u8], 
    detached_payload: Option<&'b [u8]>
  ) -> Result<DecodedItem<'b>> {
      let data: Flatten<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;
      let payload = Self::expand_payload(detached_payload, data.payload)?; 
      let signature = data.signature; 
      self.decode_signature(payload, signature)
  }

  pub fn decode_general_serialization<'decoder, 'data>(
    &'decoder self, 
    data: &'data [u8],
    detached_payload: Option<&'data [u8]>
  ) -> Result<DecodedSignaturesIter<'decoder, 'data, 'data>> {
    let data: General<'data> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

    let payload = Self::expand_payload(detached_payload, data.payload)?; 
    let signatures = data.signatures; 

    Ok(DecodedSignaturesIter{
      decoder: &self, 
      payload,
      signatures: signatures.into_iter()
    })
  }

  fn decode_signature<'a, 'b>(&self, payload: &'b [u8], jws_signature: JwsSignature<'a>) -> Result<DecodedItem<'b>> {
    let JwsSignature {
      header: unprotected_header,
      protected,
      signature,
    } = jws_signature;

    let protected_header: Option<JwsHeader> = protected.map(decode_b64_json).transpose()?;
    validate_jws_headers(
      protected_header.as_ref(),
      unprotected_header.as_ref(),
      self.config.crits.as_deref(),
    )?;

    let protected_bytes: &[u8] = protected.map(str::as_bytes).unwrap_or_default();
    let signing_input: Box<[u8]> = create_message(protected_bytes, payload).into();
    let decoded_signature: Box<[u8]> = decode_b64(signature)?.into();

    let claims: Cow<'b, [u8]> = if protected_header.as_ref().and_then(|value| value.b64()).unwrap_or(true) {
      Cow::Owned(decode_b64(payload)?)
    } else {
      Cow::Borrowed(payload)
    };

    Ok(DecodedItem {
      headers: DecodedHeaders::new(protected_header, unprotected_header)?,
      signing_input,
      decoded_signature,
      claims,
    })
  }

  /// Decode the given `data` which is a base64url-encoded JWS.
  ///
  /// The decoder will verify the JWS signature(s) using the provided [`JwsSignatureVerifier`].
  ///
  /// ### Jwk extraction
  /// The `jwk_provider` argument is a closure responsible for providing a suitable [`Jwk`] or dictating that it should
  /// be extracted from the header (by setting the provided `bool` to `true`). A suitable [`Jwk`] could be one matching
  /// the header's `kid`, or some default JWK whenever that is reasonable.
  ///
  /// ### Working with detached payloads
  /// If using `detached_payload` one should supply a `Some` value for the `detached_payload` parameter.
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  pub fn decode<'b, 'c, T, F>(
    &self,
    data: &'b [u8],
    verifier: &T,
    jwk_provider: F,
    detached_payload: Option<&'b [u8]>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(&JwsHeaderSet<'_>, &mut bool) -> Option<&'c Jwk>,
    T: JwsSignatureVerifier,
  {
    // TODO: Only Vec in the general case, consider using OneOrMany or Either to remove the allocation for the other two
    // cases.
    let (payload, signatures): (&[u8], Vec<JwsSignature>) = match self.config.format {
      JwsFormat::Compact => {
        let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

        if split.len() != COMPACT_SEGMENTS {
          return Err(Error::InvalidContent("invalid segments count"));
        }

        let signature: JwsSignature<'_> = JwsSignature {
          header: None,
          protected: Some(parse_utf8(split[0])?),
          signature: parse_utf8(split[2])?,
        };

        (Self::expand_payload(detached_payload, Some(split[1]))?, vec![signature])
      }
      JwsFormat::General => {
        let data: General<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        (Self::expand_payload(detached_payload, data.payload)?, data.signatures)
      }
      JwsFormat::Flatten => {
        let data: Flatten<'_> = serde_json::from_slice(data).map_err(Error::InvalidJson)?;

        (
          Self::expand_payload(detached_payload, data.payload)?,
          vec![data.signature],
        )
      }
    };

    let mut result: Result<Token> = Err(Error::InvalidContent("recipient not found"));

    for jws_signature in signatures {
      result = self.decode_one(verifier, &jwk_provider, payload, jws_signature);
      if result.is_err() && self.config.strict_signature_verification {
        // With strict signature verification all validations must be successful
        // hence we return on the first error discovered.
        return result;
      }
      if result.is_ok() && !self.config.strict_signature_verification {
        // If signature verification is not strict only one verification must succeed
        // hence we return on the first one.
        return result;
      }
    }
    result
  }

  fn decode_one<'a, 'b, 'c, T, F>(
    &self,
    verifier: &T,
    jwk_provider: &F,
    payload: &'b [u8],
    jws_signature: JwsSignature<'a>,
  ) -> Result<Token<'b>>
  where
    F: 'c,
    F: Fn(&JwsHeaderSet<'_>, &mut bool) -> Option<&'c Jwk>,
    T: JwsSignatureVerifier,
  {
    let JwsSignature {
      header: unprotected_header,
      protected,
      signature,
    } = jws_signature;

    let protected_header: Option<JwsHeader> = protected.map(decode_b64_json).transpose()?;
    validate_jws_headers(
      protected_header.as_ref(),
      unprotected_header.as_ref(),
      self.config.crits.as_deref(),
    )?;

    let merged = JwsHeaderSet::new()
      .with_protected(protected_header.as_ref())
      .with_unprotected(unprotected_header.as_ref());

    let payload_is_b64_encoded = merged.b64().unwrap_or(true);

    // Obtain a JWK before proceeding.
    //let kid = merged.kid();
    let mut fallback_to_header = false;
    let provided_key: Option<&Jwk> = jwk_provider(&merged, &mut fallback_to_header);
    let key = provided_key
      .or_else(|| if fallback_to_header { merged.jwk() } else { None })
      .ok_or(Error::JwkNotProvided)?;

    // Validate the header's alg against the requirements of the JWK.
    let alg: JwsAlgorithm = merged.try_alg()?;
    {
      if let Some(key_alg) = key.alg() {
        if alg.name() != key_alg {
          return Err(crate::error::Error::AlgorithmMismatch);
        }
      } else if self.config.jwk_must_have_alg {
        return Err(crate::error::Error::JwkWithoutAlg);
      }
    }

    // Verify the signature
    {
      let protected_bytes: &[u8] = protected.map(str::as_bytes).unwrap_or_default();
      let message: Vec<u8> = create_message(protected_bytes, payload);
      let signature: Vec<u8> = decode_b64(signature)?;
      let verification_input = VerificationInput {
        jose_header: &merged,
        signing_input: message,
        signature: &signature,
      };

      verifier
        .verify(&verification_input, key)
        .map_err(Error::SignatureVerificationError)?;
    }

    let claims: Cow<'b, [u8]> = if payload_is_b64_encoded {
      Cow::Owned(decode_b64(payload)?)
    } else {
      Cow::Borrowed(payload)
    };

    Ok(Token {
      protected: protected_header,
      unprotected: unprotected_header,
      claims,
    })
  }

  fn expand_payload<'b>(
    detached_payload: Option<&'b [u8]>,
    parsed_payload: Option<&'b (impl AsRef<[u8]> + ?Sized)>,
  ) -> Result<&'b [u8]> {
    //TODO: Do we allow an empty detached payload? (The previous stateful version did).
    match (detached_payload, filter_non_empty_bytes(parsed_payload)) {
      (Some(payload), None) => Ok(payload),
      (None, Some(payload)) => Ok(payload),
      (Some(_), Some(_)) => Err(Error::InvalidContent("multiple payloads")),
      (None, None) => Err(Error::InvalidContent("missing payload")),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::jwt::JwtClaims;

use super::*;

  #[test]
  fn rfc7515_appendix_a_6() {
    let general_jws_json_serialized: &str = r#"
    {
      "payload":
       "eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ",
      "signatures":[
       {"protected":"eyJhbGciOiJSUzI1NiJ9",
        "header": {"kid":"2010-12-29"},
        "signature": "cC4hiUPoj9Eetdgtv3hF80EGrhuB__dzERat0XF9g2VtQgr9PJbu3XOiZj5RZmh7AAuHIm4Bh-0Qc_lF5YKt_O8W2Fp5jujGbds9uJdbF9CUAr7t1dnZcAcQjbKBYNX4BAynRFdiuB--f_nZLgrnbyTyWzO75vRK5h6xBArLIARNPvkSjtQBMHlb1L07Qe7K0GarZRmB_eSN9383LcOLn6_dO--xi12jzDwusC-eOkHWEsqtFZESc6BfI7noOPqvhJ1phCnvWh6IeYI2w9QOYEUipUTI8np6LbgGY9Fs98rqVt5AXLIhWkWywlVmtVrBp0igcN_IoypGlUPQGe77Rw"
        },
       {"protected":"eyJhbGciOiJFUzI1NiJ9",
        "header": {"kid":"e9bc097a-ce51-4036-9562-d2ade882db0d"},
        "signature":"DtEhU3ljbEg8L38VWAfUAqOyKAM6-Xx-F4GawxaepmXFCgfTjDxw5djxLa8ISlSApmWQxfKTUJqPP3-Kg6NU1Q"
      }]
    }"#; 
    let claims_str : &str = r#"
    {
      "iss":"joe",
    "exp":1300819380,
    "http://example.com/is_root":true
    }
    "#; 
    let claims: JwtClaims::<serde_json::Value> = serde_json::from_str(claims_str).unwrap(); 

    let decoder = Decoder::new();

    let mut signature_iter = decoder.decode_general_serialization(&general_jws_json_serialized.as_bytes(), None).unwrap().filter_map(|decoded| decoded.ok());
    let first_signature_decoding = signature_iter.next().unwrap(); 
    assert_eq!(first_signature_decoding.alg().unwrap(), JwsAlgorithm::RS256); 
    assert_eq!(first_signature_decoding.unprotected_header().and_then(|value|value.kid()).unwrap(),"2010-12-29"); 
    let decoded_claims: JwtClaims::<serde_json::Value> = serde_json::from_slice(first_signature_decoding.claims()).unwrap();
    assert_eq!(claims, decoded_claims);

  }
}
