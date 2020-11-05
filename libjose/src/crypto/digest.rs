// Copyright 2020 IOTA Stiftung
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub use ::digest_crate::*;
pub use ::sha2;

use crate::crypto::error::Result;

pub type SHA2_224 = sha2::Sha224;
pub type SHA2_256 = sha2::Sha256;
pub type SHA2_384 = sha2::Sha384;
pub type SHA2_512 = sha2::Sha512;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum HashAlgorithm {
  SHA2_224,
  SHA2_256,
  SHA2_384,
  SHA2_512,
}

impl HashAlgorithm {
  pub fn digest_fn(
    self,
    message: impl AsRef<[u8]>,
    f: impl FnOnce(&[u8]) -> Result<()>,
  ) -> Result<()> {
    match self {
      Self::SHA2_224 => f(&SHA2_224::digest(message.as_ref())),
      Self::SHA2_256 => f(&SHA2_256::digest(message.as_ref())),
      Self::SHA2_384 => f(&SHA2_384::digest(message.as_ref())),
      Self::SHA2_512 => f(&SHA2_512::digest(message.as_ref())),
    }
  }
}
