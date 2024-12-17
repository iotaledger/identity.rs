use async_trait::async_trait;
use identity_iota::core::Url;
use identity_iota::credential::sd_jwt_vc::resolver::Error as ErrorR;
use identity_iota::credential::sd_jwt_vc::Resolver;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub type ResolverUrlToU8Array;

  #[wasm_bindgen(structural, method, catch)]
  pub async fn resolve(this: &ResolverUrlToU8Array, input: &str) -> Result<Uint8Array, js_sys::Error>;
}


#[async_trait(?Send)]
impl Resolver<Url, Vec<u8>> for ResolverUrlToU8Array {
  async fn resolve(&self, input: &Url) -> std::result::Result<Vec<u8>, ErrorR> {
    self
      .resolve(input.as_str())
      .await
      .map(|arr| arr.to_vec())
      .map_err(|e| ErrorR::Generic(anyhow::anyhow!("{}", e.to_string())))
  }
}
