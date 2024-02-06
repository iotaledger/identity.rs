// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota::block::address::dto::AddressDto;
use identity_iota::iota::block::address::Address;
use identity_iota::iota::block::output::dto::AliasOutputDto;
use identity_iota::iota::block::output::AliasOutput;
use identity_iota::iota::block::output::RentStructure;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::identity_client::WasmIotaIdentityClient;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;

// `IAliasOutput`, `AddressTypes`, and `IRent` are external interfaces.
// See the custom TypeScript section in `identity_client.rs` for the first import statement.
#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_IMPORTS: &'static str =
  r#"import type { AliasOutputBuilderParams, Address, IRent } from '~sdk-wasm';"#;
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<AliasOutputBuilderParams>")]
  pub type PromiseAliasOutputBuilderParams;

  #[wasm_bindgen(typescript_type = "Promise<IotaDocument>")]
  pub type PromiseIotaDocument;

  #[wasm_bindgen(typescript_type = "Address")]
  pub type WasmAddress;

  #[wasm_bindgen(typescript_type = "AliasOutputBuilderParams")]
  pub type WasmAliasOutput;

  #[wasm_bindgen(typescript_type = "IRent")]
  pub type IRent;
}

/// An extension interface that provides helper functions for publication
/// and resolution of DID documents in Alias Outputs.
#[wasm_bindgen(js_name = IotaIdentityClientExt)]
pub struct WasmIotaIdentityClientExt;

#[wasm_bindgen(js_class = IotaIdentityClientExt)]
impl WasmIotaIdentityClientExt {
  /// Create a DID with a new Alias Output containing the given `document`.
  ///
  /// The `address` will be set as the state controller and governor unlock conditions.
  /// The minimum required token deposit amount will be set according to the given
  /// `rent_structure`, which will be fetched from the node if not provided.
  /// The returned Alias Output can be further customised before publication, if desired.
  ///
  /// NOTE: this does *not* publish the Alias Output.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = newDidOutput)]
  pub fn new_did_output(
    client: WasmIotaIdentityClient,
    address: WasmAddress,
    document: &WasmIotaDocument,
    rentStructure: Option<IRent>,
  ) -> Result<PromiseAliasOutputBuilderParams> {
    let address_dto: AddressDto = address.into_serde().wasm_result()?;
    let address: Address = Address::try_from(address_dto.clone())
      .map_err(|err| {
        identity_iota::iota::Error::JsError(format!("newDidOutput failed to decode Address: {err}: {address_dto:?}"))
      })
      .wasm_result()?;
    let doc: IotaDocument = document.0.try_read()?.clone();

    let promise: Promise = future_to_promise(async move {
      let rent_structure: Option<RentStructure> = rentStructure
        .map(|rent| rent.into_serde::<RentStructure>())
        .transpose()
        .wasm_result()?;

      let output: AliasOutput = IotaIdentityClientExt::new_did_output(&client, address, doc, rent_structure)
        .await
        .wasm_result()?;
      // Use DTO for correct serialization.
      let dto: AliasOutputDto = AliasOutputDto::from(&output);
      JsValue::from_serde(&dto).wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseAliasOutputBuilderParams>())
  }

  /// Fetches the associated Alias Output and updates it with `document` in its state metadata.
  /// The storage deposit on the output is left unchanged. If the size of the document increased,
  /// the amount should be increased manually.
  ///
  /// NOTE: this does *not* publish the updated Alias Output.
  #[wasm_bindgen(js_name = updateDidOutput)]
  pub fn update_did_output(
    client: WasmIotaIdentityClient,
    document: &WasmIotaDocument,
  ) -> Result<PromiseAliasOutputBuilderParams> {
    let document: IotaDocument = document.0.try_read()?.clone();
    let promise: Promise = future_to_promise(async move {
      let output: AliasOutput = IotaIdentityClientExt::update_did_output(&client, document)
        .await
        .wasm_result()?;
      // Use DTO for correct serialization.
      let dto: AliasOutputDto = AliasOutputDto::from(&output);
      JsValue::from_serde(&dto).wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseAliasOutputBuilderParams>())
  }

  /// Removes the DID document from the state metadata of its Alias Output,
  /// effectively deactivating it. The storage deposit on the output is left unchanged,
  /// and should be reallocated manually.
  ///
  /// Deactivating does not destroy the output. Hence, it can be re-activated by publishing
  /// an update containing a DID document.
  ///
  /// NOTE: this does *not* publish the updated Alias Output.
  #[wasm_bindgen(js_name = deactivateDidOutput)]
  pub fn deactivate_did_output(
    client: WasmIotaIdentityClient,
    did: &WasmIotaDID,
  ) -> Result<PromiseAliasOutputBuilderParams> {
    let did: IotaDID = did.0.clone();
    let promise: Promise = future_to_promise(async move {
      let output: AliasOutput = IotaIdentityClientExt::deactivate_did_output(&client, &did)
        .await
        .wasm_result()?;
      // Use DTO for correct serialization.
      let dto: AliasOutputDto = AliasOutputDto::from(&output);
      JsValue::from_serde(&dto).wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseAliasOutputBuilderParams>())
  }

  /// Resolve a {@link IotaDocument}. Returns an empty, deactivated document if the state metadata
  /// of the Alias Output is empty.
  #[wasm_bindgen(js_name = resolveDid)]
  pub fn resolve_did(client: WasmIotaIdentityClient, did: &WasmIotaDID) -> Result<PromiseIotaDocument> {
    let did: IotaDID = did.0.clone();
    let promise: Promise = future_to_promise(async move {
      IotaIdentityClientExt::resolve_did(&client, &did)
        .await
        .map(WasmIotaDocument::from)
        .map(Into::into)
        .wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseIotaDocument>())
  }

  /// Fetches the `IAliasOutput` associated with the given DID.
  #[wasm_bindgen(js_name = resolveDidOutput)]
  pub fn resolve_did_output(
    client: WasmIotaIdentityClient,
    did: &WasmIotaDID,
  ) -> Result<PromiseAliasOutputBuilderParams> {
    let did: IotaDID = did.0.clone();
    let promise: Promise = future_to_promise(async move {
      let output: AliasOutput = IotaIdentityClientExt::resolve_did_output(&client, &did)
        .await
        .wasm_result()?;
      // Use DTO for correct serialization.
      let dto: AliasOutputDto = AliasOutputDto::from(&output);
      JsValue::from_serde(&dto).wasm_result()
    });

    // WARNING: this does not validate the return type. Check carefully.
    Ok(promise.unchecked_into::<PromiseAliasOutputBuilderParams>())
  }
}
