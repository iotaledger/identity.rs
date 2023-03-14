// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Errors that can occur when working with the [`JwkStorageDocumentExt`](crate::storage::JwkStorageDocumentExt) API. 
pub enum JwkStorageDocumentError {
    KeyStorageError(KeyStorageError), 
    KeyIdStorageError(KeyIdStorageError),
    FragmentExists, 
    MethodNotFound,
    SigningError(Box<dyn std::error::Error + Send + Sync + 'static>),
}