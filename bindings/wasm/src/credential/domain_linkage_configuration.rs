// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::ArrayString;
use crate::credential::ArrayCredential;
use crate::credential::WasmCredential;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::credential::Credential;
use identity_iota::credential::DomainLinkageConfiguration;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// DID Configuration Resource which contains Domain Linkage Credentials.
/// It can be placed in an origin's `.well-known` directory to prove linkage between the origin and a DID.
/// See: <https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource>
///
/// Note:
/// - Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
///   is supported.
#[wasm_bindgen(js_name = DomainLinkageConfiguration, inspectable)]
pub struct WasmDomainLinkageConfiguration(DomainLinkageConfiguration);

#[wasm_bindgen(js_class = DomainLinkageConfiguration)]
impl WasmDomainLinkageConfiguration {
  /// Constructs a new `DomainLinkageConfiguration`.
  #[wasm_bindgen(constructor)]
  pub fn new(linked_dids: ArrayCredential) -> Result<WasmDomainLinkageConfiguration> {
    let wasm_credentials: Vec<Credential> = linked_dids.into_serde().wasm_result()?;
    Ok(Self(DomainLinkageConfiguration::new(wasm_credentials)))
  }

  /// List of the Domain Linkage Credentials.
  #[wasm_bindgen(js_name = linkedDids)]
  pub fn linked_dids(&self) -> ArrayCredential {
    self
      .0
      .linked_dids()
      .iter()
      .cloned()
      .map(WasmCredential::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCredential>()
  }

  /// List of the issuers of the Domain Linkage Credentials.
  #[wasm_bindgen]
  pub fn issuers(&self) -> ArrayString {
    self
      .0
      .issuers()
      .map(|url| url.to_string())
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }
}

impl_wasm_json!(WasmDomainLinkageConfiguration, DomainLinkageConfiguration);
impl_wasm_clone!(WasmDomainLinkageConfiguration, DomainLinkageConfiguration);
