// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_did::document::CoreDocument;

use crate::identity_updater::IdentityUpdater;
use crate::IdentitySuite;
use crate::KeyStorage;
use crate::MethodType1;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait CoreDocumentExt {
  fn update_identity(&mut self) -> IdentityUpdater;
  async fn sign<K: KeyStorage>(&self, fragment: &str, data: Vec<u8>, suite: &IdentitySuite<K>) -> Vec<u8>;
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl CoreDocumentExt for CoreDocument {
  fn update_identity(&mut self) -> IdentityUpdater {
    IdentityUpdater { document: self }
  }

  async fn sign<K: KeyStorage>(&self, fragment: &str, data: Vec<u8>, suite: &IdentitySuite<K>) -> Vec<u8> {
    let method = self.resolve_method(fragment, Default::default()).expect("TODO");

    // TODO: Remove after refactoring VerificationMethod to hold the new MethodType.
    let method_type = match method.type_() {
      identity_did::verification::MethodType::Ed25519VerificationKey2018 => {
        MethodType1::ed_25519_verification_key_2018()
      }
      identity_did::verification::MethodType::X25519KeyAgreementKey2019 => MethodType1::x_25519_verification_key_2018(),
    };

    suite.sign(&method_type, data).await
  }
}
