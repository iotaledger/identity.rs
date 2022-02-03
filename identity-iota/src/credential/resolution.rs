// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::ControlFlow;

use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;

use crate::credential::ResolvedCredential;
use crate::credential::ResolvedPresentation;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use std::collections::BTreeMap;
use crate::Result; 


type ResolutionErrors = OneOrMany<super::error::ValidationError>; 

pub(crate) struct InitialisationEvent<'a, T> {
    pub(crate) credential: &'a Credential<T>,
  }
  pub(crate) struct ResolvedIssuerEvent<'a, T> {
    pub(crate) credential: &'a Credential<T>,
    pub(crate) resolved_issuer_doc: &'a ResolvedIotaDocument,
  }
  pub(crate) struct ResolvedSubjectEvent<'a> {
    pub(crate) subject_doc: &'a ResolvedIotaDocument,
  }

  // Resolve and validate a credential in "episodes".
  // Each episode contains an "event" that gets valuated by an event validator that determines whether the process may
  // continue. The `issuer_event_validator` is called at most once, while `subject_event_validator` may be called
  // multiple times.
  // failure_description should be a string briefly describing the 
  #[inline(always)]
  pub(crate) async fn resolve_credential_episodic<R, F, G, H, T>(
    resolver: &R,
    credential: Credential<T>,
    initial_event_validator: F,
    issuer_event_validator: G,
    subject_event_validator: H,
    failure_description: &'static str, 
  ) -> std::result::Result<ResolvedCredential<T>, super::errors::AccumulatedError>
  where
    R: ?Sized + TangleResolve,
    F: Fn(InitialisationEvent<'_, T>, &mut ResolutionErrors) -> ControlFlow<()>,
    G: Fn(Option<ResolvedIssuerEvent<'_, T>>, &mut ResolutionErrors) -> ControlFlow<()>,
    H: Fn(Option<ResolvedSubjectEvent<'_>>, &mut ValidationUnitErrors) -> ControlFlow<()>,
    T: Serialize,
  {
    let mut validation_errors = OneOrMany::<super::errors::Error>::empty();
    // We need this until: https://doc.rust-lang.org/stable/std/ops/enum.ControlFlow.html#method.is_break becomes stable
    let is_break = |outcome: ControlFlow<()>| -> bool {
      if let ControlFlow::Break(_) = outcome {
        true
      } else {
        false
      }
    };

    // We start with some validation checks we can do directly on the Credential
    if is_break(initial_event_validator(
      InitialisationEvent {
        credential: &credential,
      },
      &mut validation_errors,
    )) {
      return Err(validation_errors.into());
    }

    // Resolve the issuer DID Document.
    let issuer_url: &str = credential.issuer.url().as_str();
    let resolved_issuer_doc = resolve_document(resolver, issuer_url).await; 

    // Now validations on the credential requiring a resolved issuer document can take place
    if is_break(issuer_event_validator(
      ResolvedIssuerEvent {
        credential: &credential,
        resolved_issuer_doc: &resolved_issuer_doc,
      },
      &mut validation_errors,
    )) {
      return Err(validation_errors.into());
    }

    // Resolve all credential subjects with `id`s - we assume all ids are DIDs.
    let mut subject_documents: BTreeMap<String, ResolvedIotaDocument> = BTreeMap::new();

    for id in credential
      .credential_subject
      .iter()
      .filter_map(|subject| subject.id.as_ref())
    {
      let subject_document = resolve_document(resolver, id.as_str()).await;
      // Now run any potential validations on this resolved subject document
      if is_break(subject_event_validator(
        ResolvedSubjectEvent {
          subject_doc: &subject_document,
        },
        &mut validation_errors,
      )) {
        return Err(validation_errors.into());
      }
      if validation_errors.is_empty() {
        // only insert if there are no validation errors
        subject_documents.insert(id.to_string(), subject_document);
      }
    }
    if !validation_errors.is_empty() {
      // return all reported errors during credential resolution
      Err(validation_errors.into())
    } else {
      let resolved_credential = ResolvedCredential {
        credential,
        issuer: resolved_issuer_doc,
        subjects: subject_documents,
      };
      Ok(resolved_credential)
    }
  }

  async fn resolve_document<R: ?Sized + TangleResolve>(
    resolver: &R,
    did: impl AsRef<str>,
  ) -> Result<ResolvedIotaDocument> {
    let did: IotaDID = did.as_ref().parse()?;
    resolver.resolve(&did).await
  }