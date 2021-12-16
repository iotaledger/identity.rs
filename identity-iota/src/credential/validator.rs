// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::marker::PhantomData;

use either::Either;
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

use self::private::Refuted;
use self::private::Sealed;
use self::private::Unspecified;
use self::private::Verified;

/* 
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
*/ 

mod private {
  pub trait Sealed {}
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Verified; 
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Refuted; 

  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Unspecified; 
  impl Sealed for Verified {}
  impl Sealed for Refuted {}
  impl Sealed for Unspecified {} 
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DocumentValidation<T = private::Verified> 
  where T: private::Sealed 
{
  pub did: IotaDID, 
  pub document: ResolvedIotaDocument, 
  pub metadata: Object,
  _marker: PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
/// A credential validation that may or may not have been completely successful 
// todo: Add more explanations here 
pub struct PartialCredentialValidation<Filter = Unspecified, T = Object>
    where Filter: private::Sealed
{
  pub credential: Credential<T>,
  issuer: Either<DocumentValidation, DocumentValidation<Refuted>>,
  verified_subjects: Option<BTreeMap<String, DocumentValidation>>,
  refuted_subjects: Option<BTreeMap<String, DocumentValidation<Refuted>>>,
  encountered_refutation_categories: HashSet<CredentialValidationRefutationCategory>, 
  _marker: PhantomData<Filter>,
}

impl<Filter: Sealed, T: Sealed> PartialCredentialValidation<Filter, T> {

  pub fn successful_validation(&self) -> bool {
    self.encountered_refutation_categories.len() == 0 
  }

  pub fn valid_signature(&self) -> bool {
    !self.encountered_refutation_categories.contains(&CredentialValidationRefutationCategory::InvalidCredentialSignature) 
  }

  pub fn with_verified_filter(self) -> PartialCredentialValidation<Verified, T> {
    let Self {credential, issuer, verified_subjects, refuted_subjects: unverified_subjects, encountered_refutation_categories: encountered_error_categories,..} = self; 
    PartialCredentialValidation::<Verified,T>{ credential, issuer, verified_subjects, refuted_subjects: unverified_subjects, encountered_refutation_categories: encountered_error_categories, _marker: PhantomData::<Verified> }
  }

  pub fn with_refuted_filter(self) -> PartialCredentialValidation<Refuted, T> {
    let Self {credential, issuer, verified_subjects, refuted_subjects: unverified_subjects, encountered_refutation_categories: encountered_error_categories,..} = self; 
    PartialCredentialValidation::<Refuted,T>{ credential, issuer, verified_subjects, refuted_subjects: unverified_subjects, encountered_refutation_categories: encountered_error_categories, _marker: PhantomData::<Refuted>}
  }


  pub fn encountered_refutation_categories(&self) -> impl Iterator<Item = &CredentialValidationRefutationCategory> {
    self.encountered_refutation_categories.iter()
  }
}

impl PartialCredentialValidation<Verified> {
  pub fn issuer(&self) -> Option<&DocumentValidation> {
    self.issuer.left().as_ref()
  }

  pub fn subjects(&self) -> Option<&BTreeMap<String, DocumentValidation>> {
    self.verified_subjects.as_ref()
  }
}

impl PartialCredentialValidation<Refuted> {
  pub fn issuer(&self) -> Option<&DocumentValidation<Refuted>> {
    self.issuer.right().as_ref()
  }

  pub fn subjects(&self) -> Option<&BTreeMap<String, DocumentValidation<Refuted>>> {
    self.refuted_subjects.as_ref()
  }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CredentialValidation<T = Object> {
  pub credential: Credential<T>,
  pub issuer: DocumentValidation,
  pub subjects: BTreeMap<String, DocumentValidation>,
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
    /* 
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
    */
    todo!()
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
    /*
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
    */
    todo!()
  }

  /// Resolves the document from the Tangle, which performs checks on all signatures etc.
  async fn validate_document(&self, did: impl AsRef<str>) -> Result<Either<DocumentValidation,DocumentValidation<Refuted>>> {
    let did: IotaDID = did.as_ref().parse()?;
    let document: ResolvedIotaDocument = self.client.resolve(&did).await?;
    // TODO: check if document is deactivated, does that matter? If it does then we return the Right variant 
    Ok(Either::Left(DocumentValidation{did, document, metadata: Object::new(), _marker: PhantomData::<Verified>}))
  }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize)]
enum CredentialValidationRefutationCategory {
  // The credential signature does not match the expected value 
  InvalidCredentialSignature,
  // At least one subject document is not verified 
  InvalidSubjectDocuments,
  // The issuers document is not verified 
  InvalidIssuerDocument,
  // The credential has expired 
  Expired, 
  // The credential has not yet become active (issuance_date is in the future)
  Dormant 
}
