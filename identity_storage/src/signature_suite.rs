// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::KeyAlias;
use crate::KeyStorage;
use crate::Signature;
use crate::SigningAlgorithm;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait SignatureSuite {
  async fn sign(&self, alias: &KeyAlias, data: Vec<u8>) -> Signature;
}

pub struct JcsEd25519SignatureSuite<K: KeyStorage> {
  storage: K,
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K: KeyStorage> SignatureSuite for JcsEd25519SignatureSuite<K> {
  async fn sign(&self, alias: &KeyAlias, data: Vec<u8>) -> Signature {
    self.storage.sign(alias, SigningAlgorithm::Ed25519, data).await.unwrap()
  }
}
