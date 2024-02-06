// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! JSON Web Signatures ([JWS](https://tools.ietf.org/html/rfc7515))
//!
//! The encoding and decoding APIs are strongly informed by the requirements of the higher level functionality
//! offered by the IOTA Identity library. Hence these APIs may possibly not be immediately recognizable from a standard
//! JWT/JWS perspective. See `identity_jose/examples/jws_encoding_decoding.rs` for a complete example of how to encode
//! and then decode a JWS.

mod algorithm;
mod charset;
mod custom_verification;
mod decoder;
mod encoding;
mod format;
mod header;
mod recipient;

pub use self::algorithm::*;
pub use self::charset::*;
pub use self::custom_verification::*;
pub use self::decoder::*;
pub use self::encoding::*;
pub use self::format::*;
pub use self::header::*;
pub use self::recipient::*;
