use _sd_jwt::{
  verification_server::{Verification, VerificationServer},
  VerificationRequest, VerificationResponse,
};
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::{
  core::{Object, ToJson},
  credential::{FailFast, Jwt, JwtCredentialValidationOptions, JwtCredentialValidatorUtils, SdJwtCredentialValidator},
  iota::{IotaDID, IotaDocument},
  resolver::Resolver,
  sd_jwt_payload::{Error as SdJwtError, SdJwt, SdObjectDecoder},
};
use iota_sdk::client::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod _sd_jwt {
  tonic::include_proto!("sd_jwt");
}

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "error", content = "reason")]
enum SdJwtVerificationError {
  #[error("Failed to parse SD-JWT: {0}")]
  DeserializationError(String),
  #[error("Failed to parse JWT: {0}")]
  JwtError(String),
  #[error("Credential verification failed: {0}")]
  VerificationError(String),
  #[error("Failed to resolve DID Document: {0}")]
  DidResolutionError(String),
}

impl From<SdJwtError> for SdJwtVerificationError {
  fn from(value: SdJwtError) -> Self {
    match value {
      SdJwtError::DeserializationError(e) => Self::DeserializationError(e),
      _ => unreachable!(),
    }
  }
}

impl From<SdJwtVerificationError> for tonic::Status {
  fn from(value: SdJwtVerificationError) -> Self {
    let code = match &value {
      SdJwtVerificationError::DeserializationError(_) => tonic::Code::InvalidArgument,
      SdJwtVerificationError::JwtError(_) => tonic::Code::InvalidArgument,
      SdJwtVerificationError::VerificationError(_) => tonic::Code::InvalidArgument,
      SdJwtVerificationError::DidResolutionError(_) => tonic::Code::NotFound,
    };
    let message = value.to_string();
    let error_json = serde_json::to_vec(&value).expect("plenty of memory!");

    tonic::Status::with_details(code, message, error_json.into())
  }
}

#[derive(Debug)]
pub struct SdJwtService {
  resolver: Resolver<IotaDocument>,
}

impl SdJwtService {
  pub fn new(client: &Client) -> Self {
    let mut resolver = Resolver::new();
    resolver.attach_iota_handler(client.clone());
    Self { resolver }
  }
}

#[tonic::async_trait]
impl Verification for SdJwtService {
  async fn verify(
    &self,
    request: tonic::Request<VerificationRequest>,
  ) -> Result<tonic::Response<VerificationResponse>, tonic::Status> {
    let VerificationRequest { jwt } = request.into_inner();
    let mut sd_jwt = SdJwt::parse(&jwt).map_err(SdJwtVerificationError::from)?;
    let jwt = Jwt::new(sd_jwt.jwt);

    let issuer_did = JwtCredentialValidatorUtils::extract_issuer_from_jwt::<IotaDID>(&jwt)
      .map_err(|e| SdJwtVerificationError::VerificationError(e.to_string()))?;
    let issuer_document = self
      .resolver
      .resolve(&issuer_did)
      .await
      .map_err(|e| SdJwtVerificationError::DidResolutionError(e.to_string()))?;
    sd_jwt.jwt = jwt.into();

    let decoder = SdObjectDecoder::new_with_sha256();
    let validator = SdJwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default(), decoder);
    let credential = validator
      .validate_credential::<_, Object>(
        &sd_jwt,
        &issuer_document,
        &JwtCredentialValidationOptions::default(),
        FailFast::FirstError,
      )
      .map_err(|e| SdJwtVerificationError::VerificationError(e.to_string()))?;

    if let Some(kb_jwt) = sd_jwt.key_binding_jwt {
      todo!();
    }

    Ok(tonic::Response::new(VerificationResponse {
      credential: credential.credential.to_json().unwrap(),
    }))
  }
}

pub fn service(client: &Client) -> VerificationServer<SdJwtService> {
  VerificationServer::new(SdJwtService::new(client))
}
