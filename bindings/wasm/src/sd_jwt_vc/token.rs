// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use anyhow::Context;
use identity_iota::core::Url;
use identity_iota::credential::sd_jwt_vc::vct_to_url as vct_to_url_impl;
use identity_iota::credential::sd_jwt_vc::SdJwtVc;
use wasm_bindgen::prelude::*;

use crate::credential::WasmKeyBindingJWTValidationOptions;
use crate::error::Result;
use crate::error::WasmResult;
use crate::jose::WasmJwk;
use crate::sd_jwt_vc::metadata::WasmTypeMetadata;
use crate::verification::IJwsVerifier;
use crate::verification::WasmJwsVerifier;

use super::metadata::WasmIssuerMetadata;
use super::resolver::ResolverStringToUint8Array;
use super::sd_jwt_v2::WasmHasher;
use super::sd_jwt_v2::WasmSdJwt;
use super::WasmSdJwtVcClaims;
use super::WasmSdJwtVcPresentationBuilder;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "[TypeMetadata, Uint8Array]")]
  pub type TypeMetadataResult;
}

#[derive(Clone)]
#[wasm_bindgen(js_name = SdJwtVc)]
pub struct WasmSdJwtVc(pub(crate) SdJwtVc);

#[wasm_bindgen(js_class = "SdJwtVc")]
impl WasmSdJwtVc {
  /// Parses a `string` into an {@link SdJwtVc}.
  #[wasm_bindgen]
  pub fn parse(s: &str) -> Result<Self> {
    SdJwtVc::parse(s).map(WasmSdJwtVc).wasm_result()
  }

  #[wasm_bindgen]
  pub fn claims(&self) -> Result<WasmSdJwtVcClaims> {
    serde_wasm_bindgen::to_value(self.0.claims())
      .map(JsCast::unchecked_into)
      .wasm_result()
  }

  #[wasm_bindgen(js_name = "issuerJwk")]
  pub async fn issuer_jwk(&self, resolver: &ResolverStringToUint8Array) -> Result<WasmJwk> {
    self.0.issuer_jwk(resolver).await.map(WasmJwk).wasm_result()
  }

  #[wasm_bindgen(js_name = "issuerMetadata")]
  pub async fn issuer_metadata(&self, resolver: &ResolverStringToUint8Array) -> Result<Option<WasmIssuerMetadata>> {
    self
      .0
      .issuer_metadata(resolver)
      .await
      .map(|maybe_metadata| maybe_metadata.map(WasmIssuerMetadata))
      .wasm_result()
  }

  #[wasm_bindgen(js_name = "typeMetadata")]
  pub async fn type_metadata(&self, resolver: &ResolverStringToUint8Array) -> Result<TypeMetadataResult> {
    self
      .0
      .type_metadata(resolver)
      .await
      .context("resolution error")
      .and_then(|(metadata, integrity)| {
        serde_wasm_bindgen::to_value(&(WasmTypeMetadata(metadata), integrity)).map_err(|e| anyhow!("{}", e.to_string()))
      })
      .wasm_result()
      .and_then(JsCast::dyn_into)
  }

  #[wasm_bindgen(js_name = "verifySignature")]
  pub fn verify_signature(&self, jws_verifier: Option<IJwsVerifier>, jwk: &WasmJwk) -> Result<()> {
    let verifier = WasmJwsVerifier::new(jws_verifier);
    self.0.verify_signature(&verifier, &jwk.0).wasm_result()
  }

  /// Verify the signature of this {@link SdJwtVc}'s {@link KeyBindingJwt}.
  #[wasm_bindgen(js_name = "verifyKeyBinding")]
  pub fn verify_key_binding(&self, jws_verifier: Option<IJwsVerifier>, jwk: &WasmJwk) -> Result<()> {
    let verifier = WasmJwsVerifier::new(jws_verifier);
    self.0.verify_key_binding(&verifier, &jwk.0).wasm_result()
  }

  #[wasm_bindgen(js_name = "validateKeyBinding")]
  pub fn validate_key_binding(
    &self,
    jws_verifier: Option<IJwsVerifier>,
    jwk: &WasmJwk,
    hasher: &WasmHasher,
    options: &WasmKeyBindingJWTValidationOptions,
  ) -> Result<()> {
    let jws_verifier = WasmJwsVerifier::new(jws_verifier);
    self
      .0
      .validate_key_binding(&jws_verifier, &jwk.0, hasher, &options.0)
      .wasm_result()
  }

  #[wasm_bindgen(js_name = "intoSdJwt")]
  pub fn into_sd_jwt(self) -> WasmSdJwt {
    WasmSdJwt(self.0.into())
  }

  #[wasm_bindgen(js_name = "intoDisclosedObject")]
  pub fn into_disclosed_object(self, hasher: &WasmHasher) -> Result<js_sys::Object> {
    self
      .0
      .into_disclosed_object(hasher)
      .map(|obj| serde_wasm_bindgen::to_value(&obj).expect("JSON object is a valid JS object"))
      .map(JsCast::unchecked_into)
      .wasm_result()
  }

  #[wasm_bindgen(js_name = "intoPresentation")]
  pub fn into_presentation(self, hasher: &WasmHasher) -> Result<WasmSdJwtVcPresentationBuilder> {
    WasmSdJwtVcPresentationBuilder::new(self, hasher)
  }
}

#[wasm_bindgen(js_name = "vctToUrl")]
pub fn vct_to_url(resource: &str) -> Option<String> {
  let url = resource.parse::<Url>().ok()?;
  vct_to_url_impl(&url).map(|url| url.to_string())
}
