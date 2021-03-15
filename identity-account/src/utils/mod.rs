// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod crypto;

pub mod fs;

pub use self::crypto::derive_encryption_key;
pub use self::crypto::EncryptionKey;
