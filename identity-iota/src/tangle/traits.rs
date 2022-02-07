// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::FromJson;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::verifiable::VerifierOptions;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::credential::CredentialValidationOptions;
use crate::credential::PresentationValidationOptions;
use crate::credential::ResolvedCredential;
use crate::credential::ResolvedPresentation;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;

use crate::tangle::MessageId;

pub trait TangleRef {
  fn did(&self) -> &IotaDID;

  fn message_id(&self) -> &MessageId;

  fn set_message_id(&mut self, message_id: MessageId);

  fn previous_message_id(&self) -> &MessageId;

  fn set_previous_message_id(&mut self, message_id: MessageId);
}

// TODO: remove TangleResolve with ClientMap refactor?
#[async_trait::async_trait(?Send)]
pub trait TangleResolve {
  /// Resolves a DID on the Tangle
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument>;

  /// Resolves and validates a `Credential` in accordance with the given `validation_options`.
  ///
  /// # Errors
  /// Fails if the credential does not meet the criteria set in `validation_options` or if the DID Document of the
  /// issuer or any credential subjects cannot be resolved.
  ///
  /// This method fails on the first encountered error if the `fail_fast` parameter is true,
  /// otherwise all encountered errors will be aggregated in the returned error.
  // Todo: Should we declare in the documentation that one may only match on the `CredentialResolution` variant of
  // `Error` when this function fails?
  async fn resolve_credential<T: Serialize>(
    &self,
    credential: Credential<T>,
    validation_options: &CredentialValidationOptions,
    fail_fast: bool,
  ) -> Result<ResolvedCredential<T>> {
    crate::credential::resolution::resolve_credential(self, credential, validation_options, fail_fast)
      .await
      .map_err(Error::CredentialResolution)
  }

  ///  Resolves a json-encoded `Credential` and validates it in accordance with the given `validation_options`.
  ///
  /// # Errors
  /// Fails if the credential does not meet the criteria set in `validation_options` or if the DID Document of the
  /// issuer or any credential subjects cannot be resolved.
  ///
  /// This method fails on the first encountered error if the `fail_fast` parameter is true,
  /// otherwise all encountered errors will be aggregated in the returned error.
  async fn resolve_credential_json<T: DeserializeOwned + Serialize>(
    &self,
    data: &str,
    validation_options: &CredentialValidationOptions,
    fail_fast: bool,
  ) -> Result<ResolvedCredential<T>> {
    self
      .resolve_credential(Credential::from_json(data)?, validation_options, fail_fast)
      .await
  }

  /// Resolves the `Credential` and verifies its signature using the issuers DID Document.
  ///
  /// # Errors
  /// Fails if the DID Document of the issuer or any of the credential subjects cannot be resolved,
  /// or the credential signature cannot be verified using the issuer's DID Document.
  // Todo: Should we declare in the documentation that one may only match on the `CredentialResolution` variant of
  // `Error` when this function fails?
  async fn resolve_credential_unvalidated<T: Serialize>(
    &self,
    credential: Credential<T>,
    verifier_options: &VerifierOptions,
  ) -> Result<ResolvedCredential<T>> {
    crate::credential::resolution::resolve_credential_unvalidated(self, credential, verifier_options)
      .await
      .map_err(Error::CredentialResolution)
  }

  /// Resolves a json-encoded `Credential` and verifies its signature using the issuers DID Document.
  ///
  /// # Errors
  /// Fails if the DID Document of the issuer or any of the credential subjects cannot be resolved,
  /// or the credential signature cannot be verified using the issuer's DID Document.
  async fn resolve_credential_unvalidated_json<T: DeserializeOwned + Serialize>(
    &self,
    data: &str,
    verifier_options: &VerifierOptions,
  ) -> Result<ResolvedCredential<T>> {
    self
      .resolve_credential_unvalidated(Credential::from_json(data)?, verifier_options)
      .await
  }

  /// Resolves a `Credential` without applying any checks.
  ///
  /// If `fail_on_unresolved_documents` is false then one may not assume a 1-1 relationship between the subjects in
  /// `credential` and subjects in the returned [ResolvedCredential].
  ///
  /// # Errors
  ///
  /// Fails if the issuer's DID Document cannot be resolved, and the same holds for subject DID Documents if
  /// `fail_on_unresolved_subjects` is true.
  // Todo: Should we declare in the documentation that one may only match on the `CredentialResolution` variant of
  // `Error` when this function fails?
  async fn resolve_credential_unchecked<T: Serialize>(
    &self,
    credential: Credential<T>,
    fail_on_unresolved_subjects: bool,
  ) -> Result<ResolvedCredential<T>> {
    crate::credential::resolution::resolve_credential_unchecked(self, credential, fail_on_unresolved_subjects)
      .await
      .map_err(Error::CredentialResolution)
  }

  /// Resolves a json-encoded `Credential` without applying any checks.
  ///
  /// If `fail_on_unresolved_documents` is false then one may not assume a 1-1 relationship between the subjects in
  /// `credential` and subjects in the returned [ResolvedCredential].
  ///
  /// # Errors
  ///
  /// Fails if the issuer's DID Document cannot be resolved, and the same holds for subject DID Documents if
  /// `fail_on_unresolved_subjects` is true.
  async fn resolve_credential_unchecked_json<T: DeserializeOwned + Serialize>(
    &self,
    data: &str,
    fail_on_unresolved_subjects: bool,
  ) -> Result<ResolvedCredential<T>> {
    self
      .resolve_credential_unchecked(Credential::from_json(data)?, fail_on_unresolved_subjects)
      .await
  }

  /// Resolves and validates a `Presentation` in accordance with the given `validation_options`.
  ///
  /// # Errors
  ///
  /// Fails if the presentation does not meet the criteria set in `validation_options`.
  // Todo: Should we declare in the documentation that one may only match on the `PresentationResolution` variant of
  // Èrror` when this function fails?
  async fn resolve_presentation<T, U>(
    &self,
    presentation: Presentation<T, U>,
    validation_options: &PresentationValidationOptions,
    fail_fast: bool,
  ) -> Result<ResolvedPresentation<T, U>>
  where
    T: Serialize + Clone,
    U: Serialize + Clone,
  {
    crate::credential::resolution::resolve_presentation(self, presentation, validation_options, fail_fast)
      .await
      .map_err(Error::PresentationResolution)
  }

  /// Resolves and validates a json-encoded `Presentation` in accordance with the given `validation_options`.
  ///
  /// # Errors
  ///
  /// Fails if the presentation does not meet the criteria set in `validation_options`.
  async fn resolve_presentation_json<T, U>(
    &self,
    data: &str,
    validation_options: &PresentationValidationOptions,
    fail_fast: bool,
  ) -> Result<ResolvedPresentation<T, U>>
  where
    T: DeserializeOwned + Serialize + Clone,
    U: DeserializeOwned + Serialize + Clone,
  {
    self
      .resolve_presentation(Presentation::from_json(data)?, validation_options, fail_fast)
      .await
  }

  /// Resolves the `Presentation` and verifies the signatures of the holder and the issuer of each `Credential`.
  // Todo: Should we declare in the documentation that one may only match on the `PresentationResolution` variant of
  // Èrror` when this function fails?
  async fn resolve_presentation_unvalidated<T, U>(
    &self,
    presentation: Presentation<T, U>,
    verifier_options: &VerifierOptions,
  ) -> Result<ResolvedPresentation<T, U>>
  where
    T: Serialize + Clone,
    U: Serialize + Clone,
  {
    crate::credential::resolution::resolve_presentation_unvalidated(self, presentation, verifier_options)
      .await
      .map_err(Error::PresentationResolution)
  }

  /// Resolves the json-encoded `Presentation` and verifies the signatures of the holder and the issuer of each
  /// `Credential`.
  async fn resolve_presentation_unvalidated_json<T, U>(
    &self,
    data: &str,
    verifier_options: &VerifierOptions,
  ) -> Result<ResolvedPresentation<T, U>>
  where
    T: DeserializeOwned + Serialize + Clone,
    U: DeserializeOwned + Serialize + Clone,
  {
    self
      .resolve_presentation_unvalidated(Presentation::from_json(data)?, verifier_options)
      .await
  }

  /// Resolves the `Presentation` without applying any checks.  
  // Todo: Should we declare in the documentation that one may only match on the `PresentationResolution` variant of
  // Èrror` when this function fails?
  async fn resolve_presentation_unchecked<T, U>(
    &self,
    presentation: Presentation<T, U>,
    fail_on_unresolved_subjects: bool,
    fail_on_unresolved_credentials: bool,
  ) -> Result<ResolvedPresentation<T, U>>
  where
    T: Serialize + Clone,
    U: Serialize + Clone,
  {
    crate::credential::resolution::resolve_presentation_unchecked(
      self,
      presentation,
      fail_on_unresolved_subjects,
      fail_on_unresolved_credentials,
    )
    .await
    .map_err(Error::PresentationResolution)
  }

  /// Resolves the json-encoded `Presentation` without applying any checks.  
  async fn resolve_presentation_unchecked_json<T, U>(
    &self,
    data: &str,
    fail_on_unresolved_subjects: bool,
    fail_on_unresolved_credentials: bool,
  ) -> Result<ResolvedPresentation<T, U>>
  where
    T: DeserializeOwned + Serialize + Clone,
    U: DeserializeOwned + Serialize + Clone,
  {
    self
      .resolve_presentation_unchecked(
        Presentation::from_json(data)?,
        fail_on_unresolved_subjects,
        fail_on_unresolved_credentials,
      )
      .await
  }
}

#[cfg(test)]
mod tests {}
