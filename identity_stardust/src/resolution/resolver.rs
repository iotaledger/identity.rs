// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use identity_credential::{
  credential::Credential,
  presentation::Presentation,
  validator::{
    BorrowValidator, CredentialValidator, FailFast, PresentationValidationOptions, PresentationValidator,
    ValidatorDocument,
  },
};
use identity_did::{
  did::{CoreDID, DID},
  document::Document,
};
use serde::Serialize;

use crate::{Error, ResolutionHandler, Result};

use super::resolver_delegate::{AsyncFnPtr, ResolverDelegate};

/// Convenience type for resolving did documents from different did methods.   
///  
/// Also provides functions for resolving DID Documents associated with
/// verifiable [`Credentials`][Credential] and [`Presentations`][Presentation].
///
/// # Configuration
/// The resolver will only be able to resolve did documents corresponding to a certain method after it has been
/// configured to do so. This setup is achieved by implementing the [`MethodBoundedResolver` trait](super::MethodBoundResolver) for your client
/// and then attaching it with [`Self::attach_method_handler`](`Resolver::attach_method_handler`).
pub struct Resolver<DOC = Box<dyn ValidatorDocument>>
where
  DOC: BorrowValidator,
{
  method_map: HashMap<String, AsyncFnPtr<str, Result<DOC>>>,
}

impl<DOC> Resolver<DOC>
where
  DOC: BorrowValidator,
{
  /// Constructs a new [`Resolver`].
  pub fn new() -> Self {
    Self {
      method_map: HashMap::new(),
    }
  }

  /// Constructs a new [`Resolver`] that operates with DID Documents abstractly.
  pub fn new_dynamic() -> Resolver<Box<dyn ValidatorDocument>> {
    Resolver::<Box<dyn ValidatorDocument>>::new()
  }

  /// Attach a [`ResolverHandler`] to this resolver. If the resolver has previously been configured to handle the same DID method
  /// the new handler will replace the previous.  
  pub fn attach_method_handler<D, R>(&mut self, handler: Arc<R>)
  where
    D: DID + Send + for<'r> TryFrom<&'r str> + 'static,
    R: ResolutionHandler<D> + 'static,
    DOC: From<<R as ResolutionHandler<D>>::Resolved> + 'static,
  {
    let ResolverDelegate::<DOC> { method, handler } = ResolverDelegate::new(handler);
    self.method_map.insert(method, handler);
  }

  /// Fetches the DID Document of the given DID and attempts to cast the result to the desired type.
  ///
  /// If this Resolver was constructed by the [`Resolver::new_dynamic`](Resolver::new_dynamic()) method, one may also want to consider [`Resolver::resolve_to`](Resolver::<Box<dyn ValidatorDocument>>::resolve_to()).
  ///
  /// # Errors
  /// Errors if the resolver has not been configured to handle the method corresponding to the given did or the resolution process itself fails.  
  //TODO: Improve error handling.
  pub async fn resolve<D: DID>(&self, did: &D) -> Result<DOC> {
    self.delegate_resolution(did.method(), did.as_str()).await
  }

  /// Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
  /// Issuer documents are returned in arbitrary order.
  ///
  /// # Errors
  ///
  /// Errors if any issuer URL cannot be parsed to a DID whose associated method is supported by this Resolver, or resolution fails.
  // TODO: Improve error handling.
  pub async fn resolve_presentation_issuers<U, V>(&self, presentation: &Presentation<U, V>) -> Result<Vec<DOC>> {
    // Extract unique issuers.
    //TODO: Improve error handling.
    let issuers: HashSet<CoreDID> = presentation
      .verifiable_credential
      .iter()
      .map(|credential| {
        CredentialValidator::extract_issuer::<CoreDID, V>(credential)
          .map_err(|_| Error::ResolutionProblem("Failed to parse the issuer's did from a credential".into()))
      })
      .collect::<Result<_>>()?;

    if let Some(unsupported_method) = issuers
      .iter()
      .find(|issuer| !self.method_map.contains_key(issuer.method()))
      .map(|issuer| issuer.method())
    {
      // The presentation contains did's whose methods are not attached to this Resolver.
      // TODO: Find a much better error!
      return Err(Error::ResolutionProblem(format!(
        "the presentation contains a credential issued with the following unsupported did method: {}",
        unsupported_method
      )));
    }
    // Resolve issuers concurrently.
    futures::future::try_join_all(
      issuers
        .iter()
        .map(|issuer| self.delegate_resolution(issuer.method(), issuer.as_str()))
        .collect::<Vec<_>>(),
    )
    .await
  }

  /// Fetches the DID Document of the holder of a [`Presentation`].
  ///
  /// # Errors
  ///
  /// Errors if the holder URL is missing, cannot be parsed to a valid DID whose method is supported by the resolver, or DID resolution fails.
  //TODO: Improve error handling
  pub async fn resolve_presentation_holder<U, V>(&self, presentation: &Presentation<U, V>) -> Result<DOC> {
    let holder: CoreDID = PresentationValidator::extract_holder(presentation)
      .map_err(|error| Error::ResolutionProblem(error.to_string()))?;
    self.delegate_resolution(holder.method(), holder.as_str()).await
  }

  /// Fetches the DID Document of the issuer of a [`Credential`].
  ///
  /// # Errors
  ///
  /// Errors if the issuer URL cannot be parsed to a DID with a method supported by the resolver, or resolution fails.
  // TODO: Improve errors!
  pub async fn resolve_credential_issuer<U: Serialize>(&self, credential: &Credential<U>) -> Result<DOC> {
    let issuer_did: CoreDID = CredentialValidator::extract_issuer(credential)
      .map_err(|_| Error::ResolutionProblem("failed to parse the issuer's did".into()))?;
    self.delegate_resolution(issuer_did.method(), issuer_did.as_str()).await
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
  /// See also [`Resolver::resolve_presentation_issuers`] and [`Resolver::resolve_presentation_holder`]. Note that
  /// DID Documents of a certain method can only be resolved if the resolver has been configured handle this method.
  /// See [Self::attach_method_handler].
  ///
  /// # Errors
  /// Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
  /// Otherwise, errors from validating the presentation and its credentials will be returned
  /// according to the `fail_fast` parameter.
  pub async fn verify_presentation<U, V, HDOC, IDOC>(
    &self,
    presentation: &Presentation<U, V>,
    options: &PresentationValidationOptions,
    fail_fast: FailFast,
    holder: Option<&HDOC>,
    issuers: Option<&[IDOC]>,
  ) -> Result<()>
  where
    U: Serialize,
    V: Serialize,
    HDOC: BorrowValidator + ?Sized,
    IDOC: BorrowValidator,
  {
    match (holder, issuers) {
      (Some(holder), Some(issuers)) => {
        PresentationValidator::validate(presentation, holder, issuers, options, fail_fast)
      }
      (Some(holder), None) => {
        let issuers: Vec<DOC> = self.resolve_presentation_issuers(presentation).await?;
        PresentationValidator::validate(presentation, holder, issuers.as_slice(), options, fail_fast)
      }
      (None, Some(issuers)) => {
        let holder = self.resolve_presentation_holder(presentation).await?;
        PresentationValidator::validate(presentation, &holder, issuers, options, fail_fast)
      }
      (None, None) => {
        let (holder, issuers): (DOC, Vec<DOC>) = futures::future::try_join(
          self.resolve_presentation_holder(presentation),
          self.resolve_presentation_issuers(presentation),
        )
        .await?;

        PresentationValidator::validate(presentation, &holder, &issuers, options, fail_fast)
      }
    }
    .map_err(Into::into)
  }

  /// Delegates Resolution to the relevant attached [`ResolutionHandler`].
  ///
  /// The first input parameters `method` and `did` must be &str representations of the DID method name and the DID
  /// respectively.  
  async fn delegate_resolution(&self, method: &str, did: &str) -> Result<DOC> {
    let delegate = self
      .method_map
      .get(method)
      .ok_or_else(|| Error::ResolutionProblem("did method not supported".into()))?;

    delegate(did).await
  }
}

impl Resolver<Box<dyn ValidatorDocument>> {
  /// Fetches the DID Document of the given DID and attempts to cast the result to the desired document type.
  ///
  ///
  /// # Errors
  /// Errors if the resolver has not been configured to handle the method corresponding to the given did, the resolution process itself fails, or the
  /// resolved document is of another type than the specified [`Document`] implementer.  
  //TODO: Improve error handling.
  pub async fn resolve_to<DOCUMENT, D>(&self, did: &D) -> Result<DOCUMENT>
  where
    D: DID,
    DOCUMENT: Document + 'static,
  {
    let validator_doc = self.delegate_resolution(did.method(), did.as_str()).await?;

    validator_doc
      .into_any()
      .downcast::<DOCUMENT>()
      .map(|boxed| *boxed)
      .map_err(|_| Error::ResolutionProblem("failed to convert the resolved document to the desired type".into()))
  }
}

impl<DOC: BorrowValidator> Default for Resolver<DOC> {
  fn default() -> Self {
    Self::new()
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
  use identity_did::did::CoreDID;
  use identity_did::did::DID;
  use identity_did::document::CoreDocument;
  use identity_did::verifiable::VerifierOptions;
  use identity_did::verification::MethodScope;
use identity_did::verification::VerificationMethod;
  use crate::StardustDID;
  use crate::StardustDocument;
  use crate::StardustVerificationMethod; 
  use super::*;



  fn generate_stardust_document(keypair: &KeyPair) -> StardustDocument {
    let mut document: StardustDocument = StardustDocument::new_with_id(
      "did:stardust:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
    .parse()
    .unwrap()
  );
    let method: StardustVerificationMethod = StardustVerificationMethod::new(
      document.id().clone(),
      keypair.type_(),
      keypair.public(),
      "issuerKey"
    ).unwrap();
    document.insert_method(method, MethodScope::VerificationMethod);
    document 

  }

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
    issuer_stardust_doc: StardustDocument,
    issuer_stardust_key: KeyPair,
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
      let (issuer_stardust_doc, issuer_stardust_key) = {
        let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
        (generate_stardust_document(&keypair), keypair)
      };
      let (subject_doc, subject_key) = generate_core_document();
      let credential_iota = generate_credential(issuer_stardust_doc.id().as_str(), subject_doc.id().as_str());

      let (issuer_core_doc, issuer_core_key) = generate_core_document();
      let credential_core = generate_credential(issuer_core_doc.id().as_str(), subject_doc.id().as_str());

      Self {
        issuer_stardust_doc: issuer_stardust_doc,
        issuer_stardust_key,
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
        issuer_stardust_doc: ref issuer_stardust_doc,
        ref issuer_stardust_key,
        ref mut credential_iota,
        ref issuer_core_doc,
        ref issuer_core_key,
        ref mut credential_core,
        ..
      } = setup;

      issuer_stardust_doc
      .signer(issuer_stardust_key.private()) 
      .options(ProofOptions::default())
      .method(issuer_stardust_doc.methods().next().unwrap().id())
      .sign(credential_iota).unwrap();

      issuer_core_doc
        .signer(issuer_core_key.private())
        .options(ProofOptions::default())
        .method(issuer_core_doc.methods().next().unwrap().id())
        .sign(credential_core)
        .unwrap();
      setup
    }
  }

  #[tokio::test]
  async fn test_resolver_verify_presentation_mixed() {
    let MixedTestSetup {
      issuer_stardust_doc,
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
      .method(subject_doc.methods().next().unwrap().id())
      .sign(&mut presentation)
      .unwrap();

    // VALID: resolver supports presentations with issuers from different DID Methods.
    let resolver: Resolver = Resolver::default(); 
    assert!(resolver
      .verify_presentation(
        &presentation,
        &PresentationValidationOptions::new()
          .presentation_verifier_options(VerifierOptions::new().challenge(challenge))
          .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
        FailFast::FirstError,
        Some(&subject_doc),
        Some(&[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()])
      )
      .await
      .is_ok());
  }

  #[test]
  fn test_validate_presentation_mixed() {
    let MixedTestSetup {
      issuer_stardust_doc: issuer_stardust_doc,
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
      .method(subject_doc.methods().next().unwrap().id())
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
      &[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_ok());

    /*
    let issuer_iota_box: Box<dyn ValidatorDocument> = Box::new(issuer_stardust_doc.clone());
    let issuer_core_box: Box<dyn ValidatorDocument> = Box::new(issuer_core_doc.clone());
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_iota_box, issuer_core_box],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_ok());
    */

    // INVALID: wrong holder fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &issuer_stardust_doc,
      &[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());
    assert!(PresentationValidator::validate(
      &presentation,
      &issuer_core_doc,
      &[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()],
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
      &[&issuer_stardust_doc],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: using the wrong core DID Document fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_stardust_doc.as_validator(), subject_doc.as_validator()],
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
      issuer_stardust_doc: issuer_stardust_doc,
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
    assert!(CredentialValidator::validate(&credential_iota, &issuer_stardust_doc, &options, FailFast::FirstError).is_ok());
    assert!(CredentialValidator::validate(&credential_core, &issuer_core_doc, &options, FailFast::FirstError).is_ok());

    // INVALID: wrong issuer fails.
    assert!(CredentialValidator::validate(&credential_iota, &issuer_core_doc, &options, FailFast::FirstError).is_err());
    assert!(CredentialValidator::validate(&credential_iota, &subject_doc, &options, FailFast::FirstError).is_err());
    assert!(CredentialValidator::validate(&credential_core, &issuer_stardust_doc, &options, FailFast::FirstError).is_err());
    assert!(CredentialValidator::validate(&credential_core, &subject_doc, &options, FailFast::FirstError).is_err());
  }
}