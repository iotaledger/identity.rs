// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


pub enum MethodRemovalErrorKind {
    /// The key could not be found in the key storage.
    KeyNotFound,

    /// Unable to find the necessary key metadata in the [`IdentityStorage`](crate::identity_storage::IdentityStorage). 
    MethodMetadataNotFound,

    /// Caused by an unsuccessful I/O operation that may be retried, such as temporary connection failure or timeouts.
    ///
    /// It is at the caller's discretion whether to retry or not, and how often.
    RetryableIOFailure, 

    /// An attempt was made to authenticate with the key storage, but this operation did not succeed.
    KeyStorageAuthenticationFailure,
    
    /// Indicates that an attempt was made to authenticate with the identity storage, but this operation did not succeed.
    IdentityStorageAuthenticationFailure,

    /// The key storage is currently not available. See
    /// [`KeyStorageErrorKind::UnavailableKeyStorage`](crate::key_storage::error::KeyStorageErrorKind::UnavailableKeyStorage).
    UnavailableKeyStorage,

    /// The identity storage is currently not available. See
    /// [`IdentityStorageErrorKind::UnavailableKeyStorage`](crate::identity_storage::error::IdentityStorageErrorKind::UnavailableIdentityStorage).
    UnavailableIdentityStorage,

    /// The key storage failed in an unspecified manner.
    UnspecifiedKeyStorageFailure,

    /// The identity storage failed in an unspecified manner.
    UnspecifiedIdentityStorageFailure,


}