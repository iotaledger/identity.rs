use std::cell::LazyCell;

use identity_core::common::{StringOrUrl, Timestamp, Url};
use sd_jwt_payload_rework::{Hasher, JsonObject, JwsSigner, RequiredKeyBinding};
use sd_jwt_payload_rework::{SdJwtBuilder, Sha256Hasher};
use serde::Serialize;
use serde_json::json;

use super::SdJwtVc;
use super::{Error, Status};
use super::{Result, SD_JWT_VC_TYP};

const DEFAULT_HEADER: LazyCell<JsonObject> = LazyCell::new(|| {
  let mut object = JsonObject::default();
  object.insert("typ".to_string(), SD_JWT_VC_TYP.into());
  object
});

macro_rules! claim_to_key_value_pair {
  ( $( $claim:ident ),+ ) => {
    {
      let mut claim_list = Vec::<(&'static str, serde_json::Value)>::new();
      $(
        claim_list.push((stringify!($claim), serde_json::to_value($claim).unwrap()));
      )*
      claim_list
    }
  };
}

/// A structure to ease the creation of an [`SdJwtVc`].
#[derive(Debug)]
pub struct SdJwtVcBuilder<H = Sha256Hasher> {
  inner_builder: SdJwtBuilder<H>,
  header: JsonObject,
  iss: Option<Url>,
  nbf: Option<i64>,
  exp: Option<i64>,
  iat: Option<i64>,
  vct: Option<StringOrUrl>,
  sub: Option<StringOrUrl>,
  status: Option<Status>,
}

impl Default for SdJwtVcBuilder {
  fn default() -> Self {
    Self {
      inner_builder: SdJwtBuilder::<Sha256Hasher>::new(json!({})).unwrap(),
      header: DEFAULT_HEADER.clone(),
      iss: None,
      nbf: None,
      exp: None,
      iat: None,
      vct: None,
      sub: None,
      status: None,
    }
  }
}

impl SdJwtVcBuilder {
  /// Creates a new [`SdJwtVcBuilder`] using `object` JSON representation and default
  /// `sha-256` hasher.
  pub fn new<T: Serialize>(object: T) -> Result<Self> {
    let inner_builder = SdJwtBuilder::<Sha256Hasher>::new(object)?;
    Ok(Self {
      header: DEFAULT_HEADER.clone(),
      inner_builder,
      ..Default::default()
    })
  }
}

impl<H: Hasher> SdJwtVcBuilder<H> {
  /// Creates a new [`SdJwtVcBuilder`] using `object` JSON representation and a given
  /// hasher `hasher`.
  pub fn new_with_hasher<T: Serialize>(object: T, hasher: H) -> Result<Self> {
    let inner_builder = SdJwtBuilder::new_with_hasher(object, hasher)?;
    Ok(Self {
      inner_builder,
      header: DEFAULT_HEADER.clone(),
      iss: None,
      nbf: None,
      exp: None,
      iat: None,
      vct: None,
      sub: None,
      status: None,
    })
  }

  /// Substitutes a value with the digest of its disclosure.
  ///
  /// ## Notes
  /// - `path` indicates the pointer to the value that will be concealed using the syntax of [JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901).
  ///
  /// ## Example
  /// ```rust
  /// use serde_json::json;  
  /// use identity_credential::sd_jwt_vc::SdJwtVcBuilder;
  ///
  /// let obj = json!({
  ///   "id": "did:value",
  ///   "claim1": {
  ///      "abc": true
  ///   },
  ///   "claim2": ["val_1", "val_2"]
  /// });
  /// let builder = SdJwtVcBuilder::new(obj)
  ///   .unwrap()
  ///   .make_concealable("/id").unwrap() //conceals "id": "did:value"
  ///   .make_concealable("/claim1/abc").unwrap() //"abc": true
  ///   .make_concealable("/claim2/0").unwrap(); //conceals "val_1"
  /// ```
  pub fn make_concealable(mut self, path: &str) -> Result<Self> {
    self.inner_builder = self.inner_builder.make_concealable(path)?;
    Ok(self)
  }

  /// Sets the JWT header.
  /// ## Notes
  /// - if [`SdJwtVcBuilder::header`] is not called, the default header is used:
  ///   ```json
  ///   {
  ///     "typ": "sd-jwt",
  ///     "alg": "<algorithm used in SdJwtBulider::finish>"
  ///   }
  ///   ```
  /// - `alg` is always replaced with the value passed to [`SdJwtVcBuilder::finish`].
  pub fn header(mut self, header: JsonObject) -> Self {
    self.header = header;
    self
  }

  /// Adds a decoy digest to the specified path.
  ///
  /// `path` indicates the pointer to the value that will be concealed using the syntax of
  /// [JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901).
  ///
  /// Use `path` = "" to add decoys to the top level.
  pub fn add_decoys(mut self, path: &str, number_of_decoys: usize) -> Result<Self> {
    self.inner_builder = self.inner_builder.add_decoys(path, number_of_decoys)?;

    Ok(self)
  }

  /// Require a proof of possession of a given key from the holder.
  ///
  /// This operation adds a JWT confirmation (`cnf`) claim as specified in
  /// [RFC8300](https://www.rfc-editor.org/rfc/rfc7800.html#section-3).
  pub fn require_key_binding(mut self, key_bind: RequiredKeyBinding) -> Self {
    self.inner_builder = self.inner_builder.require_key_binding(key_bind);
    self
  }

  /// Inserts an `iss` claim. See [`super::SdJwtVcClaims::iss`].
  pub fn iss(mut self, issuer: Url) -> Self {
    self.iss = Some(issuer);
    self
  }

  /// Inserts a `nbf` claim. See [`super::SdJwtVcClaims::nbf`].
  pub fn nbf(mut self, nbf: Timestamp) -> Self {
    self.nbf = Some(nbf.to_unix());
    self
  }

  /// Inserts a `exp` claim. See [`super::SdJwtVcClaims::exp`].
  pub fn exp(mut self, exp: Timestamp) -> Self {
    self.exp = Some(exp.to_unix());
    self
  }

  /// Inserts a `iat` claim. See [`super::SdJwtVcClaims::iat`].
  pub fn iat(mut self, iat: Timestamp) -> Self {
    self.iat = Some(iat.to_unix());
    self
  }

  /// Inserts a `vct` claim. See [`super::SdJwtVcClaims::vct`].
  pub fn vct(mut self, vct: impl Into<StringOrUrl>) -> Self {
    self.vct = Some(vct.into());
    self
  }

  /// Inserts a `sub` claim. See [`super::SdJwtVcClaims::sub`].
  pub fn sub(mut self, sub: impl Into<StringOrUrl>) -> Self {
    self.sub = Some(sub.into());
    self
  }

  /// Inserts a `status` claim. See [`super::SdJwtVcClaims::status`].
  pub fn status(mut self, status: Status) -> Self {
    self.status = Some(status);
    self
  }

  /// Creates an [`SdJwtVc`] with the provided data.
  pub async fn finish<S>(self, signer: &S, alg: &str) -> Result<SdJwtVc>
  where
    S: JwsSigner,
  {
    let Self {
      inner_builder,
      mut header,
      iss,
      nbf,
      exp,
      iat,
      vct,
      sub,
      status,
    } = self;
    // Check header.
    header
      .entry("typ")
      .or_insert_with(|| SD_JWT_VC_TYP.to_owned().into())
      .as_str()
      .filter(|typ| typ.contains(SD_JWT_VC_TYP))
      .ok_or_else(|| Error::InvalidJoseType(String::default()))?;

    let builder = inner_builder.header(header);

    // Insert SD-JWT VC claims into object.
    let builder = claim_to_key_value_pair![iss, nbf, exp, iat, vct, sub, status]
      .into_iter()
      .filter(|(_, value)| !value.is_null())
      .fold(builder, |builder, (key, value)| builder.insert_claim(key, value));

    let sd_jwt = builder.finish(signer, alg).await?;
    SdJwtVc::try_from(sd_jwt)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use async_trait::async_trait;
  use josekit::{
    jwt::{self, JwtPayload},
    {jws::JwsHeader, jws::HS256},
  };
  use serde_json::json;

  struct TestSigner;

  #[async_trait]
  impl JwsSigner for TestSigner {
    type Error = josekit::JoseError;
    async fn sign(&self, header: &JsonObject, payload: &JsonObject) -> std::result::Result<Vec<u8>, Self::Error> {
      let signer = HS256.signer_from_bytes(b"0123456789ABCDEF0123456789ABCDEF")?;
      let header = JwsHeader::from_map(header.clone())?;
      let payload = JwtPayload::from_map(payload.clone())?;
      let jws = jwt::encode_with_signer(&payload, &header, &signer)?;

      Ok(jws.into_bytes())
    }
  }

  #[tokio::test]
  async fn building_valid_vc_works() -> anyhow::Result<()> {
    let credential = json!({
      "name": "John Doe",
      "birthdate": "1970-01-01"
    });

    SdJwtVcBuilder::new(credential)?
      .vct("https://bmi.bund.example/credential/pid/1.0".parse::<Url>()?)
      .iat(Timestamp::now_utc())
      .iss("https://example.com/".parse()?)
      .make_concealable("/birthdate")?
      .finish(&TestSigner, "HS256")
      .await?;

    Ok(())
  }

  #[tokio::test]
  async fn building_vc_with_missing_mandatory_claims_fails() -> anyhow::Result<()> {
    let credential = json!({
      "name": "John Doe",
      "birthdate": "1970-01-01"
    });

    let err = SdJwtVcBuilder::new(credential)?
      .vct("https://bmi.bund.example/credential/pid/1.0".parse::<Url>()?)
      .iat(Timestamp::now_utc())
      // issuer is missing.
      .make_concealable("/birthdate")?
      .finish(&TestSigner, "HS256")
      .await
      .unwrap_err();
    assert!(matches!(err, Error::MissingClaim("iss")));

    Ok(())
  }

  #[tokio::test]
  async fn building_vc_with_invalid_mandatory_claims_fails() -> anyhow::Result<()> {
    let credential = json!({
      "name": "John Doe",
      "birthdate": "1970-01-01",
      "vct": { "id": 1234567890 }
    });

    let err = SdJwtVcBuilder::new(credential)?
      .iat(Timestamp::now_utc())
      .iss("https://example.com".parse()?)
      .make_concealable("/birthdate")?
      .finish(&TestSigner, "HS256")
      .await
      .unwrap_err();
    
    assert!(matches!(
      err,
      Error::InvalidClaimValue {
        name: "vct",
        ..
      }
    ));

    Ok(())
  }
}
