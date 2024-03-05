use std::fmt::Debug;

use identity_core::common::Timestamp;
use identity_core::common::Url;
use itertools::Itertools;
use sd_jwt_payload::SdJwt;
use sd_jwt_payload::SdObjectDecoder;

use crate::credential::CredentialT;
use crate::credential::Issuer;
use crate::credential::Jwt;
use crate::credential::JwtCredential;
use crate::credential::JwtCredentialClaims;
use identity_core::ResolverT;
use crate::credential::ValidableCredential;
use crate::revocation::StatusCredentialT;
use identity_verification::VerifierT;

pub struct SdJwtCredential<C> {
  jwt_credential: JwtCredential<C>,
  disclosures: Vec<String>,
  key_binding_jwt: Option<String>,
}

impl<C: Debug> Debug for SdJwtCredential<C> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdJwtCredential")
      .field("jwt_credential", &self.jwt_credential)
      .field("disclosures", &self.disclosures)
      .field("key_binding_jwt", &self.key_binding_jwt)
      .finish()
  }
}

impl<C: Clone> Clone for SdJwtCredential<C> {
  fn clone(&self) -> Self {
    Self {
      jwt_credential: self.jwt_credential.clone(),
      disclosures: self.disclosures.clone(),
      key_binding_jwt: self.key_binding_jwt.clone(),
    }
  }
}

impl<C> serde::Serialize for SdJwtCredential<C> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let disclosures = self.disclosures.iter().join("~");
    let key_bindings = self.key_binding_jwt.as_deref().unwrap_or("");
    format!("{}~{}~{}", &self.jwt_credential.inner, disclosures, key_bindings).serialize(serializer)
  }
}

impl<'de, C> serde::Deserialize<'de> for SdJwtCredential<C>
where
  C: for<'a> TryFrom<&'a JwtCredentialClaims>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    todo!()
  }
}

impl<C> TryFrom<SdJwt> for SdJwtCredential<C>
where
  C: for<'a> TryFrom<&'a JwtCredentialClaims>,
{
  type Error = ();
  fn try_from(sd_jwt: SdJwt) -> Result<Self, Self::Error> {
    Self::parse(sd_jwt)
  }
}

impl<C> SdJwtCredential<C>
where
  C: for<'a> TryFrom<&'a JwtCredentialClaims>,
{
  pub fn parse(sd_jwt: SdJwt) -> Result<Self, ()> {
    Self::parse_with_decoder(sd_jwt, SdObjectDecoder::default())
  }
  pub fn parse_with_decoder(sd_jwt: SdJwt, decoder: SdObjectDecoder) -> Result<Self, ()> {
    let SdJwt {
      jwt,
      disclosures,
      key_binding_jwt,
    } = sd_jwt;
    let jwt = Jwt::parse(jwt).map_err(|_| ())?;
    let serde_json::Value::Object(raw_claims) =
      serde_json::from_slice::<serde_json::Value>(jwt.decoded_jws.claims()).map_err(|_| ())?
    else {
      todo!("invalid claims")
    };
    let parsed_claims = decoder
      .decode(&raw_claims, &disclosures)
      .map_err(|_| ())
      .map(serde_json::Value::Object)
      .and_then(|claims| serde_json::from_value::<JwtCredentialClaims>(claims).map_err(|_| ()))?;
    let credential = C::try_from(&parsed_claims).map_err(|_| ())?;
    let jwt_credential = JwtCredential {
      decoded_jws: jwt.decoded_jws,
      inner: jwt.inner,
      credential,
      parsed_claims,
    };

    Ok(Self {
      jwt_credential,
      disclosures,
      key_binding_jwt,
    })
  }
}

impl<C> CredentialT for SdJwtCredential<C> {
  type Claim = JwtCredentialClaims;
  type Issuer = Issuer;

  fn id(&self) -> &Url {
    self.jwt_credential.id()
  }
  fn issuer(&self) -> &Self::Issuer {
    self.jwt_credential.issuer()
  }
  fn claim(&self) -> &Self::Claim {
    self.jwt_credential.claim()
  }
  fn valid_from(&self) -> Timestamp {
    self.jwt_credential.valid_from()
  }
  fn valid_until(&self) -> Option<Timestamp> {
    self.jwt_credential.valid_until()
  }
}

impl<C: StatusCredentialT> StatusCredentialT for SdJwtCredential<C> {
  type Status = C::Status;
  fn status(&self) -> Option<&Self::Status> {
    self.jwt_credential.status()
  }
}

impl<C> Into<SdJwt> for SdJwtCredential<C> {
  fn into(self) -> SdJwt {
    let Self {
      jwt_credential,
      disclosures,
      key_binding_jwt,
    } = self;
    SdJwt {
      jwt: jwt_credential.inner,
      disclosures,
      key_binding_jwt,
    }
  }
}

impl<C> SdJwtCredential<C> {
  pub fn credential(&self) -> &C {
    &self.jwt_credential.credential
  }
}

impl<R, V, C, K> ValidableCredential<R, V, K> for SdJwtCredential<C>
where
  R: ResolverT<K>,
  R::Input: TryFrom<Url>,
  V: VerifierT<K>,
{
  async fn validate(&self, resolver: &R, verifier: &V) -> Result<(), ()> {
    self.jwt_credential.validate(resolver, verifier).await
  }
}
