// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[non_exhaustive]
#[derive(Debug)]
pub enum MethodCreationErrorKind {
    /// The provided fragment 
    FragmentInUse, 

    /// The provided fragment representation does not comply with the [specified syntax](https://www.w3.org/TR/did-core/#fragment). 
    InvalidFragmentSyntax,

    /// The provided [`KeyStorage`] implementation does not support generating keys of the given form. 
    UnsupportedMultikeySchema, 

    /// Caused by an attempt to create a method 
    /// whose metadata has already been persisted. 
    /// 
    /// This could be caused by the [`KeyStorage`] returning a previously 
    /// generated key rather than generating a new one contrary to the prescribed behaviour. 
    /// Using the same verification material in different verification methods goes against SSI principles. 
    /// If you want to use the same verification material across different context consider [referring to a single verification method](https://www.w3.org/TR/did-core/#referring-to-verification-methods) 
    /// containing the given verification material instead. 
    MethodMetadataAlreadyStored, 

    /// Caused by an unsuccessful I/O operation that may be retried, such as temporary connection failure or timeouts.
    ///
    /// Returning this error signals to the caller that the operation may be retried with a chance of success.
    /// It is at the caller's discretion whether to retry or not, and how often.
    RetryableIOFailure, 

     /// An attempt was made to authenticate with the key storage, but this operation did not succeed.
    KeyStorageAuthenticationFailure, 

    /// Indicates that an attempt was made to authenticate with the identity storage, but this operation did not succeed.
    IdentityStorageAuthenticationFailure, 

    /// The key storage is currently not available. See [`KeyStorageErrorKind::UnavailableKeyStorage`](crate::key_storage::error::KeyStorageErrorKind::UnavailableKeyStorage). 
    UnavailableKeyStorage,
    /// The identity storage is currently not available. See [`IdentityStorageErrorKind::UnavailableKeyStorage`](crate::identity_storage::error::IdentityStorageErrorKind::UnavailableIdentityStorage). 
    UnavailableIdentityStorage,

    /// The key storage failed in an unspecified manner. 
    UnspecifiedKeyStorageFailure, 

    /// The identity storage failed in an unspecified manner. 
    UnspecifiedIdentityStorageFailure, 

    /// A key was generated, but the necessary metadata could not be persisted in the [`IdentityStorage`], 
    /// the follow up attempt to remove the generated key from storage did not succeed. 
    // TODO: Do we want to communicate this? 
    // TODO: Should the variant wrap the `KeyId` so users can try deleting the corresponding key 
    // at a later point themselves? 
    // TODO: What expectations do we have for `MethodCreationError::source()` whenever this variant is encountered?  
    TransactionRollbackFailure, 
}