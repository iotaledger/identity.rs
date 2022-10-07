// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Ed25519Signature;
use crate::KeyAlias;
use crate::KeyStorage;
use crate::MethodType1;
use async_trait::async_trait;

#[cfg(feature = "send-sync-storage")]
#[async_trait::async_trait]
pub trait SignatureHandler<K: KeyStorage>: Send + Sync {
  async fn sign(&self, data: Vec<u8>, key_storage: &K) -> Vec<u8>;
}

#[cfg(not(feature = "send-sync-storage"))]
#[async_trait::async_trait(?Send)]
pub trait SignatureHandler<K: KeyStorage> {
  async fn sign(&self, data: Vec<u8>, key_storage: &K) -> Vec<u8>;
}

pub trait SignatureMethodType {
  fn name() -> MethodType1;
}

pub struct JcsEd25519;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K> SignatureHandler<K> for JcsEd25519
where
  K: KeyStorage,
  K::SigningAlgorithm: From<Ed25519Signature>,
{
  async fn sign(&self, data: Vec<u8>, key_storage: &K) -> Vec<u8> {
    // TODO: Alias needs to be passed in.
    let private_key: KeyAlias = KeyAlias::new("random_string");
    key_storage
      .sign(&private_key, Ed25519Signature, data)
      .await
      .expect("TODO")
      .0
  }
}

impl SignatureMethodType for JcsEd25519 {
  /// Returns the method type of a signature handler.
  fn name() -> MethodType1 {
    MethodType1::ed_25519_verification_key_2018()
  }
}
