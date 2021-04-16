// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;

use crate::error::Error;
use crate::error::Result;

pub trait Integer: Sized {
  fn decode(bytes: &[u8]) -> Result<Self>;

  fn encode(&self) -> Vec<u8>;

  fn decode_vec(bytes: Vec<u8>) -> Result<Self> {
    Self::decode(&bytes)
  }

  fn decode_opt(bytes: &[u8]) -> Option<Self> {
    Self::decode(bytes).ok()
  }
}

macro_rules! impl_Integer {
  ($ident:ident) => {
    impl Integer for $ident {
      fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() {
          Ok(0)
        } else {
          bytes
            .try_into()
            .map(Self::from_be_bytes)
            .map_err(|_| Error::InvalidIntegerBytes(stringify!($ident)))
        }
      }

      fn encode(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
      }
    }
  };
  ($($ident:ident)+) => {
    $(
      impl_Integer!($ident);
    )+
  };
}

impl_Integer!(u8 u16 u32 u64);
impl_Integer!(i8 i16 i32 i64);
