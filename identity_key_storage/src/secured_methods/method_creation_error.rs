// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub enum MethodCreationErrorKind {
    /// The provided fragment 
    FragmentInUse, 
    /// The provided fragment representation does not comply with the [specified syntax](https://www.w3.org/TR/did-core/#fragment). 
    InvalidFragmentSyntax,
    /// The provided [`KeyStorage`] implementation does not support generating keys of the given form. 
    UnsupportedMultikeySchema, 
    /// The key storage is currently not available. See [`KeyStorageErrorKind::UnavailableKeyStorage`](crate::key_storage::error::KeyStorageErrorKind::UnavailableKeyStorage). 
    UnavailableKeyStorage,
    /// The identity storage is currently not available. See [`IdentityStorageErrorKind::UnavailableKeyStorage`](crate::identity_storage::error::IdentityStorageErrorKind::UnavailableIdentityStorage). 
    UnavailableIdentityStorage,
    /// Indicates an attempt to create a method 
    /// whose metadata has already been persisted. 
    /// 
    /// This could be caused by the [`KeyStorage`] returning a previously 
    /// generated key rather than generating a new one contrary to the prescribed behaviour. 
    /// Using the same verification material in different verification methods goes against SSI principles. 
    /// If you want to use the same verification material across different context consider [referring to a single verification method](https://www.w3.org/TR/did-core/#referring-to-verification-methods) 
    /// containing the given verification material instead. 
    MethodMetadataAlreadyStored, 


}