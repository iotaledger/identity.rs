// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

mod key;
mod pair;

pub use self::key::PublicKey;
pub use self::key::SecretKey;
pub use self::pair::KeyPair;
