// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::block::output::dto::AliasOutputDto;
use identity_iota::iota::block::output::AliasId;
use identity_iota::iota::block::output::AliasOutput;
use identity_iota::iota::block::output::OutputId;
use identity_iota::iota::IotaIdentityClient;
use iota_client::block::protocol::dto::ProtocolParametersDto; 
use iota_types::block::protocol::ProtocolParameters;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::error::JsValueResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IIotaIdentityClient")]
  pub type WasmIotaIdentityClient;

  /* 
  #[wasm_bindgen(method, js_name = getNetworkHrp)]
  pub fn get_network_hrp(this: &WasmIotaIdentityClient) -> JsValue;
*/

  #[allow(non_snake_case)]
  #[wasm_bindgen(method, js_name = getAliasOutput)]
  pub fn get_alias_output(this: &WasmIotaIdentityClient, aliasId: String) -> JsValue;
  /* 
  #[wasm_bindgen(method, js_name = getRentStructure)]
  pub fn get_rent_structure(this: &WasmIotaIdentityClient) -> JsValue;

  #[wasm_bindgen(method, js_name = getTokenSupply)]
  pub fn get_token_supply(this: &WasmIotaIdentityClient) -> JsValue;
  */
  #[wasm_bindgen(method, js_name = getProtocolParameters)]
  pub fn get_protocol_parameters(this: &WasmIotaIdentityClient) -> JsValue;
}

impl Debug for WasmIotaIdentityClient {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("WasmIotaIdentityClient")
  }
}

#[async_trait::async_trait(?Send)]
impl IotaIdentityClient for WasmIotaIdentityClient {
  /* 
  async fn get_network_hrp(&self) -> Result<String, identity_iota::iota::Error> {
    let promise: Promise = Promise::resolve(&WasmIotaIdentityClient::get_network_hrp(self));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let js: JsValue = result.to_iota_core_error()?;
    let network_hrp = match js.as_string() {
      Some(hrp) => hrp,
      None => js.into_serde().map_err(|err| {
        identity_iota::iota::Error::JsError(format!("get_network_hrp failed to deserialize String: {}", err))
      })?,
    };
    Ok(network_hrp)
  }
  */

  async fn get_alias_output(&self, id: AliasId) -> Result<(OutputId, AliasOutput), identity_iota::iota::Error> {
    let promise: Promise = Promise::resolve(&WasmIotaIdentityClient::get_alias_output(self, id.to_string()));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let tuple: js_sys::Array = js_sys::Array::from(&result.to_iota_core_error()?);

    let mut iter: js_sys::ArrayIter = tuple.iter();

    let output_id: OutputId = iter
      .next()
      .ok_or_else(|| identity_iota::iota::Error::JsError("get_alias_output expected a tuple of size 2".to_owned()))?
      .into_serde()
      .map_err(|err| {
        identity_iota::iota::Error::JsError(format!("get_alias_output failed to deserialize OutputId: {}", err))
      })?;
    let alias_dto: AliasOutputDto = iter
      .next()
      .ok_or_else(|| identity_iota::iota::Error::JsError("get_alias_output expected a tuple of size 2".to_owned()))?
      .into_serde()
      .map_err(|err| {
        identity_iota::iota::Error::JsError(format!(
          "get_alias_output failed to deserialize AliasOutputDto: {}",
          err
        ))
      })?;

    /* 
    let token_supply_promise: Promise = Promise::resolve(&<WasmIotaIdentityClient as IotaIdentityClientExt>::get_token_supply(self));
    let token_supply: u64 = JsValueResult::from(JsFuture::from(token_supply_promise).await)
      .to_iota_core_error()
      .and_then(|value| {
        if let Some(big_int) = value.as_f64() {
          Ok(big_int as u64)
        } else {
          Err(identity_iota::iota::Error::JsError(
            "could not retrieve a token supply of the required type".into(),
          ))
        }
      })?;
      */

    let alias_output = AliasOutput::try_from_dto(&alias_dto, <Self as IotaIdentityClientExt>::get_token_supply(self).await?).map_err(|err| {
      identity_iota::iota::Error::JsError(format!("get_alias_output failed to convert AliasOutputDto: {}", err))
    })?;
    Ok((output_id, alias_output))
  }

  /* 
  async fn get_rent_structure(&self) -> Result<RentStructure, identity_iota::iota::Error> {
    let promise: Promise = Promise::resolve(&WasmIotaIdentityClient::get_rent_structure(self));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let rent_structure: RentStructureBuilder = result.to_iota_core_error()?.into_serde().map_err(|err| {
      identity_iota::iota::Error::JsError(format!("get_rent_structure failed to deserialize: {}", err))
    })?;
    Ok(rent_structure.finish())
  }

  async fn get_token_supply(&self) -> Result<u64, identity_iota::iota::Error> {
    let promise: Promise = Promise::resolve(&WasmIotaIdentityClient::get_token_supply(self));
    Ok(
      JsValueResult::from(JsFuture::from(promise).await)
        .to_iota_core_error()
        .and_then(|value| {
          if let Some(big_int) = value
            .as_string()
            .and_then(|converted_string| converted_string.parse().ok())
          {
            Ok(big_int)
          } else {
            Err(identity_iota::iota::Error::JsError(
              "could not retrieve a token supply of the required type".into(),
            ))
          }
        })?,
    )
  }
  */
  async fn get_protocol_parameters(&self) -> Result<ProtocolParameters, identity_iota::iota::Error> {
    let promise: Promise = Promise::resolve(&WasmIotaIdentityClient::get_protocol_parameters(self));
    let result: JsValueResult = JsFuture::from(promise).await.into(); 
    let protocol_parameters: ProtocolParametersDto = result.to_iota_core_error().and_then(|parameters| parameters.into_serde().map_err(|_|identity_iota::iota::Error::SerializationError("TODO", None)))?; 
    // TODO: Remove this serde flavoured hack once ProtocolParametersDto becomes available in iota_types. 
    let parameters_iota_client = iota_client::block::protocol::ProtocolParameters::try_from(protocol_parameters)
    .map_err(|_| identity_iota::iota::Error::JsError("Could not get protocol parameters".into()))?;
    let protocol_parameters_json: Vec<u8> = parameters_iota_client.to_json_vec().map_err(|_| identity_iota::iota::Error::JsError("Could not serialize protocol parameters".into()))?;
    let parameters_iota_types: ProtocolParameters = ProtocolParameters::from_json_slice(&protocol_parameters_json).map_err(|_| identity_iota::iota::Error::JsError("could not deserialize protocol parameters".into()))?;
    Ok(parameters_iota_types)
  }
}

#[wasm_bindgen(typescript_custom_section)]
const I_IOTA_IDENTITY_CLIENT: &'static str = r#"
import type { IAliasOutput, IRent } from '@iota/types';
/** Helper interface necessary for `IotaIdentityClientExt`. */
interface IIotaIdentityClient {
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

  /** Gets the token supply of the node we're connecting to. */
  getTokenSupply(): Promise<string>;

  /** Returns the protocol parameters. */
  getProtocolParameters(): Promise<INodeInfoProtocol>; 
}"#;
