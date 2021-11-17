// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use serde::Serialize;

use identity_core::convert::ToJson;
use identity_core::crypto::Named;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::SignatureValue;
use identity_core::error::Error;
use identity_core::error::Result;
use identity_core::utils::encode_b58;

use crate::identity::IdentityId;
use crate::storage::Storage;
use crate::types::KeyLocation;

pub struct RemoteEd25519;

impl Named for RemoteEd25519 {
  const NAME: &'static str = "JcsEd25519Signature2020";
}

impl RemoteEd25519 {
  pub async fn create_signature<U, T>(data: &mut U, method: impl Into<String>, secret: &RemoteKey<'_, T>) -> Result<()>
  where
    U: Serialize + SetSignature,
    T: Storage,
  {
    data.set_signature(Signature::new(Self::NAME, method));

    let value: SignatureValue = Self::sign(&data, secret).await?;
    let write: &mut Signature = data.try_signature_mut()?;

    write.set_value(value);

    Ok(())
  }

  pub async fn sign<T, X>(data: &X, remote_key: &RemoteKey<'_, T>) -> Result<SignatureValue>
  where
    X: Serialize,
    T: Storage,
  {
    let message: Vec<u8> = data.to_jcs()?;
    let signature: Vec<u8> = RemoteSign::sign(&message, remote_key).await?;
    let signature: String = encode_b58(&signature);
    Ok(SignatureValue::Signature(signature))
  }
}

/// A reference to a storage instance and identity key location.
#[derive(Debug)]
pub struct RemoteKey<'a, T> {
  id: IdentityId,
  location: &'a KeyLocation,
  store: &'a T,
}

impl<'a, T> RemoteKey<'a, T> {
  /// Creates a new `RemoteKey` instance.
  pub fn new(id: IdentityId, location: &'a KeyLocation, store: &'a T) -> Self {
    Self { id, location, store }
  }
}

// =============================================================================
// RemoteSign
// =============================================================================

/// A remote signature that delegates to a storage implementation.
///
/// Note: The signature implementation is specified by the associated `RemoteKey`.
#[derive(Clone, Copy, Debug)]
pub struct RemoteSign<'a, T> {
  marker: PhantomData<RemoteKey<'a, T>>,
}

impl<'a, T> RemoteSign<'a, T>
where
  T: Storage,
{
  pub async fn sign(message: &[u8], key: &RemoteKey<'a, T>) -> Result<Vec<u8>> {
    key
      .store
      .key_sign(key.id, key.location, message.to_vec())
      .await
      .map_err(|_| Error::InvalidProofValue("remote sign"))
      .map(|signature| signature.data)
  }
}
