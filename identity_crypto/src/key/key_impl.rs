macro_rules! impl_key_bytes {
  ($ident:ident) => {
    #[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct $ident(Vec<u8>);

    impl $ident {
      pub fn new(data: Vec<u8>) -> Self {
        Self(data)
      }

      pub fn to_hex(&self) -> String {
        ::hex::encode(self)
      }

      pub fn len(&self) -> usize {
        self.0.len()
      }

      pub fn is_empty(&self) -> bool {
        self.0.is_empty()
      }

      pub fn check_length(&self, valid: &[usize]) -> $crate::error::Result<()> {
        if valid.contains(&self.len()) {
          Ok(())
        } else {
          Err($crate::error::Error::InvalidKeyLength(stringify!($ident)))
        }
      }
    }

    impl Drop for $ident {
      fn drop(&mut self) {
        ::zeroize::Zeroize::zeroize(&mut self.0);
      }
    }

    impl ::zeroize::Zeroize for $ident {
      fn zeroize(&mut self) {
        ::zeroize::Zeroize::zeroize(&mut self.0);
      }
    }

    impl ::std::fmt::Debug for $ident {
      fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.to_hex())
      }
    }

    impl ::std::fmt::Display for $ident {
      fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.to_hex())
      }
    }

    impl From<Vec<u8>> for $ident {
      fn from(other: Vec<u8>) -> Self {
        Self(other)
      }
    }

    impl AsRef<[u8]> for $ident {
      fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
      }
    }
  };
}

impl_key_bytes!(PublicKey);
impl_key_bytes!(SecretKey);
