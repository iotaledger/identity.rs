use core::convert::TryFrom;
use crypto::ciphers::aes::AES_128_CBC_HMAC_SHA_256 as A128CBC_HS256;
use crypto::ciphers::aes::AES_128_GCM as A128GCM;
use crypto::ciphers::aes::AES_192_CBC_HMAC_SHA_384 as A192CBC_HS384;
use crypto::ciphers::aes::AES_192_GCM as A192GCM;
use crypto::ciphers::aes::AES_256_CBC_HMAC_SHA_512 as A256CBC_HS512;
use crypto::ciphers::aes::AES_256_GCM as A256GCM;
use crypto::ciphers::chacha::CHACHA20_POLY1305 as C20P;
use crypto::ciphers::chacha::XCHACHA20_POLY1305 as XC20P;
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
struct Recipient<'a> {
  header: Option<JweHeader>,
  encrypted_key: Option<&'a str>,
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
  recipients: Vec<Recipient<'a>>,
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
  recipient: Recipient<'a>,
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
    self.expand(data, |expanded| {
      let protected: Option<JweHeader> = expanded.protected.map(decode_b64_json).transpose()?;
      let unprotected: Option<JweHeader> = expanded.unprotected.map(decode_b64_json).transpose()?;

      let protected_ref: Option<&JweHeader> = protected.as_ref();
      let unprotected_ref: Option<&JweHeader> = unprotected.as_ref();

      validate_jwe_headers(
        protected_ref,
        unprotected_ref,
        expanded.recipients.iter().map(|recipient| recipient.header.as_ref()),
        self.crits.as_deref(),
      )?;

      let cek: Option<(Cow<[u8]>, Option<JweHeader>)> = expanded
        .recipients
        .into_iter()
        .find_map(|recipient| self.expand_recipient(protected_ref, unprotected_ref, recipient).ok());

      if let Some((cek, header)) = cek {
        let merged: HeaderSet = HeaderSet::new()
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
    })
  }

  fn expand_recipient(
    &self,
    protected: Option<&JweHeader>,
    unprotected: Option<&JweHeader>,
    mut recipient: Recipient<'_>,
  ) -> Result<(Cek<'a>, Option<JweHeader>)> {
    let header: Option<JweHeader> = recipient.header.take();

    let merged: HeaderSet = HeaderSet::new()
      .header(header.as_ref())
      .protected(protected)
      .unprotected(unprotected);

    Ok((self.decrypt_cek(merged, recipient)?, header))
  }

  fn decrypt_cek(&self, header: HeaderSet<'_>, recipient: Recipient<'_>) -> Result<Cek<'a>> {
    let alg: JweAlgorithm = header.try_alg()?;
    let enc: JweEncryption = header.try_enc()?;

    self.check_alg(alg)?;
    self.check_enc(enc)?;
    self.check_kid(header.kid())?;

    let cek: Cow<[u8]> = self.decrypt_key(alg, enc, header, recipient)?;

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
      Recipient {
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
    recipient: Recipient<'_>,
  ) -> Result<Cek<'a>> {
    macro_rules! rsa {
      ($padding:ident, $recipient:expr, $secret:expr) => {{
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: _ = $secret.to_rsa_secret()?;
        let ptx: _ = key.decrypt($crate::rsa_padding!(@$padding), &ctx)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! aead {
      ($impl:ident, $header:expr, $recipient:expr, $secret:expr) => {{
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: Cow<[u8]> = $secret.to_oct_key($impl::KEY_LENGTH)?;
        let iv: Vec<u8> = $header.try_iv().and_then(decode_b64)?;
        let tag: Vec<u8> = $header.try_tag().and_then(decode_b64)?;
        let ptx: Vec<u8> = $impl::decrypt_vec(&key, &iv, &[], &tag, &ctx)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! pbes2 {
      (($impl:ident, $size:ident), $wrap:ident, $header:expr, $recipient:expr, $secret:expr) => {{
        let name: &str = $header.try_alg()?.name();
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: Cow<[u8]> = $secret.to_oct_key(0)?;
        let p2s: Vec<u8> = $header.try_p2s().and_then(decode_b64)?;
        let p2c: usize = usize::try_from($header.try_p2c()?).map_err(|_| Error::InvalidClaim("p2c"))?;
        let salt: Vec<u8> = create_pbes2_salt(name, &p2s);

        let mut derived: [u8; ::crypto::hashes::sha::$size / 2] = [0; ::crypto::hashes::sha::$size / 2];

        ::crypto::kdfs::pbkdf::$impl(&key, &salt, p2c, &mut derived)?;

        let ptx: Vec<u8> = ::crypto::aes_kw::$wrap::new(&derived).unwrap_key_vec(&ctx)?;

        Ok(Cow::Owned(ptx))
      }};
    }

    macro_rules! aes_kw {
      ($impl:ident, $recipient:expr, $secret:expr) => {{
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let key: Cow<[u8]> = $secret.to_oct_key(::crypto::aes_kw::$impl::key_length())?;
        let ptx: Vec<u8> = ::crypto::aes_kw::$impl::new(&key).unwrap_key_vec(&ctx)?;

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
        let name: &str = $header.try_alg()?.name();
        let size: usize = $header.try_alg()?.try_key_len()?;
        let shared: Vec<u8> = EcdhDeriver::new($this).$derive(&$header, name, size)?;
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let ptx: Vec<u8> = ::crypto::aes_kw::$wrap::new(&shared).unwrap_key_vec(&ctx)?;

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
        let name: &str = $header.try_alg()?.name();
        let size: usize = $header.try_alg()?.try_key_len()?;
        let shared: Vec<u8> = EcdhDeriver::new($this).$derive(&header, name, size)?;
        let ctx: Vec<u8> = parse_cek($recipient.encrypted_key)?;
        let iv: Vec<u8> = $header.try_iv().and_then(decode_b64)?;
        let tag: Vec<u8> = $header.try_tag().and_then(decode_b64)?;
        let ptx: Vec<u8> = $wrap::decrypt_vec(&shared, &iv, &[], &tag, &ctx)?;

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
      JweAlgorithm::ECDH_ES_C20PKW => ecdh_chacha!(@es, C20P, header, recipient, self),
      JweAlgorithm::ECDH_ES_XC20PKW => ecdh_chacha!(@es, XC20P, header, recipient, self),
      JweAlgorithm::A128GCMKW => aead!(A128GCM, header, recipient, self.secret),
      JweAlgorithm::A192GCMKW => aead!(A192GCM, header, recipient, self.secret),
      JweAlgorithm::A256GCMKW => aead!(A256GCM, header, recipient, self.secret),
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
      JweAlgorithm::C20PKW => aead!(C20P, header, recipient, self.secret),
      JweAlgorithm::XC20PKW => aead!(XC20P, header, recipient, self.secret),
    }
  }

  fn expand<T>(&self, data: &[u8], format: impl Fn(Expanded<'_>) -> Result<T>) -> Result<T> {
    match self.format {
      JweFormat::Compact => {
        let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

        if split.len() != COMPACT_SEGMENTS {
          return Err(Error::InvalidContent("Invalid Segments"));
        }

        format(Expanded {
          protected: filter_non_empty_bytes(split[0]),
          unprotected: None,
          recipients: vec![Recipient {
            header: None,
            encrypted_key: filter_non_empty_bytes(split[1]).map(parse_utf8).transpose()?,
          }],
          iv: filter_non_empty_bytes(split[2]),
          aad: None,
          ciphertext: split[3],
          tag: filter_non_empty_bytes(split[4]),
        })
      }
      JweFormat::General => {
        let data: General = from_slice(data)?;

        format(Expanded {
          protected: filter_non_empty_bytes(data.protected),
          unprotected: filter_non_empty_bytes(data.unprotected),
          recipients: data.recipients,
          iv: filter_non_empty_bytes(data.iv),
          aad: filter_non_empty_bytes(data.aad),
          ciphertext: data.ciphertext.as_bytes(),
          tag: filter_non_empty_bytes(data.tag),
        })
      }
      JweFormat::Flatten => {
        let data: Flatten = from_slice(data)?;

        format(Expanded {
          protected: filter_non_empty_bytes(data.protected),
          unprotected: filter_non_empty_bytes(data.unprotected),
          recipients: vec![data.recipient],
          iv: filter_non_empty_bytes(data.iv),
          aad: filter_non_empty_bytes(data.aad),
          ciphertext: data.ciphertext.as_bytes(),
          tag: filter_non_empty_bytes(data.tag),
        })
      }
    }
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
struct Expanded<'a> {
  protected: Option<&'a [u8]>,
  unprotected: Option<&'a [u8]>,
  recipients: Vec<Recipient<'a>>,
  iv: Option<&'a [u8]>,
  aad: Option<&'a [u8]>,
  ciphertext: &'a [u8],
  tag: Option<&'a [u8]>,
}

fn decrypt_content(
  encryption: JweEncryption,
  key: &[u8],
  iv: &[u8],
  aad: &[u8],
  tag: &[u8],
  ciphertext: &[u8],
) -> Result<Vec<u8>> {
  match encryption {
    JweEncryption::A128CBC_HS256 => A128CBC_HS256::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
    JweEncryption::A192CBC_HS384 => A192CBC_HS384::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
    JweEncryption::A256CBC_HS512 => A256CBC_HS512::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
    JweEncryption::A128GCM => A128GCM::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
    JweEncryption::A192GCM => A192GCM::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
    JweEncryption::A256GCM => A256GCM::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
    JweEncryption::C20P => C20P::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
    JweEncryption::XC20P => XC20P::decrypt_vec(key, iv, aad, tag, ciphertext).map_err(Into::into),
  }
}

struct EcdhDeriver<'a: 'b, 'b>(&'b Decoder<'a>);

impl<'a: 'b, 'b> EcdhDeriver<'a, 'b> {
  fn new(decoder: &'b Decoder<'a>) -> Self {
    Self(decoder)
  }

  fn derive_ecdh_es(&self, header: &HeaderSet, algorithm: &str, key_len: usize) -> Result<Vec<u8>> {
    self.derive_ecdh_key(header, algorithm, key_len, |epk| {
      diffie_hellman(self.0.ecdh_curve, epk, self.0.secret)
    })
  }

  fn derive_ecdh_1pu(&self, header: &HeaderSet, algorithm: &str, key_len: usize) -> Result<Vec<u8>> {
    self.derive_ecdh_key(header, algorithm, key_len, |epk| {
      let public: Secret<'a> = self.0.public.ok_or(Error::EncError("Missing ECDH-1PU Public Key"))?;
      let ze: Vec<u8> = diffie_hellman(self.0.ecdh_curve, epk, self.0.secret)?;
      let zs: Vec<u8> = diffie_hellman(self.0.ecdh_curve, public, self.0.secret)?;

      Ok([ze, zs].concat())
    })
  }

  fn derive_ecdh_key(
    &self,
    header: &HeaderSet,
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
