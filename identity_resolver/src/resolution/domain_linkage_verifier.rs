// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::Error;
use crate::ErrorCause;
use crate::ErrorCause::DNSVerificationError;
use crate::Resolver;
use identity_core::common::OneOrMany;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_credential::credential::Credential;
use identity_credential::credential::DIDConfigurationResource;

use identity_credential::validator::AbstractThreadSafeValidatorDocument;
use identity_credential::validator::ValidatorDocument;
use identity_did::did::CoreDID;
use identity_did::did::DIDError;

use identity_did::did::DID;
use identity_did::verifiable::VerifierOptions;

use reqwest::redirect::Policy;


pub struct DomainLinkageVerifier {
  resolver: Resolver,
}

impl DomainLinkageVerifier {
  /// Creates a new `DomainLinkageVerifier`.
  pub fn new(resolver: Resolver) -> Self {
    Self { resolver }
  }

  /// Fetches the the [DID Configuration resource](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource) via a GET request at the
  /// well-known location: `origin`/.well-known/did-configuration.json.
  ///
  /// And verifies it for the following criteria:
  /// - [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential)s
  ///   are spec compliant and not expired.
  /// - [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification)
  ///
  /// Note:
  /// - Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
  ///   is supported.
  /// - The controller of the DID is not verified.
  /// - If multiple Domain Linkage Credentials exist, all of them are verified.
  pub async fn fetch_and_verify_configuration(self, origin: Url) -> Result<DIDConfigurationResource> {
    let config = Self::fetch_configuration(origin).await?;
    self.verify_configuration(&config).await?;
    Ok(config)
  }

  /// Fetches the the DID Configuration resource via a GET request at the
  /// well-known location: `origin`/.well-known/did-configuration.json.
  pub async fn fetch_configuration(origin: Url) -> Result<DIDConfigurationResource> {
    if origin.scheme() != "https" {
      return Err(Error::new(DNSVerificationError {
        reason: "`origin` does not use `https` protocol.".to_owned(),
      }));
    }
    let client = reqwest::ClientBuilder::new()
      .redirect(Policy::none())
      .build()
      .map_err(|_err| {
        Error::new(DNSVerificationError {
          reason: "reqwest client error".to_owned(),
        })
      })?;

    let res = client
      .get(origin.to_string() + "/.well-known/did-configuration.json")
      .send()
      .await
      .map_err(|_err| {
        Error::new(ErrorCause::DNSVerificationError {
          reason: "Fetching configuration failed!".to_owned(),
        })
      })?
      .json::<DIDConfigurationResource>()
      .await
      .map_err(|_err| {
        Error::new(ErrorCause::DNSVerificationError {
          reason: "Incorrect DID Configuration resource format. Deserialization failed!".to_owned(),
        })
      })?;
    Ok(res)
  }

  /// Verifies a [DID Configuration Resource](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource) for the following criteria:
  /// - [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential)s
  ///   are spec compliant and not expired.
  /// - [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification)
  ///
  /// Note:
  /// - Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
  ///   is supported.
  /// - The controller of the DID is not verified.
  /// - If multiple Domain Linkage Credentials exist, all of them are verified.
  pub async fn verify_configuration(self, config: &DIDConfigurationResource) -> Result<()> {
    // Additional members MUST NOT be present in the DID Configuration Resource.
    if !config.properties.is_empty() {
      return Err(Error::new(ErrorCause::DNSVerificationError {
        reason: "Configuration Resource contains additional members".to_owned(),
      }));
    }

    for credential in config.linked_dids() {
      self.verify_domain_linkage_credential(credential).await?;
    }

    Ok(())
  }

  async fn verify_domain_linkage_credential(&self, credential: &Credential) -> Result<()> {
    if credential.id.is_some() {
      return Err(Error::new(ErrorCause::DNSVerificationError {
        reason: "`id` property is present in credential".to_owned(),
      }));
    }

    if credential.issuance_date > Timestamp::now_utc() {
      return Err(Error::new(ErrorCause::DNSVerificationError {
        reason: "Issuance date in the future".to_owned(),
      }));
    }

    match credential.expiration_date {
      Some(expiration_date) => {
        if expiration_date <= Timestamp::now_utc() {
          return Err(Error::new(ErrorCause::DNSVerificationError {
            reason: "Credential is expired".to_owned(),
          }));
        }
      }
      None => {
        return Err(Error::new(ErrorCause::DNSVerificationError {
          reason: "`Expiration_date` is not set.".to_owned(),
        }));
      }
    }

    let issuer: Result<CoreDID, DIDError> = CoreDID::parse(credential.issuer.url().to_string());
    match issuer {
      Ok(issuer_did) => {
        match &credential.credential_subject {
          OneOrMany::One(credential_subject) => match &credential_subject.id {
            None => {
              return Err(Error::new(ErrorCause::DNSVerificationError {
                reason: "Missing subject id.".to_owned(),
              }));
            }
            Some(id) => match CoreDID::parse(id.to_string()) {
              Ok(subject_did) => {
                if issuer_did != subject_did {
                  return Err(Error::new(ErrorCause::DNSVerificationError {
                    reason: "Issuer and subject DIDs don't match.".to_owned(),
                  }));
                }
              }
              Err(_) => {
                return Err(Error::new(ErrorCause::DNSVerificationError {
                  reason: "Invalid subject id.".to_owned(),
                }));
              }
            },
          },
          OneOrMany::Many(_) => {
            return Err(Error::new(ErrorCause::DNSVerificationError {
              reason: "Only one credential subject must be present".to_owned(),
            }));
          }
        }
        // Resolve issuer
        let issuer_resolved = self.resolve_issuer(&issuer_did).await?;
        issuer_resolved
          .verify_data(credential, &VerifierOptions::default())
          .map_err(|err| {
            Error::new(ErrorCause::DNSVerificationError {
              reason: format!("Signature verification failed! {}", err),
            })
          })?;
      }
      Err(_e) => {
        return Err(Error::new(ErrorCause::DNSVerificationError {
          reason: "Invalid issuer".to_owned(),
        }));
      }
    }
    Ok(())
  }

  async fn resolve_issuer<D: DID>(&self, issuer_did: &D) -> Result<AbstractThreadSafeValidatorDocument> {
    let resolved_doc: AbstractThreadSafeValidatorDocument = self.resolver.resolve(issuer_did).await.map_err(|err| {
      Error::new(ErrorCause::DNSVerificationError {
        reason: format!("Issuer resolution failed! {}", err),
      })
    })?;
    Ok(resolved_doc)
  }
}

mod tests {
  use identity_core::convert::FromJson;
  use crate::error::Result;
  use crate::resolution::domain_linkage_verifier::DomainLinkageVerifier;
  use crate::Resolver;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::ProofOptions;
  use identity_core::crypto::KeyType;
  use identity_credential::credential::Credential;
  use identity_credential::credential::DIDConfigurationResource;
  use identity_did::verification::MethodScope;
  use identity_iota_core::IotaDID;
  use identity_iota_core::IotaDocument;
  use identity_iota_core::IotaVerificationMethod;

  #[tokio::test]
  pub async fn test() {
    let configuration_string: &str = include_str!("./tests/fixtures/did_configuration/config1.json");
    let mut configuration: DIDConfigurationResource =
      DIDConfigurationResource::from_json(configuration_string).unwrap();

    // let mut configuration = DomainLinkageVerifier::fetch_configuration(Url::parse("http://192.168.178.23:8080/".to_owned()).unwrap()).await.unwrap();
    let credential: &mut Credential = configuration.linked_dids_mut().get_mut(0).unwrap();
    let (doc, keypair) = compose_did_document(credential.issuer.url().to_string());

    // Add valid proof from the created DID.
    doc
      .sign_data(credential, keypair.private(), "#key-1", ProofOptions::default())
      .unwrap();

    verify_config(doc, &configuration).await.unwrap();
  }

  #[tokio::test]
  pub async fn no_proof() {
    let configuration_string: &str = include_str!("./tests/fixtures/did_configuration/config1.json");
    let mut configuration: DIDConfigurationResource =
      DIDConfigurationResource::from_json(configuration_string).unwrap();
    let credential: &mut Credential = configuration.linked_dids_mut().get_mut(0).unwrap();
    let (doc, _) = compose_did_document(credential.issuer.url().to_string());
    let res = verify_config(doc, &configuration).await;
    assert!(res.is_err());
  }

  // Todo: Add further tests for error cases.

  async fn verify_config(doc: IotaDocument, config: &DIDConfigurationResource) -> Result<()> {
    let method_name: String = "iota".to_owned();
    let mut resolver: Resolver = Resolver::new();
    resolver.attach_handler(method_name, move |_did: IotaDID| same_doc(doc.clone()));
    let dns_verifier: DomainLinkageVerifier = DomainLinkageVerifier::new(resolver);
    dns_verifier.verify_configuration(config).await
  }

  fn compose_did_document(did: String) -> (IotaDocument, KeyPair) {
    let iota_did = IotaDID::parse(did.clone()).unwrap();
    let mut doc = IotaDocument::new_with_id(iota_did);
    let keypair = KeyPair::new(KeyType::Ed25519).unwrap();
    let verification_method = IotaVerificationMethod::new(
      IotaDID::parse(did).unwrap(),
      KeyType::Ed25519,
      keypair.public(),
      "#key-1",
    )
    .unwrap();
    doc
      .insert_method(verification_method, MethodScope::VerificationMethod)
      .unwrap();
    (doc, keypair)
  }

  async fn same_doc(doc: IotaDocument) -> std::result::Result<IotaDocument, std::io::Error> {
    Ok(doc)
  }
}
