use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;

use bitvec::order::Lsb0;
use bitvec::view::BitView;
use thiserror::Error;

pub const BITSTRING_STATUS_LIST_DEFAULT_SIZE: usize = 16 * 1024;

pub type Result<'a, T> = std::result::Result<T, BitstringStatusListError<'a>>;

#[derive(Error, Debug, Clone)]
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

#[derive(Clone, Debug)]
pub struct BitstringStatusList {
  data: Box<[u8]>,
  statuses: Option<HashMap<usize, String>>,
}

impl Default for BitstringStatusList {
  fn default() -> Self {
    let data = vec![0; BITSTRING_STATUS_LIST_DEFAULT_SIZE].into_boxed_slice();
    Self { data, statuses: None }
  }
}

impl BitstringStatusList {
  pub fn new(size: usize, statuses: Option<HashMap<usize, String>>) -> Self {
    let data = vec![0; size].into_boxed_slice();
    Self { data, statuses }
  }
  pub fn len(&self) -> usize {
    self.data.len() * 8 / self.entry_len()
  }
  pub fn set<'s>(&mut self, index: usize, status: Status<'s>) -> Result<'s, ()> {
    if self.statuses.is_some() {
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
      .as_ref()
      .unwrap() // Safety: checked as precondition
      .iter()
      .find_map(|(k, v)| (v.as_str() == name).then_some(*k))
      .ok_or(BitstringStatusListError::InvalidStatus(status))?
      .to_le_bytes();
    let status_bits = &status_id_bytes.view_bits::<Lsb0>()[0..entry_len];
    bits[i..i + entry_len].copy_from_bitslice(status_bits);

    Ok(())
  }
  pub fn get<'a>(&'a self, index: usize) -> Option<Status<'a>> {
    if self.statuses.is_some() {
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
      .as_ref()
      .unwrap() // Safety: Checked as precondition
      .get(&status_id)
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
  pub fn from_encoded_str<'s>(&self, s: &'s str, statuses: Option<HashMap<usize, String>>) -> Result<'s, Self> {
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
      statuses,
    })
  }
  fn entry_len(&self) -> usize {
    std::mem::size_of::<usize>() * 8
      - self
        .statuses
        .as_ref()
        .map(HashMap::len)
        .unwrap_or(1)
        .next_power_of_two()
        .leading_zeros() as usize
      - 1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_entry_len() {
    let mut statuses = HashMap::new();
    statuses.insert(0, "suspended".to_owned());
    statuses.insert(1, "revoked".to_owned());
    statuses.insert(2, "pending".to_owned());
    statuses.insert(3, "whatever".to_owned());
    let bitstring = BitstringStatusList::new(1024, Some(statuses));
    assert_eq!(bitstring.entry_len(), 2);
  }

  #[test]
  fn test_bitstring_no_statuses_access() {
    let mut bitstring = BitstringStatusList::new(1024, None);
    assert!(bitstring.set(1024 * 8 + 1, true.into()).is_err());
    assert!(bitstring.set(42, true.into()).is_ok());
    assert_eq!(bitstring.get(42), Some(true.into()));
  }

  #[test]
  fn test_bitstring_with_statuses_access() {
    let mut statuses = HashMap::new();
    statuses.insert(0, "suspended".to_owned());
    statuses.insert(1, "revoked".to_owned());
    statuses.insert(2, "pending".to_owned());
    statuses.insert(3, "whatever".to_owned());
    let mut bitstring = BitstringStatusList::new(1024, Some(statuses));
    assert!(bitstring.get(1024 * 4 + 1).is_none());
    assert!(bitstring.set(42, "NOP".into()).is_err());
    assert!(bitstring.set(42, "pending".into()).is_ok());
    assert_eq!(bitstring.get(42), Some("pending".into()));
  }
}
