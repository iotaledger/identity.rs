// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::convert::FromJson;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::client::Client;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Error;
use crate::error::Result;

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
  pub document: IotaDocument,
  pub metadata: Object,
  pub verified: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct CredentialValidator<'a> {
  client: &'a Client,
}

impl<'a> CredentialValidator<'a> {
  /// Creates a new `CredentialValidator`.
  pub const fn new(client: &'a Client) -> Self {
    Self { client }
  }

  /// Deserializes the given JSON-encoded `Credential` and validates
  /// all associated DID documents.
  pub async fn check<T>(&self, data: &str) -> Result<CredentialValidation<T>>
  where
    T: DeserializeOwned + Serialize,
  {
    self.validate_credential(Credential::from_json(data)?).await
  }

  /// Deserializes the given JSON-encoded `Presentation` and
  /// validates all associated DID documents/`Credential`s.
  pub async fn check_presentation<T, U>(&self, data: &str) -> Result<PresentationValidation<T, U>>
  where
    T: Clone + DeserializeOwned + Serialize,
    U: Clone + DeserializeOwned + Serialize,
  {
    self.validate_presentation(Presentation::from_json(data)?).await
  }

  /// Validates the `Credential` proof and all relevant DID documents.
  ///
  /// Note: The credential is expected to have a proof created by the issuing party.
  /// Note: The credential issuer URL is expected to be a valid DID.
  /// Note: Credential subject IDs are expected to be valid DIDs (if present).
  pub async fn validate_credential<T>(&self, credential: Credential<T>) -> Result<CredentialValidation<T>>
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
    let credential_verified: bool = issuer_doc.document.verify_data(&credential).is_ok();

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
  pub async fn validate_presentation<T, U>(
    &self,
    presentation: Presentation<T, U>,
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
      credentials.push(self.validate_credential(credential.clone()).await?);
    }

    // Verify the presentation signature using the holders DID Document
    let presentation_verified: bool = holder_doc.document.verify_data(&presentation).is_ok();

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

  async fn validate_document(&self, did: &str) -> Result<DocumentValidation> {
    let did: IotaDID = did.parse()?;
    let document: IotaDocument = self.client.read_document(&did).await?;
    let verified: bool = document.verify().is_ok();

    Ok(DocumentValidation {
      did,
      document,
      metadata: Object::new(),
      verified,
    })
  }
}
