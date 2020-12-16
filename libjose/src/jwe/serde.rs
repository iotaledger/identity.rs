use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::str;
use serde::de::DeserializeOwned;
use serde::de::Error as _;
use serde::Serialize;
use serde_json::from_slice;
use serde_json::from_str;
use serde_json::from_value;
use serde_json::to_string;
use serde_json::to_value;
use serde_json::to_vec;
use serde_json::Map;
use serde_json::Value;

use crate::error::Error;
use crate::error::Result;
use crate::jwe::JweAlgorithm;
use crate::jwe::JweDecrypter;
use crate::jwe::JweEncrypter;
use crate::jwe::JweEncryption;
use crate::jwe::JweHeader;
use crate::jwt::JwtClaims;
use crate::lib::*;
use crate::utils::decode_b64;
use crate::utils::encode_b64;
use crate::utils::is_b64;

const PARAM_ALG: &str = "alg";
const PARAM_ENC: &str = "enc";

type Object = Map<String, Value>;

// TODO: FIXME
pub struct OsRng;

// TODO: FIXME
pub fn random_bytes(_: usize, _: OsRng) -> Result<Vec<u8>> {
  todo!("random_bytes")
}

// =============================================================================
// JWE Encoding Format
// =============================================================================

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JweFormat {
  Compact,
  General,
  Flatten,
}

impl Default for JweFormat {
  fn default() -> Self {
    Self::Compact
  }
}

// =============================================================================
// JWE Encoded Data
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JweEncodedRecipient {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub header: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub encrypted_key: Option<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JweEncodedBase {
  // The spec says this is optional but we don't support unprotected headers so
  // we require it.
  pub protected: String,
  pub ciphertext: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aad: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub iv: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tag: Option<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JweEncodedGeneral {
  #[serde(flatten)]
  pub encrypted: JweEncodedBase,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub recipients: Vec<JweEncodedRecipient>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct JweEncodedFlatten {
  #[serde(flatten)]
  pub encrypted: JweEncodedBase,
  #[serde(flatten)]
  pub recipient: JweEncodedRecipient,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JweEncoded {
  Compact(String),
  General(JweEncodedGeneral),
  Flatten(JweEncodedFlatten),
}

impl JweEncoded {
  pub fn to_string(&self) -> Result<String> {
    match self {
      Self::Compact(inner) => Ok(inner.clone()),
      Self::General(inner) => to_string(&inner).map_err(Into::into),
      Self::Flatten(inner) => to_string(&inner).map_err(Into::into),
    }
  }
}

impl Display for JweEncoded {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.to_string().map_err(|_| FmtError)?)
  }
}

// =============================================================================
// JWE Encoder
// =============================================================================

#[derive(Clone, Debug, Default)]
pub struct JweEncoder {
  /// The serialization format of the encoded token.
  format: JweFormat,
}

impl JweEncoder {
  /// Creates a new `JweEncoder` with the default format (Compact).
  pub const fn new() -> Self {
    Self {
      format: JweFormat::Compact,
    }
  }

  /// Creates a new `JweEncoder` with the given `format`.
  pub const fn with_format(format: JweFormat) -> Self {
    Self { format }
  }

  pub fn format(mut self, value: impl Into<JweFormat>) -> Self {
    self.format = value.into();
    self
  }

  pub fn encode<T>(
    &self,
    claims: &JwtClaims<T>,
    header: &JweHeader,
    recipients: &[(Option<&JweHeader>, &dyn JweEncrypter)],
    aad: Option<&[u8]>,
  ) -> Result<JweEncoded>
  where
    T: Serialize,
  {
    self.encode_slice(&to_vec(claims)?, header, recipients, aad)
  }

  pub fn encode_slice<T>(
    &self,
    claims: &T,
    header: &JweHeader,
    recipients: &[(Option<&JweHeader>, &dyn JweEncrypter)],
    aad: Option<&[u8]>,
  ) -> Result<JweEncoded>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    match (self.format, recipients, aad) {
      (JweFormat::Compact, &[(None, encrypter)], None) => self
        .encode_slice_compact(claims, header, encrypter)
        .map(JweEncoded::Compact),
      (JweFormat::General, recipients, aad) => self
        .encode_slice_general(claims, header, recipients, aad)
        .map(JweEncoded::General),
      (JweFormat::Flatten, &[(recipient, encrypter)], aad) => self
        .encode_slice_flatten(claims, header, recipient, encrypter, aad)
        .map(JweEncoded::Flatten),
      _ => Err(Error::InvalidContent("Invalid JWE Configuration")),
    }
  }

  pub fn encode_slice_compact<T>(
    &self,
    _claims: &T,
    _header: &JweHeader,
    _encrypter: &dyn JweEncrypter,
  ) -> Result<String>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    todo!("encode_slice_compact")
  }

  pub fn encode_slice_general<T>(
    &self,
    claims: &T,
    header: &JweHeader,
    recipients: &[(Option<&JweHeader>, &dyn JweEncrypter)],
    aad: Option<&[u8]>,
  ) -> Result<JweEncodedGeneral>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    let claims: &[u8] = claims.as_ref();

    let alg: JweAlgorithm = header.alg();
    let enc: JweEncryption = header.enc();

    let mut cek: Option<Cow<[u8]>> = None;
    let mut scopes: Vec<(JweHeader, JweHeader, &dyn JweEncrypter)> = Vec::new();

    for (recipient, encrypter) in recipients {
      if encrypter.alg() != alg {
        return Err(Error::InvalidContent("Invalid Encrypter Algorithm"));
      }

      let header: JweHeader = if let Some(recipient) = recipient {
        merge_header(header, recipient, &[PARAM_ALG, PARAM_ENC])?
      } else {
        header.clone()
      };

      let mut out: JweHeader = if let Some(recipient) = recipient {
        (*recipient).clone()
      } else {
        JweHeader::new(alg, enc)
      };

      if let Some(key) = encrypter.cek(enc, &header, &mut out)? {
        if let Some(cek) = cek.as_ref() {
          if cek.as_ref() != key.as_ref() {
            return Err(Error::InvalidContent("Invalid Content Encryption Key"));
          }
        } else {
          cek = Some(key);
        }
      }

      if header.kid().is_none() {
        if let Some(kid) = encrypter.kid() {
          out.set_kid(kid);
        }
      }

      scopes.push((header, out, *encrypter));
    }

    let key: Cow<[u8]> = unwrap_cek(enc, cek)?;
    let iv: Option<Vec<u8>> = generate_iv(enc)?;
    let iv: Option<&[u8]> = iv.as_deref();

    let payload: Cow<[u8]> = if let Some(zip) = header.zip() {
      Cow::Owned(zip.compress(claims)?)
    } else {
      Cow::Borrowed(claims)
    };

    let protected: String = encode_b64(&to_vec(header)?);
    let aad: Option<String> = aad.map(encode_b64);
    let aad_vec: Vec<u8> = create_aad_bytes(&protected, aad.as_deref());

    let (ciphertext, tag): (Vec<u8>, Option<Vec<u8>>) =
      enc.encrypt(&payload, &key, iv.unwrap_or_default(), &aad_vec)?;

    let mut recipients: Vec<JweEncodedRecipient> = Vec::with_capacity(scopes.len());

    for (ref header, ref mut out, encrypter) in scopes {
      let encrypted_key: Option<Vec<u8>> = encrypter.encrypt(&key, header, out)?;

      recipients.push(JweEncodedRecipient {
        header: encode_recipient_header(out)?,
        encrypted_key: encrypted_key.map(encode_b64),
      });
    }

    Ok(JweEncodedGeneral {
      encrypted: JweEncodedBase {
        protected,
        ciphertext: encode_b64(ciphertext),
        aad,
        iv: iv.map(encode_b64),
        tag: tag.map(encode_b64),
      },
      recipients,
    })
  }

  pub fn encode_slice_flatten<T>(
    &self,
    _claims: &T,
    _header: &JweHeader,
    _recipient: Option<&JweHeader>,
    _encrypter: &dyn JweEncrypter,
    _aad: Option<&[u8]>,
  ) -> Result<JweEncodedFlatten>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    todo!("encode_slice_flatten")
  }
}

// =============================================================================
// JWE Decoder
// =============================================================================

#[derive(Clone, Debug, Default)]
pub struct JweDecoder {
  /// The serialization format of the encoded data
  format: JweFormat,
  /// A set of acceptable algorithms
  ///
  /// [More Info](https://tools.ietf.org/html/rfc8725#section-3.1)
  algs: BTreeSet<JweAlgorithm>,
}

impl JweDecoder {
  /// Creates a new `JweDecoder` with the default format (Compact).
  pub fn new() -> Self {
    Self::with_format(JweFormat::Compact)
  }

  /// Creates a new `JweDecoder` with the given `format`.
  pub fn with_format(format: JweFormat) -> Self {
    Self {
      format,
      algs: BTreeSet::new(),
    }
  }

  pub fn with_algs(algs: impl IntoIterator<Item = impl Into<JweAlgorithm>>) -> Self {
    Self {
      format: JweFormat::Compact,
      algs: BTreeSet::from_iter(algs.into_iter().map(Into::into)),
    }
  }

  pub fn format(mut self, value: impl Into<JweFormat>) -> Self {
    self.format = value.into();
    self
  }

  pub fn alg(mut self, value: impl Into<JweAlgorithm>) -> Self {
    self.algs.insert(value.into());
    self
  }

  pub fn decode<T, U>(&self, data: &T, decrypter: &dyn JweDecrypter) -> Result<(JweHeader, U)>
  where
    T: AsRef<[u8]> + ?Sized,
    U: DeserializeOwned,
  {
    let (header, claims): _ = self.decode_token(data, decrypter)?;
    let claims: U = from_slice(&claims)?;

    Ok((header, claims))
  }

  pub fn decode_token<T>(
    &self,
    data: &T,
    decrypter: &dyn JweDecrypter,
  ) -> Result<(JweHeader, Vec<u8>)>
  where
    T: AsRef<[u8]> + ?Sized,
  {
    let segments: Segments = match self.format {
      JweFormat::Compact => extract_compact(data.as_ref())?,
      JweFormat::General => extract_general(data.as_ref())?,
      JweFormat::Flatten => extract_flatten(data.as_ref())?,
    };

    let Segments { protected, .. } = segments;

    for (header, encrypted_key) in segments.recipients {
      let header: JweHeader = header
        .as_deref()
        .map(from_str)
        .transpose()?
        .map(|recipient| merge_object(&protected, recipient, &[PARAM_ALG, PARAM_ENC]))
        .transpose()?
        .unwrap_or_else(|| protected.clone());

      if decrypter.alg() != header.alg() {
        continue;
      }

      if decrypter.kid() != header.kid() {
        continue;
      }

      if header.kid().is_none() {
        return Err(Error::MissingClaim("kid"));
      }

      let enc: JweEncryption = header.enc();
      let aad: Vec<u8> = create_aad_bytes(&segments.protected_b64, segments.aad.as_deref());

      let key: Option<&[u8]> = encrypted_key.as_deref();
      let key: Cow<[u8]> = decrypter.decrypt(key, enc, &header)?;

      if key.len() != enc.key_len() {
        return Err(Error::InvalidContent("Invalid Key Length"));
      }

      let iv: &[u8] = segments.iv.as_deref().unwrap_or_default();
      let tag: Option<&[u8]> = segments.tag.as_deref();
      let plaintext: Vec<u8> = enc.decrypt(&segments.ciphertext, &key, iv, &aad, tag)?;

      let claims: Vec<u8> = if let Some(zip) = protected.zip() {
        zip.decompress(&plaintext)?
      } else {
        plaintext
      };

      return Ok((header, claims));
    }

    Err(Error::InvalidContent("Recipient Not Found"))
  }
}

// =============================================================================
// Misc Helpers
// =============================================================================

fn unwrap_cek(enc: JweEncryption, key: Option<Cow<[u8]>>) -> Result<Cow<[u8]>> {
  if let Some(key) = key {
    Ok(key)
  } else {
    random_bytes(enc.key_len(), OsRng).map(Cow::Owned)
  }
}

fn generate_iv(enc: JweEncryption) -> Result<Option<Vec<u8>>> {
  if enc.iv_len() > 0 {
    random_bytes(enc.iv_len(), OsRng).map(Some)
  } else {
    Ok(None)
  }
}

fn encode_recipient_header(header: &JweHeader) -> Result<Option<String>> {
  let mut header: Object = to_object(header)?;

  // These are inherited from the parent protected header.
  //
  // We always enforce the inclusion of a protected header so these MUST
  // be removed (duplicate claims are not allowed).
  header.remove(PARAM_ALG);
  header.remove(PARAM_ENC);

  if header.is_empty() {
    Ok(None)
  } else {
    to_string(&header).map_err(Into::into).map(Some)
  }
}

fn create_aad_bytes(header: &str, aad: Option<&str>) -> Vec<u8> {
  let aad: &str = aad.unwrap_or_default();
  let len: usize = header.len() + aad.len() + 1;

  let mut output: Vec<u8> = Vec::with_capacity(len);

  output.extend_from_slice(header.as_bytes());
  output.push(b'.');
  output.extend_from_slice(aad.as_bytes());
  output
}

fn merge_header(a: &JweHeader, b: &JweHeader, ignore: &[&str]) -> Result<JweHeader> {
  merge_object(a, to_object(b)?, ignore)
}

fn merge_object(a: &JweHeader, b: Object, ignore: &[&str]) -> Result<JweHeader> {
  let mut c: Object = to_object(a)?;

  for (key, val) in b {
    if ignore.contains(&&*key) {
      continue;
    }

    if c.contains_key(&key) {
      return Err(Error::InvalidContent("Duplicate Header Key(s)"));
    }

    c.insert(key, val);
  }

  to_header(c)
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

fn to_header(object: Object) -> Result<JweHeader> {
  from_value(Value::Object(object)).map_err(Into::into)
}

// =============================================================================
// Decoding Helper
// =============================================================================

#[derive(Debug)]
struct Segments<'a> {
  protected: JweHeader,
  protected_b64: Cow<'a, str>,
  ciphertext: Vec<u8>,
  recipients: Vec<(Option<String>, Option<Vec<u8>>)>,
  iv: Option<Vec<u8>>,
  tag: Option<Vec<u8>>,
  aad: Option<String>,
}

impl Segments<'_> {
  const COUNT: usize = 5;
}

fn extract_compact(data: &[u8]) -> Result<Segments> {
  // Split the token into individual segments
  let split: Vec<&[u8]> = data.split(|byte| *byte == b'.').collect();

  // Ensure the token has the expected number of segments
  if split.len() != Segments::COUNT {
    return Err(Error::InvalidContent("Invalid Segments"));
  }

  let protected_b64: &str =
    str::from_utf8(&split[0]).map_err(|_| Error::InvalidContent("Invalid UTF-8"))?;

  let iv: Option<Vec<u8>> = if split[2].is_empty() {
    None
  } else {
    decode_b64(split[2]).map(Some)?
  };

  let tag: Option<Vec<u8>> = if split[4].is_empty() {
    None
  } else {
    decode_b64(split[4]).map(Some)?
  };

  let recipients: Vec<_> = if split[1].is_empty() {
    vec![(None, None)]
  } else {
    vec![(None, decode_b64(&split[1]).map(Some)?)]
  };

  Ok(Segments {
    protected: extract_header(protected_b64)?,
    protected_b64: Cow::Borrowed(protected_b64),
    ciphertext: extract_ciphertext(split[3])?,
    recipients,
    iv,
    tag,
    aad: None,
  })
}

fn extract_general(data: &[u8]) -> Result<Segments> {
  let data: JweEncodedGeneral = from_slice(data)?;

  let recipients: Vec<_> = {
    let mut recipients: Vec<_> = Vec::with_capacity(data.recipients.len());

    for recipient in data.recipients {
      let encrypted_key: Option<Vec<u8>> = recipient
        .encrypted_key
        .map(extract_encrypted_key)
        .transpose()?;

      recipients.push((recipient.header, encrypted_key));
    }

    recipients
  };

  Ok(Segments {
    protected: extract_header(&data.encrypted.protected)?,
    protected_b64: Cow::Owned(data.encrypted.protected),
    ciphertext: extract_ciphertext(data.encrypted.ciphertext)?,
    recipients,
    iv: data.encrypted.iv.map(extract_iv).transpose()?,
    tag: data.encrypted.tag.map(extract_tag).transpose()?,
    aad: data.encrypted.aad.map(extract_aad).transpose()?,
  })
}

fn extract_flatten(data: &[u8]) -> Result<Segments> {
  let data: JweEncodedFlatten = from_slice(data)?;

  let encrypted_key: Option<Vec<u8>> = data
    .recipient
    .encrypted_key
    .map(extract_encrypted_key)
    .transpose()?;

  Ok(Segments {
    protected: extract_header(&data.encrypted.protected)?,
    protected_b64: Cow::Owned(data.encrypted.protected),
    ciphertext: extract_ciphertext(data.encrypted.ciphertext)?,
    recipients: vec![(data.recipient.header, encrypted_key)],
    iv: data.encrypted.iv.map(extract_iv).transpose()?,
    tag: data.encrypted.tag.map(extract_tag).transpose()?,
    aad: data.encrypted.aad.map(extract_aad).transpose()?,
  })
}

fn extract_header(data: impl AsRef<[u8]>) -> Result<JweHeader> {
  let data: &[u8] = data.as_ref();

  if data.is_empty() {
    Err(Error::InvalidContent("Invalid Header (empty)"))
  } else {
    from_slice(&decode_b64(data)?).map_err(Into::into)
  }
}

fn extract_ciphertext(data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  let data: &[u8] = data.as_ref();

  if data.is_empty() {
    Err(Error::InvalidContent("Invalid Ciphertext (empty)"))
  } else {
    decode_b64(data)
  }
}

fn extract_aad(data: String) -> Result<String> {
  if data.is_empty() {
    Err(Error::InvalidContent("Invalid AAD (empty)"))
  } else if !is_b64(&data) {
    Err(Error::InvalidContent("Invalid AAD (base64)"))
  } else {
    Ok(data)
  }
}

fn extract_iv(data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  let data: &[u8] = data.as_ref();

  if data.is_empty() {
    Err(Error::InvalidContent("Invalid IV (empty)"))
  } else {
    decode_b64(data)
  }
}

fn extract_tag(data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  let data: &[u8] = data.as_ref();

  if data.is_empty() {
    Err(Error::InvalidContent("Invalid Tag (empty)"))
  } else {
    decode_b64(data)
  }
}

fn extract_encrypted_key(data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
  let data: &[u8] = data.as_ref();

  if data.is_empty() {
    Err(Error::InvalidContent("Invalid Encrypted Key (empty)"))
  } else {
    decode_b64(data)
  }
}
