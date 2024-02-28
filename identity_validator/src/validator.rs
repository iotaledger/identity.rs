use identity_credential::credential::ProofT;
use identity_credential::credential::VerifiableCredentialT;
use identity_credential::revocation::StatusCredentialT;
use identity_credential::revocation::StatusResolverT;
use identity_credential::revocation::StatusT;
use identity_did::DIDUrl;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota_core::Error as IotaError;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaIdentityClientExt;
use identity_jose::jws::JwsVerifier;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;
use identity_verification::MethodData;
use iota_sdk::client::Client;
use std::marker::PhantomData;

pub trait ResolverT<T> {
  type Error;
  type Input;

  async fn fetch(&self, input: &Self::Input) -> Result<T, Self::Error>;
}

pub trait VerifierT<K> {
  type Error;

  fn verify<P: ProofT>(&self, proof: &P, key: &K) -> Result<(), Self::Error>;
}

impl ResolverT<MethodData> for Client {
  type Input = DIDUrl;
  type Error = IotaError;

  async fn fetch(&self, input: &Self::Input) -> Result<MethodData, Self::Error> {
    let did = IotaDID::try_from_core(input.did().clone()).map_err(IotaError::DIDSyntaxError)?;
    let doc = self.resolve_did(&did).await?;

    let key = doc
      .resolve_method(input, None)
      .map(|method| method.data())
      .cloned()
      .ok_or(todo!("an error for verification method not found"))?;

    Ok(key)
  }
}

impl VerifierT<MethodData> for EdDSAJwsVerifier {
  type Error = SignatureVerificationError;
  fn verify<P: ProofT>(&self, proof: &P, key: &MethodData) -> Result<(), Self::Error> {
    let MethodData::PublicKeyJwk(jwk) = key else {
      todo!("Unsupported key")
    };
    let input = VerificationInput {
      alg: proof
        .algorithm()
        .parse()
        .map_err(|_| SignatureVerificationError::new(SignatureVerificationErrorKind::UnsupportedAlg))?,
      signing_input: proof.signing_input().into(),
      decoded_signature: proof.signature().into(),
    };

    JwsVerifier::verify(self, input, jwk)
  }
}

pub struct CredentialValidator<R, V, K = ()> {
  resolver: R,
  verifier: V,
  _key: PhantomData<K>,
}

pub type IotaCredentialValidator = CredentialValidator<Client, EdDSAJwsVerifier, MethodData>;

impl<R, V, K> CredentialValidator<R, V, K> {
  pub fn new(resolver: R, verifier: V) -> Self {
    Self {
      resolver,
      verifier,
      _key: PhantomData,
    }
  }
}

impl<R, V, K> CredentialValidator<R, V, K>
where
  R: ResolverT<K>,
  V: VerifierT<K>,
{
  pub async fn validate<'c, C>(&self, credential: &'c C) -> Result<(), ()>
  where
    C: VerifiableCredentialT<'c>,
    C::Proof: ProofT,
    <C::Proof as ProofT>::VerificationMethod: TryInto<R::Input>,
  {
    let proof = credential.proof();
    let Ok(verification_method) = proof.verification_method().try_into() else {
      todo!("Failed to convert to valid verification method type")
    };
    let key = self.resolver.fetch(&verification_method).await.map_err(|_| ())?;
    self.verifier.verify(&proof, &key).map_err(|_| ())?;

    Ok(())
  }

  async fn validate_with_status<'c, C, S, SR, F>(
    &self,
    credential: &'c C,
    status_resolver: &SR,
    state_predicate: F,
  ) -> Result<(), ()>
  where
    C: VerifiableCredentialT<'c> + StatusCredentialT,
    C::Proof: ProofT,
    <C::Proof as ProofT>::VerificationMethod: TryInto<R::Input>,
    SR: StatusResolverT<S>,
    S: StatusT + TryFrom<&'c C::Status>,
    F: FnOnce(&S::State) -> bool,
  {
    self.validate(credential).await?;
    let Some(status) = credential.status() else {
      return Ok(());
    };
    let credential_state = status_resolver.state(status).await.map_err(|_| ())?;

    if !state_predicate(&credential_state) {
      todo!("Non-valid state!")
    } else {
      Ok(())
    }
  }
}
