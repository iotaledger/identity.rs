// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use crate::utils::err;
use identity::comm;
use wasm_bindgen::prelude::*;

//This will not work until this change is made https://github.com/uuid-rs/uuid/pull/512 , because uuid v4 doen't work on wasm32-unknown-unknown

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct AuthenticationRequest(pub(crate) comm::AuthenticationRequest);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct AutenticationResponse(pub(crate) comm::AuthenticationResponse);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialIssuance(pub(crate) comm::CredentialIssuance);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialOptionRequest(pub(crate) comm::CredentialOptionRequest);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialOptionResponse(pub(crate) comm::CredentialOptionResponse);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialSchemaRequest(pub(crate) comm::CredentialSchemaRequest);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialSchemaResponse(pub(crate) comm::CredentialSchemaResponse);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct CredentialSelection(pub(crate) comm::CredentialSelection);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct FeaturesRequest(pub(crate) comm::FeaturesRequest);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct FeaturesResponse(pub(crate) comm::FeaturesResponse);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct IntroductionProposal(pub(crate) comm::IntroductionProposal);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct IntroductionResponse(pub(crate) comm::IntroductionResponse);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct PresentationRequest(pub(crate) comm::PresentationRequest);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct PresentationResponse(pub(crate) comm::PresentationResponse);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Report(pub(crate) comm::Report);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct ResolutionRequest(pub(crate) comm::ResolutionRequest);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct ResolutionResponse(pub(crate) comm::ResolutionResponse);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Revocation(pub(crate) comm::Revocation);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Timing(pub(crate) comm::Timing);
#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Trustping(pub(crate) comm::Trustping);

#[wasm_bindgen]
impl AuthenticationRequest {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<AuthenticationRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl AutenticationResponse {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<AutenticationResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl CredentialIssuance {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialIssuance, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl CredentialOptionRequest {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialOptionRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl CredentialOptionResponse {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialOptionResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl CredentialSchemaRequest {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialSchemaRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl CredentialSchemaResponse {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialSchemaResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl CredentialSelection {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<CredentialSelection, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl FeaturesRequest {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<FeaturesRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl FeaturesResponse {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<FeaturesResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl IntroductionProposal {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<IntroductionProposal, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl IntroductionResponse {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<IntroductionResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl PresentationRequest {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<PresentationRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl PresentationResponse {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<PresentationResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl Report {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Report, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl ResolutionRequest {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<ResolutionRequest, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl ResolutionResponse {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<ResolutionResponse, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl Revocation {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Revocation, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl Timing {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Timing, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
#[wasm_bindgen]
impl Trustping {
  /// Serializes a `Service` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Trustping, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
