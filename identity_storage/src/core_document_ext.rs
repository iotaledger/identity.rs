// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::SetSignature;
use identity_did::document::CoreDocument;
use serde::Serialize;

use crate::identity_updater::IdentityUpdater;
use crate::BlobStorage;
use crate::KeyStorage;
use crate::Signable;
use crate::SignatureSuite;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait CoreDocumentExt {
  fn update_identity(&mut self) -> IdentityUpdater;
  async fn sign<K: KeyStorage, B: BlobStorage, VAL>(
    &self,
    value: &mut VAL,
    fragment: &str,
    suite: &SignatureSuite<K, B>,
    proof_options: ProofOptions,
  ) where
    VAL: Serialize + SetSignature + Clone + Into<Signable> + Send + 'static;
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl CoreDocumentExt for CoreDocument {
  fn update_identity(&mut self) -> IdentityUpdater {
    IdentityUpdater { document: self }
  }

  async fn sign<K: KeyStorage, B: BlobStorage, VAL>(
    &self,
    value: &mut VAL,
    fragment: &str,
    suite: &SignatureSuite<K, B>,
    proof_options: ProofOptions,
  ) where
    VAL: Serialize + SetSignature + Clone + Into<Signable> + Send + 'static,
  {
    let method = self.resolve_method(fragment, Default::default()).expect("TODO");

    // TODO: Set only fragment as method_id?

    suite.sign(value, method.id().to_string(), &method, proof_options).await
  }
}
