use std::error::Error;

use domain_linkage::domain_linkage_server::DomainLinkage;
use domain_linkage::domain_linkage_server::DomainLinkageServer;
use domain_linkage::ValidateDidRequest;
use domain_linkage::ValidateDomainLinkedDidValidationResult;
use domain_linkage::ValidateDomainRequest;
use domain_linkage::ValidateDomainResponse;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Url;
use identity_iota::credential::DomainLinkageConfiguration;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtDomainLinkageValidator;
use identity_iota::did::CoreDID;
// use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::resolver::Resolver;
use iota_sdk::client::Client;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tonic::Request;
use tonic::Response;
use tonic::Status;

#[allow(clippy::module_inception)]
mod domain_linkage {
  tonic::include_proto!("domain_linkage");
}

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "error", content = "reason")]
enum DomainLinkageError {
  #[error("Failed to fetch domain linkage configuration: {0}")]
  DomainLinkageConfiguration(String),
  #[error("domain argument invalid: {0}")]
  DomainParsingFailed(String),
}

impl From<DomainLinkageError> for tonic::Status {
  fn from(value: DomainLinkageError) -> Self {
    let code = match &value {
      DomainLinkageError::DomainLinkageConfiguration(_) => tonic::Code::Internal,
      DomainLinkageError::DomainParsingFailed(_) => tonic::Code::InvalidArgument,
    };
    let message = value.to_string();
    let error_json = serde_json::to_vec(&value).expect("plenty of memory!"); // ?

    dbg!(&message);
    tonic::Status::with_details(code, message, error_json.into())
  }
}

#[derive(Debug)]
pub struct DomainLinkageService {
  resolver: Resolver<IotaDocument>,
}

impl DomainLinkageService {
  pub fn new(client: &Client) -> Self {
    let mut resolver = Resolver::new();
    resolver.attach_iota_handler(client.clone());
    Self { resolver }
  }
}

#[tonic::async_trait]
impl DomainLinkage for DomainLinkageService {
  #[tracing::instrument(
    name = "domain_linkage_validate_domain",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
  )]
  async fn validate_domain(
    &self,
    req: Request<ValidateDomainRequest>,
  ) -> Result<Response<ValidateDomainResponse>, Status> {
    // fetch DID configuration resource
    let domain: Url = Url::parse(&req.into_inner().domain.to_string())
      .map_err(|err| DomainLinkageError::DomainParsingFailed(err.to_string()))?;
    let configuration_resource: DomainLinkageConfiguration =
      DomainLinkageConfiguration::fetch_configuration(domain.clone())
        .await
        .map_err(|err| DomainLinkageError::DomainLinkageConfiguration(err.source().unwrap_or(&err).to_string()))?;

    // get issuers of `linked_dids` credentials
    let linked_dids: Vec<CoreDID> = configuration_resource
      .issuers()
      .map_err(|e| Status::internal(e.to_string()))?;

    // resolve all issuers
    let resolved = self
      .resolver
      .resolve_multiple(&linked_dids)
      .await
      .map_err(|e| Status::internal(e.to_string()))?;

    // check linked DIDs separately
    let errors: Vec<Option<String>> = resolved
      .values()
      .map(|issuer_did_doc| {
        JwtDomainLinkageValidator::with_signature_verifier(EdDSAJwsVerifier::default())
          .validate_linkage(
            &issuer_did_doc,
            &configuration_resource,
            &domain.clone(),
            &JwtCredentialValidationOptions::default(),
          )
          .err()
          .map(|err| err.to_string())
      })
      .collect();

    // collect resolved documents and their validation status into array following the order of `linked_dids`
    let response = ValidateDomainResponse {
      linked_dids: configuration_resource
        .linked_dids()
        .iter()
        .zip(errors.iter())
        .map(|(credential, error)| ValidateDomainLinkedDidValidationResult {
          document: credential.as_str().to_string(),
          valid: error.is_none(),
          error: error.clone(),
        })
        .collect(),
    };

    Ok(Response::new(response))
  }

  #[tracing::instrument(
    name = "domain_linkage_validate_did",
    skip_all,
    fields(request = ?_req.get_ref())
    ret,
    err,
  )]
  async fn validate_did(&self, _req: Request<ValidateDidRequest>) -> Result<Response<()>, Status> {
    todo!("implement validate_did")
  }
}

pub fn service(client: &Client) -> DomainLinkageServer<DomainLinkageService> {
  DomainLinkageServer::new(DomainLinkageService::new(client))
}
