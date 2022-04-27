// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom as _;
use core::num::NonZeroUsize;
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
use crypto::ciphers::traits::Nonce;
use crypto::ciphers::traits::Tag;
use crypto::hashes::sha::SHA256_LEN;
use crypto::hashes::sha::SHA384_LEN;
use crypto::hashes::sha::SHA512_LEN;
use crypto::keys::pbkdf::PBKDF2_HMAC_SHA256;
use crypto::keys::pbkdf::PBKDF2_HMAC_SHA384;
use crypto::keys::pbkdf::PBKDF2_HMAC_SHA512;
use serde::Serialize;
use serde_json::to_vec;

use crate::error::Error;
use crate::error::Result;
use crate::jwe::JweAlgorithm;
use crate::jwe::JweEncryption;
use crate::jwe::JweFormat;
use crate::jwe::JweHeader;
use crate::jwe::Recipient;
use crate::jwk::Jwk;
use crate::jwt::JwtHeaderSet;
use crate::lib::*;
use crate::utils::concat_kdf;
use crate::utils::create_aad;
use crate::utils::create_pbes2_salt;
use crate::utils::decode_b64;
use crate::utils::diffie_hellman;
use crate::utils::encode_b64;
use crate::utils::encode_b64_json;
use crate::utils::random_bytes;
use crate::utils::validate_jwe_headers;
use crate::utils::Secret;

type HeaderSet<'a> = JwtHeaderSet<'a, JweHeader>;

const MIN_P2S: usize = 8;
const MIN_P2C: usize = 1000;

macro_rules! to_json {
  ($data:expr) => {{
    ::serde_json::to_string(&$data).map_err(Into::into)
  }};
}

#[derive(Serialize)]
struct __Recipient<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  header: Option<&'a JweHeader>,
  #[serde(skip_serializing_if = "Option::is_none")]
  encrypted_key: Option<&'a str>,
}

#[derive(Serialize)]
struct General<'a> {
  protected: Option<&'a str>,
  unprotected: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  iv: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  aad: Option<&'a str>,
  ciphertext: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  tag: Option<&'a str>,
  recipients: Vec<__Recipient<'a>>,
}

#[derive(Serialize)]
struct Flatten<'a> {
  protected: Option<&'a str>,
  unprotected: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  iv: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  aad: Option<&'a str>,
  ciphertext: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  tag: Option<&'a str>,
  #[serde(flatten)]
  recipient: __Recipient<'a>,
}

// =============================================================================
// =============================================================================

pub struct Encoder<'a> {
  /// The output format of the encoded token.
  format: JweFormat,
  /// The secret key used for key agreements.
  secret: Option<Secret<'a>>,
  /// Additional authenticated data.
  aad: Option<&'a [u8]>,
  /// Agreement PartyUInfo used with Concat KDF.
  apu: Option<&'a [u8]>,
  /// Agreement PartyVInfo used with Concat KDF
  apv: Option<&'a [u8]>,
  /// The salt input used with PBES2.
  pbes2_p2s: usize,
  /// The PBKDF2 iteration count used with PBES2.
  pbes2_p2c: usize,
  /// The integrity-protected JOSE header.
  protected: Option<&'a JweHeader>,
  /// The non integrity-protected JOSE header.
  unprotected: Option<&'a JweHeader>,
  /// Per-recipient configuration.
  recipients: Vec<Recipient<'a>>,
}

impl Default for Encoder<'_> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Encoder<'a> {
  pub const fn new() -> Self {
    Self {
      format: JweFormat::Compact,
      secret: None,
      aad: None,
      apu: None,
      apv: None,
      pbes2_p2s: MIN_P2S,
      pbes2_p2c: MIN_P2C,
      protected: None,
      unprotected: None,
      recipients: Vec::new(),
    }
  }

  pub fn format(mut self, value: JweFormat) -> Self {
    self.format = value;
    self
  }

  pub fn secret(mut self, value: impl Into<Secret<'a>>) -> Self {
    self.secret = Some(value.into());
    self
  }

  pub fn aad(mut self, value: &'a [u8]) -> Self {
    self.aad = Some(value);
    self
  }

  pub fn apu(mut self, value: &'a [u8]) -> Self {
    self.apu = Some(value);
    self
  }

  pub fn apv(mut self, value: &'a [u8]) -> Self {
    self.apv = Some(value);
    self
  }

  pub fn protected(mut self, value: &'a JweHeader) -> Self {
    self.protected = Some(value);
    self
  }

  pub fn unprotected(mut self, value: &'a JweHeader) -> Self {
    self.unprotected = Some(value);
    self
  }

  pub fn recipient(mut self, value: impl Into<Recipient<'a>>) -> Self {
    self.recipients.push(value.into());
    self
  }

  pub fn encode_serde<T>(&self, claims: &T) -> Result<String>
  where
    T: Serialize,
  {
    self.encode(&to_vec(claims)?)
  }

  pub fn encode(&self, claims: &[u8]) -> Result<String> {
    if self.recipients.is_empty() {
      return Err(Error::EncError("Missing Recipients"));
    }

    self.validate()?;

    let mut context: __Context<'_, '_> = __Context::new(self, self.recipients.len());

    for recipient in self.recipients.iter() {
      context.expand_recipient(self.protected, self.unprotected, recipient.clone())?;
    }

    let encryption: JweEncryption = HeaderSet::new()
      .protected(self.protected)
      .unprotected(self.unprotected)
      .try_enc()?;

    let encryption_key: Cow<'_, [u8]> = if let Some(cek) = context.encryption_key {
      cek
    } else {
      Cow::Owned(random_bytes(encryption.key_len())?)
    };

    let compressed: Vec<u8>;
    let payload: &[u8] = if let Some(zip) = self.protected.and_then(JweHeader::zip) {
      compressed = zip.compress(claims)?;
      &compressed
    } else {
      claims
    };

    let recipients: Vec<(Option<String>, JweHeader)> = context
      .recipients
      .into_iter()
      .map(|(recipient, mut output)| {
        let encrypted_key: Option<String> = self
          .encrypt_cek(&encryption_key, &mut output, recipient)?
          .map(encode_b64);

        Ok((encrypted_key, output))
      })
      .collect::<Result<_>>()?;

    let protected_b64: Option<String> = if self.format == JweFormat::Compact {
      assert_eq!(recipients.len(), 1);
      Some(encode_b64_json(&recipients[0].1)?)
    } else {
      assert_eq!(recipients.len(), self.recipients.len());
      self.protected.map(encode_b64_json).transpose()?
    };

    let aad_b64: Option<String> = self.aad.map(encode_b64);
    let aad: Vec<u8> = create_aad(protected_b64.as_deref(), aad_b64.as_deref());
    let iv: Vec<u8> = random_bytes(encryption.iv_len())?;
    let (ciphertext, tag): _ = encrypt_content(encryption, &encryption_key, &iv, &aad, payload)?;

    match (self.format, &*recipients) {
      (JweFormat::Compact, [(encrypted_key, _)]) => Ok(format!(
        "{}.{}.{}.{}.{}",
        protected_b64.as_deref().unwrap_or_default(),
        encrypted_key.as_deref().unwrap_or_default(),
        encode_b64(iv),
        encode_b64(ciphertext),
        encode_b64(tag),
      )),
      (JweFormat::General, _) => {
        let recipients: Vec<__Recipient<'_>> = recipients
          .iter()
          .map(|recipient| __Recipient {
            encrypted_key: recipient.0.as_deref(),
            header: Some(&recipient.1),
          })
          .collect();

        to_json!(General {
          protected: protected_b64.as_deref(),
          unprotected: self.unprotected.map(encode_b64_json).transpose()?.as_deref(),
          ciphertext: &encode_b64(ciphertext),
          aad: aad_b64.as_deref(),
          iv: Some(&encode_b64(iv)),
          tag: Some(&encode_b64(tag)),
          recipients,
        })
      }
      (JweFormat::Flatten, [(encrypted_key, header)]) => {
        to_json!(Flatten {
          protected: protected_b64.as_deref(),
          unprotected: self.unprotected.map(encode_b64_json).transpose()?.as_deref(),
          ciphertext: &encode_b64(ciphertext),
          aad: aad_b64.as_deref(),
          iv: Some(&encode_b64(iv)),
          tag: Some(&encode_b64(tag)),
          recipient: __Recipient {
            encrypted_key: encrypted_key.as_deref(),
            header: Some(header),
          },
        })
      }
      _ => unreachable!(),
    }
  }

  fn generate_cek<'cek>(
    &self,
    algorithm: JweAlgorithm,
    encryption: JweEncryption,
    output: &mut JweHeader,
    recipient: Recipient<'cek>,
  ) -> Result<Option<Cow<'cek, [u8]>>> {
    match algorithm {
      JweAlgorithm::DIR => {
        let key: Cow<'_, [u8]> = recipient.public.to_oct_key(0)?;

        if key.len() != encryption.key_len() {
          return Err(Error::EncError("CEK (length)"));
        }

        Ok(Some(key))
      }
      JweAlgorithm::ECDH_ES => EcdhDeriver::new(self, &recipient)
        .derive_ecdh_es(output, encryption.name(), encryption.key_len())
        .map(Cow::Owned)
        .map(Some),
      JweAlgorithm::ECDH_1PU => EcdhDeriver::new(self, &recipient)
        .derive_ecdh_1pu(output, encryption.name(), encryption.key_len())
        .map(Cow::Owned)
        .map(Some),
      _ => Ok(None),
    }
  }

  fn extract_p2s(&self, output: &mut JweHeader) -> Result<Vec<u8>> {
    match output.p2s() {
      Some(p2s) => {
        let p2s: Vec<u8> = decode_b64(p2s)?;

        if p2s.len() < MIN_P2S {
          return Err(Error::InvalidClaim("p2s"));
        }

        Ok(p2s)
      }
      None => {
        let p2s: Vec<u8> = random_bytes(self.pbes2_p2s)?;
        output.set_p2s(encode_b64(&p2s));
        Ok(p2s)
      }
    }
  }

  fn extract_p2c(&self, output: &mut JweHeader) -> Result<usize> {
    match output.p2c() {
      Some(p2c) => usize::try_from(p2c).map_err(|_| Error::InvalidClaim("p2c")),
      None => {
        output.set_p2c(self.pbes2_p2c as u64);

        Ok(self.pbes2_p2c)
      }
    }
  }

  fn encrypt_cek(
    &self,
    encryption_key: &[u8],
    output: &mut JweHeader,
    recipient: Recipient<'_>,
  ) -> Result<Option<Vec<u8>>> {
    macro_rules! rsa {
      ($padding:ident, $encryption_key:expr, $public:expr) => {{
        use ::rsa::PublicKey;
        use ::rand::rngs::OsRng;

        let key: _ = $public.to_rsa_public()?;
        let pad: _ = $crate::rsa_padding!(@$padding);
        let ctx: Vec<u8> = key.encrypt(&mut OsRng, pad, $encryption_key)?;

        Ok(Some(ctx))
      }};
    }

    macro_rules! aead {
      ($impl:ident, $encryption_key:expr, $public:expr, $output:expr) => {{
        let key: Cow<'_, [u8]> = $public.to_oct_key($impl::KEY_LENGTH)?;
        let nonce: Nonce<$impl> = $impl::random_nonce()?;

        let mut ctx: Vec<u8> = vec![0; $encryption_key.len()];
        let mut tag: Tag<$impl> = Default::default();

        $impl::try_encrypt(&key, &nonce, &[], $encryption_key, &mut ctx, &mut tag)?;

        $output.set_iv(encode_b64(&nonce));
        $output.set_tag(encode_b64(&tag));

        Ok(Some(ctx))
      }};
    }

    macro_rules! pbes2 {
      (($impl:ident, $digest_len:ident), $wrap:ident, $encryption_key:expr, $public:expr, $output:expr, $this:expr) => {{
        let key: Cow<'_, [u8]> = $public.to_oct_key(0)?;
        let p2s: Vec<u8> = $this.extract_p2s($output)?;
        let p2c: usize = $this.extract_p2c($output)?;
        let salt: Vec<u8> = create_pbes2_salt($output.alg().name(), &p2s);
        let mut derived: [u8; $digest_len / 2] = [0; $digest_len / 2];

        $impl(&key, &salt, p2c, &mut derived)?;

        let mut ctx: Vec<u8> = vec![0; $encryption_key.len() + $wrap::BLOCK];

        $wrap::new(&derived).wrap_key($encryption_key, &mut ctx)?;

        Ok(Some(ctx))
      }};
    }

    macro_rules! aes_kw {
      ($impl:ident, $encryption_key:expr, $public:expr) => {{
        let key: Cow<'_, [u8]> = $public.to_oct_key($impl::KEY_LENGTH)?;
        let mut ctx: Vec<u8> = vec![0; $encryption_key.len() + $impl::BLOCK];

        $impl::new(&key).wrap_key($encryption_key, &mut ctx)?;

        Ok(Some(ctx))
      }};
    }

    macro_rules! ecdh_kw {
      (@es, $wrap:ident, $encryption_key:expr, $recipient:expr, $output:expr, $this:expr) => {{
        ecdh_kw!(derive_ecdh_es, $wrap, $encryption_key, $recipient, $output, $this)
      }};
      (@1pu, $wrap:ident, $encryption_key:expr, $recipient:expr, $output:expr, $this:expr) => {{
        ecdh_kw!(derive_ecdh_1pu, $wrap, $encryption_key, $recipient, $output, $this)
      }};
      ($derive:ident, $wrap:ident, $encryption_key:expr, $recipient:expr, $output:expr, $this:expr) => {{
        let algorithm: &str = $output.alg().name();
        let key_len: usize = $output.alg().try_key_len()?;
        let deriver: EcdhDeriver<'_, '_> = EcdhDeriver::new($this, &$recipient);
        let derived: Vec<u8> = deriver.$derive($output, algorithm, key_len)?;
        let mut ctx: Vec<u8> = vec![0; $encryption_key.len() + $wrap::BLOCK];

        $wrap::new(&derived).wrap_key($encryption_key, &mut ctx)?;

        Ok(Some(ctx))
      }};
    }

    macro_rules! ecdh_chacha {
      (@es, $wrap:ident, $encryption_key:expr, $recipient:expr, $output:expr, $this:expr) => {{
        ecdh_chacha!(derive_ecdh_es, $wrap, $encryption_key, $recipient, $output, $this)
      }};
      (@1pu, $wrap:ident, $encryption_key:expr, $recipient:expr, $output:expr, $this:expr) => {{
        ecdh_chacha!(derive_ecdh_1pu, $wrap, $encryption_key, $recipient, $output, $this)
      }};
      ($derive:ident, $wrap:ident, $encryption_key:expr, $recipient:expr, $output:expr, $this:expr) => {{
        let algorithm: &str = $output.alg().name();
        let key_len: usize = $output.alg().try_key_len()?;
        let deriver: EcdhDeriver<'_, '_> = EcdhDeriver::new($this, &$recipient);
        let derived: Vec<u8> = deriver.$derive($output, algorithm, key_len)?;
        let nonce: Nonce<$wrap> = $wrap::random_nonce()?;

        let mut ctx: Vec<u8> = vec![0; $encryption_key.len()];
        let mut tag: Tag<$wrap> = Default::default();

        $wrap::try_encrypt(&derived, &nonce, &[], $encryption_key, &mut ctx, &mut tag)?;

        $output.set_iv(encode_b64(&nonce));
        $output.set_tag(encode_b64(&tag));

        Ok(Some(ctx))
      }};
    }

    match output.alg() {
      JweAlgorithm::RSA1_5 => rsa!(RSA1_5, encryption_key, recipient.public),
      JweAlgorithm::RSA_OAEP => rsa!(RSA_OAEP, encryption_key, recipient.public),
      JweAlgorithm::RSA_OAEP_256 => rsa!(RSA_OAEP_256, encryption_key, recipient.public),
      JweAlgorithm::RSA_OAEP_384 => rsa!(RSA_OAEP_384, encryption_key, recipient.public),
      JweAlgorithm::RSA_OAEP_512 => rsa!(RSA_OAEP_512, encryption_key, recipient.public),
      JweAlgorithm::A128KW => aes_kw!(Aes128Kw, encryption_key, recipient.public),
      JweAlgorithm::A192KW => aes_kw!(Aes192Kw, encryption_key, recipient.public),
      JweAlgorithm::A256KW => aes_kw!(Aes256Kw, encryption_key, recipient.public),
      JweAlgorithm::DIR => Ok(None),
      JweAlgorithm::ECDH_ES => Ok(None),
      JweAlgorithm::ECDH_ES_A128KW => {
        ecdh_kw!(@es, Aes128Kw, encryption_key, recipient, output, self)
      }
      JweAlgorithm::ECDH_ES_A192KW => {
        ecdh_kw!(@es, Aes192Kw, encryption_key, recipient, output, self)
      }
      JweAlgorithm::ECDH_ES_A256KW => {
        ecdh_kw!(@es, Aes256Kw, encryption_key, recipient, output, self)
      }
      JweAlgorithm::ECDH_ES_C20PKW => {
        ecdh_chacha!(@es, ChaCha20Poly1305, encryption_key, recipient, output, self)
      }
      JweAlgorithm::ECDH_ES_XC20PKW => {
        ecdh_chacha!(@es, XChaCha20Poly1305, encryption_key, recipient, output, self)
      }
      JweAlgorithm::A128GCMKW => {
        aead!(Aes128Gcm, encryption_key, recipient.public, output)
      }
      JweAlgorithm::A192GCMKW => {
        aead!(Aes192Gcm, encryption_key, recipient.public, output)
      }
      JweAlgorithm::A256GCMKW => {
        aead!(Aes256Gcm, encryption_key, recipient.public, output)
      }
      JweAlgorithm::PBES2_HS256_A128KW => {
        pbes2!(
          (PBKDF2_HMAC_SHA256, SHA256_LEN),
          Aes128Kw,
          encryption_key,
          recipient.public,
          output,
          self
        )
      }
      JweAlgorithm::PBES2_HS384_A192KW => {
        pbes2!(
          (PBKDF2_HMAC_SHA384, SHA384_LEN),
          Aes192Kw,
          encryption_key,
          recipient.public,
          output,
          self
        )
      }
      JweAlgorithm::PBES2_HS512_A256KW => {
        pbes2!(
          (PBKDF2_HMAC_SHA512, SHA512_LEN),
          Aes256Kw,
          encryption_key,
          recipient.public,
          output,
          self
        )
      }
      JweAlgorithm::ECDH_1PU => Ok(None),
      JweAlgorithm::ECDH_1PU_A128KW => {
        ecdh_kw!(@1pu, Aes128Kw, encryption_key, recipient, output, self)
      }
      JweAlgorithm::ECDH_1PU_A192KW => {
        ecdh_kw!(@1pu, Aes192Kw, encryption_key, recipient, output, self)
      }
      JweAlgorithm::ECDH_1PU_A256KW => {
        ecdh_kw!(@1pu, Aes256Kw, encryption_key, recipient, output, self)
      }
      JweAlgorithm::C20PKW => {
        aead!(ChaCha20Poly1305, encryption_key, recipient.public, output)
      }
      JweAlgorithm::XC20PKW => {
        aead!(XChaCha20Poly1305, encryption_key, recipient.public, output)
      }
    }
  }

  fn validate(&self) -> Result<()> {
    validate_jwe_headers(
      self.protected,
      self.unprotected,
      self.recipients.iter().map(|recipient| recipient.header.as_ref()),
      self.protected.and_then(|header| header.crit()),
    )?;

    match (self.format, &*self.recipients, self.aad) {
      (JweFormat::Compact, &[Recipient { header: None, .. }], None) => {
        if self.protected.is_some() {
          Ok(())
        } else {
          Err(Error::EncError("JWE Compact Serialization requires a protected header"))
        }
      }
      (JweFormat::Compact, _, _) => Err(Error::EncError(
        "JWE Compact Serialization doesn't support multiple recipients, per-recipient headers, or AAD",
      )),
      (JweFormat::General, _, _) => Ok(()),
      (JweFormat::Flatten, &[_], _) => Ok(()),
      (JweFormat::Flatten, _, _) => Err(Error::EncError(
        "JWE Flattened Serialization doesn't support multiple recipients",
      )),
    }
  }
}

// =============================================================================
// =============================================================================

struct __Context<'a, 'b> {
  encoder: &'b Encoder<'a>,
  recipients: Vec<(Recipient<'a>, JweHeader)>,
  encryption_key: Option<Cow<'a, [u8]>>,
}

impl<'a, 'b> __Context<'a, 'b> {
  pub fn new(encoder: &'b Encoder<'a>, recipients: usize) -> Self {
    Self {
      encoder,
      recipients: Vec::with_capacity(recipients),
      encryption_key: None,
    }
  }

  pub fn expand_recipient(
    &mut self,
    protected: Option<&'a JweHeader>,
    unprotected: Option<&'a JweHeader>,
    recipient: Recipient<'a>,
  ) -> Result<()> {
    let merged: HeaderSet<'_> = HeaderSet::new()
      .header(&recipient.header)
      .protected(protected)
      .unprotected(unprotected);

    let algorithm: JweAlgorithm = merged.try_alg()?;
    let encryption: JweEncryption = merged.try_enc()?;

    let mut output: JweHeader = if let Some(recipient) = recipient.header.clone() {
      recipient
    } else if self.encoder.format == JweFormat::Compact {
      protected.cloned().unwrap()
    } else {
      JweHeader::new(algorithm, encryption)
    };

    let cek: Option<Cow<'_, [u8]>> =
      self
        .encoder
        .generate_cek(algorithm, encryption, &mut output, recipient.clone())?;

    if let Some(encryption_key) = cek {
      if let Some(cek) = self.encryption_key.as_ref() {
        if cek.as_ref() != encryption_key.as_ref() {
          return Err(Error::EncError("CEK (mismatch)"));
        }
      } else {
        self.encryption_key = Some(encryption_key);
      }
    }

    self.recipients.push((recipient, output));

    Ok(())
  }
}

fn encrypt_content(
  encryption: JweEncryption,
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  plaintext: &[u8],
) -> Result<(Vec<u8>, Vec<u8>)> {
  let mut ciphertext: Vec<u8> = vec![0; plaintext.len()];

  let tag: Vec<u8> = match encryption {
    JweEncryption::A128CBC_HS256 => {
      __encrypt_content::<Aes128CbcHmac256>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec()
    }
    JweEncryption::A192CBC_HS384 => {
      __encrypt_content::<Aes192CbcHmac384>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec()
    }
    JweEncryption::A256CBC_HS512 => {
      __encrypt_content::<Aes256CbcHmac512>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec()
    }
    JweEncryption::A128GCM => __encrypt_content::<Aes128Gcm>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec(),
    JweEncryption::A192GCM => __encrypt_content::<Aes192Gcm>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec(),
    JweEncryption::A256GCM => __encrypt_content::<Aes256Gcm>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec(),
    JweEncryption::C20P => __encrypt_content::<ChaCha20Poly1305>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec(),
    JweEncryption::XC20P => __encrypt_content::<XChaCha20Poly1305>(key, iv, aad, plaintext, &mut ciphertext)?.to_vec(),
  };

  Ok((ciphertext, tag))
}

fn __encrypt_content<A>(key: &[u8], iv: &[u8], aad: &[u8], plaintext: &[u8], ciphertext: &mut Vec<u8>) -> Result<Tag<A>>
where
  A: Aead,
{
  let padding: usize = A::padsize(plaintext).map(NonZeroUsize::get).unwrap_or_default();

  ciphertext.resize(plaintext.len() + padding, 0);

  let mut tag: Tag<A> = Default::default();

  A::try_encrypt(key, iv, aad, plaintext, ciphertext, &mut tag)?;

  Ok(tag)
}

struct EcdhDeriver<'a, 'b>(&'b Encoder<'a>, &'b Recipient<'a>);

impl<'a, 'b> EcdhDeriver<'a, 'b> {
  fn new(encoder: &'b Encoder<'a>, recipient: &'b Recipient<'a>) -> Self {
    Self(encoder, recipient)
  }

  fn derive_ecdh_es(&self, output: &mut JweHeader, algorithm: &str, key_len: usize) -> Result<Vec<u8>> {
    self.derive_ecdh_key(output, algorithm, key_len, |eph_secret| {
      diffie_hellman(self.1.ecdh_curve, self.1.public, eph_secret)
    })
  }

  fn derive_ecdh_1pu(&self, output: &mut JweHeader, algorithm: &str, key_len: usize) -> Result<Vec<u8>> {
    self.derive_ecdh_key(output, algorithm, key_len, |eph_secret| {
      let secret: Secret<'_> = self.0.secret.ok_or(Error::EncError("Missing ECDH-1PU Secret Key"))?;
      let ze: Vec<u8> = diffie_hellman(self.1.ecdh_curve, self.1.public, eph_secret)?;
      let zs: Vec<u8> = diffie_hellman(self.1.ecdh_curve, self.1.public, secret)?;

      Ok([ze, zs].concat())
    })
  }

  fn derive_ecdh_key(
    &self,
    output: &mut JweHeader,
    algorithm: &str,
    key_len: usize,
    key_exchange: impl Fn(&Jwk) -> Result<Vec<u8>>,
  ) -> Result<Vec<u8>> {
    let __apu: Vec<u8>;
    let __apv: Vec<u8>;

    let apu: &[u8] = match output.apu() {
      Some(value) => {
        __apu = decode_b64(value)?;
        &__apu
      }
      None => match self.0.apu {
        Some(value) => {
          output.set_apu(encode_b64(value));
          value
        }
        None => &[],
      },
    };

    let apv: &[u8] = match output.apv() {
      Some(value) => {
        __apv = decode_b64(value)?;
        &__apv
      }
      None => match self.0.apv {
        Some(value) => {
          output.set_apv(encode_b64(value));
          value
        }
        None => &[],
      },
    };

    // Generate an ephemeral key pair
    let eph_secret: Jwk = Jwk::random(self.1.ecdh_curve)?;
    let eph_public: Jwk = eph_secret.to_public();

    // Compute the shared secret
    let z: Vec<u8> = key_exchange(&eph_secret)?;

    // Set the ephemeral public key claim
    output.set_epk(eph_public);

    // Concat KDF
    concat_kdf(algorithm, key_len, &z, apu, apv)
  }
}
