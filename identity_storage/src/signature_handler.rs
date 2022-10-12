// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::ProofValue;

use crate::KeyStorage;
use crate::MethodType1;
use crate::Signable;
use crate::StorageResult;

#[cfg(feature = "send-sync-storage")]
#[async_trait::async_trait]
pub trait SignatureHandler<K: KeyStorage>: Send + Sync {
  fn signature_name(&self) -> String;
  async fn sign(&self, value: Signable, key_storage: &K) -> StorageResult<ProofValue>;
}

#[cfg(not(feature = "send-sync-storage"))]
#[async_trait::async_trait(?Send)]
pub trait SignatureHandler<K: KeyStorage> {
  fn signature_name(&self) -> String;
  async fn sign(&self, value: Signable, key_storage: &K) -> StorageResult<ProofValue>;
}

pub trait SignatureMethodType {
  fn method_type() -> MethodType1;
}
