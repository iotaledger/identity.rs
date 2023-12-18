use std::fmt::Display;

use thiserror::Error;

const DEFAULT_LIST_SIZE: usize = 16 * 1024;

#[derive(Debug, Error, Eq, PartialEq)]
/// [`std::error::Error`] type representing an invalidly encoded [`StatusList2021`]
pub struct InvalidEncodedStatusList(String);

impl Display for InvalidEncodedStatusList {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
/// StatusList2021 data structure as described in [W3C's VC status list 2021](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/)
pub struct StatusList2021(Box<[u8]>);

impl Default for StatusList2021 {
  fn default() -> Self {
    StatusList2021::new(DEFAULT_LIST_SIZE)
  }
}

impl StatusList2021 {
  /// Returns a new empty [`StatusList2021`] of `num_entries` entries
  /// ## Note
  /// The actual length of the list might be slightly bigger
  /// depending on the number of bytes the list will use.
  pub fn new(num_entries: usize) -> Self {
    let size = num_entries / 8 + (num_entries % 8 != 0) as usize;
    let store = vec![0; size];
    StatusList2021(store.into_boxed_slice())
  }
  /// Returns the number of entries
  pub const fn len(&self) -> usize {
    self.0.len() * 8
  }
  /// Returns the status of the entry at `index` without bound checking
  /// ## Panic:
  /// * if `index` is greater than or equal to `self.len()`
  pub const fn get_unchecked(&self, index: usize) -> bool {
    let (i, offset) = Self::entry_index_to_store_index(index);
    self.0[i] & (0b1000_0000 >> offset) != 0
  }
  /// Sets the status of the `index`-th entry to `value`
  /// ## Panic:
  /// * if `index` is greater than or equal to `self.len()`
  pub fn set_unchecked(&mut self, index: usize, value: bool) {
    let (i, offset) = Self::entry_index_to_store_index(index);
    if value {
      self.0[i] |= 0b1000_0000 >> offset
    } else {
      self.0[i] &= 0b0111_1111 >> offset
    }
  }
  /// Returns the status of the `index`-th entry, if it exists
  pub fn get(&self, index: usize) -> Option<bool> {
    (index < self.len()).then_some(self.get_unchecked(index))
  }
  /// Sets the status fo the `index`-th entry to `value`
  pub fn set(&mut self, index: usize, value: bool) {
    if index < self.len() {
      self.set_unchecked(index, value)
    }
  }
  /// Attempts to parse a [`StatusList2021`] from a string, following the
  /// [StatusList2021 expansion algorithm](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#bitstring-expansion-algorithm)
  pub fn try_from_encoded_str(s: &str) -> Result<Self, InvalidEncodedStatusList> {
    use flate2::read::GzDecoder;
    use identity_core::convert::Base;
    use identity_core::convert::BaseEncoding;

    let compressed_status_list =
      BaseEncoding::decode(s, Base::Base64Url).or(Err(InvalidEncodedStatusList(s.to_owned())))?;
    let status_list = {
      use std::io::Read;

      let mut decompressor = GzDecoder::new(&compressed_status_list[..]);
      let mut status_list = vec![];
      decompressor
        .read_to_end(&mut status_list)
        .or(Err(InvalidEncodedStatusList(s.to_owned())))?;

      StatusList2021(status_list.into_boxed_slice())
    };

    Ok(status_list)
  }
  /// Encode this [`StatusList2021`] into its string representation following
  /// [StatusList2021 generation algorithm](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#bitstring-generation-algorithm)
  pub fn into_encoded_str(self) -> String {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use identity_core::convert::Base;
    use identity_core::convert::BaseEncoding;

    let compressed_status_list = {
      use std::io::Write;

      let mut compressor = GzEncoder::new(vec![], Compression::best());
      compressor.write_all(&self.0).expect("Out of memory");
      compressor.finish().unwrap()
    };

    BaseEncoding::encode(&compressed_status_list[..], Base::Base64Url)
  }
  const fn entry_index_to_store_index(index: usize) -> (usize, usize) {
    (index / 8, index % 8)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn status_list_entry_access() {
    let mut status_list = StatusList2021::new(128);
    status_list.set(42, true);
    assert_eq!(status_list.get(42), Some(true));

    status_list.set(42, false);
    assert_eq!(status_list, StatusList2021::new(128));
  }

  #[test]
  fn status_list_encode_decode() {
    let mut status_list = StatusList2021::default();
    status_list.set(42, true);
    status_list.set(420, true);
    status_list.set(4200, true);
    let encoded = status_list.clone().into_encoded_str();
    let decoded = StatusList2021::try_from_encoded_str(&encoded);
    assert_eq!(decoded, Ok(status_list));
  }
}
