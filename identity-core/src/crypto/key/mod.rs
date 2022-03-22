// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

pub use self::collection::KeyCollection;
pub use self::key::PrivateKey;
pub use self::key::PublicKey;
pub use self::pair::KeyPair;
pub use self::reference::KeyRef;
pub use self::type_::KeyType;
pub use self::x25519::X25519;

mod collection;
mod key;
mod pair;
mod reference;
mod type_;
mod x25519;
