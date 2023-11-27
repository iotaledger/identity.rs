use credential_verification::{
  credential_revocation_server::{CredentialRevocation, CredentialRevocationServer},
  RevocationCheckRequest, RevocationCheckResponse, RevocationStatus,
};
use identity_iota::{
  credential::{self, JwtCredentialValidatorUtils, JwtValidationError, RevocationBitmapStatus},
  prelude::{IotaDocument, Resolver},
};
use iota_sdk::client::Client;

use thiserror::Error;
use tonic::{self, Request, Response};

mod credential_verification {
  use super::RevocationCheckError;
  use identity_iota::credential::{RevocationBitmapStatus, Status};

  tonic::include_proto!("credentials");

  impl TryFrom<RevocationCheckRequest> for Status {
    type Error = RevocationCheckError;
    fn try_from(req: RevocationCheckRequest) -> Result<Self, Self::Error> {
      use identity_iota::core::{Object, Url};

      if req.r#type.as_str() != RevocationBitmapStatus::TYPE {
        Err(Self::Error::UnknownRevocationType(req.r#type))
      } else {
        let parsed_url = req
          .url
          .parse::<Url>()
          .map_err(|_| Self::Error::InvalidRevocationUrl(req.url))?;
        let properties = req
          .properties
          .into_iter()
          .map(|(k, v)| serde_json::to_value(v).map(|v| (k, v)))
          .collect::<Result<Object, _>>()
          .map_err(|_| Self::Error::MalformedPropertiesObject)?;

        Ok(Status {
          id: parsed_url,
          type_: req.r#type,
          properties,
        })
      }
    }
  }
}

#[derive(Debug, Error)]
pub enum RevocationCheckError {
  #[error("Unknown revocation type {0}")]
  UnknownRevocationType(String),
  #[error("Could not parse {0} into a valid URL")]
  InvalidRevocationUrl(String),
  #[error("Properties isn't a valid JSON object")]
  MalformedPropertiesObject,
  #[error("Invalid credential status")]
  InvalidCredentialStatus(#[source] credential::Error),
  #[error("Issuer's DID resolution error")]
  ResolutionError(#[source] identity_iota::resolver::Error),
  #[error("Revocation map not found")]
  RevocationMapNotFound(#[source] credential::JwtValidationError),
}

impl From<RevocationCheckError> for tonic::Status {
  fn from(e: RevocationCheckError) -> Self {
    let message = e.to_string();
    let code = match &e {
      RevocationCheckError::InvalidCredentialStatus(_)
        | RevocationCheckError::MalformedPropertiesObject
        | RevocationCheckError::UnknownRevocationType(_)
        | RevocationCheckError::InvalidRevocationUrl(_) => tonic::Code::InvalidArgument,
      RevocationCheckError::ResolutionError(_) => tonic::Code::Internal,
      RevocationCheckError::RevocationMapNotFound(_) => tonic::Code::NotFound,
    };

    tonic::Status::new(code, message)
  }
}

#[derive(Debug)]
pub struct CredentialVerifier {
  resolver: Resolver<IotaDocument>,
}

impl CredentialVerifier {
  pub fn new(client: &Client) -> Self {
    let mut resolver = Resolver::new();
    resolver.attach_iota_handler(client.clone());
    Self { resolver }
  }
}

#[tonic::async_trait]
impl CredentialRevocation for CredentialVerifier {
  async fn check(
    &self,
    req: Request<RevocationCheckRequest>,
  ) -> Result<Response<RevocationCheckResponse>, tonic::Status> {
    let credential_revocation_status = {
      let revocation_status = credential::Status::try_from(req.into_inner())?;
      RevocationBitmapStatus::try_from(revocation_status).map_err(RevocationCheckError::InvalidCredentialStatus)?
    };
    let issuer_did = credential_revocation_status.id().unwrap(); // Safety: already parsed as a valid URL
    let issuer_doc = self
      .resolver
      .resolve(issuer_did.did())
      .await
      .map_err(RevocationCheckError::ResolutionError)?;

    if let Err(e) =
      JwtCredentialValidatorUtils::check_revocation_bitmap_status(&issuer_doc, credential_revocation_status)
    {
      match &e {
        JwtValidationError::Revoked => Ok(Response::new(RevocationCheckResponse {
          status: RevocationStatus::Revoked.into(),
        })),
        _ => Err(RevocationCheckError::RevocationMapNotFound(e).into()),
      }
    } else {
      Ok(Response::new(RevocationCheckResponse {
        status: RevocationStatus::Valid.into(),
      }))
    }
  }
}

pub fn service(client: &Client) -> CredentialRevocationServer<CredentialVerifier> {
  CredentialRevocationServer::new(CredentialVerifier::new(client))
}
