// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::SetSignature;
use identity_did::document::CoreDocument;
use serde::Serialize;

use crate::identity_updater::IdentityUpdater;
use crate::KeyStorage;
use crate::Signable;
use crate::SignatureSuite;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait CoreDocumentExt {
  fn update_identity(&mut self) -> IdentityUpdater;
  async fn sign<K: KeyStorage, VAL>(
    &self,
    value: &mut VAL,
    fragment: &str,
    suite: &SignatureSuite<K>,
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

  async fn sign<K: KeyStorage, VAL>(
    &self,
    value: &mut VAL,
    fragment: &str,
    suite: &SignatureSuite<K>,
    proof_options: ProofOptions,
  ) where
    VAL: Serialize + SetSignature + Clone + Into<Signable> + Send + 'static,
  {
    let method = self.resolve_method(fragment, Default::default()).expect("TODO");

    // TODO: Remove after refactoring VerificationMethod to hold the new MethodType.
    let method_type = method.type_();

    // TODO: Set only fragment as method_id?

    suite
      .sign(value, method.id().to_string(), &method_type, proof_options)
      .await
  }
}
