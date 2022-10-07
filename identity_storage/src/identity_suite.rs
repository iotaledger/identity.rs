// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::KeyStorage;
use crate::MethodType1;
use crate::SignatureHandler;
use crate::SignatureMethodType;

pub struct IdentitySuite<K: KeyStorage> {
  key_storage: K,
  signature_handlers: HashMap<MethodType1, Box<dyn SignatureHandler<K>>>,
}

impl<K: KeyStorage> IdentitySuite<K> {
  pub fn new(key_storage: K) -> Self {
    Self {
      key_storage,
      signature_handlers: HashMap::new(),
    }
  }

  pub fn register<SIG>(&mut self, handler: SIG)
  where
    SIG: SignatureHandler<K> + 'static,
    SIG: SignatureMethodType,
  {
    self.signature_handlers.insert(SIG::name(), Box::new(handler));
  }

  #[cfg(target_family = "wasm")]
  pub fn register_unchecked(&mut self, signature_name: MethodType1, handler: Box<dyn SignatureHandler<K>>) {
    self.signature_handlers.insert(signature_name, handler);
  }

  pub async fn sign(&self, method_type: &MethodType1, data: Vec<u8>) -> Vec<u8> {
    match self.signature_handlers.get(method_type) {
      Some(handler) => handler.sign(data, &self.key_storage).await,
      None => todo!("return missing handler error"),
    }
  }
}
