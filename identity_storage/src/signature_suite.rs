// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use identity_core::crypto::GetSignature;
use identity_core::crypto::GetSignatureMut;
use identity_core::crypto::Proof;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::ProofValue;
use identity_core::crypto::SetSignature;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
use serde::Serialize;

use crate::method_hash::MethodHash;
use crate::BlobStorage;
use crate::KeyId;
use crate::KeyStorage;
use crate::SignatureHandler;
use crate::SignatureMethodType;
use crate::Storage;

pub struct SignatureSuite<K: KeyStorage, B: BlobStorage> {
  storage: Storage<K, B>,
  signature_handlers: HashMap<MethodType, Box<dyn SignatureHandler<K>>>,
}

impl<K: KeyStorage, B: BlobStorage> SignatureSuite<K, B> {
  pub fn new(storage: Storage<K, B>) -> Self {
    Self {
      storage,
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
  pub fn register_unchecked(&mut self, method_type: MethodType, handler: Box<dyn SignatureHandler<K>>) {
    self.signature_handlers.insert(method_type, handler);
  }

  pub async fn sign<VAL>(
    &self,
    value: &mut VAL,
    method_id: impl Into<String>,
    method: &VerificationMethod,
    proof_options: ProofOptions,
  ) where
    VAL: Serialize + SetSignature + Clone + Into<Signable>,
  {
    let proof_value: ProofValue = {
      let signable: Signable = value.clone().into();

      let method_hash = MethodHash::from_verification_method(method).expect("TODO");
      let key_id: KeyId = KeyId::try_from(
        self
          .storage
          .load(&method_hash.to_string())
          .await
          .expect("TODO")
          .expect("TODO"),
      )
      .expect("TODO");

      match self.signature_handlers.get(method.type_()) {
        Some(handler) => {
          let signature: Proof = Proof::new_with_options(handler.signature_name(), method_id, proof_options);
          value.set_signature(signature);
          handler
            .sign(signable, key_id, &self.storage.key_storage)
            .await
            .expect("TODO")
        }
        None => todo!("signature suite: return missing handler error"),
      }
    };

    let write: &mut Proof = value
      .signature_mut()
      .expect("the signature should have been set if a handler was found");
    write.set_value(proof_value);
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
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

impl GetSignature for Signable {
  fn signature(&self) -> Option<&Proof> {
    match self {
      Signable::Credential(cred) => cred.signature(),
      Signable::Presentation(pres) => pres.signature(),
      Signable::Json(_) => None,
    }
  }
}

impl GetSignatureMut for Signable {
  fn signature_mut(&mut self) -> Option<&mut Proof> {
    match self {
      Signable::Credential(cred) => cred.signature_mut(),
      Signable::Presentation(pres) => pres.signature_mut(),
      Signable::Json(_) => None,
    }
  }
}

impl SetSignature for Signable {
  fn set_signature(&mut self, signature: Proof) {
    match self {
      Signable::Credential(cred) => cred.set_signature(signature),
      Signable::Presentation(pres) => pres.set_signature(signature),
      Signable::Json(_) => unimplemented!("fix me"),
    }
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
