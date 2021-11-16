// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use std::marker::PhantomData;
use zeroize::Zeroize;

/// A cryptographic key with `Public` components.
pub type PublicKey = Key<Public>;

/// A cryptographic key with `Private` components.
pub type PrivateKey = Key<Private>;

// =============================================================================
// =============================================================================

pub use errors::KeyFormatError;
pub use errors::KeyLengthError; 
pub use errors::KeyParsingError; 
mod errors {
  use thiserror::Error as DeriveError;

  /// Caused by attempting to parse an invalid cryptographic key.
  #[derive(Debug, DeriveError)]
  #[error("failed to parse cryptographic key(s): the provided key format is invalid")]
  pub struct KeyFormatError;

  /// Caused by an attempt to create a key of an invalid length 
  #[derive(Debug, DeriveError)]
  #[error("invalid key length: expected {EXPECTED}, but found {actual}")]
  pub struct KeyLengthError<const EXPECTED: usize> { 
    /// The actual key length 
    pub actual: usize,
  }
  
  #[derive(Debug, DeriveError)]
  /// Aggregate of errors related to parsing cryptographic keys. 
  pub enum KeyParsingError<const PUBLIC_KEY_LENGTH: usize, const PRIVATE_KEY_LENGTH: usize>  {
    /// Caused by attempting to parse a public key of incorrect length 
    #[error("{0}")]
    InvalidPublicKeyLength(KeyLengthError::<PUBLIC_KEY_LENGTH>),
    /// Caused by attempting to parse a private key of incorrect length 
    #[error("{0}")]
    InvalidPrivateKeyLength(KeyLengthError::<PRIVATE_KEY_LENGTH>), 
    /// Caused by failing to parse a cryptographic key for any reason
    #[error("failed to parse cryptographic key(s): the provided key format is invalid")]
    FormatError, 
  }

  impl<const PUBLIC_KEY_LENGTH: usize, const PRIVATE_KEY_LENGTH: usize> From<KeyFormatError> for KeyParsingError<PUBLIC_KEY_LENGTH, PRIVATE_KEY_LENGTH> {
    fn from(_: KeyFormatError) -> Self {
        Self::FormatError
    }
  }  
}
mod private {
  pub trait Sealed {}
}

// A marker type for the `Public` components of an asymmetric cryptographic key.
#[derive(Clone, Copy, Debug)]
pub enum Public {}

// A marker type for the `Private` components of an asymmetric cryptographic key.
#[derive(Clone, Copy, Debug)]
pub enum Private {}

impl private::Sealed for Public {}

impl private::Sealed for Private {}

// =============================================================================
// =============================================================================

/// A cryptographic key.
#[derive(Clone)]
pub struct Key<V: private::Sealed> {
  key: Box<[u8]>,
  vis: PhantomData<V>,
}

impl<V: private::Sealed> Debug for Key<V> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str("Key")
  }
}

impl<V: private::Sealed> Display for Key<V> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str("Key")
  }
}

impl<V: private::Sealed> Drop for Key<V> {
  fn drop(&mut self) {
    self.key.zeroize();
  }
}

impl<V: private::Sealed> Zeroize for Key<V> {
  fn zeroize(&mut self) {
    self.key.zeroize();
  }
}

impl<V: private::Sealed> AsRef<[u8]> for Key<V> {
  fn as_ref(&self) -> &[u8] {
    &self.key
  }
}

impl<V: private::Sealed> From<Box<[u8]>> for Key<V> {
  fn from(other: Box<[u8]>) -> Self {
    Self {
      key: other,
      vis: PhantomData,
    }
  }
}

impl<V: private::Sealed> From<Vec<u8>> for Key<V> {
  fn from(other: Vec<u8>) -> Self {
    other.into_boxed_slice().into()
  }
}
