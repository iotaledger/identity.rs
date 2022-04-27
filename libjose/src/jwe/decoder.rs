// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use crypto::ciphers::aes::Aes128Gcm;
use crypto::ciphers::aes::Aes192Gcm;
use crypto::ciphers::aes::Aes256Gcm;
use crypto::ciphers::aes_cbc::Aes128CbcHmac256;
use crypto::ciphers::aes_cbc::Aes192CbcHmac384;
use crypto::ciphers::aes_cbc::Aes256CbcHmac512;
use crypto::ciphers::aes_kw::Aes128Kw;
use crypto::ciphers::aes_kw::Aes192Kw;
use crypto::ciphers::aes_kw::Aes256Kw;
use crypto::ciphers::chacha::ChaCha20Poly1305;
use crypto::ciphers::chacha::XChaCha20Poly1305;
use crypto::ciphers::traits::Aead;
use crypto::hashes::sha::SHA256_LEN;
use crypto::hashes::sha::SHA384_LEN;
use crypto::hashes::sha::SHA512_LEN;
use crypto::keys::pbkdf::PBKDF2_HMAC_SHA256;
use crypto::keys::pbkdf::PBKDF2_HMAC_SHA384;
use crypto::keys::pbkdf::PBKDF2_HMAC_SHA512;
use serde_json::from_slice;

use crate::error::Error;
use crate::error::Result;
use crate::jwe::JweAlgorithm;
use crate::jwe::JweEncryption;
use crate::jwe::JweFormat;
use crate::jwe::JweHeader;
use crate::jwk::EcdhCurve;
use crate::jwk::EcxCurve;
use crate::jwk::Jwk;
use crate::jwk::KeyManagement;
use crate::jwt::JwtHeaderSet;
use crate::lib::*;
use crate::utils::check_slice_param;
use crate::utils::concat_kdf;
use crate::utils::create_aad;
use crate::utils::create_pbes2_salt;
use crate::utils::decode_b64;
use crate::utils::decode_b64_json;
use crate::utils::diffie_hellman;
use crate::utils::filter_non_empty_bytes;
use crate::utils::parse_cek;
use crate::utils::parse_utf8;
use crate::utils::validate_jwe_headers;
use crate::utils::Secret;

pub type Token = (JweHeader, Vec<u8>);

type HeaderSet<'a> = JwtHeaderSet<'a, JweHeader>;

type Cek<'a> = Cow<'a, [u8]>;

const COMPACT_SEGMENTS: usize = 5;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecoderRecipient<'a> {
  pub header: Option<JweHeader>,
  pub encrypted_key: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct General<'a> {
  protected: Option<&'a str>,
  unprotected: Option<&'a str>,
  iv: Option<&'a str>,
  aad: Option<&'a str>,
  ciphertext: &'a str,
  tag: Option<&'a str>,
  recipients: Vec<DecoderRecipient<'a>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Flatten<'a> {
  protected: Option<&'a str>,
  unprotected: Option<&'a str>,
  iv: Option<&'a str>,
  aad: Option<&'a str>,
  ciphertext: &'a str,
  tag: Option<&'a str>,
  #[serde(flatten)]
  recipient: DecoderRecipient<'a>,
}

// =============================================================================
// =============================================================================

pub struct Decoder<'a> {
  /// The expected format of the encoded token.
  format: JweFormat,
  /// The curve used for Ecdh key agreements.
  ecdh_curve: EcdhCurve,
  /// The secret key used for key agreements and content decryption.
  secret: Secret<'a>,
  /// The public key used for key agreements.
  public: Option<Secret<'a>>,
  /// A list of permitted key management algorithms.
  algs: Option<Vec<JweAlgorithm>>,
  /// A list of permitted content encryption algorithms.
  encs: Option<Vec<JweEncryption>>,
  /// A list of permitted extension parameters.
  crits: Option<Vec<String>>,
  /// The expected key id of the decoded token.
  key_id: Option<String>,
}

impl<'a> Decoder<'a> {
  pub fn new(secret: impl Into<Secret<'a>>) -> Self {
    Self {
      format: JweFormat::Compact,
      ecdh_curve: EcdhCurve::Ecx(EcxCurve::X25519),
      secret: secret.into(),
      public: None,
      algs: None,
      encs: None,
      crits: None,
      key_id: None,
    }
  }

  pub fn format(mut self, value: JweFormat) -> Self {
    self.format = value;
    self
  }

  pub fn ecdh_curve(mut self, value: impl Into<EcdhCurve>) -> Self {
    self.ecdh_curve = value.into();
    self
  }

  pub fn public(mut self, value: impl Into<Secret<'a>>) -> Self {
    self.public = Some(value.into());
    self
  }

  pub fn algorithm(mut self, value: JweAlgorithm) -> Self {
    self.algs.get_or_insert_with(Vec::new).push(value);
    self
  }

  pub fn encryption(mut self, value: JweEncryption) -> Self {
    self.encs.get_or_insert_with(Vec::new).push(value);
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

  pub fn decode(&self, data: &[u8]) -> Result<Token> {
    let (expanded, protected, unprotected): (Expanded<'_>, Option<JweHeader>, Option<JweHeader>) =
      Self::expand(self.format, &self.crits, data)?;

    let protected_ref: Option<&JweHeader> = protected.as_ref();
    let unprotected_ref: Option<&JweHeader> = unprotected.as_ref();

    let cek: Option<(Cow<'_, [u8]>, Option<JweHeader>)> = expanded
      .recipients
      .into_iter()
      .find_map(|recipient| self.expand_recipient(protected_ref, unprotected_ref, recipient).ok());

    if let Some((cek, header)) = cek {
      let merged: HeaderSet<'_> = HeaderSet::new()
        .header(&header)
        .protected(protected_ref)
        .unprotected(unprotected_ref);

      let iv: Vec<u8> = expanded.iv.map(decode_b64).transpose()?.unwrap_or_default();
      let tag: Vec<u8> = expanded.tag.map(decode_b64).transpose()?.unwrap_or_default();
      let aad: Vec<u8> = create_aad(expanded.protected, expanded.aad);
      let ciphertext: Vec<u8> = decode_b64(expanded.ciphertext)?;
      let encryption: JweEncryption = merged.try_enc()?;
      let plaintext: Vec<u8> = decrypt_content(encryption, &cek, &iv, &aad, &tag, &ciphertext)?;

      let claims: Vec<u8> = if let Some(zip) = protected_ref.and_then(JweHeader::zip) {
        zip.decompress(&plaintext)?
      } else {
        plaintext
      };

      // TODO: Return Owned Header Set
      Ok((header.or(protected).or(unprotected).unwrap(), claims))
    } else {
      Err(Error::InvalidContent("Recipient (not found)"))
    }
  }

  fn expand_recipient(
    &self,
    protected: Option<&JweHeader>,
    unprotected: Option<&JweHeader>,
    mut recipient: DecoderRecipient<'_>,
  ) -> Result<(Cek<'a>, Option<JweHeader>)> {
    let header: Option<JweHeader> = recipient.header.take();

    let merged: HeaderSet<'_> = HeaderSet::new()
      .header(header.as_ref())
      .protected(protected)
      .unprotected(unprotected);

    Ok((self.decrypt_cek(merged, recipient)?, header))
  }

  fn decrypt_cek(&self, header: HeaderSet<'_>, recipient: DecoderRecipient<'_>) -> Result<Cek<'a>> {
    let alg: JweAlgorithm = header.try_alg()?;
    let enc: JweEncryption = header.try_enc()?;

    self.check_alg(alg)?;
    self.check_enc(enc)?;
    self.check_kid(header.kid())?;

    let cek: Cow<'_, [u8]> = self.decrypt_key(alg, enc, header, recipient)?;

    // THE CEK length MUST be equal to the required length of the content
    // encryption algorithm.
    if cek.len() == enc.key_len() {
      Ok(cek)
    } else {
      Err(Error::InvalidContent("CEK (length)"))
    }
  }

  #[doc(hidden)]
  pub fn __test_decrypt_key(&self, protected: &JweHeader) -> Result<Cek<'a>> {
    self.decrypt_key(
      protected.alg(),
      protected.enc(),
      HeaderSet::new().protected(protected),
      DecoderRecipient {
        header: None,
        encrypted_key: None,
      },
    )
  }

  fn decrypt_key(
    &self,
    algorithm: JweAlgorithm,
    encryption: JweEncryption,
    header: HeaderSet<'_>,
    recipient: DecoderRecipient<'_>,
  ) -> Result<Cek<'a>> {
    macro_rules! rsa {
      ($padding:ident, $recipient:expr, $secret:expr) => {{
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: _ = $secret.to_rsa_secret()?;
        let pad: _ = $crate::rsa_padding!(@$padding);
        let ptx: Vec<u8> = key.decrypt(pad, &ctx)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! aead {
      ($impl:ident, $header:expr, $recipient:expr, $secret:expr) => {{
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: Cow<'_, [u8]> = $secret.to_oct_key($impl::KEY_LENGTH)?;
        let iv: Vec<u8> = $header.try_iv().and_then(decode_b64)?;
        let tag: Vec<u8> = $header.try_tag().and_then(decode_b64)?;
        let mut ptx: Vec<u8> = vec![0; ctx.len()];

        $impl::try_decrypt(&key, &iv, &[], &mut ptx, &ctx, &tag)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! pbes2 {
      (($impl:ident, $digest_len:ident), $wrap:ident, $header:expr, $recipient:expr, $secret:expr) => {{
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: Cow<'_, [u8]> = $secret.to_oct_key(0)?;
        let p2s: Vec<u8> = $header.try_p2s().and_then(decode_b64)?;
        let p2c: usize = usize::try_from($header.try_p2c()?).map_err(|_| Error::InvalidClaim("p2c"))?;
        let salt: Vec<u8> = create_pbes2_salt($header.try_alg()?.name(), &p2s);
        let mut derived: [u8; $digest_len / 2] = [0; $digest_len / 2];

        $impl(&key, &salt, p2c, &mut derived)?;

        let mut ptx: Vec<u8> = ctx
          .len()
          .checked_sub($wrap::BLOCK)
          .ok_or(Error::InvalidContent("CEK (length)"))
          .map(|length| vec![0; length])?;

        $wrap::new(&derived).unwrap_key(&ctx, &mut ptx)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! aes_kw {
      ($impl:ident, $recipient:expr, $secret:expr) => {{
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: Cow<'_, [u8]> = $secret.to_oct_key($impl::KEY_LENGTH)?;

        let mut ptx: Vec<u8> = ctx
          .len()
          .checked_sub($impl::BLOCK)
          .ok_or(Error::InvalidContent("CEK (length)"))
          .map(|length| vec![0; length])?;

        $impl::new(&key).unwrap_key(&ctx, &mut ptx)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! ecdh_kw {
      (@es, $wrap:ident, $header:expr, $recipient:expr, $this:expr) => {
        ecdh_kw!(derive_ecdh_es, $wrap, $header, $recipient, $this)
      };
      (@1pu, $wrap:ident, $header:expr, $recipient:expr, $this:expr) => {
        ecdh_kw!(derive_ecdh_1pu, $wrap, $header, $recipient, $this)
      };
      ($derive:ident, $wrap:ident, $header:expr, $recipient:expr, $this:expr) => {{
        let algorithm: &str = $header.try_alg()?.name();
        let key_len: usize = $header.try_alg()?.try_key_len()?;
        let deriver: EcdhDeriver<'_, '_> = EcdhDeriver::new($this);
        let derived: Vec<u8> = deriver.$derive(&$header, algorithm, key_len)?;
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;

        let mut ptx: Vec<u8> = ctx
          .len()
          .checked_sub($wrap::BLOCK)
          .ok_or(Error::InvalidContent("CEK (length)"))
          .map(|length| vec![0; length])?;

        $wrap::new(&derived).unwrap_key(&ctx, &mut ptx)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! ecdh_chacha {
      (@es, $wrap:ident, $header:expr, $recipient:expr, $this:expr) => {
        ecdh_chacha!(derive_ecdh_es, $wrap, $header, $recipient, $this)
      };
      (@1pu, $wrap:ident, $header:expr, $recipient:expr, $this:expr) => {
        ecdh_chacha!(derive_ecdh_1pu, $wrap, $header, $recipient, $this)
      };
      ($derive:ident, $wrap:ident, $header:expr, $recipient:expr, $this:expr) => {{
        let algorithm: &str = $header.try_alg()?.name();
        let key_len: usize = $header.try_alg()?.try_key_len()?;
        let deriver: EcdhDeriver<'_, '_> = EcdhDeriver::new($this);
        let derived: Vec<u8> = deriver.$derive(&header, algorithm, key_len)?;
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let iv: Vec<u8> = $header.try_iv().and_then(decode_b64)?;
        let tag: Vec<u8> = $header.try_tag().and_then(decode_b64)?;

        let mut ptx: Vec<u8> = vec![0; ctx.len()];

        $wrap::try_decrypt(&derived, &iv, &[], &mut ptx, &ctx, &tag)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    if KeyManagement::from(algorithm).is_direct() && recipient.encrypted_key.is_some() {
      return Err(Error::EncError("CEK (non-empty)"));
    }

    match algorithm {
      JweAlgorithm::RSA1_5 => rsa!(RSA1_5, recipient, self.secret),
      JweAlgorithm::RSA_OAEP => rsa!(RSA_OAEP, recipient, self.secret),
      JweAlgorithm::RSA_OAEP_256 => rsa!(RSA_OAEP_256, recipient, self.secret),
      JweAlgorithm::RSA_OAEP_384 => rsa!(RSA_OAEP_384, recipient, self.secret),
      JweAlgorithm::RSA_OAEP_512 => rsa!(RSA_OAEP_512, recipient, self.secret),
      JweAlgorithm::A128KW => aes_kw!(Aes128Kw, recipient, self.secret),
      JweAlgorithm::A192KW => aes_kw!(Aes192Kw, recipient, self.secret),
      JweAlgorithm::A256KW => aes_kw!(Aes256Kw, recipient, self.secret),
      JweAlgorithm::DIR => self.secret.to_oct_key(0),
      JweAlgorithm::ECDH_ES => EcdhDeriver::new(self)
        .derive_ecdh_es(&header, encryption.name(), encryption.key_len())
        .map(Cow::Owned),
      JweAlgorithm::ECDH_ES_A128KW => ecdh_kw!(@es, Aes128Kw, header, recipient, self),
      JweAlgorithm::ECDH_ES_A192KW => ecdh_kw!(@es, Aes192Kw, header, recipient, self),
      JweAlgorithm::ECDH_ES_A256KW => ecdh_kw!(@es, Aes256Kw, header, recipient, self),
      JweAlgorithm::ECDH_ES_C20PKW => ecdh_chacha!(@es, ChaCha20Poly1305, header, recipient, self),
      JweAlgorithm::ECDH_ES_XC20PKW => ecdh_chacha!(@es, XChaCha20Poly1305, header, recipient, self),
      JweAlgorithm::A128GCMKW => {
        aead!(Aes128Gcm, header, recipient, self.secret)
      }
      JweAlgorithm::A192GCMKW => {
        aead!(Aes192Gcm, header, recipient, self.secret)
      }
      JweAlgorithm::A256GCMKW => {
        aead!(Aes256Gcm, header, recipient, self.secret)
      }
      JweAlgorithm::PBES2_HS256_A128KW => {
        pbes2!(
          (PBKDF2_HMAC_SHA256, SHA256_LEN),
          Aes128Kw,
          header,
          recipient,
          self.secret
        )
      }
      JweAlgorithm::PBES2_HS384_A192KW => {
        pbes2!(
          (PBKDF2_HMAC_SHA384, SHA384_LEN),
          Aes192Kw,
          header,
          recipient,
          self.secret
        )
      }
      JweAlgorithm::PBES2_HS512_A256KW => {
        pbes2!(
          (PBKDF2_HMAC_SHA512, SHA512_LEN),
          Aes256Kw,
          header,
          recipient,
          self.secret
        )
      }
      JweAlgorithm::ECDH_1PU => EcdhDeriver::new(self)
        .derive_ecdh_1pu(&header, encryption.name(), encryption.key_len())
        .map(Cow::Owned),
      JweAlgorithm::ECDH_1PU_A128KW => ecdh_kw!(@1pu, Aes128Kw, header, recipient, self),
      JweAlgorithm::ECDH_1PU_A192KW => ecdh_kw!(@1pu, Aes192Kw, header, recipient, self),
      JweAlgorithm::ECDH_1PU_A256KW => ecdh_kw!(@1pu, Aes256Kw, header, recipient, self),
      JweAlgorithm::C20PKW => {
        aead!(ChaCha20Poly1305, header, recipient, self.secret)
      }
      JweAlgorithm::XC20PKW => {
        aead!(XChaCha20Poly1305, header, recipient, self.secret)
      }
    }
  }

  pub fn expand<'data>(
    format: JweFormat,
    crits: &Option<Vec<String>>,
    data: &'data [u8],
  ) -> Result<(Expanded<'data>, Option<JweHeader>, Option<JweHeader>)> {
    let expanded: Expanded<'_> = match format {
      JweFormat::Compact => {
        let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

        if split.len() != COMPACT_SEGMENTS {
          return Err(Error::InvalidContent("Invalid Segments"));
        }

        Expanded {
          protected: filter_non_empty_bytes(split[0]),
          unprotected: None,
          recipients: vec![DecoderRecipient {
            header: None,
            encrypted_key: filter_non_empty_bytes(split[1]).map(parse_utf8).transpose()?,
          }],
          iv: filter_non_empty_bytes(split[2]),
          aad: None,
          ciphertext: split[3],
          tag: filter_non_empty_bytes(split[4]),
        }
      }
      JweFormat::General => {
        let data: General<'_> = from_slice(data)?;

        Expanded {
          protected: filter_non_empty_bytes(data.protected),
          unprotected: filter_non_empty_bytes(data.unprotected),
          recipients: data.recipients,
          iv: filter_non_empty_bytes(data.iv),
          aad: filter_non_empty_bytes(data.aad),
          ciphertext: data.ciphertext.as_bytes(),
          tag: filter_non_empty_bytes(data.tag),
        }
      }
      JweFormat::Flatten => {
        let data: Flatten<'_> = from_slice(data)?;

        Expanded {
          protected: filter_non_empty_bytes(data.protected),
          unprotected: filter_non_empty_bytes(data.unprotected),
          recipients: vec![data.recipient],
          iv: filter_non_empty_bytes(data.iv),
          aad: filter_non_empty_bytes(data.aad),
          ciphertext: data.ciphertext.as_bytes(),
          tag: filter_non_empty_bytes(data.tag),
        }
      }
    };

    let protected: Option<JweHeader> = expanded.protected.map(decode_b64_json).transpose()?;
    let unprotected: Option<JweHeader> = expanded.unprotected.map(decode_b64_json).transpose()?;

    let protected_ref: Option<&JweHeader> = protected.as_ref();
    let unprotected_ref: Option<&JweHeader> = unprotected.as_ref();

    validate_jwe_headers(
      protected_ref,
      unprotected_ref,
      expanded.recipients.iter().map(|recipient| recipient.header.as_ref()),
      crits.as_deref(),
    )?;

    Ok((expanded, protected, unprotected))
  }

  fn check_alg(&self, value: JweAlgorithm) -> Result<()> {
    check_slice_param("alg", self.algs.as_deref(), &value)
  }

  fn check_enc(&self, value: JweEncryption) -> Result<()> {
    check_slice_param("enc", self.encs.as_deref(), &value)
  }

  fn check_kid(&self, value: Option<&str>) -> Result<()> {
    if self.key_id.as_deref() == value {
      Ok(())
    } else {
      Err(Error::InvalidParam("kid"))
    }
  }
}

// =============================================================================
// =============================================================================

#[derive(Debug)]
pub struct Expanded<'a> {
  pub protected: Option<&'a [u8]>,
  pub unprotected: Option<&'a [u8]>,
  pub recipients: Vec<DecoderRecipient<'a>>,
  pub iv: Option<&'a [u8]>,
  pub aad: Option<&'a [u8]>,
  pub ciphertext: &'a [u8],
  pub tag: Option<&'a [u8]>,
}

fn decrypt_content(
  encryption: JweEncryption,
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
  ciphertext: &[u8],
) -> Result<Vec<u8>> {
  let mut plaintext: Vec<u8> = vec![0; ciphertext.len()];

  let length: usize = match encryption {
    JweEncryption::A128CBC_HS256 => Aes128CbcHmac256::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
    JweEncryption::A192CBC_HS384 => Aes192CbcHmac384::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
    JweEncryption::A256CBC_HS512 => Aes256CbcHmac512::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
    JweEncryption::A128GCM => Aes128Gcm::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
    JweEncryption::A192GCM => Aes192Gcm::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
    JweEncryption::A256GCM => Aes256Gcm::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
    JweEncryption::C20P => ChaCha20Poly1305::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
    JweEncryption::XC20P => XChaCha20Poly1305::try_decrypt(key, iv, aad, &mut plaintext, ciphertext, tag)?,
  };

  plaintext.truncate(length);

  Ok(plaintext)
}

struct EcdhDeriver<'a, 'b>(&'b Decoder<'a>);

impl<'a, 'b> EcdhDeriver<'a, 'b> {
  fn new(decoder: &'b Decoder<'a>) -> Self {
    Self(decoder)
  }

  fn derive_ecdh_es(&self, header: &HeaderSet<'_>, algorithm: &str, key_len: usize) -> Result<Vec<u8>> {
    self.derive_ecdh_key(header, algorithm, key_len, |epk| {
      diffie_hellman(self.0.ecdh_curve, epk, self.0.secret)
    })
  }

  fn derive_ecdh_1pu(&self, header: &HeaderSet<'_>, algorithm: &str, key_len: usize) -> Result<Vec<u8>> {
    self.derive_ecdh_key(header, algorithm, key_len, |epk| {
      let public: Secret<'a> = self.0.public.ok_or(Error::EncError("Missing ECDH-1PU Public Key"))?;
      let ze: Vec<u8> = diffie_hellman(self.0.ecdh_curve, epk, self.0.secret)?;
      let zs: Vec<u8> = diffie_hellman(self.0.ecdh_curve, public, self.0.secret)?;

      Ok([ze, zs].concat())
    })
  }

  fn derive_ecdh_key(
    &self,
    header: &HeaderSet<'_>,
    algorithm: &str,
    key_len: usize,
    key_exchange: impl Fn(&Jwk) -> Result<Vec<u8>>,
  ) -> Result<Vec<u8>> {
    let apu: Option<Vec<u8>> = header.apu().map(decode_b64).transpose()?;
    let apv: Option<Vec<u8>> = header.apv().map(decode_b64).transpose()?;

    concat_kdf(
      algorithm,
      key_len,
      &key_exchange(header.try_epk()?)?,
      apu.as_deref().unwrap_or_default(),
      apv.as_deref().unwrap_or_default(),
    )
  }
}
