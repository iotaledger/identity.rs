// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use serde::Serialize;

use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_credential::validator::CredentialValidator;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::PresentationValidator;
use identity_credential::validator::ValidatorDocument;
use identity_iota_core_legacy::did::IotaDID;
use identity_iota_core_legacy::diff::DiffMessage;
use identity_iota_core_legacy::document::IotaDocument;
use identity_iota_core_legacy::tangle::NetworkName;

use crate::chain::ChainHistory;
use crate::chain::DocumentHistory;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::ClientBuilder;
use crate::tangle::SharedPtr;
use crate::tangle::TangleResolve;

/// A `Resolver` supports resolving DID Documents across different Tangle networks using
/// multiple [`Clients`][Client].
///
/// Also provides convenience functions for resolving DID Documents associated with
/// verifiable [`Credentials`][Credential] and [`Presentations`][Presentation].
#[derive(Debug)]
pub struct Resolver<C = Arc<Client>>
where
  C: SharedPtr<Client>,
{
  client_map: HashMap<NetworkName, C>,
}

impl<C> Resolver<C>
where
  C: SharedPtr<Client>,
{
  /// Constructs a new [`Resolver`] with a default [`Client`] for
  /// the [`Mainnet`](identity_iota_core_legacy::tangle::Network::Mainnet).
  ///
  /// See also [`Resolver::builder`].
  pub async fn new() -> Result<Self> {
    let client: Client = Client::new().await?;

    let mut client_map: HashMap<NetworkName, C> = HashMap::new();
    client_map.insert(client.network.name(), C::from(client));
    Ok(Self { client_map })
  }

  /// Returns a new [`ResolverBuilder`] with no configured [`Clients`](Client).
  pub fn builder() -> ResolverBuilder {
    ResolverBuilder::new()
  }

  /// Returns the [`Client`] corresponding to the given [`NetworkName`] if one exists.
  pub fn get_client(&self, network_name: &NetworkName) -> Option<&C> {
    self.client_map.get(network_name)
  }

  /// Returns the [`Client`] corresponding to the [`NetworkName`] on the given ['IotaDID'].
  fn get_client_for_did(&self, did: &IotaDID) -> Result<&C> {
    self.get_client(&did.network()?.name()).ok_or_else(|| {
      Error::DIDNotFound(format!(
        "DID network '{}' does not match any resolver client network",
        did.network_str(),
      ))
    })
  }

  /// Fetches the [`ResolvedIotaDocument`] of the given [`IotaDID`].
  pub async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    let client: &Client = self.get_client_for_did(did)?.deref();
    client.read_document(did).await
  }

  /// Fetches the [`DocumentHistory`] of the given [`IotaDID`].
  pub async fn resolve_history(&self, did: &IotaDID) -> Result<DocumentHistory> {
    let client: &Client = self.get_client_for_did(did)?.deref();
    client.resolve_history(did).await
  }

  /// Fetches the [`ChainHistory`] of a diff chain starting from a [`ResolvedIotaDocument`] on the
  /// integration chain.
  ///
  /// NOTE: the document must have been published to the Tangle and have a valid message id.
  #[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
  pub async fn resolve_diff_history(&self, document: &ResolvedIotaDocument) -> Result<ChainHistory<DiffMessage>> {
    let client: &Client = self.get_client_for_did(document.document.id())?.deref();
    client.resolve_diff_history(document).await
  }

  /// Fetches the DID Document of the issuer on a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_credential_issuer<U: Serialize>(
    &self,
    credential: &Credential<U>,
  ) -> Result<ResolvedIotaDocument> {
    let issuer: IotaDID = CredentialValidator::extract_issuer(credential).map_err(Error::IsolatedValidationError)?;
    self.resolve(&issuer).await
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL is not a valid [`IotaDID`] or DID resolution fails.
  pub async fn resolve_presentation_issuers<U, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<Vec<ResolvedIotaDocument>> {
    // Extract unique issuers.
    let issuers: HashSet<IotaDID> = presentation
      .verifiable_credential
      .iter()
      .map(|credential| {
        CredentialValidator::extract_issuer::<IotaDID, V>(credential).map_err(Error::IsolatedValidationError)
      })
      .collect::<Result<_>>()?;

    // Resolve issuers concurrently.
    futures::future::try_join_all(issuers.iter().map(|issuer| self.resolve(issuer)).collect::<Vec<_>>()).await
  }

  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, is not a valid [`IotaDID`], or DID resolution fails.
  pub async fn resolve_presentation_holder<U, V>(
    &self,
    presentation: &Presentation<U, V>,
  ) -> Result<ResolvedIotaDocument> {
    let holder: IotaDID =
      PresentationValidator::extract_holder(presentation).map_err(Error::IsolatedValidationError)?;
    self.resolve(&holder).await
  }

  /// Verifies a [`Presentation`].
  ///
  /// # Important
  /// See [`PresentationValidator::validate`](PresentationValidator::validate()) for information about which properties
  /// get validated and what is expected of the optional arguments `holder` and `issuer`.
  ///
  /// # Resolution
  /// The DID Documents for the `holder` and `issuers` are optionally resolved if not given.
  /// If you already have up-to-date versions of these DID Documents, you may want
  /// to use [`PresentationValidator::validate`].
  /// See also [`Resolver::resolve_presentation_issuers`] and [`Resolver::resolve_presentation_holder`].
  ///
  /// # Errors
  /// Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
  /// Otherwise, errors from validating the presentation and its credentials will be returned
  /// according to the `fail_fast` parameter.
  pub async fn verify_presentation<U: Serialize, V: Serialize>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    fail_fast: FailFast,
    holder: Option<&dyn ValidatorDocument>,
    issuers: Option<&[&dyn ValidatorDocument]>,
  ) -> Result<()> {
    match (holder, issuers) {
      (Some(holder), Some(issuers)) => {
        PresentationValidator::validate(presentation, &holder, issuers, options, fail_fast)
      }
      (Some(holder), None) => {
        let resolved_issuers: Vec<ResolvedIotaDocument> = self.resolve_presentation_issuers(presentation).await?;
        let issuers: Vec<IotaDocument> = resolved_issuers.into_iter().map(|resolved| resolved.document).collect();
        PresentationValidator::validate(presentation, &holder, issuers.as_slice(), options, fail_fast)
      }
      (None, Some(issuers)) => {
        let holder: ResolvedIotaDocument = self.resolve_presentation_holder(presentation).await?;
        PresentationValidator::validate(presentation, &holder.document, issuers, options, fail_fast)
      }
      (None, None) => {
        let (holder, resolved_issuers): (ResolvedIotaDocument, Vec<ResolvedIotaDocument>) = futures::future::try_join(
          self.resolve_presentation_holder(presentation),
          self.resolve_presentation_issuers(presentation),
        )
        .await?;
        let issuers: Vec<IotaDocument> = resolved_issuers.into_iter().map(|resolved| resolved.document).collect();
        PresentationValidator::validate(presentation, &holder.document, &issuers, options, fail_fast)
      }
    }
    .map_err(Into::into)
  }
}

/// Builder for configuring [`Clients`][Client] when constructing a [`Resolver`].
#[derive(Default)]
pub struct ResolverBuilder<C = Arc<Client>>
where
  C: SharedPtr<Client>,
{
  clients: HashMap<NetworkName, ClientOrBuilder<C>>,
}

#[allow(clippy::large_enum_variant)]
enum ClientOrBuilder<C> {
  Client(C),
  Builder(ClientBuilder),
}

impl<C> ResolverBuilder<C>
where
  C: SharedPtr<Client>,
{
  /// Constructs a new [`ResolverBuilder`] with no [`Clients`][Client] configured.
  pub fn new() -> Self {
    Self {
      clients: Default::default(),
    }
  }

  /// Inserts a [`Client`].
  ///
  /// NOTE: replaces any previous [`Client`] or [`ClientBuilder`] with the same [`NetworkName`].
  #[must_use]
  pub fn client(mut self, client: C) -> Self {
    self
      .clients
      .insert(client.deref().network.name(), ClientOrBuilder::Client(client));
    self
  }

  /// Inserts a [`ClientBuilder`].
  ///
  /// NOTE: replaces any previous [`Client`] or [`ClientBuilder`] with the same [`NetworkName`].
  pub fn client_builder(mut self, builder: ClientBuilder) -> Self {
    self
      .clients
      .insert(builder.network.name(), ClientOrBuilder::Builder(builder));
    self
  }

  /// Constructs a new [`Resolver`] based on the builder configuration.
  pub async fn build(self) -> Result<Resolver<C>> {
    let mut client_map: HashMap<NetworkName, C> = HashMap::new();
    for (network_name, client_or_builder) in self.clients {
      let client: C = match client_or_builder {
        ClientOrBuilder::Client(client) => client,
        ClientOrBuilder::Builder(builder) => C::from(builder.build().await?),
      };
      client_map.insert(network_name, client);
    }

    Ok(Resolver { client_map })
  }
}

#[async_trait::async_trait(?Send)]
impl TangleResolve for Resolver {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument> {
    self.resolve(did).await
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Duration;
  use identity_core::common::Object;
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_core::crypto::ProofOptions;
  use identity_core::json;
  use identity_core::utils::BaseEncoding;
  use identity_credential::credential::Credential;
  use identity_credential::credential::CredentialBuilder;
  use identity_credential::credential::Subject;
  use identity_credential::validator::CredentialValidationOptions;
  use identity_credential::validator::SubjectHolderRelationship;
  use identity_did::CoreDID;
  use identity_did::DID;
  use identity_did::document::CoreDocument;
  use identity_did::verifiable::VerifierOptions;
  use identity_did::verification::VerificationMethod;
  use identity_iota_core_legacy::document::IotaDocument;
  use identity_iota_core_legacy::tangle::Network;

  use super::*;

  fn generate_core_document() -> (CoreDocument, KeyPair) {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let did: CoreDID = CoreDID::parse(&format!(
      "did:example:{}",
      BaseEncoding::encode_base58(keypair.public())
    ))
    .unwrap();
    let document: CoreDocument = CoreDocument::builder(Object::new())
      .id(did.clone())
      .verification_method(VerificationMethod::new(did, KeyType::Ed25519, keypair.public(), "#root").unwrap())
      .build()
      .unwrap();
    (document, keypair)
  }

  fn generate_credential(issuer: &str, subject: &str) -> Credential {
    let credential_subject: Subject = Subject::from_json_value(json!({
      "id": subject,
      "name": "Alice",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts",
      },
      "GPA": "4.0",
    }))
    .unwrap();

    // Build credential using subject above and issuer.
    CredentialBuilder::default()
      .id(Url::parse("https://example.edu/credentials/3732").unwrap())
      .issuer(Url::parse(issuer).unwrap())
      .type_("UniversityDegreeCredential")
      .subject(credential_subject)
      .issuance_date(Timestamp::now_utc())
      .expiration_date(Timestamp::now_utc().checked_add(Duration::days(1)).unwrap())
      .build()
      .unwrap()
  }

  fn generate_presentation(holder: &str, credentials: Vec<Credential>) -> Presentation {
    let mut builder = Presentation::builder(Object::new())
      .id(Url::parse("https://example.org/credentials/3732").unwrap())
      .holder(Url::parse(holder).unwrap());
    for credential in credentials {
      builder = builder.credential(credential);
    }
    builder.build().unwrap()
  }

  // Convenience struct for setting up tests.
  struct MixedTestSetup {
    // Issuer of credential_iota.
    issuer_iota_doc: IotaDocument,
    issuer_iota_key: KeyPair,
    credential_iota: Credential,
    // Issuer of credential_core.
    issuer_core_doc: CoreDocument,
    issuer_core_key: KeyPair,
    credential_core: Credential,
    // Subject of both credentials.
    subject_doc: CoreDocument,
    subject_key: KeyPair,
  }

  impl MixedTestSetup {
    // Creates DID Documents and unsigned credentials.
    fn new() -> Self {
      let (issuer_iota_doc, issuer_iota_key) = {
        let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
        (IotaDocument::new(&keypair).unwrap(), keypair)
      };
      let (subject_doc, subject_key) = generate_core_document();
      let credential_iota = generate_credential(issuer_iota_doc.id().as_str(), subject_doc.id().as_str());

      let (issuer_core_doc, issuer_core_key) = generate_core_document();
      let credential_core = generate_credential(issuer_core_doc.id().as_str(), subject_doc.id().as_str());

      Self {
        issuer_iota_doc,
        issuer_iota_key,
        subject_doc,
        subject_key,
        credential_iota,
        issuer_core_doc,
        issuer_core_key,
        credential_core,
      }
    }

    // Creates DID Documents with signed credentials.
    fn new_with_signed_credentials() -> Self {
      let mut setup = Self::new();
      let MixedTestSetup {
        ref issuer_iota_doc,
        ref issuer_iota_key,
        ref mut credential_iota,
        ref issuer_core_doc,
        ref issuer_core_key,
        ref mut credential_core,
        ..
      } = setup;

      issuer_iota_doc
        .sign_data(
          credential_iota,
          issuer_iota_key.private(),
          issuer_iota_doc.default_signing_method().unwrap().id(),
          ProofOptions::default(),
        )
        .unwrap();
      issuer_core_doc
        .signer(issuer_core_key.private())
        .options(ProofOptions::default())
        .method(issuer_core_doc.methods(None).get(0).unwrap().id())
        .sign(credential_core)
        .unwrap();
      setup
    }
  }

  #[tokio::test]
  async fn test_resolver_verify_presentation_mixed() {
    let MixedTestSetup {
      issuer_iota_doc,
      credential_iota,
      issuer_core_doc,
      credential_core,
      subject_doc,
      subject_key,
      ..
    } = MixedTestSetup::new_with_signed_credentials();

    // Subject signs the presentation.
    let mut presentation =
      generate_presentation(subject_doc.id().as_str(), [credential_iota, credential_core].to_vec());
    let challenge: String = "475a7984-1bb5-4c4c-a56f-822bccd46441".to_owned();
    subject_doc
      .signer(subject_key.private())
      .options(ProofOptions::new().challenge(challenge.clone()))
      .method(subject_doc.methods(None).get(0).unwrap().id())
      .sign(&mut presentation)
      .unwrap();

    // VALID: resolver supports presentations with issuers from different DID Methods.
    let resolver: Resolver = Resolver::<Arc<Client>>::builder()
      .client_builder(Client::builder().network(Network::Devnet).node_sync_disabled())
      .build()
      .await
      .unwrap();
    assert!(resolver
      .verify_presentation(
        &presentation,
        &PresentationValidationOptions::new()
          .presentation_verifier_options(VerifierOptions::new().challenge(challenge))
          .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
        FailFast::FirstError,
        Some(&subject_doc),
        Some(&[issuer_iota_doc.as_validator(), issuer_core_doc.as_validator()])
      )
      .await
      .is_ok());
  }

  #[test]
  fn test_validate_presentation_mixed() {
    let MixedTestSetup {
      issuer_iota_doc,
      credential_iota,
      issuer_core_doc,
      credential_core,
      subject_doc,
      subject_key,
      ..
    } = MixedTestSetup::new_with_signed_credentials();

    // Subject signs the presentation.
    let mut presentation =
      generate_presentation(subject_doc.id().as_str(), [credential_iota, credential_core].to_vec());
    let challenge: String = "475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned();
    subject_doc
      .signer(subject_key.private())
      .options(ProofOptions::new().challenge(challenge.clone()))
      .method(subject_doc.methods(None).get(0).unwrap().id())
      .sign(&mut presentation)
      .unwrap();

    // Validate presentation.
    let presentation_validation_options = PresentationValidationOptions::new()
      .shared_validation_options(
        CredentialValidationOptions::new()
          .earliest_expiry_date(Timestamp::now_utc().checked_add(Duration::days(1)).unwrap())
          .latest_issuance_date(Timestamp::now_utc()),
      )
      .presentation_verifier_options(VerifierOptions::new().challenge(challenge))
      .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject);

    // VALID: presentations with issuers from different DID Methods are supported.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_iota_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_ok());

    // INVALID: wrong holder fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &issuer_iota_doc,
      &[issuer_iota_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());
    assert!(PresentationValidator::validate(
      &presentation,
      &issuer_core_doc,
      &[issuer_iota_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: excluding the IOTA DID Document issuer fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: excluding the core DID Document issuer fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[&issuer_iota_doc],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: using the wrong core DID Document fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_iota_doc.as_validator(), subject_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: excluding all issuers fails.
    let empty_issuers: &[&dyn ValidatorDocument] = &[];
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      empty_issuers,
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());
  }

  #[test]
  fn test_validate_credential_mixed() {
    let MixedTestSetup {
      issuer_iota_doc,
      credential_iota,
      issuer_core_doc,
      credential_core,
      subject_doc,
      ..
    } = MixedTestSetup::new_with_signed_credentials();
    let options = CredentialValidationOptions::new()
      .earliest_expiry_date(Timestamp::now_utc().checked_add(Duration::days(1)).unwrap())
      .latest_issuance_date(Timestamp::now_utc());

    // VALID: credential validation works for issuers with different DID Methods.
    assert!(CredentialValidator::validate(&credential_iota, &issuer_iota_doc, &options, FailFast::FirstError).is_ok());
    assert!(CredentialValidator::validate(&credential_core, &issuer_core_doc, &options, FailFast::FirstError).is_ok());

    // INVALID: wrong issuer fails.
    assert!(CredentialValidator::validate(&credential_iota, &issuer_core_doc, &options, FailFast::FirstError).is_err());
    assert!(CredentialValidator::validate(&credential_iota, &subject_doc, &options, FailFast::FirstError).is_err());
    assert!(CredentialValidator::validate(&credential_core, &issuer_iota_doc, &options, FailFast::FirstError).is_err());
    assert!(CredentialValidator::validate(&credential_core, &subject_doc, &options, FailFast::FirstError).is_err());
  }
}
