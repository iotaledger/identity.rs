// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::KeyAlias;
use crate::KeyStorage;
use crate::NewMethodType;
use crate::SigningAlgorithm;
use async_trait::async_trait;

#[cfg(not(feature = "send-sync-storage"))]
mod signature_handler_sub_trait {
  pub trait SignatureHandlerSendSyncMaybe {}
  impl<S: super::SignatureHandler> SignatureHandlerSendSyncMaybe for S {}
}

#[cfg(feature = "send-sync-storage")]
mod signature_handler_sub_trait {
  pub trait SignatureHandlerSendSyncMaybe: Send + Sync {}
  impl<S: Send + Sync + super::SignatureHandler> SignatureHandlerSendSyncMaybe for S {}
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait SignatureHandler: signature_handler_sub_trait::SignatureHandlerSendSyncMaybe {
  fn typ(&self) -> NewMethodType;
  async fn sign(&self, data: Vec<u8>, key_storage: &dyn KeyStorage) -> Vec<u8>;
}

pub struct JcsEd25519;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl SignatureHandler for JcsEd25519 {
  fn typ(&self) -> NewMethodType {
    NewMethodType::ed25519_verification_key_2018()
  }

  async fn sign(&self, data: Vec<u8>, key_storage: &dyn KeyStorage) -> Vec<u8> {
    // TODO: Alias needs to be passed in.
    let private_key: KeyAlias = KeyAlias::new("random_string");
    key_storage
      .sign(&private_key, SigningAlgorithm::Ed25519, data)
      .await
      .expect("TODO")
      .0
  }
}
