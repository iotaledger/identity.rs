// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use self::document_state::Active;
use self::document_state::Deactivated;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::collections::BTreeMap;
use std::marker::PhantomData;

use super::deficiencies::CredentialDeficiencySet;

use identity_core::common::Object;

use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;

use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;

pub(super) mod document_state {
  // Used to parameterise whether a resolved document is active.
  pub trait Sealed {}
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Active;
  #[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
  pub struct Deactivated;
  impl Sealed for Active {}
  impl Sealed for Deactivated {}
}
#[derive(Clone, Debug, PartialEq)]
pub struct DocumentValidation<T>
where
  T: document_state::Sealed,
{
  pub did: IotaDID,
  pub document: ResolvedIotaDocument,
  pub metadata: Object,
  pub(super) _marker: PhantomData<T>,
}

// Temporary workarounds for backward compatibility with the current Wasm implementation.
impl Serialize for DocumentValidation<Active> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("DocumentValidation", 4)?;
    state.serialize_field("did", &self.did)?;
    state.serialize_field("document", &self.document)?;
    state.serialize_field("metadata", &self.metadata)?;
    state.serialize_field("verified", &true)?;
    state.end()
  }
}

// this is not really necessary, but we might as well implement it
impl Serialize for DocumentValidation<Deactivated> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("DocumentValidation", 4)?;
    state.serialize_field("did", &self.did)?;
    state.serialize_field("document", &self.document)?;
    state.serialize_field("metadata", &self.metadata)?;
    state.serialize_field("verified", &false)?;
    state.end()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CredentialValidation<T = Object> {
  pub credential: Credential<T>,
  pub issuer: DocumentValidation<Active>,
  pub active_subject_documents: Option<BTreeMap<String, DocumentValidation<Active>>>,
  pub(super) deactivated_subject_documents: Option<BTreeMap<String, DocumentValidation<Deactivated>>>,
  pub encountered_deficiencies: CredentialDeficiencySet,
}

impl<T> CredentialValidation<T> {
  /// Returns true if no deficiencies were detected during credential validation.
  /// See [`crate::credential::CredentialDeficiency`].
  pub fn no_deficiencies(&self) -> bool {
    self.encountered_deficiencies.count() == 0
  }
}

impl CredentialValidation {
  #[doc(hidden)] // hidden until we have decided on what to do with deactivated documents
  /// Gets the credential's deactivated resolved documents if such documents were in compliance with the deficiency
  /// acceptance policy set during credential validation. See [`crate::credential::Validator::validate_credential`].
  pub fn deactivated_subject_documents(&self) -> Option<&BTreeMap<String, DocumentValidation<Deactivated>>> {
    self.deactivated_subject_documents.as_ref()
  }
}

// Temporary workaround for backward compatibility with the current Wasm implementation
impl<T: Serialize> Serialize for CredentialValidation<T> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("CredentialValidation", 4)?;
    state.serialize_field("credential", &self.credential)?;
    state.serialize_field("issuer", &self.issuer)?;
    let no_subjects: BTreeMap<String, DocumentValidation<Active>> = BTreeMap::new();
    let subjects = self.active_subject_documents.as_ref().unwrap_or(&no_subjects);
    state.serialize_field("subjects", subjects)?;
    state.serialize_field("verified", &self.no_deficiencies())?;
    state.end()
  }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PresentationValidation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: DocumentValidation<Active>,
  pub credentials: Vec<CredentialValidation<U>>,
}

impl<T: Serialize, U: Serialize> Serialize for PresentationValidation<T, U> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("PresentationValidation", 4)?;
    state.serialize_field("presentation", &self.presentation)?;
    state.serialize_field("holder", &self.holder)?;
    state.serialize_field("credentials", &self.credentials)?;
    state.serialize_field("verified", &self.no_deficiencies())?;
    state.end()
  }
}

impl<T, U> PresentationValidation<T, U> {
  /// Returns `true` if all of the presentation's credentials were validated without encountering any deficiencies.
  /// See [`crate::credential::CredentialDeficiency`].
  pub fn no_deficiencies(&self) -> bool {
    self
      .credentials
      .iter()
      .all(|credential_validation| credential_validation.no_deficiencies())
  }
}
