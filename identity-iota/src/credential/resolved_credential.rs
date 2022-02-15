// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::credential_validator::CredentialValidator;
use super::CredentialValidationOptions;
use crate::document::ResolvedIotaDocument;
use crate::Result;
use identity_credential::credential::Credential;
use serde::Serialize;

/// A verifiable credential whose associated DID documents have been resolved from the Tangle.
#[non_exhaustive]
pub struct ResolvedCredential<T> {
  pub credential: Credential<T>,
  pub issuer: ResolvedIotaDocument,
}

// Todo: Provide a way to construct this object on the new Resolver.

impl<T: Serialize> ResolvedCredential<T> {
  /// Validate the credential.
  ///
  /// Common concerns are checked such as the credential's signature, expiration date, issuance date and semantic
  /// structure.
  ///
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the credential is not semantically valid
  /// - The expiration date does not meet the requirement set in `options`
  /// - The issuance date does not meet the requirement set in `options`
  /// - The issuer has not been specified as trust
  /// - The credential's signature cannot be verified using the issuer's DID Document
  // Todo: Should we also check for deactivated subject documents here?
  pub fn validate(&self, options: &CredentialValidationOptions) -> Result<()> {
    CredentialValidator::new(&self.credential).full_validation(options, std::slice::from_ref(&self.issuer))
  }
}
