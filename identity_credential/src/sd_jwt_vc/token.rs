// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use super::claims::SdJwtVcClaims;
use super::metadata::IssuerMetadata;
use super::metadata::TypeMetadata;
#[allow(unused_imports)]
use super::metadata::WELL_KNOWN_VCT;
use super::metadata::WELL_KNOWN_VC_ISSUER;
use super::resolver::Error as ResolverErr;
use super::Error;
use super::Resolver;
use super::Result;
use super::SdJwtVcPresentationBuilder;
use identity_core::common::StringOrUrl;
use identity_core::common::Url;
use sd_jwt_payload_rework::Hasher;
use sd_jwt_payload_rework::JsonObject;
use sd_jwt_payload_rework::SdJwt;
use serde_json::Value;

/// SD-JWT VC's JOSE header `typ`'s value.
pub const SD_JWT_VC_TYP: &str = "vc+sd-jwt";

#[derive(Debug, Clone, PartialEq, Eq)]
/// An SD-JWT carrying a verifiable credential as described in
/// [SD-JWT VC specification](https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-04.html).
pub struct SdJwtVc {
  pub(crate) sd_jwt: SdJwt,
  pub(crate) parsed_claims: SdJwtVcClaims,
}

impl Deref for SdJwtVc {
  type Target = SdJwt;
  fn deref(&self) -> &Self::Target {
    &self.sd_jwt
  }
}

impl SdJwtVc {
  pub(crate) fn new(sd_jwt: SdJwt, claims: SdJwtVcClaims) -> Self {
    Self {
      sd_jwt,
      parsed_claims: claims,
    }
  }

  /// Parses a string into an [`SdJwtVc`].
  pub fn parse(s: &str) -> Result<Self> {
    s.parse()
  }

  /// Returns a reference to this [`SdJwtVc`]'s JWT claims.
  pub fn claims(&self) -> &SdJwtVcClaims {
    &self.parsed_claims
  }

  /// Prepares this [`SdJwtVc`] for a presentation, returning an [`SdJwtVcPresentationBuilder`].
  /// ## Errors
  /// - [`Error::SdJwt`] is returned if the provided `hasher`'s algorithm doesn't match the algorithm specified by
  ///   SD-JWT's `_sd_alg` claim. "sha-256" is used if the claim is missing.
  pub fn into_presentation(self, hasher: &dyn Hasher) -> Result<SdJwtVcPresentationBuilder> {
    SdJwtVcPresentationBuilder::new(self, hasher)
  }

  /// Returns the JSON object obtained by replacing all disclosures into their
  /// corresponding JWT concealable claims.
  pub fn into_disclosed_object(self, hasher: &dyn Hasher) -> Result<JsonObject> {
    SdJwt::from(self).into_disclosed_object(hasher).map_err(Error::SdJwt)
  }

  /// Retrieves this SD-JWT VC's issuer's metadata by querying its default location.
  /// ## Notes
  /// This method doesn't perform any validation of the retrieved [`IssuerMetadata`]
  /// besides its syntactical validity.
  /// To check if the retrieved [`IssuerMetadata`] is valid use [`IssuerMetadata::validate`].
  pub async fn issuer_metadata<R>(&self, resolver: &R) -> Result<Option<IssuerMetadata>>
  where
    R: Resolver<Url, Target = Vec<u8>>,
  {
    let metadata_url = {
      let origin = self.claims().iss.origin().ascii_serialization();
      let path = self.claims().iss.path();
      format!("{origin}{WELL_KNOWN_VC_ISSUER}{path}").parse().unwrap()
    };
    match resolver.resolve(&metadata_url).await {
      Err(ResolverErr::NotFound(_)) => Ok(None),
      Err(e) => Err(Error::Resolution {
        input: metadata_url.to_string(),
        source: e,
      }),
      Ok(json_res) => serde_json::from_slice(&json_res)
        .map_err(|e| Error::InvalidIssuerMetadata(e.into()))
        .map(Some),
    }
  }

  /// Retrieve this SD-JWT VC credential's type metadata [`TypeMetadata`].
  /// ## Notes
  /// `resolver` is fed with whatever value [`SdJwtVc`]'s `vct` might have.
  /// If `vct` is a URI with scheme `https`, `resolver` must fetch the [`TypeMetadata`]
  /// resource by combining `vct`'s value with [`WELL_KNOWN_VCT`]. To simplify this process
  /// the utility function [`vct_to_url`] is provided.
  ///
  /// Returns the parsed [`TypeMetadata`] along with the raw [`Resolver`]'s response.
  /// The latter can be used to validate the `vct#integrity` claim if present.
  pub async fn type_metadata<R>(&self, resolver: &R) -> Result<(TypeMetadata, Vec<u8>)>
  where
    R: Resolver<StringOrUrl, Target = Vec<u8>>,
  {
    let vct = &self.claims().vct;
    let raw = resolver.resolve(vct).await.map_err(|e| Error::Resolution {
      input: vct.to_string(),
      source: e,
    })?;
    let metadata = serde_json::from_slice(&raw).map_err(|e| Error::InvalidTypeMetadata(e.into()))?;

    Ok((metadata, raw))
  }
}

/// Converts `vct` claim's URI value into the appropriate well-known URL.
/// ## Warnings
/// Returns an [`Option::None`] if the URI's scheme is not `https`.
pub fn vct_to_url(resource: &Url) -> Option<Url> {
  if resource.scheme() != "https" {
    None
  } else {
    let origin = resource.origin().ascii_serialization();
    let path = resource.path();
    Some(format!("{origin}{WELL_KNOWN_VC_ISSUER}{path}").parse().unwrap())
  }
}

impl TryFrom<SdJwt> for SdJwtVc {
  type Error = Error;
  fn try_from(mut sd_jwt: SdJwt) -> std::result::Result<Self, Self::Error> {
    // Validate claims.
    let claims = {
      let claims = std::mem::take(sd_jwt.claims_mut());
      SdJwtVcClaims::try_from_sd_jwt_claims(claims, sd_jwt.disclosures())?
    };

    // Validate Header's typ.
    let typ = sd_jwt
      .header()
      .get("typ")
      .and_then(Value::as_str)
      .ok_or_else(|| Error::InvalidJoseType("null".to_string()))?;
    if !typ.contains(SD_JWT_VC_TYP) {
      return Err(Error::InvalidJoseType(typ.to_string()));
    }

    Ok(Self {
      sd_jwt,
      parsed_claims: claims,
    })
  }
}

impl FromStr for SdJwtVc {
  type Err = Error;
  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    s.parse::<SdJwt>().map_err(Error::SdJwt).and_then(TryInto::try_into)
  }
}

impl Display for SdJwtVc {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.sd_jwt)
  }
}

impl From<SdJwtVc> for SdJwt {
  fn from(value: SdJwtVc) -> Self {
    let SdJwtVc {
      mut sd_jwt,
      parsed_claims,
    } = value;
    // Put back `parsed_claims`.
    *sd_jwt.claims_mut() = parsed_claims.into();

    sd_jwt
  }
}

#[cfg(test)]
mod tests {
  use std::cell::LazyCell;

  use identity_core::common::StringOrUrl;
  use identity_core::common::Url;

  use super::*;

  const EXAMPLE_SD_JWT_VC: &str = "eyJhbGciOiAiRVMyNTYiLCAidHlwIjogInZjK3NkLWp3dCJ9.eyJfc2QiOiBbIjBIWm1uU0lQejMzN2tTV2U3QzM0bC0tODhnekppLWVCSjJWel9ISndBVGciLCAiOVpicGxDN1RkRVc3cWFsNkJCWmxNdHFKZG1lRU9pWGV2ZEpsb1hWSmRSUSIsICJJMDBmY0ZVb0RYQ3VjcDV5eTJ1anFQc3NEVkdhV05pVWxpTnpfYXdEMGdjIiwgIklFQllTSkdOaFhJbHJRbzU4eWtYbTJaeDN5bGw5WmxUdFRvUG8xN1FRaVkiLCAiTGFpNklVNmQ3R1FhZ1hSN0F2R1RyblhnU2xkM3o4RUlnX2Z2M2ZPWjFXZyIsICJodkRYaHdtR2NKUXNCQ0EyT3RqdUxBY3dBTXBEc2FVMG5rb3ZjS09xV05FIiwgImlrdXVyOFE0azhxM1ZjeUE3ZEMtbU5qWkJrUmVEVFUtQ0c0bmlURTdPVFUiLCAicXZ6TkxqMnZoOW80U0VYT2ZNaVlEdXZUeWtkc1dDTmcwd1RkbHIwQUVJTSIsICJ3elcxNWJoQ2t2a3N4VnZ1SjhSRjN4aThpNjRsbjFqb183NkJDMm9hMXVnIiwgInpPZUJYaHh2SVM0WnptUWNMbHhLdUVBT0dHQnlqT3FhMXoySW9WeF9ZRFEiXSwgImlzcyI6ICJodHRwczovL2V4YW1wbGUuY29tL2lzc3VlciIsICJpYXQiOiAxNjgzMDAwMDAwLCAiZXhwIjogMTg4MzAwMDAwMCwgInZjdCI6ICJodHRwczovL2JtaS5idW5kLmV4YW1wbGUvY3JlZGVudGlhbC9waWQvMS4wIiwgImFnZV9lcXVhbF9vcl9vdmVyIjogeyJfc2QiOiBbIkZjOElfMDdMT2NnUHdyREpLUXlJR085N3dWc09wbE1Makh2UkM0UjQtV2ciLCAiWEx0TGphZFVXYzl6Tl85aE1KUm9xeTQ2VXNDS2IxSXNoWnV1cVVGS1NDQSIsICJhb0NDenNDN3A0cWhaSUFoX2lkUkNTQ2E2NDF1eWNuYzh6UGZOV3o4bngwIiwgImYxLVAwQTJkS1dhdnYxdUZuTVgyQTctRVh4dmhveHY1YUhodUVJTi1XNjQiLCAiazVoeTJyMDE4dnJzSmpvLVZqZDZnNnl0N0Fhb25Lb25uaXVKOXplbDNqbyIsICJxcDdaX0t5MVlpcDBzWWdETzN6VnVnMk1GdVBOakh4a3NCRG5KWjRhSS1jIl19LCAiX3NkX2FsZyI6ICJzaGEtMjU2IiwgImNuZiI6IHsiandrIjogeyJrdHkiOiAiRUMiLCAiY3J2IjogIlAtMjU2IiwgIngiOiAiVENBRVIxOVp2dTNPSEY0ajRXNHZmU1ZvSElQMUlMaWxEbHM3dkNlR2VtYyIsICJ5IjogIlp4amlXV2JaTVFHSFZXS1ZRNGhiU0lpcnNWZnVlY0NFNnQ0alQ5RjJIWlEifX19.CaXec2NNooWAy4eTxYbGWI--UeUL0jpC7Zb84PP_09Z655BYcXUTvfj6GPk4mrNqZUU5GT6QntYR8J9rvcBjvA~WyJuUHVvUW5rUkZxM0JJZUFtN0FuWEZBIiwgIm5hdGlvbmFsaXRpZXMiLCBbIkRFIl1d~WyJNMEpiNTd0NDF1YnJrU3V5ckRUM3hBIiwgIjE4IiwgdHJ1ZV0~eyJhbGciOiAiRVMyNTYiLCAidHlwIjogImtiK2p3dCJ9.eyJub25jZSI6ICIxMjM0NTY3ODkwIiwgImF1ZCI6ICJodHRwczovL2V4YW1wbGUuY29tL3ZlcmlmaWVyIiwgImlhdCI6IDE3MjA0NTQyOTUsICJzZF9oYXNoIjogIlZFejN0bEtqOVY0UzU3TTZoRWhvVjRIc19SdmpXZWgzVHN1OTFDbmxuZUkifQ.GqtiTKNe3O95GLpdxFK_2FZULFk6KUscFe7RPk8OeVLiJiHsGvtPyq89e_grBplvGmnDGHoy8JAt1wQqiwktSg";
  const EXAMPLE_ISSUER: LazyCell<Url> = LazyCell::new(|| "https://example.com/issuer".parse().unwrap());
  const EXAMPLE_VCT: LazyCell<StringOrUrl> = LazyCell::new(|| {
    "https://bmi.bund.example/credential/pid/1.0"
      .parse::<Url>()
      .unwrap()
      .into()
  });

  #[test]
  fn simple_sd_jwt_is_not_a_valid_sd_jwt_vc() {
    let sd_jwt: SdJwt = "eyJhbGciOiAiRVMyNTYiLCAidHlwIjogImV4YW1wbGUrc2Qtand0In0.eyJfc2QiOiBbIkM5aW5wNllvUmFFWFI0Mjd6WUpQN1FyazFXSF84YmR3T0FfWVVyVW5HUVUiLCAiS3VldDF5QWEwSElRdlluT1ZkNTloY1ZpTzlVZzZKMmtTZnFZUkJlb3d2RSIsICJNTWxkT0ZGekIyZDB1bWxtcFRJYUdlcmhXZFVfUHBZZkx2S2hoX2ZfOWFZIiwgIlg2WkFZT0lJMnZQTjQwVjd4RXhad1Z3ejd5Um1MTmNWd3Q1REw4Ukx2NGciLCAiWTM0em1JbzBRTExPdGRNcFhHd2pCZ0x2cjE3eUVoaFlUMEZHb2ZSLWFJRSIsICJmeUdwMFdUd3dQdjJKRFFsbjFsU2lhZW9iWnNNV0ExMGJRNTk4OS05RFRzIiwgIm9tbUZBaWNWVDhMR0hDQjB1eXd4N2ZZdW8zTUhZS08xNWN6LVJaRVlNNVEiLCAiczBCS1lzTFd4UVFlVTh0VmxsdE03TUtzSVJUckVJYTFQa0ptcXhCQmY1VSJdLCAiaXNzIjogImh0dHBzOi8vaXNzdWVyLmV4YW1wbGUuY29tIiwgImlhdCI6IDE2ODMwMDAwMDAsICJleHAiOiAxODgzMDAwMDAwLCAiYWRkcmVzcyI6IHsiX3NkIjogWyI2YVVoelloWjdTSjFrVm1hZ1FBTzN1MkVUTjJDQzFhSGhlWnBLbmFGMF9FIiwgIkF6TGxGb2JrSjJ4aWF1cFJFUHlvSnotOS1OU2xkQjZDZ2pyN2ZVeW9IemciLCAiUHp6Y1Z1MHFiTXVCR1NqdWxmZXd6a2VzRDl6dXRPRXhuNUVXTndrclEtayIsICJiMkRrdzBqY0lGOXJHZzhfUEY4WmN2bmNXN3p3Wmo1cnlCV3ZYZnJwemVrIiwgImNQWUpISVo4VnUtZjlDQ3lWdWIyVWZnRWs4anZ2WGV6d0sxcF9KbmVlWFEiLCAiZ2xUM2hyU1U3ZlNXZ3dGNVVEWm1Xd0JUdzMyZ25VbGRJaGk4aEdWQ2FWNCIsICJydkpkNmlxNlQ1ZWptc0JNb0d3dU5YaDlxQUFGQVRBY2k0MG9pZEVlVnNBIiwgInVOSG9XWWhYc1poVkpDTkUyRHF5LXpxdDd0NjlnSkt5NVFhRnY3R3JNWDQiXX0sICJfc2RfYWxnIjogInNoYS0yNTYifQ.gR6rSL7urX79CNEvTQnP1MH5xthG11ucIV44SqKFZ4Pvlu_u16RfvXQd4k4CAIBZNKn2aTI18TfvFwV97gJFoA~WyJHMDJOU3JRZmpGWFE3SW8wOXN5YWpBIiwgInJlZ2lvbiIsICJcdTZlMmZcdTUzM2EiXQ~WyJsa2x4RjVqTVlsR1RQVW92TU5JdkNBIiwgImNvdW50cnkiLCAiSlAiXQ~"
      .parse().unwrap();
    let err = SdJwtVc::try_from(sd_jwt).unwrap_err();
    assert!(matches!(err, Error::MissingClaim("vct")))
  }

  #[test]
  fn parsing_a_valid_sd_jwt_vc_works() {
    let sd_jwt_vc: SdJwtVc = EXAMPLE_SD_JWT_VC.parse().unwrap();
    assert_eq!(sd_jwt_vc.claims().iss, *EXAMPLE_ISSUER);
    assert_eq!(sd_jwt_vc.claims().vct, *EXAMPLE_VCT);
  }
}