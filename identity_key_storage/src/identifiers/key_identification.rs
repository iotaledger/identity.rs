// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// An identifier for a secured private key.
///
/// This type is expected to be returned by a [`KeyStorage`](crate::key_storage::KeyStorage) implementation when
/// generating cryptographic key pairs, and later used when signing data.  
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyId(pub String);
