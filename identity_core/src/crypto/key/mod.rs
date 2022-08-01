// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

pub use self::ed25519::Ed25519;
pub use self::key::PrivateKey;
pub use self::key::PublicKey;
pub use self::pair::KeyPair;
pub use self::secp256k1::Secp256k1;
pub use self::type_::KeyType;
pub use self::x25519::X25519;

mod ed25519;
mod key;
mod pair;
mod secp256k1;
mod type_;
mod x25519;
