// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// An error explaining how [`KeyStorage`] operations went wrong. 
struct KeyStorageError {
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    cause: StorageErrorCause,
}

/// The cause of the failed [`KeyStorage`] operation.
enum StorageErrorCause {
    /// Occurs when a user tries to generate a key with a [`MultikeySchema`] which the [`KeyStorage`] implementation does not support. 
    UnsupportedMultikeySchema,

    /// Occurs when trying to sign with a key type that the [`KeyStorage`] implementation deems incompatible with the given signature algorithm. 
    UnsupportedSigningKey,

    /// Any error occurring while attempting to generate a new key.  
    /// 
    /// # Note 
    /// It is recommended to only use this variant in situations where there is no other variant of this type providing more 
    /// precise information about why key generation failed. Examples could be [`Self::UnsupportedMultikeySchema`] or [`Self::UnavailableKeyStorage`]. 
    UnsuccessfulKeyGeneration,
 
    /// Any error occurring while attempting to remove a key from the [`KeyStorage`] implementation.
    /// 
    /// # Note 
    /// It is recommended to only use this variant in situations where there is no other variant of this type providing more 
    /// precise information about why key removal failed. Examples could be [`Self::KeyNotFound`] or [`Self::UnavailableKeyStorage`].  
    UnsuccessfulKeyRemoval,
   
    /// Occurs when the [`KeyStorage`] implementation is not able to find the requested key material. 
    KeyNotFound,  
    
    /// Occurs if the storage becomes unavailable for an unpredictable amount of time. 
    /// 
    /// Occurrences of this variant should hopefully be rare, but could occur if hardware fails, or a subscription with a cloud provider ceases during for example. 
    UnavailableKeyStorage,
}