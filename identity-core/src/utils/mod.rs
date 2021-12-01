// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Misc. utility functions (encoding, decoding, and ed25519 utils).

mod base_encoding;
mod ed25519;

pub use self::base_encoding::Base58DecodingError;
pub use self::base_encoding::Base64DecodingError;
pub use self::base_encoding::MultiBaseDecodingError;
pub use self::base_encoding::*;
pub use self::ed25519::generate_ed25519_keypair;
pub use self::ed25519::generate_ed25519_keypairs;
pub(crate) use self::ed25519::keypair_from_ed25519_private_key;
