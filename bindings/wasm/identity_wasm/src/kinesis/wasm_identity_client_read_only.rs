// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;
use std::str::FromStr;

use identity_iota::iota_interaction::types::base_types::ObjectID;
use identity_iota::iota::rebased::client::IdentityClientReadOnly;
use identity_iota::iota::rebased::migration::Identity;
use iota_interaction_ts::bindings::WasmIotaClient;
use wasm_bindgen::prelude::*;

use super::types::WasmObjectID;
use super::WasmOnChainIdentity;
use crate::iota::IotaDocumentLock;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocument;

#[wasm_bindgen(js_name = Identity)]
pub struct IdentityContainer(pub(crate) Identity);
#[wasm_bindgen(js_class = Identity)]
impl IdentityContainer {
  /// TODO: check if we can actually do this like this w/o consuming the container on the 1st try
  /// TODO: add support for unmigrated aliases
  #[wasm_bindgen(js_name = toFullFledged)]
  pub fn to_full_fledged(&self) -> Option<WasmOnChainIdentity> {
    match self.0.clone() {
      Identity::FullFledged(v) => Some(WasmOnChainIdentity(v)),
      _ => None,
    }
  }

  // #[wasm_bindgen(js_name = toLegacy)]
  // pub fn to_legacy(self) -> Option<UnmigratedAlias> {
  //   match self.0 {
  //     Identity::Legacy (v) => Some(v),
  //     _ => None,
  //   }
  // }
}

#[wasm_bindgen(js_name = KinesisIdentityClientReadOnly)]
pub struct WasmKinesisIdentityClientReadOnly(pub(crate) IdentityClientReadOnly);

// builder related functions
#[wasm_bindgen(js_class = KinesisIdentityClientReadOnly)]
impl WasmKinesisIdentityClientReadOnly {
  #[wasm_bindgen(js_name = create)]
  pub async fn new(iota_client: WasmIotaClient) -> Result<WasmKinesisIdentityClientReadOnly, JsError> {
    let inner_client = IdentityClientReadOnly::new(iota_client).await?;
    Ok(WasmKinesisIdentityClientReadOnly(inner_client))
  }

  #[wasm_bindgen(js_name = createWithPkgId)]
  pub async fn new_new_with_pkg_id(iota_client: WasmIotaClient, iota_identity_pkg_id: String) -> Result<WasmKinesisIdentityClientReadOnly, JsError> {
    let inner_client = IdentityClientReadOnly::new_with_pkg_id(iota_client, ObjectID::from_str(&iota_identity_pkg_id)?).await?;
    Ok(WasmKinesisIdentityClientReadOnly(inner_client))
  }

  #[wasm_bindgen(js_name = packageId)]
  pub fn package_id(&self) -> Result<String, JsError> {
    Ok(self.0.package_id().to_string())
  }

  #[wasm_bindgen]
  pub fn network(&self) -> String {
    self.0.network().to_string()
  }

  #[wasm_bindgen(js_name = migrationRegistryId)]
  pub fn migration_registry_id(&self) -> String {
    self.0.migration_registry_id().to_string()
  }

  // TODO: implement later on
  // <

  // pub async fn get_object_by_id<T>(&self, id: ObjectID) -> Result<T, Error> where T: DeserializeOwned {}

  // pub async fn get_object_ref_by_id(&self, obj: ObjectID) -> Result<Option<OwnedObjectRef>, Error> {}

  // pub async fn find_owned_ref_for_address<P>(
  //   &self,
  //   address: IotaAddress,
  //   tag: StructTag,
  //   predicate: P,
  // ) -> Result<Option<ObjectRef>, Error> {}

  // >

  #[wasm_bindgen(js_name = resolveDid)]
  pub async fn resolve_did(&self, did: &WasmIotaDID) -> Result<WasmIotaDocument, JsError> {
    let document = self
      .0
      .resolve_did(&did.0)
      .await
      .map_err(<identity_iota::iota::rebased::Error as std::convert::Into<JsError>>::into)?;
    Ok(WasmIotaDocument(Rc::new(IotaDocumentLock::new(document))))
  }

  #[wasm_bindgen(js_name = getIdentity)]
  pub async fn get_identity(&self, object_id: WasmObjectID) -> Result<IdentityContainer, JsError> {
    let inner_value = self.0.get_identity(object_id.parse()?)
      .await
      .map_err(|err| JsError::new(&format!("failed to resolve identity by object id; {err:?}")))?;
    Ok(IdentityContainer(inner_value))
  }
}
