// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::did::DID;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::credential_validator::CredentialValidator;
use super::errors::ValidationError;
use super::CredentialValidationOptions;
use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::tangle::TangleResolve;
use crate::Error;
use crate::Result;
use delegate::delegate;

/// A verifiable credential whose associated DID documents have been resolved from the Tangle.
///
/// This struct enables low-level control over how a [`Credential`] gets validated by offering the following validation
/// units
/// - [`Self::verify_signature()`]
/// - [`Self::try_issued_before()`]
/// - [`Self::try_only_active_subject_documents`]
/// - [`Self::try_expires_after()`]
/// - [`Self::check_structure()`]
///
/// # Security
/// This struct uses resolved DID Documents received upon construction. These associated documents may become outdated
/// at any point in time and will then no longer be fit for purpose. We encourage disposing these objects as soon as
/// possible.
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
