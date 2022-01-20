// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;

use crate::credential::CredentialResolutionError;
use crate::credential::CredentialValidationOptions;
use crate::credential::ResolvedCredential;
use crate::credential::ValidationUnitError;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::error::Result;
use crate::tangle::MessageId;

use self::credential_resolution_internals::ResolvedIssuerEvent;

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

  async fn resolve_credential(
    &self,
    credential: Credential,
    validation_options: &CredentialValidationOptions,
    fail_fast: bool,
  ) -> std::result::Result<ResolvedCredential, CredentialResolutionError> {
    // In the case of a validation error we either want to immediately return it (`fail_fast = true`), or store it in a container. 
    // We introduce a closure to avoid some code duplication. 
    let handle_validation_unit_error = 
    |store: &mut OneOrMany<ValidationUnitError>, result: Result<(), ValidationUnitError>| -> Result<(), CredentialResolutionError> {
      if fail_fast {
        result.map_err(Into::into) 
      } else if let Err(validation_error) = result {
        store.push(validation_error);
        Ok(())
      } else {
        Ok(())
      }
    }; 

    // Now we validate what we can directly from the credential before we resolve anything. 
    let mut basic_validation_errors = OneOrMany::Many(Vec::<ValidationUnitError>::new()); 
    
    // simple converter 
    let bool_to_result = |value: bool, error: ValidationUnitError| -> Result<(), ValidationUnitError> {
      value.then(||()).ok_or(error)
    }; 

    // validate the issuance date 
    let issued_before_result = bool_to_result(
      credential.issued_before(validation_options.issued_before), 
      ValidationUnitError::InvalidIssuanceDate
    ); 
    handle_validation_unit_error(&mut basic_validation_errors, issued_before_result)?; 


    // validate the expiration date 
    let expires_after_result = bool_to_result(
      credential.expires_after(validation_options.expires_after), 
      ValidationUnitError::InvalidExpirationDate); 
    handle_validation_unit_error(&mut basic_validation_errors, expires_after_result)?; 

    // Now we start resolving DID documents  
    // We setup an event handler that verifies the credential signature using the issuers DID Document 
    // once this document gets resolved from the Tangle. 
    let mut issuer_event_validator = |resolved_issuer_event: ResolvedIssuerEvent| -> Result<(), ValidationUnitError> {
      let resolved_issuer_doc = resolved_issuer_event.resolved_issuer_doc; 
      //issuer.document.document.verify_data(&credential, 
        todo!()
      };
    todo!()
  }
}

mod credential_resolution_internals {
  use identity_core::common::OneOrMany;

  use super::Credential;
  use super::CredentialResolutionError;
  use super::IotaDID;
  use super::ResolvedCredential;
  use super::ResolvedIotaDocument;
  use super::Result;
  use super::TangleResolve;
  use super::ValidationUnitError;
  use std::collections::BTreeMap;

  pub(super) struct ResolvedIssuerEvent<'a> {
    pub(super) credential: &'a Credential,
    pub(super) resolved_issuer_doc: &'a ResolvedIotaDocument,
  }
  pub(super) struct ResolvedSubjectEvent<'a> {
    pub(super) subject_doc: &'a ResolvedIotaDocument,
  }

  // Resolve and validate a credential in "episodes".
  // Each episode contains an "event" that gets valuated by an event validator that determines whether the process may
  // continue. The `issuer_event_validator` is called at most once, while `subject_event_validator` may be called
  // multiple times.
  pub(super) async fn resolve_credential_episodic<R, F, G>(
    credential: Credential,
    resolver: &R,
    mut issuer_event_validator: F,
    mut subject_event_validator: G,
  ) -> std::result::Result<ResolvedCredential, CredentialResolutionError>
  where
    R: TangleResolve,
    F: FnMut(ResolvedIssuerEvent<'_>) -> std::result::Result<(), ValidationUnitError>,
    G: FnMut(ResolvedSubjectEvent<'_>) -> std::result::Result<(), ValidationUnitError>,
  {
    // We start with some validation checks we can do directly on the Credential
    // Resolve the issuer DID Document and validate the digital signature.
    let issuer_url: &str = credential.issuer.url().as_str();
    let resolved_issuer_doc = resolve_document(resolver, issuer_url)
      .await
      .map_err(|err| CredentialResolutionError::DIDResolution { source: Box::from(err) })?;
    // now validate
    issuer_event_validator(ResolvedIssuerEvent {
      credential: &credential,
      resolved_issuer_doc: &resolved_issuer_doc,
    })?;
    // Resolve all credential subjects with `id`s - we assume all ids are DIDs.
    let mut subject_documents: BTreeMap<String, ResolvedIotaDocument> = BTreeMap::new();

    for id in credential
      .credential_subject
      .iter()
      .filter_map(|subject| subject.id.as_ref())
    {
      let subject_document = resolve_document(resolver, id.as_str())
        .await
        .map_err(|err| CredentialResolutionError::DIDResolution { source: Box::from(err) })?;
      subject_event_validator(ResolvedSubjectEvent {
        subject_doc: &subject_document,
      })?;
      subject_documents.insert(id.to_string(), subject_document);
    }
    let resolved_credential = ResolvedCredential {
      credential,
      issuer: resolved_issuer_doc,
      subjects: subject_documents,
    };
    Ok(resolved_credential)
  }

  async fn resolve_document<R: TangleResolve>(resolver: &R, did: impl AsRef<str>) -> Result<ResolvedIotaDocument> {
    let did: IotaDID = did.as_ref().parse()?;
    resolver.resolve(&did).await
  }
}
