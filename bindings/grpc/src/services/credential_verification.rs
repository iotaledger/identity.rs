use credential_verification::{
    credential_verification_server::{CredentialVerificationServer, CredentialVerification},
    CvRequest,
    CvResponse,
};
use tonic::{Request, Response, Status};
use identity_credential::{credential::{Jwt, Credential}, validator::JwtCredentialValidatorUtils};

mod credential_verification {
    tonic::include_proto!("credentials");
}

#[derive(Debug, Default)]
pub struct CredentialVerifier {}

#[tonic::async_trait]
impl CredentialVerification for CredentialVerifier {
    async fn verify_credential(&self, req: Request<CvRequest>) -> Result<Response<CvResponse>, Status> {
        let CvRequest { jwt, verification_options } = req.into_inner();        
        let jwt = Jwt::from(jwt);
        let issuer = JwtCredentialValidatorUtils::extract_issuer_from_jwt(&jwt)
            .map_err(|e| Status::from_error(Box::new(e)))?;


        Err(Status::unknown("ooops"))
    }
}

pub fn service() -> CredentialVerificationServer<CredentialVerifier> {
    CredentialVerificationServer::new(CredentialVerifier::default())
}