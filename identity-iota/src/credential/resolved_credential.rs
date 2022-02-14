// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_credential::credential::Credential;
use identity_did::did::DID;
use identity_did::verifiable::VerifierOptions;
use serde::Serialize;

use super::errors::ValidationError;
use super::CredentialValidationOptions;
use super::CredentialValidator;
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

impl<T: Serialize> ResolvedCredential<T> {
  /// Resolves the issuer's DID Document and combines it with the credential as a [ResolvedCredential].
  ///
  /// Note: This method only resolves the issuer's DID document. If checks concerning the DID documents of the
  /// credential's subjects are necessary then one should use [`Self::assemble()`] instead.
  // Todo: Remove this method and let the new Resolver be responsible for construction instead. 
  pub async fn from_remote_issuer_document<R: TangleResolve>(credential: Credential<T>, resolver: &R) -> Result<Self> {
    let issuer_url: &str = credential.issuer.url().as_str();
    let did: IotaDID = issuer_url
      .parse::<IotaDID>()
      .map_err(|error| ValidationError::IssuerUrl { source: error.into() })
      .map_err(Error::InvalidCredentialPairing)?;
    let issuer: ResolvedIotaDocument = resolver.resolve(&did).await?;

    Ok(Self { credential, issuer })
  }

  /// Validate the credential using the issuer's resolved DID Document received upon creation.
  /// # Errors
  /// Fails if any of the following conditions occur
  /// - The structure of the credential is not semantically valid
  /// - The expiration date does not meet the requirement set in `options`
  /// - The issuance date does not meet the requirement set in `options`
  /// - The issuer has not been specified as trust
  /// - The credential's signature cannot be verified using the issuer's DID Document
  // Todo: Should we also check for deactivated subject documents here?
  pub fn full_validation(&self, validation_options: &CredentialValidationOptions, fail_fast: bool) -> Result<()> {
    CredentialValidator::new().validate_credential(
      &self.credential,
      validation_options,
      std::slice::from_ref(&self.issuer),
      fail_fast,
    )
  }
}
