// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::OneOrMany;
use identity_core::common::Url;
use identity_core::crypto::Proof;
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
  exp: Option<i64>,
  /// Represents the issuer of the presentation who is the same as the holder of the verifiable
  /// credentials.
  iss: Cow<'presentation, Url>,

  /// Represents the issuanceDate encoded as a UNIX timestamp.
  #[serde(flatten)]
  issuance_date: Option<IssuanceDateClaims>,

  /// Represents the id property of the credential.
  #[serde(skip_serializing_if = "Option::is_none")]
  jti: Option<Cow<'presentation, Url>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  aud: Option<Url>,

  vp: InnerPresentation<'presentation, T>,
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
        holder: Some(holder_url),
        refresh_service,
        terms_of_use,
        properties,
        proof
    } = presentation else {
            return Err(Error::MissingHolder)
        };

    Ok(Self {
      iss: Cow::Borrowed(holder_url),
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
struct InnerPresentation<'presentation, T = Object>
where
  T: ToOwned + Serialize,
  <T as ToOwned>::Owned: DeserializeOwned,
{
  /// The JSON-LD context(s) applicable to the `Presentation`.
  #[serde(rename = "@context")]
  context: Cow<'presentation, OneOrMany<Context>>,
  /// A unique `URI` that may be used to identify the `Presentation`.
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<Url>,
  /// One or more URIs defining the type of the `Presentation`.
  #[serde(rename = "type")]
  types: Cow<'presentation, OneOrMany<String>>,
  /// Credential(s) expressing the claims of the `Presentation`.
  #[serde(default = "Default::default", rename = "verifiableCredential")]
  verifiable_credential: Cow<'presentation, OneOrMany<Jwt>>,
  /// Service(s) used to refresh an expired [`Credential`] in the `Presentation`.
  #[serde(default, rename = "refreshService", skip_serializing_if = "OneOrMany::is_empty")]
  refresh_service: Cow<'presentation, OneOrMany<RefreshService>>,
  /// Terms-of-use specified by the `Presentation` holder.
  #[serde(default, rename = "termsOfUse", skip_serializing_if = "OneOrMany::is_empty")]
  terms_of_use: Cow<'presentation, OneOrMany<Policy>>,
  /// Miscellaneous properties.
  #[serde(flatten)]
  properties: Cow<'presentation, T>,
  /// Proof(s) used to verify a `Presentation`
  #[serde(skip_serializing_if = "Option::is_none")]
  proof: Option<Cow<'presentation, Proof>>,
}

impl<'presentation, T> PresentationJwtClaims<'presentation, T>
where
  T: ToOwned<Owned = T> + Serialize + DeserializeOwned,
{
  pub(crate) fn try_into_presentation(&self) -> Result<JwtPresentation> {
    OK(())
  }

  fn check_consistency(&self) -> Result<()> {
    // todo!
    OK(())
  }
}
