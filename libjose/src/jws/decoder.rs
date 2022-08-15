// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::str;
use crypto::hashes::sha::SHA256;
use crypto::hashes::sha::SHA256_LEN;
use crypto::hashes::sha::SHA384;
use crypto::hashes::sha::SHA384_LEN;
use crypto::hashes::sha::SHA512;
use crypto::hashes::sha::SHA512_LEN;
use crypto::macs::hmac::HMAC_SHA256;
use crypto::macs::hmac::HMAC_SHA384;
use crypto::macs::hmac::HMAC_SHA512;
use serde_json::from_slice;
use subtle::ConstantTimeEq as _;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::EdCurve;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsFormat;
use crate::jws::JwsHeader;
use crate::jwt::JwtHeaderSet;
use crate::lib::*;
use crate::utils::check_slice_param;
use crate::utils::create_message;
use crate::utils::decode_b64;
use crate::utils::decode_b64_json;
use crate::utils::filter_non_empty_bytes;
use crate::utils::parse_utf8;
use crate::utils::validate_jws_headers;
use crate::utils::Secret;

type HeaderSet<'a> = JwtHeaderSet<'a, JwsHeader>;

type Ed25519Signature = crypto::signatures::ed25519::Signature;

const COMPACT_SEGMENTS: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
  pub protected: Option<JwsHeader>,
  pub unprotected: Option<JwsHeader>,
  pub claims: Cow<'a, [u8]>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Signature<'a> {
  header: Option<JwsHeader>,
  protected: Option<&'a str>,
  signature: &'a str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct General<'a> {
  payload: Option<&'a str>,
  signatures: Vec<Signature<'a>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Flatten<'a> {
  payload: Option<&'a str>,
  #[serde(flatten)]
  signature: Signature<'a>,
}

// =============================================================================
// =============================================================================

pub struct Decoder<'a, 'b> {
  /// The expected format of the encoded token.
  format: JwsFormat,
  /// The curve used for EdDSA signatures.
  eddsa_curve: EdCurve,
  /// The public key used for signature verification.
  public: Secret<'a>,
  /// A list of permitted signature algorithms.
  algs: Option<Vec<JwsAlgorithm>>,
  /// A list of permitted extension parameters.
  crits: Option<Vec<String>>,
  /// The expected key id of the decoded token.
  key_id: Option<String>,
  /// The detached payload, if using detached content
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  payload: Option<&'b [u8]>,
}

impl<'a, 'b> Decoder<'a, 'b> {
  pub fn new(public: impl Into<Secret<'a>>) -> Self {
    Self {
      format: JwsFormat::Compact,
      eddsa_curve: EdCurve::Ed25519,
      public: public.into(),
      algs: None,
      crits: None,
      key_id: None,
      payload: None,
    }
  }

  pub fn format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }

  pub fn eddsa_curve(mut self, value: EdCurve) -> Self {
    self.eddsa_curve = value;
    self
  }

  pub fn algorithm(mut self, value: JwsAlgorithm) -> Self {
    self.algs.get_or_insert_with(Vec::new).push(value);
    self
  }

  pub fn critical(mut self, value: impl Into<String>) -> Self {
    self.crits.get_or_insert_with(Vec::new).push(value.into());
    self
  }

  pub fn key_id(mut self, value: impl Into<String>) -> Self {
    self.key_id = Some(value.into());
    self
  }

  pub fn payload(mut self, value: &'b [u8]) -> Self {
    self.payload = Some(value);
    self
  }

  pub fn decode(&self, data: &'b [u8]) -> Result<Token<'b>> {
    self.expand(data, |payload, signatures| {
      for signature in signatures {
        if let Ok(token) = self.decode_one(payload, signature) {
          return Ok(token);
        }
      }

      Err(Error::InvalidContent("Recipient (not found)"))
    })
  }

  fn decode_one(&self, payload: &'b [u8], signature: Signature<'a>) -> Result<Token<'b>> {
    let protected: Option<JwsHeader> = signature.protected.map(decode_b64_json).transpose()?;

    validate_jws_headers(protected.as_ref(), signature.header.as_ref(), self.crits.as_deref())?;

    let merged: HeaderSet<'_> = HeaderSet::new()
      .protected(protected.as_ref())
      .unprotected(signature.header.as_ref());

    self.check_alg(merged.try_alg()?)?;
    self.check_kid(merged.kid())?;

    {
      let protected: &[u8] = signature.protected.map(str::as_bytes).unwrap_or_default();
      let message: Vec<u8> = create_message(protected, payload);
      let signature: Vec<u8> = decode_b64(signature.signature)?;

      self.verify(merged.try_alg()?, &message, &signature)?;
    }

    let claims: Cow<'b, [u8]> = if merged.b64().unwrap_or(true) {
      Cow::Owned(decode_b64(payload)?)
    } else {
      Cow::Borrowed(payload)
    };

    Ok(Token {
      protected,
      unprotected: signature.header,
      claims,
    })
  }

  fn expand<T>(&self, data: &'b [u8], format: impl Fn(&'b [u8], Vec<Signature<'_>>) -> Result<T>) -> Result<T> {
    match self.format {
      JwsFormat::Compact => {
        let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

        if split.len() != COMPACT_SEGMENTS {
          return Err(Error::InvalidContent("Segments (count)"));
        }

        let signature: Signature<'_> = Signature {
          header: None,
          protected: Some(parse_utf8(split[0])?),
          signature: parse_utf8(split[2])?,
        };

        format(self.expand_payload(Some(split[1]))?, vec![signature])
      }
      JwsFormat::General => {
        let data: General<'_> = from_slice(data)?;

        format(self.expand_payload(data.payload)?, data.signatures)
      }
      JwsFormat::Flatten => {
        let data: Flatten<'_> = from_slice(data)?;

        format(self.expand_payload(data.payload)?, vec![data.signature])
      }
    }
  }

  fn expand_payload(&self, payload: Option<&'b (impl AsRef<[u8]> + ?Sized)>) -> Result<&'b [u8]> {
    match (self.payload, filter_non_empty_bytes(payload)) {
      (Some(payload), None) => Ok(payload),
      (None, Some(payload)) => Ok(payload),
      (Some(_), Some(_)) => Err(Error::InvalidContent("Payload (multiple)")),
      (None, None) => Err(Error::InvalidContent("Payload (missing)")),
    }
  }

  fn check_alg(&self, value: JwsAlgorithm) -> Result<()> {
    check_slice_param("alg", self.algs.as_deref(), &value)
  }

  fn check_kid(&self, value: Option<&str>) -> Result<()> {
    if self.key_id.as_deref() == value {
      Ok(())
    } else {
      Err(Error::InvalidParam("kid"))
    }
  }

  fn verify(&self, algorithm: JwsAlgorithm, message: &[u8], signature: &[u8]) -> Result<()> {
    macro_rules! hmac {
      ($impl:ident, $key_len:ident, $message:expr, $signature:expr, $secret:expr) => {{
        let secret: Cow<'_, [u8]> = $secret.to_oct_key($key_len)?;
        let mut mac: [u8; $key_len] = [0; $key_len];

        $impl($message, &secret, &mut mac);

        if $signature.ct_eq(&mac).unwrap_u8() != 1 {
          return Err(Error::SigError("HMAC"));
        }
      }};
    }

    macro_rules! rsa {
      ($padding:ident, $digest:ident, $digest_len:ident, $message:expr, $signature:expr, $secret:expr) => {{
        let mut digest: [u8; $digest_len] = [0; $digest_len];

        $digest($message, &mut digest);

        let secret: _ = $secret.to_rsa_public()?;
        let padding: _ = $crate::rsa_padding!(@$padding);

        rsa::PublicKey::verify(&secret, padding, &digest, $signature)?;
      }};
    }

    let public: Secret<'_> = self.public;

    public.check_verifying_key(algorithm.name())?;

    match algorithm {
      JwsAlgorithm::HS256 => hmac!(HMAC_SHA256, SHA256_LEN, message, signature, public),
      JwsAlgorithm::HS384 => hmac!(HMAC_SHA384, SHA384_LEN, message, signature, public),
      JwsAlgorithm::HS512 => hmac!(HMAC_SHA512, SHA512_LEN, message, signature, public),
      JwsAlgorithm::RS256 => rsa!(PKCS1_SHA256, SHA256, SHA256_LEN, message, signature, public),
      JwsAlgorithm::RS384 => rsa!(PKCS1_SHA384, SHA384, SHA384_LEN, message, signature, public),
      JwsAlgorithm::RS512 => rsa!(PKCS1_SHA512, SHA512, SHA512_LEN, message, signature, public),
      JwsAlgorithm::PS256 => rsa!(PSS_SHA256, SHA256, SHA256_LEN, message, signature, public),
      JwsAlgorithm::PS384 => rsa!(PSS_SHA384, SHA384, SHA384_LEN, message, signature, public),
      JwsAlgorithm::PS512 => rsa!(PSS_SHA512, SHA512, SHA512_LEN, message, signature, public),
      JwsAlgorithm::ES256 => public.to_p256_public()?.verify(message, signature)?,
      JwsAlgorithm::ES384 => return Err(Error::AlgError("ES384")),
      JwsAlgorithm::ES512 => return Err(Error::AlgError("ES512")),
      JwsAlgorithm::ES256K => public.to_k256_public()?.verify(message, signature)?,
      JwsAlgorithm::NONE => return Err(Error::AlgError("NONE")),
      JwsAlgorithm::EdDSA => match self.eddsa_curve {
        EdCurve::Ed25519 => verify_ed25519(public, message, signature)?,
        EdCurve::Ed448 => return Err(Error::AlgError("EdDSA/Ed448")),
      },
    }

    Ok(())
  }
}

fn verify_ed25519(key: Secret<'_>, message: &[u8], signature: &[u8]) -> Result<()> {
  let signature: Ed25519Signature = signature
    .try_into()
    .map_err(|_| Error::SigError("Ed25519"))
    .map(Ed25519Signature::from_bytes)?;

  if key.to_ed25519_public()?.verify(&signature, message) {
    return Ok(());
  }

  Err(Error::SigError("Ed25519"))
}
