use core::str;
use crypto::hashes::sha::SHA256_LEN;
use crypto::hashes::sha::SHA384_LEN;
use crypto::hashes::sha::SHA512_LEN;

use crate::crypto::hmac_sha256;
use crate::crypto::hmac_sha384;
use crate::crypto::hmac_sha512;
use crate::crypto::sha256;
use crate::crypto::sha384;
use crate::crypto::sha512;
use crate::error::Error;
use crate::error::Result;
use crate::jwk::EdCurve;
use crate::jws::CharSet;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsFormat;
use crate::jws::JwsHeader;
use crate::jws::Recipient;
use crate::lib::*;
use crate::rsa_padding;
use crate::utils::create_message;
use crate::utils::encode_b64;
use crate::utils::encode_b64_json;
use crate::utils::extract_b64;
use crate::utils::validate_jws_headers;
use crate::utils::Secret;

macro_rules! to_json {
  ($data:expr) => {{
    ::serde_json::to_string(&$data).map_err(Into::into)
  }};
}

#[derive(Serialize)]
struct Signature<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  header: Option<&'a JwsHeader>,
  #[serde(skip_serializing_if = "Option::is_none")]
  protected: Option<String>,
  signature: String,
}

#[derive(Serialize)]
struct General<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  payload: Option<&'a str>,
  signatures: Vec<Signature<'a>>,
}

#[derive(Serialize)]
struct Flatten<'a: 'b, 'b> {
  #[serde(skip_serializing_if = "Option::is_none")]
  payload: Option<&'a str>,
  #[serde(flatten)]
  signature: &'b Signature<'a>,
}

// =============================================================================
// =============================================================================

pub struct Encoder<'a> {
  /// The output format of the encoded token.
  format: JwsFormat,
  /// Content validation rules for unencoded content using the compact format.
  charset: CharSet,
  /// Encode the token with detached content.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7515#appendix-F)
  detached: bool,
  /// Per-recipient configuration.
  recipients: Vec<Recipient<'a>>,
}

impl Default for Encoder<'_> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Encoder<'a> {
  pub fn new() -> Self {
    Self {
      format: JwsFormat::Compact,
      charset: CharSet::Default,
      detached: false,
      recipients: Vec::new(),
    }
  }

  pub fn format(mut self, value: JwsFormat) -> Self {
    self.format = value;
    self
  }

  pub fn charset(mut self, value: CharSet) -> Self {
    self.charset = value;
    self
  }

  pub fn detached(mut self, value: bool) -> Self {
    self.detached = value;
    self
  }

  pub fn recipient(mut self, value: impl Into<Recipient<'a>>) -> Self {
    self.recipients.push(value.into());
    self
  }

  pub fn encode(&self, claims: &[u8]) -> Result<String> {
    if self.recipients.is_empty() {
      return Err(Error::SigError("Missing Recipients"));
    }

    self.validate()?;

    let b64: bool = extract_b64(self.recipients[0].protected);
    let tmp: String;

    // Extract the "b64" header parameter and encode the payload as required.
    //
    // See: https://tools.ietf.org/html/rfc7797#section-3
    let payload: &[u8] = if b64 {
      tmp = encode_b64(claims);
      tmp.as_bytes()
    } else if self.detached {
      claims
    } else {
      self.charset.validate(claims)?;
      claims
    };

    let encoded: Vec<Signature<'a>> = self
      .recipients
      .iter()
      .copied()
      .map(|recipient| encode_recipient(payload, recipient))
      .collect::<Result<_>>()?;

    assert_eq!(encoded.len(), self.recipients.len());

    match (self.format, &*encoded) {
      (JwsFormat::Compact, [recipient]) => {
        let protected: &str = recipient.protected.as_deref().unwrap_or_default();

        if let Some(payload) = self.format_payload(payload) {
          Ok(format!("{}.{}.{}", protected, payload, recipient.signature))
        } else {
          Ok(format!("{}..{}", protected, recipient.signature))
        }
      }
      (JwsFormat::General, _) => {
        to_json!(General {
          payload: self.format_payload(payload),
          signatures: encoded,
        })
      }
      (JwsFormat::Flatten, [recipient]) => {
        to_json!(Flatten {
          payload: self.format_payload(payload),
          signature: recipient,
        })
      }
      _ => {
        unreachable!()
      }
    }
  }

  fn format_payload(&self, payload: &'a [u8]) -> Option<&'a str> {
    if self.detached {
      None
    } else {
      // SAFETY: We validated the payload and ensured valid UTF-8 as an earlier
      // step in the encoding process; this function is not exposed to users.
      Some(unsafe { str::from_utf8_unchecked(payload) })
    }
  }

  fn validate(&self) -> Result<()> {
    match (self.format, &*self.recipients) {
      (JwsFormat::Compact, &[recipient @ Recipient { unprotected: None, .. }]) => validate_jws_headers(
        recipient.protected,
        None,
        recipient.protected.and_then(|header| header.crit()),
      ),
      (JwsFormat::Compact, _) => Err(Error::SigError(
        "JWS Compact Serialization doesn't support multiple recipients or unprotected headers",
      )),
      (JwsFormat::General, recipients) => {
        let mut __b64: bool = false;

        for recipient in recipients {
          if !__b64 && recipient.protected.and_then(JwsHeader::b64).is_some() {
            __b64 = true;
          }

          validate_jws_headers(
            recipient.protected,
            recipient.unprotected,
            recipient.protected.and_then(|header| header.crit()),
          )?;
        }

        if __b64 {
          validate_recipient_b64(recipients)?;
        }

        Ok(())
      }
      (JwsFormat::Flatten, &[recipient]) => validate_jws_headers(
        recipient.protected,
        recipient.unprotected,
        recipient.protected.and_then(|header| header.crit()),
      ),
      (JwsFormat::Flatten, _) => Err(Error::SigError(
        "JWS Flattened Serialization doesn't support multiple recipients",
      )),
    }
  }
}

// =============================================================================
// =============================================================================

fn validate_recipient_b64(recipients: &[Recipient<'_>]) -> Result<()> {
  // The "b64" header parameter value MUST be the same for all recipients
  recipients
    .iter()
    .map(|recipient| crate::utils::extract_b64(recipient.protected))
    .try_fold::<_, _, Result<_>>(None, |acc, item| match acc {
      Some(current) if current == item => Ok(acc),
      Some(_) => Err(Error::InvalidParam("b64")),
      None => Ok(Some(item)),
    })
    .map(|_| ())
}

fn encode_recipient<'a>(payload: &[u8], recipient: Recipient<'a>) -> Result<Signature<'a>> {
  let algorithm: JwsAlgorithm = recipient
    .protected
    .map(JwsHeader::alg)
    .or_else(|| recipient.unprotected.map(JwsHeader::alg))
    .ok_or(Error::InvalidParam("alg"))?;

  let protected: Option<String> = recipient.protected.map(encode_b64_json).transpose()?;
  let header: &[u8] = protected.as_deref().map(str::as_bytes).unwrap_or_default();
  let message: Vec<u8> = create_message(header, payload);
  let signature: String = sign(algorithm, &message, recipient)?;

  Ok(Signature {
    header: recipient.unprotected,
    protected,
    signature,
  })
}

fn sign(algorithm: JwsAlgorithm, message: &[u8], recipient: Recipient<'_>) -> Result<String> {
  macro_rules! rsa {
    ($padding:ident, $digest:ident, $message:expr, $secret:expr) => {{
      $secret.to_rsa_secret()?.sign(rsa_padding!(@$padding), &$digest($message))?
    }};
  }

  let secret: Secret<'_> = recipient.secret;

  secret.check_signing_key(algorithm.name())?;

  match algorithm {
    JwsAlgorithm::HS256 => Ok(encode_b64(hmac_sha256(&secret.to_oct_key(SHA256_LEN)?, message))),
    JwsAlgorithm::HS384 => Ok(encode_b64(hmac_sha384(&secret.to_oct_key(SHA384_LEN)?, message))),
    JwsAlgorithm::HS512 => Ok(encode_b64(hmac_sha512(&secret.to_oct_key(SHA512_LEN)?, message))),
    JwsAlgorithm::RS256 => Ok(encode_b64(rsa!(PKCS1_SHA256, sha256, message, secret))),
    JwsAlgorithm::RS384 => Ok(encode_b64(rsa!(PKCS1_SHA384, sha384, message, secret))),
    JwsAlgorithm::RS512 => Ok(encode_b64(rsa!(PKCS1_SHA512, sha512, message, secret))),
    JwsAlgorithm::PS256 => Ok(encode_b64(rsa!(PSS_SHA256, sha256, message, secret))),
    JwsAlgorithm::PS384 => Ok(encode_b64(rsa!(PSS_SHA384, sha384, message, secret))),
    JwsAlgorithm::PS512 => Ok(encode_b64(rsa!(PSS_SHA512, sha512, message, secret))),
    JwsAlgorithm::ES256 => Ok(encode_b64(secret.to_p256_secret()?.sign(message)?)),
    JwsAlgorithm::ES384 => todo!("ES384"),
    JwsAlgorithm::ES512 => todo!("ES512"),
    JwsAlgorithm::ES256K => Ok(encode_b64(secret.to_k256_secret()?.sign(message)?)),
    JwsAlgorithm::NONE => todo!("NONE"),
    JwsAlgorithm::EdDSA => match recipient.eddsa_curve {
      EdCurve::Ed25519 => Ok(encode_b64(secret.to_ed25519_secret()?.sign(message).to_bytes())),
      EdCurve::Ed448 => todo!("EdDSA/Ed448"),
    },
  }
}
