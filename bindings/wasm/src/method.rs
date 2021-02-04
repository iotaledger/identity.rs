// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::Method as Method_;
use identity::did::MethodBuilder;
use identity::did::MethodData;
use identity::did::MethodType;
use identity::iota::IotaDID;
use wasm_bindgen::prelude::*;

use crate::did::DID;
use crate::key::Algorithm;
use crate::key::KeyPair;
use crate::utils::err;

pub const DEFAULT_TAG: &str = "authentication";

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Params {
  None,
  Object { did: Option<String>, tag: Option<String> },
}

impl Params {
  fn did(&self) -> Option<&str> {
    match self {
      Self::None => None,
      Self::Object { did, .. } => did.as_deref(),
    }
  }

  fn tag(&self) -> Option<&str> {
    match self {
      Self::None => None,
      Self::Object { tag, .. } => tag.as_deref(),
    }
  }
}

// =============================================================================
// =============================================================================

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Method(pub(crate) Method_);

#[wasm_bindgen]
impl Method {
  pub(crate) fn create(key: &KeyPair, did: DID, tag: Option<&str>) -> Result<Method, JsValue> {
    let tag: String = format!("#{}", tag.as_deref().unwrap_or(DEFAULT_TAG));
    let kid: DID = did.0.join(tag).map_err(err).map(DID)?;

    let type_: MethodType = match key.alg {
      Algorithm::Ed25519 => MethodType::Ed25519VerificationKey2018,
    };

    let data: MethodData = match key.alg {
      Algorithm::Ed25519 => MethodData::PublicKeyBase58(key.public()),
    };

    MethodBuilder::default()
      .id(kid.0.into())
      .controller(did.0.into())
      .key_type(type_)
      .key_data(data)
      .build()
      .map_err(err)
      .map(Self)
  }

  /// Creates a new Verification Method object.
  #[wasm_bindgen(constructor)]
  pub fn new(key: &KeyPair, value: &JsValue) -> Result<Method, JsValue> {
    let params: Params = value.into_serde().map_err(err)?;

    let did: DID = match params.did() {
      Some(did) => DID::parse(did)?,
      None => DID::new(key, None, None)?,
    };

    Self::create(key, did, params.tag())
  }

  /// Returns the `id` DID of the `Method` object.
  #[wasm_bindgen(getter)]
  pub fn id(&self) -> Result<DID, JsValue> {
    IotaDID::try_from_borrowed(self.0.id())
      .map_err(err)
      .map(|did| DID(did.clone()))
  }

  /// Returns the `controller` DID of the `Method` object.
  #[wasm_bindgen(getter)]
  pub fn controller(&self) -> Result<DID, JsValue> {
    IotaDID::try_from_borrowed(self.0.controller())
      .map_err(err)
      .map(|did| DID(did.clone()))
  }

  #[wasm_bindgen(getter, js_name = type)]
  pub fn type_(&self) -> JsValue {
    self.0.key_type().as_str().into()
  }

  #[wasm_bindgen(getter)]
  pub fn data(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(self.0.key_data()).map_err(err)
  }

  /// Serializes a `Method` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(err)
  }

  /// Deserializes a `Method` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(value: &JsValue) -> Result<Method, JsValue> {
    value.into_serde().map_err(err).map(Self)
  }
}
