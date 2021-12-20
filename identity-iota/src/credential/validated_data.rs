// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use self::document_state::Active;
use self::document_state::Deactivated;
use serde::Serialize;
use serde::ser::SerializeStruct;
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

// Now some workarounds for backward compatibility with the Wasm bindings until we get round to implementing the changes there too. 
impl Serialize for DocumentValidation<Active> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
          S: serde::Serializer {
      let mut state = serializer.serialize_struct("DocumentValidation", 4)?;
      state.serialize_field("did", &self.did);
      state.serialize_field("document", &self.document);
      state.serialize_field("metadata", &self.metadata); 
      state.serialize_field("verified", &true); 
      state.end()
  }
}

impl Serialize for DocumentValidation<Deactivated> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
          S: serde::Serializer {
      let mut state = serializer.serialize_struct("DocumentValidation", 4)?;
      state.serialize_field("did", &self.did);
      state.serialize_field("document", &self.document);
      state.serialize_field("metadata", &self.metadata); 
      state.serialize_field("verified", &false); 
      state.end()
  }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PresentationValidation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: DocumentValidation<Active>,
  pub credentials: Vec<CredentialValidation<U>>,
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
