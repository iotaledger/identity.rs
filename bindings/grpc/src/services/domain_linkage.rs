use std::error::Error;

use domain_linkage::domain_linkage_server::DomainLinkage;
use domain_linkage::domain_linkage_server::DomainLinkageServer;
use domain_linkage::LinkedDidEndpointValidationStatus;
use domain_linkage::LinkedDidValidationStatus;
use domain_linkage::ValidateDidRequest;
use domain_linkage::ValidateDidResponse;
use domain_linkage::ValidateDomainRequest;
use domain_linkage::ValidateDomainResponse;
use futures::stream::FuturesOrdered;
use futures::TryStreamExt;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Url;
use identity_iota::credential::DomainLinkageConfiguration;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtDomainLinkageValidator;
use identity_iota::credential::LinkedDomainService;
use identity_iota::did::CoreDID;
use identity_iota::iota::IotaDID;
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
  #[error("domain argument invalid: {0}")]
  DomainParsingFailed(String),
}

impl From<DomainLinkageError> for tonic::Status {
  fn from(value: DomainLinkageError) -> Self {
    let code = match &value {
      DomainLinkageError::DomainParsingFailed(_) => tonic::Code::InvalidArgument,
    };
    let message = value.to_string();
    let error_json = serde_json::to_vec(&value).expect("plenty of memory!"); // ?

    tonic::Status::with_details(code, message, error_json.into())
  }
}

fn get_validation_failed_status(message: &str, err: &impl Error) -> LinkedDidValidationStatus {
  let source_suffix = err
    .source()
    .map(|err| format!("; {}", &err.to_string()))
    .unwrap_or_default();
  let inner_error_message = format!("{}{}", &err.to_string(), source_suffix);

  LinkedDidValidationStatus {
    valid: false,
    document: None,
    error: Some(format!("{}; {}", message, inner_error_message)),
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

    // get validation status for all issuer dids
    let status_infos = self.validate_domains_linked_dids(domain, None).await?;

    Ok(Response::new(ValidateDomainResponse {
      linked_dids: status_infos,
    }))
  }

  #[tracing::instrument(
    name = "domain_linkage_validate_did",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
  )]
  async fn validate_did(&self, req: Request<ValidateDidRequest>) -> Result<Response<ValidateDidResponse>, Status> {
    // fetch DID document for given DID
    let did: IotaDID = IotaDID::parse(&req.into_inner().did).map_err(|e| Status::internal(e.to_string()))?;
    let did_document = self
      .resolver
      .resolve(&did)
      .await
      .map_err(|e| Status::internal(e.to_string()))?;

    let services: Vec<LinkedDomainService> = did_document
      .service()
      .iter()
      .cloned()
      .filter_map(|service| LinkedDomainService::try_from(service).ok())
      .collect();

    // check validation for all services and endpoints in them
    let mut service_futures = FuturesOrdered::new();
    for service in services {
      let service_id = service.id().did().clone();
      let domains: Vec<Url> = service.domains().into();
      service_futures.push_back(async move {
        let mut domain_futures = FuturesOrdered::new();
        for domain in domains {
          domain_futures.push_back(self.validate_domains_linked_dids(domain.clone(), Some(service_id.clone())));
        }
        domain_futures
          .try_collect::<Vec<Vec<LinkedDidValidationStatus>>>()
          .await
          .map(|value| LinkedDidEndpointValidationStatus {
            id: service_id.to_string(),
            service_endpoint: value.into_iter().flatten().collect(),
          })
      });
    }
    let endpoint_validation_status = service_futures
      .try_collect::<Vec<LinkedDidEndpointValidationStatus>>()
      .await?;

    let response = ValidateDidResponse {
      service: endpoint_validation_status,
    };

    Ok(Response::new(response))
  }
}

impl DomainLinkageService {
  async fn validate_domains_linked_dids(
    &self,
    domain: Url,
    did: Option<CoreDID>,
  ) -> Result<Vec<LinkedDidValidationStatus>, DomainLinkageError> {
    // get domain linkage config
    let configuration_resource: DomainLinkageConfiguration =
      match DomainLinkageConfiguration::fetch_configuration(domain.clone()).await {
        Ok(value) => value,
        Err(err) => {
          return Ok(vec![get_validation_failed_status(
            "could not get domain linkage config",
            &err,
          )]);
        }
      };

    // get issuers of `linked_dids` credentials
    let linked_dids: Vec<CoreDID> = if let Some(issuer_did) = did {
      vec![issuer_did]
    } else {
      match configuration_resource.issuers() {
        Ok(value) => value,
        Err(err) => {
          return Ok(vec![get_validation_failed_status(
            "could not get issuers from domain linkage config credential",
            &err,
          )]);
        }
      }
    };

    // resolve all issuers
    let resolved = match self.resolver.resolve_multiple(&linked_dids).await {
      Ok(value) => value,
      Err(err) => {
        return Ok(vec![get_validation_failed_status(
          "could not resolve linked DIDs from domain linkage config",
          &err,
        )]);
      }
    };

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
    let status_infos = configuration_resource
      .linked_dids()
      .iter()
      .zip(errors.iter())
      .map(|(credential, error)| LinkedDidValidationStatus {
        valid: error.is_none(),
        document: Some(credential.as_str().to_string()),
        error: error.clone(),
      })
      .collect();

    Ok(status_infos)
  }
}

pub fn service(client: &Client) -> DomainLinkageServer<DomainLinkageService> {
  DomainLinkageServer::new(DomainLinkageService::new(client))
}
