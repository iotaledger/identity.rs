use identity_iota::core::Url;
use identity_iota::credential::sd_jwt_vc::vct_to_url as vct_to_url_impl;
use identity_iota::credential::sd_jwt_vc::SdJwtVc;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::jose::WasmJwk;
use crate::verification::IJwsVerifier;
use crate::verification::WasmJwsVerifier;

use super::resolver::ResolverUrlToU8Array;

#[wasm_bindgen(js_name = SdJwtVc)]
pub struct WasmSdJwtVc(pub(crate) SdJwtVc);

#[wasm_bindgen(js_class = "SdJwtVc")]
impl WasmSdJwtVc {
  /// Parses a `string` into an {@link SdJwtVc}.
  #[wasm_bindgen]
  pub fn parse(s: &str) -> Result<Self> {
    SdJwtVc::parse(s).map(WasmSdJwtVc).wasm_result()
  }

  #[wasm_bindgen(js_name = "issuerJwk")]
  pub async fn issuer_jwk(&self, resolver: &ResolverUrlToU8Array) -> Result<WasmJwk> {
    self 
      .0
      .issuer_jwk(resolver)
      .await
      .map(WasmJwk)
      .wasm_result()
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
}

#[wasm_bindgen(js_name = "vctToUrl")]
pub fn vct_to_url(resource: &str) -> Option<String> {
  let url = resource.parse::<Url>().ok()?;
  vct_to_url_impl(&url).map(|url| url.to_string())
}
