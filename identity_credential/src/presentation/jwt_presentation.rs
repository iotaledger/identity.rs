// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use identity_core::convert::ToJson;
use serde::Serialize;

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_verification::MethodUriType;
use identity_verification::TryMethod;

use crate::credential::Credential;
use crate::credential::Jwt;
use crate::credential::Policy;
use crate::credential::RefreshService;
use crate::error::Error;
use crate::error::Result;

use super::jwt_serialization::PresentationJwtClaims;
use super::JwtPresentationBuilder;
use super::JwtPresentationOptions;

/// Represents a bundle of one or more [Credential]s.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct JwtPresentation<T = Object> {
  /// The JSON-LD context(s) applicable to the `Presentation`.
  #[serde(rename = "@context")]
  pub context: OneOrMany<Context>,
  /// A unique `URI` that may be used to identify the `Presentation`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<Url>,
  /// One or more URIs defining the type of the `Presentation`.
  #[serde(rename = "type")]
  pub types: OneOrMany<String>,
  /// Credential(s) expressing the claims of the `Presentation`.
  #[serde(default = "Default::default", rename = "verifiableCredential")]
  pub verifiable_credential: OneOrMany<Jwt>,
  /// The entity that generated the `Presentation`.
  pub holder: Url,
  /// Service(s) used to refresh an expired [`Credential`] in the `Presentation`.
  #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
  pub refresh_service: OneOrMany<RefreshService>,
  /// Terms-of-use specified by the `Presentation` holder.
  #[serde(default, rename = "termsOfUse", skip_serializing_if = "OneOrMany::is_empty")]
  pub terms_of_use: OneOrMany<Policy>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  pub properties: T,
  /// Optional proof that can be verified by users in addition to JWS.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proof: Option<Object>,
}

impl<T> JwtPresentation<T> {
  /// Returns the base JSON-LD context for `JwtPresentation`s.
  pub fn base_context() -> &'static Context {
    Credential::<Object>::base_context()
  }

  /// Returns the base type for `JwtPresentation`s.
  pub const fn base_type() -> &'static str {
    "VerifiablePresentation"
  }

  /// Creates a `JwtPresentationBuilder` to configure a new Presentation.
  ///
  /// This is the same as [JwtPresentationBuilder::new].
  pub fn builder(holder: Url, properties: T) -> JwtPresentationBuilder<T> {
    JwtPresentationBuilder::new(holder, properties)
  }

  /// Returns a new `JwtPresentation` based on the `JwtPresentationBuilder` configuration.
  pub fn from_builder(builder: JwtPresentationBuilder<T>) -> Result<Self> {
    let this: Self = Self {
      context: builder.context.into(),
      id: builder.id,
      types: builder.types.into(),
      verifiable_credential: builder.credentials.into(),
      holder: builder.holder,
      refresh_service: builder.refresh_service.into(),
      terms_of_use: builder.terms_of_use.into(),
      properties: builder.properties,
      proof: None,
    };
    this.check_structure()?;

    Ok(this)
  }

  /// Validates the semantic structure of the `JwtPresentation`.
  pub fn check_structure(&self) -> Result<()> {
    // Ensure the base context is present and in the correct location
    match self.context.get(0) {
      Some(context) if context == Self::base_context() => {}
      Some(_) | None => return Err(Error::MissingBaseContext),
    }

    // The set of types MUST contain the base type
    if !self.types.iter().any(|type_| type_ == Self::base_type()) {
      return Err(Error::MissingBaseType);
    }

    //Todo: should check credentials structure?

    Ok(())
  }

  /// Serializes the [`JwtPresentation`] as a JWT claims set
  /// in accordance with [VC-JWT version 1.1.](https://w3c.github.io/vc-jwt/#version-1.1).
  ///
  /// The resulting string can be used as the payload of a JWS when issuing the credential.  
  pub fn serialize_jwt(&self, options: &JwtPresentationOptions) -> Result<String>
  where
    T: ToOwned<Owned = T> + serde::Serialize + serde::de::DeserializeOwned,
  {
    let jwt_representation: PresentationJwtClaims<'_, T> = PresentationJwtClaims::new(self, options)?;
    jwt_representation
      .to_json()
      .map_err(|err| Error::JwtClaimsSetSerializationError(err.into()))
  }

  /// Returns a reference to the `JwtPresentation` proof.
  pub fn proof(&self) -> Option<&Object> {
    self.proof.as_ref()
  }

  /// Returns a mutable reference to the `JwtPresentation` proof.
  pub fn proof_mut(&mut self) -> Option<&mut Object> {
    self.proof.as_mut()
  }
}

impl<T> Display for JwtPresentation<T>
where
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl<T> TryMethod for JwtPresentation<T> {
  const TYPE: MethodUriType = MethodUriType::Absolute;
}
