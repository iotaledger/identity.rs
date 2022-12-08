// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

/// An error explaining how [`KeyStorage`] operations went wrong.
#[derive(Debug)] 
pub struct KeyStorageError {
    repr: Repr
}

#[derive(Debug)]
struct Extensive {
    cause: StorageErrorCause, 
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    message: Option<Cow<'static, str>>
}

#[derive(Debug)]
enum Repr {
    Simple(StorageErrorCause), 
    Extensive(Box<Extensive>)
}

impl From<StorageErrorCause> for KeyStorageError {
    fn from(cause: StorageErrorCause) -> Self {
        Self::new(cause)
    }
}

impl From<Box<Extensive>> for KeyStorageError {
    fn from(extensive: Box<Extensive>) -> Self {
        Self {
            repr: Repr::Extensive(extensive)
        }
    }
}

impl KeyStorageError {
    /// Constructs a new [`KeyStorageError`].  
    pub fn new(cause: StorageErrorCause) -> Self {
        Self { repr: Repr::Simple(cause) }
    }

    /// Returns a reference to corresponding [`StorageErrorCause`] for this error. 
    pub fn cause(&self) -> &StorageErrorCause {
        match self.repr {
            Repr::Simple(ref cause) => cause, 
            Repr::Extensive(ref extensive) => &extensive.cause
        }
    }

    /// Converts this error into the corresponding [`StorageErrorCause`]. 
    pub fn into_cause(self) -> StorageErrorCause {
        match self.repr {
            Repr::Simple(cause) => cause, 
            Repr::Extensive(extensive) => extensive.cause
        }
    }

    /// Returns a reference to the custom message of the [`KeyStorageError`] if it was set. 
    pub fn custom_message(&self) -> Option<&str> {
        self.extensive().into_iter().flat_map(|extensive| extensive.message.as_deref()).next()
    }

    fn extensive(&self) -> Option<&Extensive> {
        match self.repr {
            Repr::Extensive(ref extensive) => Some(extensive.as_ref()),
            _ => None 
        }
    }

    fn into_extensive(self) -> Box<Extensive> {
        match self.repr {
            Repr::Extensive(extensive) => extensive, 
            Repr::Simple(cause) => Box::new(Extensive { cause, source: None, message: None })
        }
    }

    /// Updates the `source` of the [`KeyStorageError`]. 
    pub fn with_source(mut self, source: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        self._with_source(source.into())
    } 

    fn _with_source(self, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        let mut extensive = self.into_extensive();
        extensive.as_mut().source = Some(source); 
        Self::from(extensive)
        }
    
    /// Updates the custom message of the [`KeyStorageError`]. 
    pub fn with_custom_message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self._with_custom_message(message.into())
    }

    fn _with_custom_message(self, message: Cow<'static, str>) -> Self {
        let mut extensive = self.into_extensive();
        extensive.as_mut().message = Some(message);
        Self::from(extensive)
    }
   
}

/// The cause of the failed [`KeyStorage`] operation.
#[derive(Debug)]
pub enum StorageErrorCause {
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

impl StorageErrorCause {
    fn as_str(&self) -> &str {
        match self {
            Self::UnsupportedMultikeySchema =>  "key generation does not support the provided multikey schema",
            Self::UnsupportedSigningKey =>  "the signing algorithm does not support the provided key type",
            Self::UnsuccessfulKeyGeneration =>  "the key generation operation did not succeed", 
            Self::UnsuccessfulKeyRemoval => "the key removal operation did not succeed",
            Self::KeyNotFound => "could not find key", 
            Self::UnavailableKeyStorage =>  "the key storage is not available"
        }
    }
}

impl std::fmt::Display for StorageErrorCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.as_str())
    }
}
