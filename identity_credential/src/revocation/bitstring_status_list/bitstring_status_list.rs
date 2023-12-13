use std::borrow::Cow;
use std::fmt::Display;

use bitvec::order::Lsb0;
use bitvec::view::BitView;
use serde::ser::Serialize;
use serde::Deserialize;
use serde::Deserializer;
use thiserror::Error;

pub const BITSTRING_STATUS_LIST_DEFAULT_SIZE: usize = 16 * 1024;

pub type Result<'a, T> = std::result::Result<T, BitstringStatusListError<'a>>;

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BitstringStatusListError<'a> {
  #[error("Status {0} is not a valid status")]
  InvalidStatus(Status<'a>),
  #[error("Index out of bound")]
  IndexOutOfBound,
  #[error("Failed to decode {0} into a BitstringStatusList")]
  DecodingError(&'a str),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Status<'a> {
  Flag(bool),
  Custom(Cow<'a, str>),
}

impl Display for Status<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Flag(v) if *v => write!(f, "set"),
      Self::Flag(_) => write!(f, "unset"),
      Self::Custom(status) => write!(f, "{status}"),
    }
  }
}

impl From<bool> for Status<'_> {
  fn from(v: bool) -> Self {
    Status::Flag(v)
  }
}

impl<'a> From<&'a str> for Status<'a> {
  fn from(s: &'a str) -> Self {
    Status::Custom(Cow::Borrowed(s))
  }
}

impl From<String> for Status<'_> {
  fn from(s: String) -> Self {
    Status::Custom(Cow::Owned(s))
  }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize)]
pub struct StatusMessage {
  #[serde(deserialize_with = "deserialize_hex_repr_string")]
  status: u64,
  message: String,
}

fn deserialize_hex_repr_string<'de, D>(deserializer: D) -> std::result::Result<u64, D::Error>
where
  D: Deserializer<'de>,
{
  use serde::de::Error;
  use serde::de::Visitor;

  struct HexReprStrVisitor;
  impl<'de> Visitor<'de> for HexReprStrVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
        formatter,
        "a string containing the hex representation of a status, such as `0x2a`"
      )
    }

    fn visit_str<E>(self, v: &str) -> std::prelude::v1::Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      let stripped = v
        .trim_matches('"')
        .strip_prefix("0x")
        .ok_or(Error::invalid_value(serde::de::Unexpected::Str(v), &self))?;
      u64::from_str_radix(stripped, 16).map_err(|_| Error::invalid_value(serde::de::Unexpected::Str(v), &self))
    }
  }

  deserializer.deserialize_str(HexReprStrVisitor)
}

impl Serialize for StatusMessage {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeMap;
    let mut obj = serializer.serialize_map(Some(2))?;
    obj.serialize_entry("status", &format!("{:#x}", self.status))?;
    obj.serialize_entry("message", self.message.as_str())?;
    obj.end()
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitstringStatusList {
  pub(crate) data: Box<[u8]>,
  pub(crate) statuses: Box<[StatusMessage]>,
}

impl Default for BitstringStatusList {
  fn default() -> Self {
    let data = vec![0; BITSTRING_STATUS_LIST_DEFAULT_SIZE].into_boxed_slice();
    Self {
      data,
      statuses: Box::new([]),
    }
  }
}

impl BitstringStatusList {
  pub fn new(size: usize, statuses: Vec<StatusMessage>) -> Self {
    let data = vec![0; size].into_boxed_slice();
    Self {
      data,
      statuses: statuses.into_boxed_slice(),
    }
  }
  pub fn len(&self) -> usize {
    self.data.len() * 8 / self.entry_len()
  }
  pub fn set<'s>(&mut self, index: usize, status: Status<'s>) -> Result<'s, ()> {
    if !self.statuses.is_empty() {
      self.set_with_statuses(index, status)
    } else {
      let bits = self.data.view_bits_mut::<Lsb0>();
      if index > bits.len() {
        return Err(BitstringStatusListError::IndexOutOfBound);
      }
      let Status::Flag(flag) = &status else {
        return Err(BitstringStatusListError::InvalidStatus(status));
      };
      bits.set(index, *flag);

      Ok(())
    }
  }
  fn set_with_statuses<'s>(&mut self, index: usize, status: Status<'s>) -> Result<'s, ()> {
    let entry_len = self.entry_len();
    let i = index * entry_len;
    let bits = self.data.view_bits_mut::<Lsb0>();
    if i > bits.len() {
      return Err(BitstringStatusListError::IndexOutOfBound);
    }
    let Status::Custom(name) = &status else {
      return Err(BitstringStatusListError::InvalidStatus(status));
    };
    let status_id_bytes = self
      .statuses
      .iter()
      .find_map(|StatusMessage { status, message }| (message.as_str() == name).then_some(*status))
      .ok_or(BitstringStatusListError::InvalidStatus(status))?
      .to_le_bytes();
    let status_bits = &status_id_bytes.view_bits::<Lsb0>()[0..entry_len];
    bits[i..i + entry_len].copy_from_bitslice(status_bits);

    Ok(())
  }
  pub fn get<'a>(&'a self, index: usize) -> Option<Status<'a>> {
    if !self.statuses.is_empty() {
      self.get_with_statuses(index)
    } else {
      self.data.view_bits::<Lsb0>().get(index).map(|bit| Status::Flag(*bit))
    }
  }
  fn get_with_statuses<'a>(&'a self, index: usize) -> Option<Status<'a>> {
    let i = index * self.entry_len();
    if i > self.len() {
      return None;
    }
    let mut status_id_bytes = [0; 8];
    status_id_bytes.view_bits_mut::<Lsb0>()[0..self.entry_len()]
      .copy_from_bitslice(&self.data.view_bits::<Lsb0>()[i..i + self.entry_len()]);
    let status_id = usize::from_le_bytes(status_id_bytes);

    self
      .statuses
      .iter()
      .find_map(|StatusMessage { status, message }| (*status as usize == status_id).then_some(message.as_str()))
      .map(|name| Status::Custom(name.into()))
  }
  pub fn as_encoded_str(&self) -> std::io::Result<String> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use identity_core::convert::Base;
    use identity_core::convert::BaseEncoding;
    use std::io::Write;

    let mut compressor = GzEncoder::new(Vec::new(), Compression::best());
    compressor.write_all(&self.data)?;
    let compressed = compressor.finish()?;
    Ok(BaseEncoding::encode(&compressed, Base::Base64Url))
  }
  pub fn try_from_encoded_str<'s>(s: &'s str, statuses: Vec<StatusMessage>) -> Result<'s, Self> {
    use flate2::read::GzDecoder;
    use identity_core::convert::Base;
    use identity_core::convert::BaseEncoding;
    use std::io::Read;

    let decoded = BaseEncoding::decode(s, Base::Base64Url).map_err(|_| BitstringStatusListError::DecodingError(s))?;
    let mut decompressor = GzDecoder::new(&decoded[..]);
    let mut bitstring_data = vec![];
    decompressor
      .read_to_end(&mut bitstring_data)
      .map_err(|_| BitstringStatusListError::DecodingError(s))?;

    Ok(Self {
      data: bitstring_data.into_boxed_slice(),
      statuses: statuses.into_boxed_slice(),
    })
  }
  pub(crate) fn entry_len(&self) -> usize {
    super::utils::bit_required(self.statuses.len())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_entry_len() {
    let statuses = vec![
      StatusMessage {
        status: 0,
        message: "suspended".to_owned(),
      },
      StatusMessage {
        status: 1,
        message: "revoked".to_owned(),
      },
      StatusMessage {
        status: 2,
        message: "pending".to_owned(),
      },
      StatusMessage {
        status: 3,
        message: "whatever".to_owned(),
      },
    ];
    let bitstring = BitstringStatusList::new(1024, statuses);
    assert_eq!(bitstring.entry_len(), 2);
  }

  #[test]
  fn test_bitstring_no_statuses_access() {
    let mut bitstring = BitstringStatusList::new(1024, vec![]);
    assert!(bitstring.set(1024 * 8 + 1, true.into()).is_err());
    assert!(bitstring.set(42, true.into()).is_ok());
    assert_eq!(bitstring.get(42), Some(true.into()));
  }

  #[test]
  fn test_bitstring_with_statuses_access() {
    let statuses = vec![
      StatusMessage {
        status: 0,
        message: "suspended".to_owned(),
      },
      StatusMessage {
        status: 1,
        message: "revoked".to_owned(),
      },
      StatusMessage {
        status: 2,
        message: "pending".to_owned(),
      },
      StatusMessage {
        status: 3,
        message: "whatever".to_owned(),
      },
    ];
    let mut bitstring = BitstringStatusList::new(1024, statuses);
    assert!(bitstring.get(1024 * 4 + 1).is_none());
    assert!(bitstring.set(42, "NOP".into()).is_err());
    assert!(bitstring.set(42, "pending".into()).is_ok());
    assert_eq!(bitstring.get(42), Some("pending".into()));
  }

  #[test]
  fn test_encoding_and_decoding() {
    let mut bitstring = BitstringStatusList::new(1024, vec![]);
    assert!(bitstring.set(1024 * 8 + 1, true.into()).is_err());
    assert!(bitstring.set(42, true.into()).is_ok());

    let encoded = bitstring.as_encoded_str().unwrap();
    let decoded = BitstringStatusList::try_from_encoded_str(&encoded, bitstring.statuses.clone().to_vec()).unwrap();

    assert_eq!(bitstring, decoded);
  }

  #[test]
  fn status_message_deserialization() {
    use serde_json::json;

    let json_status_message = json!({
      "status": "0xa",
      "message": "unresolvable",
    });

    assert_eq!(
      serde_json::from_value::<StatusMessage>(json_status_message).unwrap(),
      StatusMessage {
        status: 10,
        message: "unresolvable".to_owned()
      }
    )
  }

  #[test]
  fn status_message_serialization() {
    use serde_json::json;
    let status_message = StatusMessage {
      status: 10,
      message: "unresolvable".to_owned(),
    };

    assert_eq!(
      serde_json::to_value(&status_message).unwrap(),
      json!({"status": "0xa", "message": "unresolvable"})
    )
  }
}
