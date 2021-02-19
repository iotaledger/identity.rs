// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use zeroize::Zeroize;

macro_rules! impl_key {
  ($ident:ident, $doc:expr) => {
    #[derive(Clone)]
    #[doc = $doc]
    pub struct $ident(Box<[u8]>);

    impl Debug for $ident {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(stringify!($ident))
      }
    }

    impl Display for $ident {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(stringify!($ident))
      }
    }

    impl Drop for $ident {
      fn drop(&mut self) {
        self.0.zeroize();
      }
    }

    impl Zeroize for $ident {
      fn zeroize(&mut self) {
        self.0.zeroize();
      }
    }

    impl AsRef<[u8]> for $ident {
      fn as_ref(&self) -> &[u8] {
        &self.0
      }
    }

    impl From<Box<[u8]>> for $ident {
      fn from(other: Box<[u8]>) -> Self {
        Self(other)
      }
    }

    impl From<Vec<u8>> for $ident {
      fn from(other: Vec<u8>) -> Self {
        other.into_boxed_slice().into()
      }
    }
  };
}

impl_key!(PublicKey, "A public key object.");
impl_key!(SecretKey, "A secret key object.");
