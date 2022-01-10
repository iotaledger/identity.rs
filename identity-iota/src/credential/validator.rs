// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use serde::de::DeserializeOwned;
use serde::Serialize;

use identity_core::common::Object;
use identity_core::convert::FromJson;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::verifiable::VerifierOptions;

use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::TangleResolve;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CredentialValidation<T = Object> {
  pub credential: Credential<T>,
  pub issuer: DocumentValidation,
  pub subjects: BTreeMap<String, DocumentValidation>,
  pub verified: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PresentationValidation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: DocumentValidation,
  pub credentials: Vec<CredentialValidation<U>>,
  pub verified: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DocumentValidation {
  pub did: IotaDID,
  pub document: ResolvedIotaDocument,
  pub metadata: Object,
  pub verified: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct CredentialValidator<'a, R: TangleResolve = Client> {
  client: &'a R,
}

impl<'a, R: TangleResolve> CredentialValidator<'a, R> {
  /// Creates a new `CredentialValidator`.
  pub fn new(client: &'a R) -> Self {
    Self { client }
  }

  /// Deserializes the given JSON-encoded `Credential` and validates
  /// all associated DID documents.
  pub async fn check_credential<T>(&self, data: &str, options: VerifierOptions) -> Result<CredentialValidation<T>>
  where
    T: DeserializeOwned + Serialize,
  {
    self.validate_credential(Credential::from_json(data)?, options).await
  }

  /// Deserializes the given JSON-encoded `Presentation` and
  /// validates all associated DID documents/`Credential`s.
  pub async fn check_presentation<T, U>(
    &self,
    data: &str,
    options: VerifierOptions,
  ) -> Result<PresentationValidation<T, U>>
  where
    T: Clone + DeserializeOwned + Serialize,
    U: Clone + DeserializeOwned + Serialize,
  {
    self
      .validate_presentation(Presentation::from_json(data)?, options)
      .await
  }

  /// Validates the `Credential` proof and all relevant DID documents.
  ///
  /// Note: The credential is expected to have a proof created by the issuing party.
  /// Note: The credential issuer URL is expected to be a valid DID.
  /// Note: Credential subject IDs are expected to be valid DIDs (if present).
  pub async fn validate_credential<T>(
    &self,
    credential: Credential<T>,
    options: VerifierOptions,
  ) -> Result<CredentialValidation<T>>
  where
    T: Serialize,
  {
    // Resolve the issuer DID Document and validate the digital signature.
    let issuer_url: &str = credential.issuer.url().as_str();
    let issuer_doc: DocumentValidation = self.validate_document(issuer_url).await?;

    let mut subjects: BTreeMap<String, DocumentValidation> = BTreeMap::new();

    // Resolve all credential subjects with `id`s - we assume all ids are DIDs.
    for id in credential
      .credential_subject
      .iter()
      .filter_map(|subject| subject.id.as_ref())
    {
      subjects.insert(id.to_string(), self.validate_document(id.as_str()).await?);
    }

    // Verify the credential signature using the issuers DID Document
    let credential_verified: bool = issuer_doc.document.document.verify_data(&credential, options).is_ok();

    // Check if all subjects have valid signatures
    let subjects_verified: bool = subjects.values().all(|subject| subject.verified);

    // The credential is truly verified if all associated documents are verified
    let verified: bool = issuer_doc.verified && credential_verified && subjects_verified;

    Ok(CredentialValidation {
      credential,
      issuer: issuer_doc,
      subjects,
      verified,
    })
  }

  /// Validates the `Presentation` proof and all relevant DID documents.
  ///
  /// Note: The presentation holder is expected to be a valid DID.
  /// Note: The presentation is expected to have a proof created by the holder.
  /// Note: `options` only affects validation of the signature on the presentation and does not
  ///       affect the validation of any of its credentials.
  pub async fn validate_presentation<T, U>(
    &self,
    presentation: Presentation<T, U>,
    options: VerifierOptions,
  ) -> Result<PresentationValidation<T, U>>
  where
    T: Clone + Serialize,
    U: Clone + Serialize,
  {
    let holder_url: &str = presentation
      .holder
      .as_ref()
      .map(|holder| holder.as_str())
      .ok_or(Error::InvalidPresentationHolder)?;

    // Resolve the holder DID Document and validate the digital signature.
    let holder_doc: DocumentValidation = self.validate_document(holder_url).await?;

    let mut credentials: Vec<CredentialValidation<U>> = Vec::new();

    // Resolve and validate all associated credentials.
    for credential in presentation.verifiable_credential.iter() {
      // TODO: do we need to allow different VerifierOptions for each credential?
      credentials.push(
        self
          .validate_credential(credential.clone(), VerifierOptions::default())
          .await?,
      );
    }

    // Verify the presentation signature using the holders DID Document
    let presentation_verified: bool = holder_doc.document.document.verify_data(&presentation, options).is_ok();

    // Check if all credentials are verified
    let credentials_verified: bool = credentials.iter().all(|credential| credential.verified);

    // The presentation is truly verified if all associated documents are verified
    let verified: bool = holder_doc.verified && presentation_verified && credentials_verified;

    Ok(PresentationValidation {
      presentation,
      holder: holder_doc,
      credentials,
      verified,
    })
  }

  /// Resolves the document from the Tangle, which performs checks on all signatures etc.
  async fn validate_document(&self, did: impl AsRef<str>) -> Result<DocumentValidation> {
    let did: IotaDID = did.as_ref().parse()?;
    let document: ResolvedIotaDocument = self.client.resolve(&did).await?;
    // TODO: check if document is deactivated, does that matter?

    Ok(DocumentValidation {
      did,
      document,
      metadata: Object::new(),
      verified: true,
    })
  }
}
