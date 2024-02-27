use identity_credential::{
  credential::{ProofT, VerifiableCredentialT},
  revocation::{StatusCredentialT, StatusResolverT, StatusT},
};
use identity_did::DIDUrl;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota_core::{Error as IotaError, IotaDID, IotaIdentityClientExt};
use identity_jose::{
  jwk::Jwk,
  jws::{JwsVerifier, SignatureVerificationError, SignatureVerificationErrorKind, VerificationInput},
};
use identity_verification::MethodData;
use iota_sdk::{client::Client, Url};

pub trait ResolverT<T> {
  type Error;

  async fn fetch(&mut self, url: &Url) -> Result<T, Self::Error>;
}

pub trait VerifierT<K> {
  type Error;

  fn verify<P: ProofT>(&mut self, proof: &P, key: &K) -> Result<(), Self::Error>;
}

pub trait ValidatorT<'c, C> {
  type Error;

  async fn validate(&self, credential: &'c C) -> Result<(), Self::Error>;
}

pub trait ValidatorStatusExt<'c, C: StatusCredentialT>: ValidatorT<'c, C> {
  async fn validate_with_status<S, SR, F>(
    &self,
    credential: &'c C,
    status_resolver: &SR,
    state_predicate: F,
  ) -> Result<bool, Self::Error>
  where
    F: FnOnce(&S::State) -> bool,
    SR: StatusResolverT<S>,
    Self::Error: From<SR::Error>,
    S: StatusT + TryFrom<&'c C::Status>,
  {
    self.validate(credential).await?;
    let Some(status) = credential.status() else {
      return Ok(true);
    };
    let credential_state = status_resolver.state(status).await?;

    Ok(state_predicate(&credential_state))
  }
}

impl<'c, V, C> ValidatorStatusExt<'c, C> for V
where
  V: ValidatorT<'c, C>,
  C: StatusCredentialT,
{
}

#[derive(Debug, thiserror::Error)]
pub enum IotaValidationError {
  #[error("Resolution error")]
  Resolution(()),
  #[error("Signature verification error")]
  Verification(()),
  #[error("Status verification error")]
  Status(()),
}

#[derive(Debug)]
pub struct IotaCredentialValidator<V = EdDSAJwsVerifier> {
  resolver: Client,
  verifier: V,
}

impl<V> IotaCredentialValidator<V> {
  pub fn new(resolver: Client, verifier: V) -> Self {
    Self { resolver, verifier }
  }
}

impl<'c, C> ValidatorT<'c, C> for IotaCredentialValidator
where
  C: VerifiableCredentialT<'c>,
  C::Proof: ProofT,
{
  type Error = IotaValidationError;

  async fn validate(&self, credential: &'c C) -> Result<(), Self::Error> {
    let is_valid_time_wise = credential.check_validity_time_frame();
    let verification_method_url = credential
      .proof()
      .verification_method()
      .ok_or(todo!("missing verification method error"))?;
    let MethodData::PublicKeyJwk(jwk) = self
      .resolver
      .fetch(&verification_method_url)
      .await
      .map_err(|_| IotaValidationError::Resolution(()))?
    else {
      todo!("unsupported key error");
    };
    VerifierT::verify(&mut self.verifier, &credential.proof(), &jwk)
      .map_err(|_| IotaValidationError::Verification(()))?;

    Ok(())
  }
}

impl ResolverT<MethodData> for Client {
  type Error = IotaError;

  async fn fetch(&mut self, url: &Url) -> Result<MethodData, Self::Error> {
    let did_url = DIDUrl::parse(url.as_str()).map_err(IotaError::DIDSyntaxError)?;
    let did = IotaDID::try_from_core(did_url.did().clone()).map_err(IotaError::DIDSyntaxError)?;
    let mut doc = self.resolve_did(&did).await?;

    let key = doc
      .resolve_method(&did_url, None)
      .map(|method| method.data())
      .cloned()
      .ok_or(todo!("an error for verification method not found"))?;

    Ok(key)
  }
}

impl VerifierT<Jwk> for EdDSAJwsVerifier {
  type Error = SignatureVerificationError;
  fn verify<P: ProofT>(&mut self, proof: &P, key: &Jwk) -> Result<(), Self::Error> {
    let input = VerificationInput {
      alg: proof
        .algorithm()
        .parse()
        .map_err(|_| SignatureVerificationError::new(SignatureVerificationErrorKind::UnsupportedAlg))?,
      signing_input: proof.signing_input().into(),
      decoded_signature: proof.signature().into(),
    };

    JwsVerifier::verify(self, input, key)
  }
}
