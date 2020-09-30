//! # pem
//!
//! PEM encoding and decoding.
//!
//! ### RFC
//! [Textual Encodings of PKIX, PKCS, and CMS Structures](https://tools.ietf.org/html/rfc7468)
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::SkipWhile;
use core::str::from_utf8;
use core::str::Lines;

use crate::error::PemError;
use crate::error::Result;

const LINE: char = '\n';
const WRAP: usize = 64;

const HEADER_PREFIX: &str = "-----BEGIN ";
const HEADER_SUFFIX: &str = "-----";
const FOOTER_PREFIX: &str = "-----END ";
const FOOTER_SUFFIX: &str = "-----";

/// A PEM-encoded document.
///
/// [More Info](https://tools.ietf.org/html/rfc7468)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pem {
  /// The binary content of the PEM-encoded document.
  pub pem_data: Vec<u8>,
  /// The extracted tag of the PEM-encoded document.
  pub pem_type: String,
}

/// Returns a PEM-encoded String of the encoded document.
///
/// [More Info](https://tools.ietf.org/html/rfc7468#section-2)
pub fn pem_encode(data: &Pem) -> Result<String> {
  let mut output: String = String::new();

  output.push_str(HEADER_PREFIX);
  output.push_str(data.pem_type.as_str());
  output.push_str(HEADER_SUFFIX);
  output.push(LINE);

  let content: String = if data.pem_data.is_empty() {
    String::new()
  } else {
    base64::encode(&data.pem_data)
  };

  for chunk in content.into_bytes().chunks(WRAP) {
    output.push_str(to_utf8(chunk)?);
    output.push(LINE);
  }

  output.push_str(FOOTER_PREFIX);
  output.push_str(data.pem_type.as_str());
  output.push_str(FOOTER_SUFFIX);
  output.push(LINE);

  Ok(output)
}

/// Decodes a PEM-encoded document.
///
/// [More Info](https://tools.ietf.org/html/rfc7468#section-2)
pub fn pem_decode(data: &(impl AsRef<[u8]> + ?Sized)) -> Result<Pem> {
  let data: &str = to_utf8(data)?;

  let mut lines: Lines = data.lines();

  // Skip any explanatory text that MAY exist.
  let mut byref: SkipWhile<&mut Lines, _> = lines
    .by_ref()
    .skip_while(|line| !line.starts_with(HEADER_PREFIX));

  let header: &str = parse_header(&mut byref)?;
  let footer: &str = parse_footer(&mut lines)?;

  if header != footer {
    return Err(PemError::InvalidHeaderFooter.into());
  }

  let pem_data: Vec<u8> = parse_content(&mut lines)?;

  debug_assert!(lines.count() == 0, "Additional Lines");

  Ok(Pem {
    pem_data,
    pem_type: header.into(),
  })
}

fn to_utf8(data: &(impl AsRef<[u8]> + ?Sized)) -> Result<&str, PemError> {
  from_utf8(data.as_ref()).map(str::trim).map_err(Into::into)
}

fn parse_content(iter: &mut dyn Iterator<Item = &str>) -> Result<Vec<u8>, PemError> {
  let mut content: Vec<u8> = Vec::new();

  for line in iter {
    content.extend_from_slice(line.trim_end().as_bytes());
  }

  base64::decode(&content).map_err(|_| PemError::InvalidContent)
}

fn parse_header<'a>(iter: &mut dyn Iterator<Item = &'a str>) -> Result<&'a str, PemError> {
  iter
    .next()
    .ok_or(PemError::InvalidHeader)?
    .strip_prefix(HEADER_PREFIX)
    .ok_or(PemError::InvalidHeader)?
    .strip_suffix(HEADER_SUFFIX)
    .ok_or(PemError::InvalidHeader)
}

fn parse_footer<'a>(
  iter: &mut dyn DoubleEndedIterator<Item = &'a str>,
) -> Result<&'a str, PemError> {
  iter
    .next_back()
    .ok_or(PemError::InvalidFooter)?
    .strip_prefix(FOOTER_PREFIX)
    .ok_or(PemError::InvalidFooter)?
    .strip_suffix(FOOTER_SUFFIX)
    .ok_or(PemError::InvalidFooter)
}

#[cfg(test)]
mod tests {
  use super::pem_decode as decode;
  use super::pem_encode as encode;

  // https://tools.ietf.org/html/rfc7468#section-5.1
  const PEM_5_1: &str = include_str!("../../tests/fixtures/pem-examples/5_1.pem");
  // https://tools.ietf.org/html/rfc7468#section-5.2
  const PEM_5_2: &str = include_str!("../../tests/fixtures/pem-examples/5_2.pem");
  // https://tools.ietf.org/html/rfc7468#section-6
  const PEM_6: &str = include_str!("../../tests/fixtures/pem-examples/6.pem");
  // https://tools.ietf.org/html/rfc7468#section-7
  const PEM_7: &str = include_str!("../../tests/fixtures/pem-examples/7.pem");
  // https://tools.ietf.org/html/rfc7468#section-8
  const PEM_8: &str = include_str!("../../tests/fixtures/pem-examples/8.pem");
  // https://tools.ietf.org/html/rfc7468#section-9
  const PEM_9: &str = include_str!("../../tests/fixtures/pem-examples/9.pem");
  // https://tools.ietf.org/html/rfc7468#section-10
  const PEM_10: &str = include_str!("../../tests/fixtures/pem-examples/10.pem");
  // https://tools.ietf.org/html/rfc7468#section-11
  const PEM_11: &str = include_str!("../../tests/fixtures/pem-examples/11.pem");
  // https://tools.ietf.org/html/rfc7468#section-12
  const PEM_12: &str = include_str!("../../tests/fixtures/pem-examples/12.pem");
  // https://tools.ietf.org/html/rfc7468#section-13
  const PEM_13: &str = include_str!("../../tests/fixtures/pem-examples/13.pem");

  const DOCUMENTS: &[(&str, &str, bool)] = &[
    ("CERTIFICATE", PEM_5_1, true),
    ("CERTIFICATE", PEM_5_2, false), // Skip roundtrip test because we don't save comments
    ("X509 CRL", PEM_6, true),
    ("CERTIFICATE REQUEST", PEM_7, true),
    ("PKCS7", PEM_8, true),
    ("CMS", PEM_9, true),
    ("PRIVATE KEY", PEM_10, true),
    ("ENCRYPTED PRIVATE KEY", PEM_11, true),
    ("ATTRIBUTE CERTIFICATE", PEM_12, true),
    ("PUBLIC KEY", PEM_13, true),
  ];

  #[test]
  fn test_decode() {
    for (type_, data, _) in DOCUMENTS {
      assert!(decode(data).unwrap().pem_type == *type_);
      assert!(decode(data).unwrap().pem_data.len() > 0);
    }
  }

  #[test]
  fn test_roundtrip() {
    for (_, data, roundtrip) in DOCUMENTS {
      if *roundtrip {
        assert!(encode(&decode(data).unwrap()).unwrap().trim() == data.trim());
      }
    }
  }
}
