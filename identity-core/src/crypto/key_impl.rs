// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

macro_rules! impl_bytes {
  ($ident:ident) => {
    /// A cryptographic key.
    #[derive(Clone)]
    pub struct $ident(Vec<u8>);

    impl From<Vec<u8>> for $ident {
      fn from(other: Vec<u8>) -> $ident {
        Self(other)
      }
    }

    impl AsRef<[u8]> for $ident {
      fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
      }
    }

    impl Drop for $ident {
      fn drop(&mut self) {
        use ::zeroize::Zeroize;
        self.0.zeroize();
      }
    }

    impl ::zeroize::Zeroize for $ident {
      fn zeroize(&mut self) {
        self.0.zeroize();
      }
    }

    impl ::core::fmt::Debug for $ident {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str(stringify!($ident))
      }
    }

    impl ::core::fmt::Display for $ident {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str(stringify!($ident))
      }
    }
  };
}

impl_bytes!(PublicKey);
impl_bytes!(SecretKey);
