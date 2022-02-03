// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;

use super::ResolvedCredential;
use crate::document::ResolvedIotaDocument;

/// A verifiable presentation whose associated DID documents have been resolved from the Tangle.
pub struct ResolvedPresentation<T = Object, U = Object> {
  pub presentation: Presentation<T, U>,
  pub holder: ResolvedIotaDocument,
  pub credentials: Vec<ResolvedCredential<U>>,
}

impl<T, U> ResolvedPresentation<T, U> {
  delegate::delegate! {
      to self.presentation {
          /// An iterator over the credentials that have the `nonTransferable` property set, but
          /// the credential subject id does not correspond to URL of the presentation's holder
          pub fn non_transferable_violations(&self) -> impl Iterator<Item = &Credential<U>> + '_ ;

          /// Validates the semantic structure of the `Presentation`.
          pub fn check_structure(&self) -> Result<(), identity_credential::Error>;
      }
  }
}
