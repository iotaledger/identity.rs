use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Object;
use identity_iota::core::ToJson;
use identity_iota::credential::FailFast;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtCredentialValidator;
use identity_iota::credential::JwtCredentialValidatorUtils;
use identity_iota::credential::JwtValidationError;
use identity_iota::iota::IotaDID;
use identity_iota::resolver;
use identity_iota::resolver::Resolver;
use iota_sdk::client::Client;

use _credentials::vc_validation_server::VcValidation;
use _credentials::vc_validation_server::VcValidationServer;
use _credentials::VcValidationRequest;
use _credentials::VcValidationResponse;
use tonic::Request;
use tonic::Response;
use tonic::Status;

mod _credentials {
  tonic::include_proto!("credentials");
}

#[derive(Debug, thiserror::Error)]
pub enum VcValidationError {
  #[error(transparent)]
  JwtValidationError(#[from] JwtValidationError),
  #[error("DID resolution error")]
  DidResolutionError(#[source] resolver::Error),
  #[error("The provided credential has been revoked")]
  RevokedCredential,
  #[error("The provided credential has expired")]
  ExpiredCredential,
}

impl From<VcValidationError> for Status {
  fn from(error: VcValidationError) -> Self {
    Status::unknown(error.to_string())
  }
}

pub struct VcValidator {
  resolver: Resolver,
}

impl VcValidator {
  pub fn new(client: &Client) -> Self {
    let mut resolver = Resolver::new();
    resolver.attach_iota_handler(client.clone());
    Self { resolver }
  }
}

#[tonic::async_trait]
impl VcValidation for VcValidator {
  #[tracing::instrument(
    name = "validate_jwt_credential",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
)]
  async fn validate(&self, req: Request<VcValidationRequest>) -> Result<Response<VcValidationResponse>, Status> {
    let VcValidationRequest { credential_jwt } = req.into_inner();
    let jwt = Jwt::new(credential_jwt);
    let issuer_did = JwtCredentialValidatorUtils::extract_issuer_from_jwt::<IotaDID>(&jwt)
      .map_err(VcValidationError::JwtValidationError)?;
    let issuer_doc = self
      .resolver
      .resolve(&issuer_did)
      .await
      .map_err(VcValidationError::DidResolutionError)?;

    let validator = JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default());
    let decoded_credential = validator
      .validate::<_, Object>(
        &jwt,
        &issuer_doc,
        &JwtCredentialValidationOptions::default(),
        FailFast::FirstError,
      )
      .map_err(|mut e| match e.validation_errors.swap_remove(0) {
        JwtValidationError::Revoked => VcValidationError::RevokedCredential,
        JwtValidationError::ExpirationDate | JwtValidationError::IssuanceDate => VcValidationError::ExpiredCredential,
        e => VcValidationError::JwtValidationError(e),
      })?;

    let response = Response::new(VcValidationResponse {
      credential_json: decoded_credential.credential.to_json().unwrap(),
    });

    Ok(response)
  }
}

pub fn service(client: &Client) -> VcValidationServer<VcValidator> {
  VcValidationServer::new(VcValidator::new(client))
}
