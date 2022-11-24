// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;

use identity_iota::iota::block::output::dto::AliasOutputDto;
use identity_iota::iota::block::output::AliasId;
use identity_iota::iota::block::output::AliasOutput;
use identity_iota::iota::block::output::OutputId;
use identity_iota::iota::IotaIdentityClient;
use identity_iota::iota::IotaIdentityClientExt;
use iota_types::block::protocol::dto::ProtocolParametersDto;
use iota_types::block::protocol::ProtocolParameters;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::error::JsValueResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IIotaIdentityClient")]
  pub type WasmIotaIdentityClient;

  #[allow(non_snake_case)]
  #[wasm_bindgen(method, js_name = getAliasOutput)]
  pub fn get_alias_output(this: &WasmIotaIdentityClient, aliasId: String) -> JsValue;

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

    let alias_output = AliasOutput::try_from_dto(
      &alias_dto,
      <Self as IotaIdentityClientExt>::get_token_supply(self).await?,
    )
    .map_err(|err| {
      identity_iota::iota::Error::JsError(format!("get_alias_output failed to convert AliasOutputDto: {}", err))
    })?;
    Ok((output_id, alias_output))
  }

  async fn get_protocol_parameters(&self) -> Result<ProtocolParameters, identity_iota::iota::Error> {
    let promise: Promise = Promise::resolve(&WasmIotaIdentityClient::get_protocol_parameters(self));
    let result: JsValueResult = JsFuture::from(promise).await.into();
    let protocol_parameters: ProtocolParametersDto = result.to_iota_core_error().and_then(|parameters| {
      parameters
        .into_serde()
        .map_err(|err| identity_iota::iota::Error::JsError(format!("could not obtain protocol parameters: {}", err)))
    })?;
    ProtocolParameters::try_from(protocol_parameters)
      .map_err(|err| identity_iota::iota::Error::JsError(format!("could not obtain protocol parameters: {}", err)))
  }
}

#[wasm_bindgen(typescript_custom_section)]
const I_IOTA_IDENTITY_CLIENT: &'static str = r#"
import type { IAliasOutput, IRent } from '@iota/types';
/** Helper interface necessary for `IotaIdentityClientExt`. */
interface IIotaIdentityClient {

  /** Resolve an Alias identifier, returning its latest `OutputId` and `AliasOutput`. */
  getAliasOutput(aliasId: string): Promise<[string, IAliasOutput]>;

  /** Returns the protocol parameters. */
  getProtocolParameters(): Promise<INodeInfoProtocol>; 
}"#;
