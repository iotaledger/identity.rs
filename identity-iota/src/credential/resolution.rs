// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::ops::ControlFlow;

use identity_core::common::OneOrMany;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_did::did::DID;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::CredentialResolutionError;
use super::errors::PresentationResolutionError;
use super::errors::ValidationError;
use super::CredentialValidationOptions;
use super::PresentationValidationOptions;
use crate::credential::ResolvedCredential;
use crate::credential::ResolvedPresentation;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::tangle::TangleResolve;
use crate::Result;
use std::collections::BTreeMap;

/// Resolves and validates a `Presentation` in accordance with the given `validation_options`
async fn resolve_presentation<T, U, R>(
  resolver: &R,
  presentation: Presentation<T, U>,
  validation_options: &PresentationValidationOptions,
  fail_fast: bool,
) -> std::result::Result<ResolvedPresentation<T, U>, PresentationResolutionError>
where
  T: Serialize + Clone,
  U: Serialize + Clone,
  R: ?Sized + TangleResolve,
{
  let initial_validator =
    |presentation: &Presentation<T, U>, validation_errors: &mut Vec<ValidationError>| -> ControlFlow<()> {
      // set up the first validator that validates as much of the presentation as possible without resolving any
      // documents.
      if let Err(error) = presentation.check_structure() {
        validation_errors.push(ValidationError::PresentationStructure(error));
        if fail_fast {
          return ControlFlow::Break(());
        }
      }
      if let Some((credential_position, _)) = presentation.non_transferable_violations().next() {
        validation_errors.push(ValidationError::NonTransferableViolation { credential_position });
        if fail_fast {
          return ControlFlow::Break(());
        }
      }

      ControlFlow::Continue(())
    };

  // now define the validator that uses the holder's resolved DID Document to verify the holder's signature.
  let holder_validator = |resolved_holder_event: ResolvedHolderEvent<'_, T, U>,
                          validation_errors: &mut Vec<ValidationError>|
   -> ControlFlow<()> {
    let holder_doc = &resolved_holder_event.resolved_holder_doc.document;
    if let Err(error) = holder_doc.verify_data(
      resolved_holder_event.presentation,
      &validation_options.common_validation_options.verifier_options,
    ) {
      validation_errors.push(ValidationError::HolderProof { source: error.into() });
      if fail_fast {
        return ControlFlow::Break(());
      }
    }
    ControlFlow::Continue(())
  };

  // now we introduce our credential resolver

  let credential_resolver = |credential: Credential<U>| async {
    resolve_credential(
      resolver,
      credential,
      &validation_options.common_validation_options,
      fail_fast,
    )
    .await
  };

  let fail_on_unresolved_holder = true;
  let fail_on_unresolved_credentials = true;

  resolve_presentation_generic(
    resolver,
    presentation,
    initial_validator,
    holder_validator,
    credential_resolver,
    fail_on_unresolved_holder,
    fail_on_unresolved_credentials,
  )
  .await
}

/// Resolves the `Presentation` and verifies the signatures of the holder and the issuer of each `Credential`.
async fn resolve_presentation_unvalidated<T, U, R>(
  resolver: &R,
  presentation: Presentation<T, U>,
  verifier_options: &VerifierOptions,
) -> std::result::Result<ResolvedPresentation<T, U>, PresentationResolutionError>
where
  T: Serialize + Clone,
  U: Serialize + Clone,
  R: ?Sized + TangleResolve,
{
  todo!()
}

/// Resolves the `Presentation` without applying any checks.  
async fn resolve_presentation_unchecked<T, U, R>(
  resolver: &R,
  presentation: Presentation<T, U>,
) -> std::result::Result<ResolvedPresentation<T, U>, PresentationResolutionError>
where
  T: Serialize + Clone,
  U: Serialize + Clone,
  R: ?Sized + TangleResolve,
{
  todo!()
}

struct ResolvedHolderEvent<'a, T, U> {
  presentation: &'a Presentation<T, U>,
  resolved_holder_doc: &'a ResolvedIotaDocument,
}

type CredentialResolutionErrors = BTreeMap<usize, CredentialResolutionError>;

async fn resolve_presentation_generic<T, U, R, I, H, C, F>(
  resolver: &R,
  presentation: Presentation<T, U>,
  initial_validator: I,
  holder_validator: H,
  credential_resolver: C,
  fail_on_unresolved_holder: bool,
  fail_on_unresolved_credentials: bool,
) -> std::result::Result<ResolvedPresentation<T, U>, PresentationResolutionError>
where
  T: Serialize + Clone,
  U: Serialize + Clone,
  R: ?Sized + TangleResolve,
  I: Fn(&Presentation<T, U>, &mut Vec<ValidationError>) -> ControlFlow<()>,
  H: Fn(ResolvedHolderEvent<'_, T, U>, &mut Vec<ValidationError>) -> ControlFlow<()>,
  C: Fn(Credential<U>) -> F, /* Would be natural to make &R an additional argument to `credential_resolver`, but async closures that take references seem to be problematic in Rust for the time being (https://users.rust-lang.org/t/async-closure-that-takes-a-reference/52079). */
  F: Future<Output = Result<ResolvedCredential<U>, CredentialResolutionError>>,
{
  let mut presentation_resolution_error = PresentationResolutionError {
    presentation_validation_errors: Vec::<ValidationError>::new(),
    credential_errors: CredentialResolutionErrors::new(),
  };
  let PresentationResolutionError {
    ref mut presentation_validation_errors,
    ref mut credential_errors,
  } = presentation_resolution_error;

  // We need this until: https://doc.rust-lang.org/stable/std/ops/enum.ControlFlow.html#method.is_break becomes stable
  let is_break = |outcome: ControlFlow<()>| -> bool {
    if let ControlFlow::Break(_) = outcome {
      true
    } else {
      false
    }
  };

  // We start with some validation checks we can do directly on the Presentation
  if is_break(initial_validator(&presentation, presentation_validation_errors)) {
    return Err(presentation_resolution_error);
  }

  // Now we try to resolve the holder's DID Document
  let mut resolved_holder_document: Option<ResolvedIotaDocument> = None;
  match presentation
    .holder
    .as_ref()
    .map(|holder| holder.as_str())
    .ok_or(ValidationError::MissingPresentationHolder)
  {
    Ok(holder_doc) => {
      let resolved_holder_doc = resolve_document(resolver, holder_doc).await;
      match resolved_holder_doc {
        Ok(resolved_document) => {
          // now try to verify the holder's signature using the holder's resolved DID Document
          if is_break(holder_validator(
            ResolvedHolderEvent {
              presentation: &presentation,
              resolved_holder_doc: &resolved_document,
            },
            presentation_validation_errors,
          )) {
            return Err(presentation_resolution_error);
          }
          resolved_holder_document = Some(resolved_document);
        }
        Err(error) => {
          presentation_validation_errors.push(ValidationError::HolderDocumentResolution { source: error.into() });
          if fail_on_unresolved_holder {
            return Err(presentation_resolution_error);
          }
        }
      }
    }
    Err(error) => {
      presentation_validation_errors.push(error);
      if fail_on_unresolved_holder {
        return Err(presentation_resolution_error);
      }
    }
  }

  // Resolve all associated credentials
  let mut credentials: Vec<ResolvedCredential<U>> = Vec::new();
  for (position, credential) in presentation.verifiable_credential.iter().cloned().enumerate() {
    match credential_resolver(credential).await {
      Ok(resolved_credential) => {
        credentials.push(resolved_credential);
      }
      Err(error) => {
        credential_errors.insert(position, error);
        if fail_on_unresolved_credentials {
          return Err(presentation_resolution_error);
        }
      }
    }
  }

  if let Some(resolved_holder_doc) = resolved_holder_document {
    let resolved_presentation = ResolvedPresentation {
      presentation,
      holder: resolved_holder_doc,
      credentials,
    };

    if credential_errors.is_empty() && presentation_validation_errors.is_empty() {
      return Ok(resolved_presentation);
    }
  }
  Err(presentation_resolution_error)
}

/// Resolves and validates a `Credential` in accordance with the given `validation_options`.
async fn resolve_credential<T: Serialize, R: ?Sized + TangleResolve>(
  resolver: &R,
  credential: Credential<T>,
  validation_options: &CredentialValidationOptions,
  fail_fast: bool,
) -> std::result::Result<ResolvedCredential<T>, CredentialResolutionError> {
  // simple converter
  let bool_to_result =
    |value: bool, error: ValidationError| -> Result<(), ValidationError> { value.then(|| ()).ok_or(error) };

  let initial_validator =
    |initial_event: InitialisationEvent<'_, T>, errors: &mut ValidationErrors| -> ControlFlow<()> {
      // validate the credential's structure
      if let Err(error) = initial_event.credential.check_structure() {
        errors.push(ValidationError::CredentialStructure(error));
        if fail_fast {
          return ControlFlow::Break(());
        }
      }

      // validate the issuance date
      if let Err(error) = bool_to_result(
        initial_event.credential.issued_before(validation_options.issued_before),
        ValidationError::IssuanceDate,
      ) {
        errors.push(error);
        if fail_fast {
          return ControlFlow::Break(());
        }
      }

      // validate the expiration date
      if let Err(error) = bool_to_result(
        initial_event.credential.expires_after(validation_options.expires_after),
        ValidationError::ExpirationDate,
      ) {
        errors.push(error);
        if fail_fast {
          return ControlFlow::Break(());
        }
      }
      ControlFlow::Continue(())
    };

  // Now we start resolving DID documents
  // We setup an event handler that verifies the credential signature using the issuers DID Document
  // once this document gets resolved from the Tangle.
  let issuer_event_validator =
    |resolved_issuer_event: ResolvedIssuerEvent<'_, T>, errors: &mut ValidationErrors| -> ControlFlow<()> {
      let resolved_issuer_doc = resolved_issuer_event.resolved_issuer_doc;
      if let Err(verification_error) = resolved_issuer_doc
        .document
        .verify_data(resolved_issuer_event.credential, &validation_options.verifier_options)
        .map_err(|error| ValidationError::IssuerProof {
          source: Box::new(error),
        })
      {
        errors.push(verification_error);
        if fail_fast {
          // Todo: Maybe we should break here regardless of fail_fast because it will not be possible to construct a
          // ResolvedCredential without a value for the resolved issuer document?
          return ControlFlow::Break(());
        }
      }
      ControlFlow::Continue(())
    };

  // And we also want an even handler for newly resolved subject documents
  let subject_event_validator =
    |resolved_subject_document_event: ResolvedSubjectEvent<'_>, errors: &mut ValidationErrors| -> ControlFlow<()> {
      if validation_options.allow_deactivated_subject_documents {
        ControlFlow::Continue(())
      } else {
        let subject_doc = resolved_subject_document_event.subject_doc;
        if !subject_doc.document.active() {
          // Todo: would be nice to avoid the clone generated by to_url, but it is not clear how to do that
          // even with an owned ResolvedIotaDocument
          let err = ValidationError::DeactivatedSubjectDocument {
            did_url: subject_doc.document.id().to_url(),
          };
          errors.push(err);
          if fail_fast {
            return ControlFlow::Break(());
          }
        }
        ControlFlow::Continue(())
      }
    };

  let fail_on_unresolved_issuer = true;
  let fail_on_unresolved_subjects = true;

  resolve_credential_episodic(
    resolver,
    credential,
    initial_validator,
    issuer_event_validator,
    subject_event_validator,
    fail_on_unresolved_issuer,
    fail_on_unresolved_subjects,
  )
  .await
}

/// Resolves the `Credential` and verifies its signature using the issuers DID Document.
async fn resolve_credential_unvalidated<T: Serialize, R: ?Sized + TangleResolve>(
  resolver: &R,
  credential: Credential<T>,
  verifier_options: &VerifierOptions,
) -> std::result::Result<ResolvedCredential<T>, CredentialResolutionError> {
  // We setup an event handler that verifies the credential signature using the issuers DID Document
  // once this document gets resolved from the Tangle.
  let issuer_event_validator =
    |resolved_issuer_event: ResolvedIssuerEvent<'_, T>, errors: &mut ValidationErrors| -> ControlFlow<()> {
      let resolved_issuer_doc = resolved_issuer_event.resolved_issuer_doc;
      if let Err(verification_error) = resolved_issuer_doc
        .document
        .verify_data(resolved_issuer_event.credential, verifier_options)
        .map_err(|error| ValidationError::IssuerProof {
          source: Box::new(error),
        })
      {
        errors.push(verification_error);
        ControlFlow::Break(())
      } else {
        ControlFlow::Continue(())
      }
    };

  // We only care about verifying the signature using the issuer's DID Document
  let initial_validator = |_: InitialisationEvent<'_, T>, _: &mut ValidationErrors| ControlFlow::Continue(());

  let subject_event_validator = |event: ResolvedSubjectEvent<'_>, _: &mut ValidationErrors| ControlFlow::Continue(());

  let fail_on_unresolved_issuer = true;
  let fail_on_unresolved_subjects = true;

  resolve_credential_episodic(
    resolver,
    credential,
    initial_validator,
    issuer_event_validator,
    subject_event_validator,
    fail_on_unresolved_issuer,
    fail_on_unresolved_subjects,
  )
  .await
}

/// Resolves a `Credential` without applying any checks
///
/// If `fail_on_unresolved_documents` is false then one may not assume a 1-1 relationship between the subjects in
/// `credential` and subjects in the returned [ResolvedCredential].
///
/// # Errors
///
/// Fails if the issuer's DID Document cannot be resolved, and the same holds for subject DID Documents if
/// `fail_on_unresolved_subjects` is true.
async fn resolve_credential_unchecked<T: Serialize, R: ?Sized + TangleResolve>(
  resolver: &R,
  credential: Credential<T>,
  fail_on_unresolved_subjects: bool,
) -> std::result::Result<ResolvedCredential<T>, CredentialResolutionError> {
  // We apply trivial validators
  let initial_validator = |_: InitialisationEvent<'_, T>, _: &mut ValidationErrors| ControlFlow::Continue(());

  let issuer_event_validator =
    |event: ResolvedIssuerEvent<'_, T>, _: &mut ValidationErrors| -> ControlFlow<()> { ControlFlow::Continue(()) };

  let subject_event_validator = |event: ResolvedSubjectEvent<'_>, _: &mut ValidationErrors| ControlFlow::Continue(());

  let fail_on_unresolved_issuer = true;
  resolve_credential_episodic(
    resolver,
    credential,
    initial_validator,
    issuer_event_validator,
    subject_event_validator,
    fail_on_unresolved_issuer,
    fail_on_unresolved_subjects,
  )
  .await
}

type ValidationErrors = OneOrMany<ValidationError>;

struct InitialisationEvent<'a, T> {
  credential: &'a Credential<T>,
}
struct ResolvedIssuerEvent<'a, T> {
  credential: &'a Credential<T>,
  resolved_issuer_doc: &'a ResolvedIotaDocument,
}
struct ResolvedSubjectEvent<'a> {
  subject_doc: &'a ResolvedIotaDocument,
}

// Resolve and validate a credential in "episodes".
// Each episode contains an "event" that gets valuated by an event validator that determines whether the process may
// continue. The `issuer_event_validator` is called at most once, while `subject_event_validator` may be called
// multiple times.
// failure_description should be a string briefly describing the
#[inline(always)]
async fn resolve_credential_episodic<R, F, G, H, T>(
  resolver: &R,
  credential: Credential<T>,
  initial_event_validator: F,
  issuer_event_validator: G,
  subject_event_validator: H,
  fail_on_unresolved_issuer: bool,
  fail_on_unresolved_subjects: bool,
) -> std::result::Result<ResolvedCredential<T>, CredentialResolutionError>
where
  R: ?Sized + TangleResolve,
  F: Fn(InitialisationEvent<'_, T>, &mut ValidationErrors) -> ControlFlow<()>,
  G: Fn(ResolvedIssuerEvent<'_, T>, &mut ValidationErrors) -> ControlFlow<()>,
  H: Fn(ResolvedSubjectEvent<'_>, &mut ValidationErrors) -> ControlFlow<()>,
  T: Serialize,
{
  let mut validation_errors = ValidationErrors::empty();
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
    return Err(CredentialResolutionError { validation_errors });
  }

  // Resolve the issuer DID Document.
  let issuer_url: &str = credential.issuer.url().as_str();
  let mut resolved_issuer_doc: Option<ResolvedIotaDocument> = None;

  // Now potential validations requiring the issuer's DID Document can take place if document resolution was successful
  let break_now = match resolve_document(resolver, issuer_url).await {
    Ok(resolved_doc) => {
      let break_now = is_break(issuer_event_validator(
        ResolvedIssuerEvent {
          credential: &credential,
          resolved_issuer_doc: &resolved_doc,
        },
        &mut validation_errors,
      ));
      resolved_issuer_doc = Some(resolved_doc);
      break_now
    }
    Err(error) => {
      validation_errors.push(ValidationError::IssuerDocumentResolution { source: error.into() });
      fail_on_unresolved_issuer
    }
  };
  if break_now {
    return Err(CredentialResolutionError { validation_errors });
  }

  // Resolve all credential subjects with `id`s - we assume all ids are DIDs.
  let mut subject_documents: BTreeMap<String, ResolvedIotaDocument> = BTreeMap::new();

  for id in credential
    .credential_subject
    .iter()
    .filter_map(|subject| subject.id.as_ref())
  {
    let resolved_subject_document = resolve_document(resolver, id.as_str()).await;
    match resolved_subject_document {
      Ok(subject_document) => {
        // Now run any potential validations on this resolved subject document
        if is_break(subject_event_validator(
          ResolvedSubjectEvent {
            subject_doc: &subject_document,
          },
          &mut validation_errors,
        )) {
          return Err(CredentialResolutionError { validation_errors });
        }
        if validation_errors.is_empty() {
          // only insert if there are no validation errors
          subject_documents.insert(id.to_string(), subject_document);
        }
      }
      Err(error) => {
        validation_errors.push(ValidationError::SubjectDocumentResolution {
          source: error.into(),
          did_url: id.to_owned(),
        });
        if fail_on_unresolved_subjects {
          return Err(CredentialResolutionError { validation_errors });
        }
      }
    }
  }

  if let Some(resolved_issuer_document) = resolved_issuer_doc {
    let resolved_credential = ResolvedCredential {
      credential,
      issuer: resolved_issuer_document,
      subjects: subject_documents,
    };
    if validation_errors.is_empty() {
      return Ok(resolved_credential);
    }
  }
  Err(CredentialResolutionError { validation_errors })
}

async fn resolve_document<R: ?Sized + TangleResolve>(
  resolver: &R,
  did: impl AsRef<str>,
) -> Result<ResolvedIotaDocument> {
  let did: IotaDID = did.as_ref().parse()?;
  resolver.resolve(&did).await
}
