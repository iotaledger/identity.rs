// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use async_trait::async_trait;
use identity_did::verification::MethodData;
use identity_did::verification::MethodType;

use crate::BlobStorage;
use crate::Ed25519KeyType;
use crate::KeyAlias;
use crate::KeyStorage;
use crate::MethodContent;
use crate::Storage;

pub struct MethodSuite<K, B>
where
  K: KeyStorage,
  B: BlobStorage,
{
  pub(crate) storage: Storage<K, B>,
  method_handlers: HashMap<MethodType, Box<dyn MethodHandler<K>>>,
}

impl<K, B> MethodSuite<K, B>
where
  K: KeyStorage,
  B: BlobStorage,
{
  pub fn new(storage: Storage<K, B>) -> Self {
    Self {
      storage,
      method_handlers: HashMap::new(),
    }
  }

  pub fn register<MET>(&mut self, handler: MET)
  where
    MET: MethodHandler<K> + 'static,
  {
    self.method_handlers.insert(handler.method_type(), Box::new(handler));
  }

  pub async fn create(&self, method_type: &MethodType, method_content: MethodContent) -> (KeyAlias, MethodData) {
    match self.method_handlers.get(method_type) {
      Some(handler) => handler.create(method_content, &self.storage.key_storage).await,
      None => todo!("method suite: return missing handler error"),
    }
  }

  #[cfg(target_family = "wasm")]
  pub fn register_unchecked(&mut self, method_type: MethodType, handler: Box<dyn MethodHandler<K>>) {
    self.method_handlers.insert(method_type, handler);
  }
}

#[cfg(feature = "send-sync-storage")]
#[async_trait::async_trait]
pub trait MethodHandler<K: KeyStorage>: Send + Sync {
  fn method_type(&self) -> MethodType;
  async fn create(&self, method_content: MethodContent, key_storage: &K) -> (KeyAlias, MethodData);
}

#[cfg(not(feature = "send-sync-storage"))]
#[async_trait::async_trait(?Send)]
pub trait MethodHandler<K: KeyStorage> {
  fn method_type(&self) -> MethodType;
  async fn create(&self, method_content: MethodContent, key_storage: &K) -> (KeyAlias, MethodData);
}

pub struct Ed25519VerificationKey2018;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl<K> MethodHandler<K> for Ed25519VerificationKey2018
where
  K: KeyStorage,
  K::KeyType: From<Ed25519KeyType> + Send,
{
  fn method_type(&self) -> MethodType {
    MethodType::ED25519_VERIFICATION_KEY_2018
  }

  async fn create(&self, method_content: MethodContent, key_storage: &K) -> (KeyAlias, MethodData) {
    if let MethodContent::Generate = method_content {
      let key_type: K::KeyType = K::KeyType::from(Ed25519KeyType);
      let key_alias: KeyAlias = key_storage.generate(key_type).await.expect("TODO");

      let pubkey = key_storage.public(&key_alias).await.expect("TODO");

      let method_data: MethodData = MethodData::new_base58(pubkey.as_ref());

      (key_alias, method_data)
    } else {
      unimplemented!("{method_content:?}")
    }
  }
}
