// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use identity_core::crypto::Proof;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::ProofValue;
use identity_core::crypto::SetSignature;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use serde::Serialize;

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
    SIG: SignatureHandler<K> + SignatureMethodType + 'static,
  {
    self.signature_handlers.insert(SIG::method_type(), Box::new(handler));
  }

  #[cfg(target_family = "wasm")]
  pub fn register_unchecked(&mut self, signature_name: MethodType1, handler: Box<dyn SignatureHandler<K>>) {
    self.signature_handlers.insert(signature_name, handler);
  }

  pub async fn sign<VAL>(
    &self,
    value: &mut VAL,
    method_id: impl Into<String>,
    method_type: &MethodType1,
    proof_options: ProofOptions,
  ) where
    VAL: Serialize + SetSignature + Clone + Into<Signable>,
  {
    let proof_value: ProofValue = {
      let signable: Signable = value.clone().into();

      match self.signature_handlers.get(method_type) {
        Some(handler) => {
          let signature: Proof = Proof::new_with_options(handler.signature_name(), method_id, proof_options);
          value.set_signature(signature);
          handler.sign(signable, &self.key_storage).await.expect("TODO")
        }
        None => todo!("return missing handler error"),
      }
    };

    let write: &mut Proof = value
      .signature_mut()
      .expect("the signature should have been set if a handler was found");
    write.set_value(proof_value);
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum Signable {
  Credential(Credential),
  Presentation(Presentation),
  Json(serde_json::Value),
}

impl From<Credential> for Signable {
  fn from(credential: Credential) -> Self {
    Signable::Credential(credential)
  }
}

impl From<Presentation> for Signable {
  fn from(presentation: Presentation) -> Self {
    Signable::Presentation(presentation)
  }
}

impl From<serde_json::Value> for Signable {
  fn from(json: serde_json::Value) -> Self {
    Signable::Json(json)
  }
}

// TODO: Try using this enum instead for efficiency.
// #[derive(Debug, Clone, Serialize)]
// pub enum Signable<'a> {
//   #[serde(serialize_with = "__serde_forward::serialize_credential")]
//   Credential(&'a Credential),
//   #[serde(serialize_with = "__serde_forward::serialize_presentation")]
//   Presentation(&'a Presentation),
//   Json(serde_json::Value),
// }

// mod __serde_forward {
//   use identity_credential::{credential::Credential, presentation::Presentation};
//   use serde::{Serialize, Serializer};

//   pub(crate) fn serialize_credential<S>(credential: &&Credential, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     Serialize::serialize(*credential, serializer)
//   }

//   pub(crate) fn serialize_presentation<S>(presentation: &&Presentation, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     Serialize::serialize(*presentation, serializer)
//   }
// }
