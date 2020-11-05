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

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
  BufferSize {
    what: &'static str,
    needs: usize,
    has: usize,
  },
  CipherError {
    alg: &'static str,
  },
  SignatureError {
    alg: &'static str,
  },
  RngError {
    what: &'static str,
  },
}

impl core::fmt::Display for Error {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self {
      Self::BufferSize { what, needs, has } => {
        write!(f, "Invalid {} Buffer: {}/{}", what, has, needs)
      }
      Self::CipherError { alg } => write!(f, "Cipher Error: {}", alg),
      Self::SignatureError { alg } => write!(f, "Signature Error: {}", alg),
      Self::RngError { what } => write!(f, "Rng Error: {}", what),
    }
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
