// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::marker::PhantomData;
use super::refutation::CredentialRefutations;
use super::CredentialRefutationCategory;

use either::Either;
use identity_core::common::{Timestamp, Url};
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

use self::document_state::Active;


mod document_state {
  pub trait Sealed {}
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Active;
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Deactivated;
  impl Sealed for Active {}
  impl Sealed for Deactivated {}

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
pub struct CredentialValidation<T = Object>
where
{
  pub credential: Credential<T>,
  pub issuer: DocumentValidation<Active>,
  pub active_subject_documents: Option<BTreeMap<String, DocumentValidation<Active>>>,
  deactivated_subject_documents: Option<BTreeMap<String, DocumentValidation<Deactivated>>>,
  encountered_refutation_categories: CredentialRefutations,
}

impl<T> CredentialValidation<T> {
  /// Returns true if this credential would have been successfully validated with the strictest credential validation options
  pub fn full_validation(&self) -> bool {
    self.encountered_refutation_categories.count() == 0
  }

  /// An iterator over the encountered refutation categories that were tolerated due to the choice of credential validation options
  pub fn encountered_refutation_categories(&self) -> impl Iterator<Item = CredentialRefutationCategory> + '_ {
    self.encountered_refutation_categories.iter()
  }
}

impl CredentialValidation {
  #[doc(hidden)]
  /// Gets the credentials deactivated resolved documents that were tolerated due to the choice of validation options
  pub fn deactivated_subject_documents(&self) -> Option<&BTreeMap<String, DocumentValidation<Deactivated>>> {
    self.deactivated_subject_documents.as_ref()
  }
}


#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PresentationValidation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: DocumentValidation<Active>,
  pub credentials: Vec<CredentialValidation<U>>,
}

impl<T,U> PresentationValidation<T,U> {
  pub fn full_validation(&self) -> bool {
    self.credentials.iter().all(|credential_validation|credential_validation.full_validation())
  }
}


// used internally during credential validation 
enum CredentialRefutationEvent<'a> {
  DeactivatedSubjectDocument((&'a Url,DocumentValidation<Deactivated>)), 
  Expired(Timestamp),
  Dormant(Timestamp), 
}
impl CredentialRefutationEvent<'_> {
  fn associated_category(&self) -> CredentialRefutationCategory {
    match self {
      &Self::DeactivatedSubjectDocument(_) => CredentialRefutationCategory::DeactivatedSubjectDocuments, 
      &Self::Expired(_) => CredentialRefutationCategory::Expired, 
      &Self::Dormant(_) => CredentialRefutationCategory::Dormant, 
    }
  }
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
    // setup refutation event handler 
    let mut encountered_refutation_categories = CredentialRefutations::empty();
    let mut deactivated_subject_documents: BTreeMap<String, DocumentValidation<Deactivated>> = BTreeMap::new();
    let refutation_event_handler = |event: CredentialRefutationEvent<'_>| -> Result<()> {
      let encountered_category = event.associated_category(); 
      if let CredentialRefutationEvent::DeactivatedSubjectDocument((id, document)) =  event {
          deactivated_subject_documents.insert(id.to_string(), document);
        }
      encountered_refutation_categories.insert(encountered_category); 
      Ok(())
    }; 
    // successful case 
    let CredentialValidation { 
      credential, 
      issuer, 
      active_subject_documents, 
      ..} = self.validate_credential_early_exit(credential, options, refutation_event_handler).await?; 
      // append the tolerable refutations recorded by our event handler 
      let deactivated_subject_documents = if deactivated_subject_documents.len() > 0 {
        Some(deactivated_subject_documents)
      } else {
        None
      };
      Ok(
        CredentialValidation{
          credential,
          issuer,
          active_subject_documents,
          deactivated_subject_documents,
          encountered_refutation_categories,
        }
      )
  }

  // validates a credential and lets the supplied closure determine if a refutation event is tolerable 
  async fn validate_credential_early_exit<T: Serialize, F: FnMut(CredentialRefutationEvent<'_>)->Result<()>>(
    &self, 
    credential: Credential<T>,
    options: VerifierOptions,
    mut f: F) -> Result<CredentialValidation<T>>
    {
       // Resolve the issuer DID Document and validate the digital signature.
    let issuer_url: &str = credential.issuer.url().as_str();
    // if the issuer's DID document is deactivated then the signature cannot be valid
    let issuer = self.validate_document(issuer_url).await?.left().ok_or(identity_did::Error::InvalidSignature("method not found"))?;

    // Resolve all credential subjects with `id`s - we assume all ids are DIDs.
    let mut verified_subjects: BTreeMap<String, DocumentValidation<Active>> = BTreeMap::new();

    for id in credential
      .credential_subject
      .iter()
      .filter_map(|subject| subject.id.as_ref())
    {
      let validated_document = self.validate_document(id.as_str()).await?;
      if let Either::Left(verified_subject_document) = validated_document {
        verified_subjects.insert(id.to_string(), verified_subject_document);
      } else if let Either::Right(refuted_subject_document) = validated_document {
        f(CredentialRefutationEvent::DeactivatedSubjectDocument((id,refuted_subject_document)))?;
          //return Err(Error::IntolerableCredentialRefutation(event.associated_category())); 
      }
    }

     // Verify the credential signature using the issuers DID Document
    // If the issuer document is deactivated then we may immediately refute the credentials signature
    issuer.document
        .document
        .verify_data(&credential, options)?;

    // Verify the expiration date
    if let Some(expiration_date) = credential.expiration_date {
      if Timestamp::now_utc() > expiration_date {
        f(CredentialRefutationEvent::Expired(expiration_date))?;
        // encountered_refutation_categories.insert(CredentialRefutationCategory::Expired);
      }
    }

    // Verify that the credential has been activated
    if Timestamp::now_utc() < credential.issuance_date {
      f(CredentialRefutationEvent::Dormant(credential.issuance_date))?;
      // encountered_refutation_categories.insert(CredentialRefutationCategory::Dormant);
    }

    // transform the data into the required formats and return the best case scenario from this function 
    let verified_subjects = if verified_subjects.len() > 0 {
      Some(verified_subjects)
    } else {
      None
    };
    Ok(CredentialValidation {
      credential,
      issuer,
      active_subject_documents: verified_subjects,
      deactivated_subject_documents: None,
      encountered_refutation_categories: CredentialRefutations::empty(),
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
    let holder_doc =  self.validate_document(holder_url).await?.left().ok_or(identity_did::Error::InvalidSignature("method not found"))?;

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
    holder_doc.document.document.verify_data(&presentation, options)?;

    Ok(PresentationValidation {
      presentation,
      holder: holder_doc,
      credentials,
    })
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
