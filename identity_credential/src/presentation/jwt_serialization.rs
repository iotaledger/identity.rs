// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Url;
use serde::de::DeserializeOwned;

use crate::credential::IssuanceDateClaims;
use crate::credential::Jwt;
use crate::credential::Policy;
use crate::credential::RefreshService;
use crate::presentation::JwtPresentation;
use crate::Error;
use crate::Result;

use super::JwtPresentationOptions;

#[derive(Serialize, Deserialize)]
pub(crate) struct PresentationJwtClaims<'presentation, T = Object>
where
  T: ToOwned + Serialize,
  <T as ToOwned>::Owned: DeserializeOwned,
{
  /// Represents the expirationDate encoded as a UNIX timestamp.  
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) exp: Option<i64>,

  /// Represents the holder of the verifiable presentation.
  pub(crate) iss: Cow<'presentation, Url>,

  /// Represents the issuanceDate encoded as a UNIX timestamp.
  #[serde(flatten)]
  pub(crate) issuance_date: Option<IssuanceDateClaims>,

  /// Represents the id property of the credential.
  #[serde(skip_serializing_if = "Option::is_none")]
  jti: Option<Cow<'presentation, Url>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) aud: Option<Url>,

  pub(crate) vp: InnerPresentation<'presentation, T>,
}

impl<'presentation, T> PresentationJwtClaims<'presentation, T>
where
  T: ToOwned<Owned = T> + Serialize + DeserializeOwned,
{
  pub(super) fn new(presentation: &'presentation JwtPresentation<T>, options: &JwtPresentationOptions) -> Result<Self> {
    let JwtPresentation {
      context,
      id,
      types,
      verifiable_credential,
      holder,
      refresh_service,
      terms_of_use,
      properties,
      proof,
    } = presentation;

    Ok(Self {
      iss: Cow::Borrowed(holder),
      jti: id.as_ref().map(Cow::Borrowed),
      vp: InnerPresentation {
        context: Cow::Borrowed(context),
        id: None,
        types: Cow::Borrowed(types),
        verifiable_credential: Cow::Borrowed(verifiable_credential),
        refresh_service: Cow::Borrowed(refresh_service),
        terms_of_use: Cow::Borrowed(terms_of_use),
        properties: Cow::Borrowed(properties),
        proof: proof.as_ref().map(Cow::Borrowed),
      },
      exp: options.expiration_date.map(|expiration_date| expiration_date.to_unix()),
      issuance_date: options.issuance_date.map(IssuanceDateClaims::new),
      aud: options.audience.clone(),
    })
  }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct InnerPresentation<'presentation, T = Object>
where
  T: ToOwned + Serialize,
  <T as ToOwned>::Owned: DeserializeOwned,
{
  /// The JSON-LD context(s) applicable to the `JwtPresentation`.
  #[serde(rename = "@context")]
  context: Cow<'presentation, OneOrMany<Context>>,
  /// A unique `URI` that may be used to identify the `JwtPresentation`.
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<Url>,
  /// One or more URIs defining the type of the `JwtPresentation`.
  #[serde(rename = "type")]
  types: Cow<'presentation, OneOrMany<String>>,
  /// Credential(s) expressing the claims of the `JwtPresentation`.
  #[serde(default = "Default::default", rename = "verifiableCredential")]
  verifiable_credential: Cow<'presentation, OneOrMany<Jwt>>,
  /// Service(s) used to refresh an expired [`Credential`] in the `JwtPresentation`.
  #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
  refresh_service: Cow<'presentation, OneOrMany<RefreshService>>,
  /// Terms-of-use specified by the `JwtPresentation` holder.
  #[serde(default, rename = "termsOfUse", skip_serializing_if = "OneOrMany::is_empty")]
  terms_of_use: Cow<'presentation, OneOrMany<Policy>>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  properties: Cow<'presentation, T>,
  /// Proof(s) used to verify a `JwtPresentation`
  #[serde(skip_serializing_if = "Option::is_none")]
  proof: Option<Cow<'presentation, Object>>,
}

#[cfg(feature = "validator")]
impl<'presentation, T> PresentationJwtClaims<'presentation, T>
where
  T: ToOwned<Owned = T> + Serialize + DeserializeOwned,
{
  pub(crate) fn try_into_presentation(self) -> Result<JwtPresentation<T>> {
    self.check_consistency()?;
    let Self {
      exp: _,
      iss,
      issuance_date: _,
      jti,
      aud: _,
      vp,
    } = self;
    let InnerPresentation {
      context,
      id: _,
      types,
      verifiable_credential,
      refresh_service,
      terms_of_use,
      properties,
      proof,
    } = vp;

    let presentation = JwtPresentation {
      context: context.into_owned(),
      id: jti.map(Cow::into_owned),
      types: types.into_owned(),
      verifiable_credential: verifiable_credential.into_owned(),
      holder: iss.into_owned(),
      refresh_service: refresh_service.into_owned(),
      terms_of_use: terms_of_use.into_owned(),
      properties: properties.into_owned(),
      proof: proof.map(Cow::into_owned),
    };

    Ok(presentation)
  }

  fn check_consistency(&self) -> Result<()> {
    if !self
      .vp
      .id
      .as_ref()
      .map(|value| self.jti.as_ref().filter(|jti| jti.as_ref() == value).is_some())
      .unwrap_or(true)
    {
      return Err(Error::InconsistentPresentationJwtClaims("inconsistent presentation id"));
    };
    Ok(())
  }
}
