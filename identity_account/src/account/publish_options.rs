// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Options to customize how identities are published to the Tangle.
#[derive(Debug, Clone)]
pub struct PublishOptions {
  pub(crate) force_integration_update: bool,
  pub(crate) sign_with: Option<String>,
}

impl PublishOptions {
  /// Creates a new set of default publish options.
  pub fn new() -> Self {
    Self {
      force_integration_update: false,
      sign_with: None,
    }
  }

  /// Whether to force the publication to be an integration update.
  ///
  /// If this option is not set, the account automatically determines whether
  /// an update needs to be published as an integration or a diff update.
  /// Publishing as an integration update is always valid, but not recommended
  /// for identities with many updates.
  ///
  /// See the IOTA DID method specification for more details.
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  #[must_use]
  pub fn force_integration_update(mut self, force: bool) -> Self {
    self.force_integration_update = force;
    self
  }

  /// Set the fragment of a verification method with which to sign the update.
  /// This must point to an Ed25519 method with a capability invocation
  /// verification relationship.
  ///
  /// If this is not set, the method used is determined by
  /// [`IotaDocument`][identity_iota_core::document::IotaDocument::default_signing_method]
  #[must_use]
  pub fn sign_with(mut self, fragment: impl Into<String>) -> Self {
    self.sign_with = Some(fragment.into());
    self
  }
}

impl Default for PublishOptions {
  fn default() -> Self {
    Self::new()
  }
}
