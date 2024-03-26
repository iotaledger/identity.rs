// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::error::Error;

use domain_linkage::domain_linkage_server::DomainLinkage;
use domain_linkage::domain_linkage_server::DomainLinkageServer;
use domain_linkage::LinkedDidEndpointValidationStatus;
use domain_linkage::LinkedDidValidationStatus;
use domain_linkage::ValidateDidAgainstDidConfigurationsRequest;
use domain_linkage::ValidateDidRequest;
use domain_linkage::ValidateDidResponse;
use domain_linkage::ValidateDomainAgainstDidConfigurationRequest;
use domain_linkage::ValidateDomainRequest;
use domain_linkage::ValidateDomainResponse;
use futures::stream::FuturesOrdered;
use futures::TryStreamExt;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::FromJson;
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
use url::Origin;

#[allow(clippy::module_inception)]
mod domain_linkage {
  tonic::include_proto!("domain_linkage");
}

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "error", content = "reason")]
enum DomainLinkageError {
  #[error("domain argument invalid: {0}")]
  DomainParsing(String),
  #[error("did configuration argument invalid: {0}")]
  DidConfigurationParsing(String),
  #[error("did resolving failed: {0}")]
  DidResolving(String),
}

impl From<DomainLinkageError> for tonic::Status {
  fn from(value: DomainLinkageError) -> Self {
    let code = match &value {
      DomainLinkageError::DomainParsing(_) => tonic::Code::InvalidArgument,
      DomainLinkageError::DidConfigurationParsing(_) => tonic::Code::InvalidArgument,
      DomainLinkageError::DidResolving(_) => tonic::Code::Internal,
    };
    let message = value.to_string();
    let error_json = serde_json::to_vec(&value).expect("plenty of memory!"); // ?

    tonic::Status::with_details(code, message, error_json.into())
  }
}

/// Helper struct that allows to convert `ValidateDomainAgainstDidConfigurationRequest` input struct
/// with `String` config to a struct with `DomainLinkageService` config.
struct DomainValidationConfig {
  domain: Url,
  config: DomainLinkageConfiguration,
}

impl DomainValidationConfig {
  /// Parses did-configuration inputs from:
  ///
  /// - `validate_domain_against_did_configuration`
  /// - `validate_did_against_did_configurations`
  pub fn try_parse(request_config: &ValidateDomainAgainstDidConfigurationRequest) -> Result<Self, DomainLinkageError> {
    Ok(Self {
      domain: Url::parse(&request_config.domain).map_err(|e| DomainLinkageError::DomainParsing(e.to_string()))?,
      config: DomainLinkageConfiguration::from_json(&request_config.did_configuration).map_err(|err| {
        DomainLinkageError::DidConfigurationParsing(format!("could not parse given DID configuration; {}", &err))
      })?,
    })
  }
}

/// Builds a validation status for a failed validation from an `Error`.
fn get_validation_failed_status(message: &str, err: &impl Error) -> LinkedDidValidationStatus {
  LinkedDidValidationStatus {
    valid: false,
    document: None,
    error: Some(format!("{}; {}", message, &err.to_string())),
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

  /// Validates a DID' `LinkedDomains` service endpoints. Pre-fetched did-configurations can be passed to skip fetching
  /// them on server.
  ///
  /// Arguments:
  ///
  /// * `did`: DID to validate
  /// * `did_configurations`: A list of domains and their did-configuration, if omitted config will be fetched
  async fn validate_did_with_optional_configurations(
    &self,
    did: &IotaDID,
    did_configurations: Option<Vec<DomainValidationConfig>>,
  ) -> Result<Vec<LinkedDidEndpointValidationStatus>, DomainLinkageError> {
    // fetch DID document for given DID
    let did_document = self
      .resolver
      .resolve(did)
      .await
      .map_err(|e| DomainLinkageError::DidResolving(e.to_string()))?;

    let services: Vec<LinkedDomainService> = did_document
      .service()
      .iter()
      .cloned()
      .filter_map(|service| LinkedDomainService::try_from(service).ok())
      .collect();

    let config_map: HashMap<Origin, DomainLinkageConfiguration> = match did_configurations {
      Some(configurations) => configurations
        .into_iter()
        .map(|value| (value.domain.origin(), value.config))
        .collect::<HashMap<Origin, DomainLinkageConfiguration>>(),
      None => HashMap::new(),
    };

    // check validation for all services and endpoints in them
    let mut service_futures = FuturesOrdered::new();
    for service in services {
      let service_id: CoreDID = did.clone().into();
      let domains: Vec<Url> = service.domains().into();
      let local_config_map = config_map.clone();
      service_futures.push_back(async move {
        let mut domain_futures = FuturesOrdered::new();
        for domain in domains {
          let config = local_config_map.get(&domain.origin()).map(|value| value.to_owned());
          domain_futures.push_back(self.validate_domains_with_optional_configuration(
            domain.clone(),
            Some(did.clone().into()),
            config,
          ));
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

    Ok(endpoint_validation_status)
  }

  /// Validates domain linkage for given origin.
  ///
  /// Arguments:
  ///
  /// * `domain`: An origin to validate domain linkage for
  /// * `did`: A DID to restrict validation to, if omitted all DIDs from config will be validated
  /// * `config`: A domain linkage configuration can be passed if already loaded, if omitted config will be fetched from
  ///   origin
  async fn validate_domains_with_optional_configuration(
    &self,
    domain: Url,
    did: Option<CoreDID>,
    config: Option<DomainLinkageConfiguration>,
  ) -> Result<Vec<LinkedDidValidationStatus>, DomainLinkageError> {
    // get domain linkage config
    let domain_linkage_configuration: DomainLinkageConfiguration = if let Some(config_value) = config {
      config_value
    } else {
      match DomainLinkageConfiguration::fetch_configuration(domain.clone()).await {
        Ok(value) => value,
        Err(err) => {
          return Ok(vec![get_validation_failed_status(
            "could not get domain linkage config",
            &err,
          )]);
        }
      }
    };

    // get issuers of `linked_dids` credentials
    let linked_dids: Vec<CoreDID> = if let Some(issuer_did) = did {
      vec![issuer_did]
    } else {
      match domain_linkage_configuration.issuers() {
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
            &domain_linkage_configuration,
            &domain.clone(),
            &JwtCredentialValidationOptions::default(),
          )
          .err()
          .map(|err| err.to_string())
      })
      .collect();

    // collect resolved documents and their validation status into array following the order of `linked_dids`
    let status_infos = domain_linkage_configuration
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

#[tonic::async_trait]
impl DomainLinkage for DomainLinkageService {
  #[tracing::instrument(
    name = "validate_domain",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
  )]
  async fn validate_domain(
    &self,
    req: Request<ValidateDomainRequest>,
  ) -> Result<Response<ValidateDomainResponse>, Status> {
    let request_data = &req.into_inner();
    // parse given domain
    let domain: Url =
      Url::parse(&request_data.domain).map_err(|err| DomainLinkageError::DomainParsing(err.to_string()))?;

    // get validation status for all issuer dids
    let status_infos = self
      .validate_domains_with_optional_configuration(domain, None, None)
      .await?;

    Ok(Response::new(ValidateDomainResponse {
      linked_dids: status_infos,
    }))
  }

  #[tracing::instrument(
    name = "validate_domain_against_did_configuration",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
  )]
  async fn validate_domain_against_did_configuration(
    &self,
    req: Request<ValidateDomainAgainstDidConfigurationRequest>,
  ) -> Result<Response<ValidateDomainResponse>, Status> {
    let request_data = &req.into_inner();
    // parse given domain
    let domain: Url =
      Url::parse(&request_data.domain).map_err(|err| DomainLinkageError::DomainParsing(err.to_string()))?;
    // parse config
    let config = DomainLinkageConfiguration::from_json(&request_data.did_configuration.to_string()).map_err(|err| {
      DomainLinkageError::DidConfigurationParsing(format!("could not parse given DID configuration; {}", &err))
    })?;

    // get validation status for all issuer dids
    let status_infos = self
      .validate_domains_with_optional_configuration(domain, None, Some(config))
      .await?;

    Ok(Response::new(ValidateDomainResponse {
      linked_dids: status_infos,
    }))
  }

  #[tracing::instrument(
    name = "validate_did",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
  )]
  async fn validate_did(&self, req: Request<ValidateDidRequest>) -> Result<Response<ValidateDidResponse>, Status> {
    // fetch DID document for given DID
    let did: IotaDID = IotaDID::parse(req.into_inner().did).map_err(|e| Status::internal(e.to_string()))?;

    let endpoint_validation_status = self.validate_did_with_optional_configurations(&did, None).await?;

    let response = ValidateDidResponse {
      service: endpoint_validation_status,
    };

    Ok(Response::new(response))
  }

  #[tracing::instrument(
    name = "validate_did_against_did_configurations",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
  )]
  async fn validate_did_against_did_configurations(
    &self,
    req: Request<ValidateDidAgainstDidConfigurationsRequest>,
  ) -> Result<Response<ValidateDidResponse>, Status> {
    let request_data = &req.into_inner();
    let did: IotaDID = IotaDID::parse(&request_data.did).map_err(|e| Status::internal(e.to_string()))?;
    let did_configurations = request_data
      .did_configurations
      .iter()
      .map(DomainValidationConfig::try_parse)
      .collect::<Result<Vec<DomainValidationConfig>, DomainLinkageError>>()?;

    let endpoint_validation_status = self
      .validate_did_with_optional_configurations(&did, Some(did_configurations))
      .await?;

    let response = ValidateDidResponse {
      service: endpoint_validation_status,
    };

    Ok(Response::new(response))
  }
}

pub fn service(client: &Client) -> DomainLinkageServer<DomainLinkageService> {
  DomainLinkageServer::new(DomainLinkageService::new(client))
}
