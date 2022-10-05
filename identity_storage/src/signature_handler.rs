// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Ed25519Signature;
use crate::KeyAlias;
use crate::KeyStorage;
use crate::NewMethodType;
use async_trait::async_trait;

// #[cfg(not(feature = "send-sync-storage"))]
// mod signature_handler_sub_trait {
//   pub trait SignatureHandlerSendSyncMaybe {}
//   impl<K: KeyStorage, S: super::SignatureHandler<K>> SignatureHandlerSendSyncMaybe for S {}
// }

// #[cfg(feature = "send-sync-storage")]
// mod signature_handler_sub_trait {
//   use crate::KeyStorage;

//   pub trait SignatureHandlerSendSyncMaybe: Send + Sync {}
//   impl<K: KeyStorage, S: Send + Sync + super::SignatureHandler<K>> SignatureHandlerSendSyncMaybe for S {}
// }

// #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
// #[cfg_attr(feature = "send-sync-storage", async_trait)]
// pub trait SignatureHandler<K: KeyStorage> {
//   fn typ(&self) -> NewMethodType;
//   async fn sign(&self, data: Vec<u8>, key_storage: &K) -> Vec<u8>;
// }

#[cfg(feature = "send-sync-storage")]
#[async_trait::async_trait]
pub trait SignatureHandler<K: KeyStorage>: Send + Sync {
  fn typ(&self) -> NewMethodType;
  async fn sign(&self, data: Vec<u8>, key_storage: &K) -> Vec<u8>;
}

#[cfg(not(feature = "send-sync-storage"))]
#[async_trait::async_trait(?Send)]
pub trait SignatureHandler<K: KeyStorage> {
  fn typ(&self) -> NewMethodType;
  async fn sign(&self, data: Vec<u8>, key_storage: &K) -> Vec<u8>;
}

pub struct JcsEd25519;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K> SignatureHandler<K> for JcsEd25519
where
  K: KeyStorage,
  K::SigningAlgorithm: From<Ed25519Signature>,
{
  fn typ(&self) -> NewMethodType {
    NewMethodType::ed25519_verification_key_2018()
  }

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
