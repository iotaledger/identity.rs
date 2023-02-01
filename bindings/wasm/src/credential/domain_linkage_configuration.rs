// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::Credential;
use identity_iota::credential::DomainLinkageConfiguration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::credential::WasmCredential;
use crate::error::Result;
use crate::credential::ArrayCredential;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = DomainLinkageConfiguration, inspectable)]
pub struct WasmDomainLinkageConfiguration(DomainLinkageConfiguration);


#[wasm_bindgen(js_class = DomainLinkageConfiguration)]
impl WasmDomainLinkageConfiguration {
  /// Constructs a new `DomainLinkageConfiguration`.
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<WasmDomainLinkageConfiguration> {

    // let controllers: Vec<WasmCredential> = credentials.into_serde().wasm_result()?;
    // let controllers: Vec<Credential> = controllers.iter().map(|credential| credential.0).collect();
    Ok(Self(DomainLinkageConfiguration::new(vec!())))
  }
}

impl_wasm_json!(WasmDomainLinkageConfiguration, DomainLinkageConfiguration);
impl_wasm_clone!(WasmDomainLinkageConfiguration, DomainLinkageConfiguration);