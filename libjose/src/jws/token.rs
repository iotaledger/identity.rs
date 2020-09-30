macro_rules! impl_jose_token {
  ($name:ident, $header:ty, $claims:ty) => {
    paste::paste! {
      #[derive(Clone, Debug, PartialEq)]
      pub struct [<$name RawToken>]<T = $crate::utils::Empty> {
        pub header: $header<T>,
        pub claims: $crate::alloc::Vec<u8>,
      }

      #[derive(Clone, Debug, PartialEq)]
      pub struct [<$name Token>]<T = $crate::utils::Empty, U = $crate::utils::Empty> {
        pub header: $header<T>,
        pub claims: $claims<U>,
      }

      impl<T, U> [<$name Token>]<T, U> {
        pub const fn new(header: $header<T>, claims: $claims<U>) -> Self {
          Self { header, claims }
        }

        pub const fn header(&self) -> &$header<T> {
          &self.header
        }

        pub fn header_mut(&mut self) -> &mut $header<T> {
          &mut self.header
        }

        pub const fn claims(&self) -> &$claims<U> {
          &self.claims
        }

        pub fn claims_mut(&mut self) -> &mut $claims<U> {
          &mut self.claims
        }

        pub fn split(self) -> ($header<T>, $claims<U>) {
          (self.header, self.claims)
        }
      }

      impl<T, U> ::core::convert::TryFrom<[<$name RawToken>]<T>> for [<$name Token>]<T, U>
      where
        U: ::serde::de::DeserializeOwned,
      {
        type Error = $crate::error::Error;

        fn try_from(other: [<$name RawToken>]<T>) -> $crate::error::Result<Self, Self::Error> {
          Ok(Self {
            header: other.header,
            claims: ::serde_json::from_slice(&other.claims)?,
          })
        }
      }
    }
  };
}

use crate::jwe::JweHeader;
use crate::jws::JwsHeader;
use crate::jwt::JwtClaims;

impl_jose_token!(Jwe, JweHeader, JwtClaims);
impl_jose_token!(Jws, JwsHeader, JwtClaims);
