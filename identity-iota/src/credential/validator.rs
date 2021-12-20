// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::deficiencies::CredentialDeficiencySet;
use super::validated_data::document_state::Active;
use super::validated_data::document_state::Deactivated;
use super::CredentialDeficiency;
use super::CredentialValidation;
use super::DocumentValidation;
use super::PresentationValidation;
use std::collections::BTreeMap;
use std::marker::PhantomData;

use either::Either;
use identity_core::common::Timestamp;
use identity_core::common::Url;
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

// used internally during credential validation
#[allow(clippy::large_enum_variant)] // The smaller variants occur at most once each
enum CredentialDeficiencyEvent<'a> {
  // the Url corresponds to the url of the deactivated document
  DeactivatedSubjectDocument((&'a Url, DocumentValidation<Deactivated>)),
  Expired(Timestamp),
  Dormant(Timestamp),
}
impl CredentialDeficiencyEvent<'_> {
  fn associated_category(&self) -> CredentialDeficiency {
    match *self {
      Self::DeactivatedSubjectDocument(_) => CredentialDeficiency::DeactivatedSubjectDocuments,
      Self::Expired(_) => CredentialDeficiency::Expired,
      Self::Dormant(_) => CredentialDeficiency::Dormant,
    }
  }
}

// simplified form of CredentialValidation used internally
struct AcceptedCredentialValidation<T = Object> {
  credential: Credential<T>,
  issuer: DocumentValidation<Active>,
  active_subject_documents: Option<BTreeMap<String, DocumentValidation<Active>>>,
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
  pub async fn check_credential<T>(
    &self,
    data: &str,
    signature_verification_options: VerifierOptions,
    allowed_deficiencies: CredentialDeficiencySet,
  ) -> Result<CredentialValidation<T>>
  where
    T: DeserializeOwned + Serialize,
  {
    self
      .validate_credential(
        Credential::from_json(data)?,
        signature_verification_options,
        allowed_deficiencies,
      )
      .await
  }

  /// Deserializes the given JSON-encoded `Presentation` and
  /// validates all associated DID documents/`Credential`s.
  pub async fn check_presentation<T, U>(
    &self,
    data: &str,
    signature_verification_options: VerifierOptions,
    allowed_credential_deficiencies: CredentialDeficiencySet,
  ) -> Result<PresentationValidation<T, U>>
  where
    T: Clone + DeserializeOwned + Serialize,
    U: Clone + DeserializeOwned + Serialize,
  {
    self
      .validate_presentation(
        Presentation::from_json(data)?,
        signature_verification_options,
        allowed_credential_deficiencies,
      )
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
    signature_verification_options: VerifierOptions,
    allowed_deficiencies: CredentialDeficiencySet,
  ) -> Result<CredentialValidation<T>>
  where
    T: Serialize,
  {
    // setup deficiency event handler
    let mut encountered_refutation_categories = CredentialDeficiencySet::empty();
    let mut deactivated_subject_documents: BTreeMap<String, DocumentValidation<Deactivated>> = BTreeMap::new();
    let deficiency_event_handler = |event: CredentialDeficiencyEvent<'_>| -> Result<()> {
      let encountered_category = event.associated_category();
      if !allowed_deficiencies.contains(&encountered_category) {
        return Err(Error::UnacceptableCredentialDeficiency(encountered_category));
      }
      if let CredentialDeficiencyEvent::DeactivatedSubjectDocument((id, document)) = event {
        deactivated_subject_documents.insert(id.to_string(), document);
      }
      encountered_refutation_categories.insert(encountered_category);
      Ok(())
    };
    // successful case
    let AcceptedCredentialValidation {
      credential,
      issuer,
      active_subject_documents,
    } = self
      .validate_credential_with_deficiency_handler(credential, signature_verification_options, deficiency_event_handler)
      .await?;
    // append the acceptable deficiencies recorded by our event handler
    let deactivated_subject_documents = if !deactivated_subject_documents.is_empty() {
      Some(deactivated_subject_documents)
    } else {
      None
    };
    Ok(CredentialValidation {
      credential,
      issuer,
      active_subject_documents,
      deactivated_subject_documents,
      encountered_refutation_categories,
    })
  }

  // validates a credential and lets the supplied closure determine whether the encountered deficiency is considered
  // acceptable
  async fn validate_credential_with_deficiency_handler<
    T: Serialize,
    F: FnMut(CredentialDeficiencyEvent<'_>) -> Result<()>,
  >(
    &self,
    credential: Credential<T>,
    signature_options: VerifierOptions,
    mut deficiency_handler: F,
  ) -> Result<AcceptedCredentialValidation<T>> {
    // Resolve the issuer DID Document and validate the digital signature.
    let issuer_url: &str = credential.issuer.url().as_str();
    // if the issuer's DID document is deactivated then the signature cannot be valid
    let issuer = self
      .validate_document(issuer_url)
      .await?
      .left()
      .ok_or(identity_did::Error::InvalidSignature("method not found"))?;

    // Resolve all credential subjects with `id`s - we assume all ids are DIDs.
    let mut accepted_subject_documents: BTreeMap<String, DocumentValidation<Active>> = BTreeMap::new();

    for id in credential
      .credential_subject
      .iter()
      .filter_map(|subject| subject.id.as_ref())
    {
      let validated_document = self.validate_document(id.as_str()).await?;
      if let Either::Left(verified_subject_document) = validated_document {
        accepted_subject_documents.insert(id.to_string(), verified_subject_document);
      } else if let Either::Right(refuted_subject_document) = validated_document {
        deficiency_handler(CredentialDeficiencyEvent::DeactivatedSubjectDocument((
          id,
          refuted_subject_document,
        )))?;
      }
    }

    // Verify the credential signature using the issuers DID Document
    issuer.document.document.verify_data(&credential, signature_options)?;

    // Verify the expiration date
    if let Some(expiration_date) = credential.expiration_date {
      if Timestamp::now_utc() > expiration_date {
        deficiency_handler(CredentialDeficiencyEvent::Expired(expiration_date))?;
        // encountered_refutation_categories.insert(CredentialRefutationCategory::Expired);
      }
    }

    // Verify that the credential has been activated
    if Timestamp::now_utc() < credential.issuance_date {
      deficiency_handler(CredentialDeficiencyEvent::Dormant(credential.issuance_date))?;
      // encountered_refutation_categories.insert(CredentialRefutationCategory::Dormant);
    }

    // transform the data into the required formats and return the best case scenario from this function
    let accepted_subject_documents = if !accepted_subject_documents.is_empty() {
      Some(accepted_subject_documents)
    } else {
      None
    };
    Ok(AcceptedCredentialValidation {
      credential,
      issuer,
      active_subject_documents: accepted_subject_documents,
    })
  }

  /// Validates the `Presentation` proof and all relevant DID documents.
  ///
  /// Note: The presentation holder is expected to be a valid DID.
  /// Note: The presentation is expected to have a proof created by the holder.
  /// Note: `signature_options` only affects validation of the signature on the presentation and does not
  ///       affect the validation of any of its credentials.
  pub async fn validate_presentation<T, U>(
    &self,
    presentation: Presentation<T, U>,
    signature_verification_options: VerifierOptions,
    allowed_credential_deficiencies: CredentialDeficiencySet,
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
    let holder_doc = self
      .validate_document(holder_url)
      .await?
      .left()
      .ok_or(identity_did::Error::InvalidSignature("method not found"))?;

    let mut credentials: Vec<CredentialValidation<U>> = Vec::new();

    // Resolve and validate all associated credentials.
    for credential in presentation.verifiable_credential.iter() {
      // TODO: do we need to allow different VerifierOptions for each credential?
      credentials.push(
        self
          .validate_credential(
            credential.clone(),
            VerifierOptions::default(),
            allowed_credential_deficiencies,
          )
          .await?,
      );
    }

    // Verify the presentation signature using the holders DID Document
    holder_doc
      .document
      .document
      .verify_data(&presentation, signature_verification_options)?;

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
      if document.document.methods().next().is_none() {
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
