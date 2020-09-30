use alloc::vec::Vec;
use core::iter::FromIterator;

use crate::jwk::Jwk;

/// JSON Web Key Set.
///
/// [More Info](https://tools.ietf.org/html/rfc7517#section-5)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
pub struct JwkSet {
  /// An array of JWK values.
  ///
  /// [More Info](https://tools.ietf.org/html/rfc7517#section-5.1)
  keys: Vec<Jwk>,
}

impl JwkSet {
  /// Creates a new `JwkSet`.
  pub const fn new() -> Self {
    Self { keys: Vec::new() }
  }

  /// Returns the total number of keys in the set.
  pub fn len(&self) -> usize {
    self.keys.len()
  }

  /// Returns a boolean indicating if the set of keys is empty.
  pub fn is_empty(&self) -> bool {
    self.keys.is_empty()
  }

  /// Returns a slice containing the entire vector of keys.
  pub fn as_slice(&self) -> &[Jwk] {
    self.keys.as_slice()
  }

  /// Returns a list of keys matching the given `kid`.
  pub fn get(&self, kid: &str) -> Vec<&Jwk> {
    self
      .keys
      .iter()
      .filter(|key| matches!(key.kid(), Some(value) if value == kid))
      .collect()
  }

  /// Adds a new `key` to the set.
  pub fn add(&mut self, key: impl Into<Jwk>) {
    self.keys.push(key.into());
  }

  /// Removes the key at position `index`, returning true if the key was
  /// removed.
  pub fn del(&mut self, index: usize) -> bool {
    if index < self.keys.len() {
      self.keys.remove(index);
      true
    } else {
      false
    }
  }

  /// Removes and returns the last `key` in the set.
  pub fn pop(&mut self) -> Option<Jwk> {
    self.keys.pop()
  }
}

impl FromIterator<Jwk> for JwkSet {
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = Jwk>,
  {
    Self {
      keys: Vec::from_iter(iter),
    }
  }
}
