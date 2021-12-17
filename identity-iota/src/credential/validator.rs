// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::marker::PhantomData;
use super::{RefutedCredentialDismissalError, RefutedPresentationDismissalError, CredentialRefutationCategory, PresentationRefutationCategory};

use either::Either;
use identity_core::common::Timestamp;
use indexmap::IndexSet;
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

use self::document_state::Deactivated;
use self::document_state::Sealed;
use self::document_state::Unspecified;
use self::document_state::Active;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PresentationValidation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: DocumentValidation,
  pub credentials: Vec<CredentialValidation<U>>,
  pub verified: bool,
}

mod document_state {
  pub trait Sealed {}
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Active;
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Deactivated;

  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Unspecified;

  impl Sealed for Active {}
  impl Sealed for Deactivated {}
  impl Sealed for Unspecified {}
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DocumentValidation<T = document_state::Active>
where
  T: document_state::Sealed,
{
  pub did: IotaDID,
  pub document: ResolvedIotaDocument,
  pub metadata: Object,
  _marker: PhantomData<T>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
/// A credential validation that may or may not have been completely successful
// todo: Add more explanations here
pub struct ProvisionalCredentialValidation<Filter = Unspecified, T = Object>
where
  Filter: document_state::Sealed,
{
  pub credential: Credential<T>,
  issuer: Either<DocumentValidation<Active>, DocumentValidation<Deactivated>>,
  verified_subjects: Option<BTreeMap<String, DocumentValidation<Active>>>,
  refuted_subjects: Option<BTreeMap<String, DocumentValidation<Deactivated>>>,
  encountered_refutation_categories: IndexSet<CredentialRefutationCategory>,
  _marker: PhantomData<Filter>,
}

impl<Filter: Sealed, T> ProvisionalCredentialValidation<Filter, T> {
  pub fn successful_validation(&self) -> bool {
    self.encountered_refutation_categories.len() == 0
  }

  pub fn valid_signature(&self) -> bool {
    !self
      .encountered_refutation_categories
      .contains(&CredentialRefutationCategory::InvalidSignature)
  }

  pub fn with_active_document_filter(self) -> ProvisionalCredentialValidation<Active, T> {
    let Self {
      credential,
      issuer,
      verified_subjects,
      refuted_subjects: unverified_subjects,
      encountered_refutation_categories: encountered_error_categories,
      ..
    } = self;
    ProvisionalCredentialValidation::<Active, T> {
      credential,
      issuer,
      verified_subjects,
      refuted_subjects: unverified_subjects,
      encountered_refutation_categories: encountered_error_categories,
      _marker: PhantomData::<Active>,
    }
  }

  pub fn with_deactivated_document_filter(self) -> ProvisionalCredentialValidation<Deactivated, T> {
    let Self {
      credential,
      issuer,
      verified_subjects,
      refuted_subjects: unverified_subjects,
      encountered_refutation_categories: encountered_error_categories,
      ..
    } = self;
    ProvisionalCredentialValidation::<Deactivated, T> {
      credential,
      issuer,
      verified_subjects,
      refuted_subjects: unverified_subjects,
      encountered_refutation_categories: encountered_error_categories,
      _marker: PhantomData::<Deactivated>,
    }
  }

  pub fn encountered_refutation_categories(&self) -> impl Iterator<Item = &CredentialRefutationCategory> {
    self.encountered_refutation_categories.iter()
  }
}

impl ProvisionalCredentialValidation<Active> {
  pub fn issuer(&self) -> Option<&DocumentValidation<Active>> {
    if let Either::Left(ref document_validation) = &self.issuer {
      Some(&document_validation)
    } else {
      None
    }
  }

  pub fn subjects(&self) -> Option<&BTreeMap<String, DocumentValidation<Active>>> {
    self.verified_subjects.as_ref()
  }
}

impl ProvisionalCredentialValidation<Deactivated> {
  pub fn issuer(&self) -> Option<&DocumentValidation<Deactivated>> {
    if let Either::Right(ref document_validation) = &self.issuer {
      Some(&document_validation)
    } else {
      None
    }
  }

  pub fn subjects(&self) -> Option<&BTreeMap<String, DocumentValidation<Deactivated>>> {
    self.refuted_subjects.as_ref()
  }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CredentialValidation<T = Object> {
  pub credential: Credential<T>,
  pub issuer: DocumentValidation,
  pub subjects: BTreeMap<String, DocumentValidation>,
}

impl<Filter: document_state::Sealed, T> TryFrom<ProvisionalCredentialValidation<Filter, T>> for CredentialValidation<T> {
  type Error = Error; // will change Error with RefutedCredentialDismissalError when we get round to fixing the errors in this crate
  fn try_from(partial_credential_validation: ProvisionalCredentialValidation<Filter, T>) -> Result<Self, Self::Error> {
    if partial_credential_validation.successful_validation() {
      let ProvisionalCredentialValidation::<Filter, T> {
        credential,
        issuer,
        verified_subjects,
        ..
      } = partial_credential_validation;
      // use of expect below is okay, the constructor ensures that if any refuted documents were encountered then the
      // successful_validation method returns false once the error refactor gets merged we could consider using
      // the FatalError type here instead.
      let issuer = issuer.expect_left(
        "unexpected deactivated issuer document in successfully validated credential. Please report this bug!",
      );
      Ok(Self {
        credential,
        issuer,
        subjects: verified_subjects.expect(
          "unexpected deactivated subject document(s) in succesfully validated credential. Please report this bug!",
        ),
      })
    } else {
      Err(Self::Error::from(RefutedCredentialDismissalError {
        categories: partial_credential_validation.encountered_refutation_categories,
      }))
    }
  }
}

pub struct ProvisionalPresentationValidation<Filter, T = Object, U = Object> {
  pub presentation: Presentation<T,U>, 
  _marker: PhantomData<Filter>,
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
    //self.validate_credential(Credential::from_json(data)?, options).await
    todo!();
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
    /*
    self
      .validate_presentation(Presentation::from_json(data)?, options)
      .await
    */
    todo!();
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
  ) -> Result<ProvisionalCredentialValidation<Unspecified, T>>
  where
    T: Serialize,
  {
    // possible forms of refutation we may encounter
    let mut encountered_refutation_categories = [Option::<CredentialRefutationCategory>::None; 5];
    let [ref mut issuer_document_deactivated, ref mut subject_documents_deactuvated, ref mut refuted_signature, ref mut expired, ref mut dormant] =
      encountered_refutation_categories;
    // could consider using EnumCount from strum to ensure that all variants are accounted for, but there is a chance
    // we will drop the strum dependency in the future.

    // Resolve the issuer DID Document and validate the digital signature.
    let issuer_url: &str = credential.issuer.url().as_str();
    let issuer = self.validate_document(issuer_url).await?;
    let resolved_issuer_document = match issuer {
      Either::Left(ref issuer_doc) => &issuer_doc.document,
      Either::Right(ref issuer_doc) => {
        *issuer_document_deactivated = Some(CredentialRefutationCategory::DeactivatedIssuerDocument);
        &issuer_doc.document
      }
    };

    let mut verified_subjects: BTreeMap<String, DocumentValidation<Active>> = BTreeMap::new();
    let mut refuted_subjects: BTreeMap<String, DocumentValidation<Deactivated>> = BTreeMap::new();

    // Resolve all credential subjects with `id`s - we assume all ids are DIDs.
    for id in credential
      .credential_subject
      .iter()
      .filter_map(|subject| subject.id.as_ref())
    {
      let validated_document = self.validate_document(id.as_str()).await?;
      if let Either::Left(verified_subject_document) = validated_document {
        verified_subjects.insert(id.to_string(), verified_subject_document);
      } else if let Either::Right(refuted_subject_document) = validated_document {
        refuted_subjects.insert(id.to_string(), refuted_subject_document);
        *subject_documents_deactuvated = Some(CredentialRefutationCategory::DeactivatedSubjectDocuments);
      }
    }

    // Verify the credential signature using the issuers DID Document
    // If the issuer document is deactivated then we may immediately refute the credentials signature
    if issuer_document_deactivated.is_some()
      || resolved_issuer_document
        .document
        .verify_data(&credential, options)
        .is_err()
    {
      *refuted_signature = Some(CredentialRefutationCategory::InvalidSignature);
    }

    // Verify the expiration date
    if let Some(expiration_date) = credential.expiration_date {
      if Timestamp::now_utc() > expiration_date {
        *expired = Some(CredentialRefutationCategory::Expired);
      }
    }

    // Verify that the credential has been activated
    if Timestamp::now_utc() < credential.issuance_date {
      *dormant = Some(CredentialRefutationCategory::Dormant);
    }

    // transform the data into the required formats
    let encountered_refutation_categories: IndexSet<CredentialRefutationCategory> = encountered_refutation_categories
      .into_iter()
      .filter_map(|x| x)
      .collect();

    let verified_subjects = if verified_subjects.len() > 0 {
      Some(verified_subjects)
    } else {
      None
    };
    let refuted_subjects = if refuted_subjects.len() > 0 {
      Some(refuted_subjects)
    } else {
      None
    };
    Ok(ProvisionalCredentialValidation {
      credential,
      issuer,
      verified_subjects,
      refuted_subjects,
      encountered_refutation_categories,
      _marker: PhantomData::<Unspecified>,
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
  async fn validate_document(
    &self,
    did: impl AsRef<str>,
  ) -> Result<Either<DocumentValidation<Active>, DocumentValidation<Deactivated>>> {
    let did: IotaDID = did.as_ref().parse()?;
    let document: ResolvedIotaDocument = self.client.resolve(&did).await?;
    // TODO: check if document is deactivated, does that matter? If it does then we return the Right variant
    // Can do this in tests for now
    #[cfg(test)]
    {
      if document.document.methods().nth(0).is_none() {
        return Ok(Either::Right(DocumentValidation::<Deactivated> {
          did,
          document,
          metadata: Object::new(),
          _marker: PhantomData::<Deactivated>,
        }));
      }
    }
    Ok(Either::Left(DocumentValidation::<Active> {
      did,
      document,
      metadata: Object::new(),
      _marker: PhantomData::<Active>,
    }))
  }
}


