// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use _presentation::credential_presentation_server::CredentialPresentation as PresentationService;
use _presentation::credential_presentation_server::CredentialPresentationServer;
use _presentation::credential_validation_result::Result as ValidationResult;
use _presentation::CredentialValidationResult;
use _presentation::JwtPresentationRequest;
use _presentation::JwtPresentationResponse;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Object;
use identity_iota::core::ToJson;
use identity_iota::credential::CompoundJwtPresentationValidationError;
use identity_iota::credential::FailFast;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::JwtCredentialValidator;
use identity_iota::credential::JwtCredentialValidatorUtils;
use identity_iota::credential::JwtPresentationValidationOptions;
use identity_iota::credential::JwtPresentationValidator;
use identity_iota::credential::JwtPresentationValidatorUtils;
use identity_iota::credential::JwtValidationError;
use identity_iota::did::CoreDID;
use identity_iota::iota::IotaDocument;
use identity_iota::resolver::Error as ResolverError;
use identity_iota::resolver::Resolver;
use iota_sdk::client::Client;
use tonic::async_trait;
use tonic::Code;
use tonic::Request;
use tonic::Response;
use tonic::Status;

mod _presentation {
  tonic::include_proto!("presentation");
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Invalid JWT presentation: {0}")]
  InvalidJwtPresentation(#[source] JwtValidationError),
  #[error("Resolution error: {0}")]
  ResolutionError(#[source] ResolverError),
  #[error("Presentation validation error: {0}")]
  PresentationValidationError(#[source] CompoundJwtPresentationValidationError),
  #[error("Failed to validate jwt credential: {0}")]
  CredentialValidationError(#[source] anyhow::Error),
}

impl From<Error> for Status {
  fn from(value: Error) -> Self {
    let code = match &value {
      Error::InvalidJwtPresentation(_) => Code::InvalidArgument,
      Error::ResolutionError(_) | Error::PresentationValidationError(_) | Error::CredentialValidationError(_) => {
        Code::Internal
      }
    };

    Status::new(code, value.to_string())
  }
}

pub struct PresentationSvc {
  resolver: Resolver<IotaDocument>,
}

impl PresentationSvc {
  pub fn new(client: Client) -> Self {
    let mut resolver = Resolver::<IotaDocument>::new_with_did_key_handler();
    resolver.attach_iota_handler(client);

    Self { resolver }
  }
}

#[async_trait]
impl PresentationService for PresentationSvc {
  async fn validate(&self, req: Request<JwtPresentationRequest>) -> Result<Response<JwtPresentationResponse>, Status> {
    let jwt_presentation = {
      let JwtPresentationRequest { jwt } = req.into_inner();
      Jwt::new(jwt)
    };

    let holder_did = JwtPresentationValidatorUtils::extract_holder::<CoreDID>(&jwt_presentation)
      .map_err(Error::InvalidJwtPresentation)?;
    let holder_doc = self
      .resolver
      .resolve(&holder_did)
      .await
      .map_err(Error::ResolutionError)?;

    let presentation_validator = JwtPresentationValidator::with_signature_verifier(EdDSAJwsVerifier::default());
    let mut decoded_presentation = presentation_validator
      .validate::<IotaDocument, Jwt, Object>(
        &jwt_presentation,
        &holder_doc,
        &JwtPresentationValidationOptions::default(),
      )
      .map_err(Error::PresentationValidationError)?;

    let credentials = std::mem::take(&mut decoded_presentation.presentation.verifiable_credential);
    let mut decoded_credentials = Vec::with_capacity(credentials.len());
    let credential_validator = JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default());
    for credential_jwt in credentials {
      let issuer_did = JwtCredentialValidatorUtils::extract_issuer_from_jwt::<CoreDID>(&credential_jwt)
        .map_err(|e| Error::CredentialValidationError(e.into()));

      if let Err(e) = issuer_did {
        let validation_result = CredentialValidationResult {
          result: Some(ValidationResult::Error(e.to_string())),
        };
        decoded_credentials.push(validation_result);
        continue;
      }
      let issuer_did = issuer_did.unwrap();

      let issuer_doc = self
        .resolver
        .resolve(&issuer_did)
        .await
        .map_err(|e| Error::CredentialValidationError(e.into()));

      if let Err(e) = issuer_doc {
        let validation_result = CredentialValidationResult {
          result: Some(ValidationResult::Error(e.to_string())),
        };
        decoded_credentials.push(validation_result);
        continue;
      }
      let issuer_doc = issuer_doc.unwrap();

      let validation_result = match credential_validator
        .validate::<IotaDocument, Object>(
          &credential_jwt,
          &issuer_doc,
          &JwtCredentialValidationOptions::default(),
          FailFast::FirstError,
        )
        .map_err(|e| Error::CredentialValidationError(e.into()))
      {
        Ok(decoded_credential) => ValidationResult::Credential(
          decoded_credential
            .credential
            .to_json()
            .map_err(|e| Status::internal(e.to_string()))?,
        ),
        Err(e) => ValidationResult::Error(e.to_string()),
      };

      decoded_credentials.push(CredentialValidationResult {
        result: Some(validation_result),
      })
    }

    Ok(Response::new(JwtPresentationResponse {
      credentials: decoded_credentials,
    }))
  }
}

pub fn service(client: &Client) -> CredentialPresentationServer<PresentationSvc> {
  CredentialPresentationServer::new(PresentationSvc::new(client.clone()))
}
