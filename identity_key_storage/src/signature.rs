// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A signature produced by [`KeyStorage::sign`](crate::key_storage::KeyStorage::sign()).
pub struct Signature(pub Vec<u8>);
