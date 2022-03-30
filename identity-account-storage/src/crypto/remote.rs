// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use serde::Serialize;

use identity_core::convert::ToJson;
use identity_core::crypto::Named;
use identity_core::crypto::Proof;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::ProofValue;
use identity_core::crypto::SetSignature;
use identity_core::error::Error;
use identity_core::error::Result;
use identity_core::utils::encode_b58;
use identity_iota_core::did::IotaDID;

use crate::storage::Storage;
use crate::types::KeyLocation;
use crate::types::Signature as StorageSignature;

pub struct RemoteEd25519;

impl Named for RemoteEd25519 {
  const NAME: &'static str = "JcsEd25519Signature2020";
}

impl RemoteEd25519 {
  pub async fn create_signature<U>(
    data: &mut U,
    method: impl Into<String>,
    secret: &RemoteKey<'_>,
    options: ProofOptions,
  ) -> Result<()>
  where
    U: Serialize + SetSignature,
  {
    let signature: Proof = Proof::new_with_options(Self::NAME, method, options);
    data.set_signature(signature);

    let value: ProofValue = Self::sign(&data, secret).await?;
    let write: &mut Proof = data.signature_mut().ok_or(Error::MissingSignature)?;

    write.set_value(value);

    Ok(())
  }

  pub async fn sign<X>(data: &X, remote_key: &RemoteKey<'_>) -> Result<ProofValue>
  where
    X: Serialize,
  {
    let message: Vec<u8> = data.to_jcs()?;
    let signature: Vec<u8> = RemoteSign::sign(&message, remote_key).await?.into();
    let signature: String = encode_b58(&signature);
    Ok(ProofValue::Signature(signature))
  }
}

/// A reference to a storage instance and identity key location.
#[derive(Debug)]
pub struct RemoteKey<'a> {
  did: &'a IotaDID,
  location: &'a KeyLocation,
  store: &'a dyn Storage,
}

impl<'a> RemoteKey<'a> {
  /// Creates a new `RemoteKey` instance.
  pub fn new(did: &'a IotaDID, location: &'a KeyLocation, store: &'a dyn Storage) -> Self {
    Self { did, location, store }
  }
}

// =============================================================================
// RemoteSign
// =============================================================================

/// A remote signature that delegates to a storage implementation.
///
/// Note: The signature implementation is specified by the associated `RemoteKey`.
#[derive(Clone, Copy, Debug)]
pub struct RemoteSign<'a> {
  marker: PhantomData<RemoteKey<'a>>,
}

impl<'a> RemoteSign<'a> {
  pub async fn sign(message: &[u8], key: &RemoteKey<'a>) -> Result<StorageSignature> {
    key
      .store
      .key_sign(key.did, key.location, message.to_vec())
      .await
      .map_err(|_| Error::InvalidProofValue("remote sign"))
  }
}
