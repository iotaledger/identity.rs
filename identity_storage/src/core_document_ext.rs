// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_did::document::CoreDocument;

use crate::identity_updater::IdentityUpdater;
use crate::IdentitySuite;
use crate::NewMethodType;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait CoreDocumentExt {
  fn update_identity(&mut self) -> IdentityUpdater;
  async fn sign(&self, fragment: &str, data: Vec<u8>, suite: &IdentitySuite) -> Vec<u8>;
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl CoreDocumentExt for CoreDocument {
  fn update_identity(&mut self) -> IdentityUpdater {
    IdentityUpdater { document: self }
  }

  async fn sign(&self, fragment: &str, data: Vec<u8>, suite: &IdentitySuite) -> Vec<u8> {
    let method = self.resolve_method(fragment, Default::default()).expect("TODO");
    // This would return NewMethodType after a refactoring.
    let new_method_type = match method.type_() {
      identity_did::verification::MethodType::Ed25519VerificationKey2018 => {
        NewMethodType::ed25519_verification_key_2018()
      }
      identity_did::verification::MethodType::X25519KeyAgreementKey2019 => {
        NewMethodType::x25519_verification_key_2018()
      }
    };

    suite.sign(data, &new_method_type).await
  }
}
