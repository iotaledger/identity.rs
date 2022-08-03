// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity_stardust::block::output::AliasId;
use identity_stardust::block::output::AliasOutput;
use identity_stardust::block::output::OutputId;
use identity_stardust::block::output::RentStructure;
use identity_stardust::StardustIdentityClient;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::error::JsValueResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IStardustIdentityClient")]
  pub type WasmStardustIdentityClient;

  #[wasm_bindgen(method, js_name = getNetworkHrp)]
  pub fn get_network_hrp(this: &WasmStardustIdentityClient) -> JsValue;

  #[allow(non_snake_case)]
  #[wasm_bindgen(method, js_name = getAliasOutput)]
  pub fn get_alias_output(this: &WasmStardustIdentityClient, aliasId: String) -> JsValue;

  #[wasm_bindgen(method, js_name = getRentStructure)]
  pub fn get_rent_structure(this: &WasmStardustIdentityClient) -> JsValue;
}

impl Debug for WasmStardustIdentityClient {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("WasmStardustIdentityClient")
  }
}

#[async_trait::async_trait(?Send)]
impl StardustIdentityClient for WasmStardustIdentityClient {
  async fn get_network_hrp(&self) -> Result<String, identity_stardust::Error> {
    let promise: Promise = Promise::resolve(&WasmStardustIdentityClient::get_network_hrp(self));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js: JsValue = result.to_stardust_error()?;
    let network_hrp = match js.as_string() {
      Some(hrp) => hrp,
      None => js.into_serde().map_err(|err| {
        identity_stardust::Error::JsError(format!("get_network_hrp failed to deserialize String: {}", err))
      })?,
    };
    Ok(network_hrp)
  }

  async fn get_alias_output(&self, id: AliasId) -> Result<(OutputId, AliasOutput), identity_stardust::Error> {
    let promise: Promise = Promise::resolve(&WasmStardustIdentityClient::get_alias_output(self, id.to_string()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let tuple: js_sys::Array = js_sys::Array::from(&result.to_stardust_error()?);
    let mut iter: js_sys::ArrayIter = tuple.iter();

    let output_id: OutputId = iter
      .next()
      .ok_or_else(|| identity_stardust::Error::JsError("get_alias_output expected a tuple of size 2".to_owned()))?
      .into_serde()
      .map_err(|err| {
        identity_stardust::Error::JsError(format!("get_alias_output failed to deserialize OutputId: {}", err))
      })?;
    let alias_output: AliasOutput = iter
      .next()
      .ok_or_else(|| identity_stardust::Error::JsError("get_alias_output expected a tuple of size 2".to_owned()))?
      .into_serde()
      .map_err(|err| {
        identity_stardust::Error::JsError(format!("get_alias_output failed to deserialize AliasOutput: {}", err))
      })?;
    Ok((output_id, alias_output))
  }

  async fn get_rent_structure(&self) -> Result<RentStructure, identity_stardust::Error> {
    let promise: Promise = Promise::resolve(&WasmStardustIdentityClient::get_network_hrp(self));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let rent_structure: RentStructure = result.to_stardust_error()?.into_serde().map_err(|err| {
      identity_stardust::Error::JsError(format!("get_rent_structure failed to deserialize String: {}", err))
    })?;
    Ok(rent_structure)
  }
}

#[wasm_bindgen(typescript_custom_section)]
const I_STARDUST_IDENTITY_CLIENT: &'static str = r#"
import { IAliasOutput, IRent } from '@iota/iota.js';
/** Helper functions necessary for the `StardustIdentityClientExt` trait. */
interface IStardustIdentityClient {
  /**
   * Return the Bech32 human-readable part (HRP) of the network.
   *
   * E.g. "iota", "atoi", "smr", "rms".
   */
  getNetworkHrp(): Promise<string>;

  /** Resolve an Alias identifier, returning its latest `OutputId` and `AliasOutput`. */
  getAliasOutput(aliasId: string): Promise<[string, IAliasOutput]>;

  /** Return the rent structure of the network, indicating the byte costs for outputs. */
  getRentStructure(): Promise<IRent>;
}"#;
