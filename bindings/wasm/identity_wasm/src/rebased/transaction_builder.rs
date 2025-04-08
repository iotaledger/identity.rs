// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// use identity_iota::iota::rebased::client::PublishDidDocument;
// use wasm_bindgen::prelude::wasm_bindgen;

// macro_rules! impl_wasm_transaction_builder {
//   ($tx:ident, $wasm_tx:ident, $read_only_literal:literal, $build_triple_literal:literal) => {
//     mod __tx_builder_impl {
//       use super::$tx;
//       use super::$wasm_tx;
//       use crate::error::Result;
//       use crate::error::WasmResult;
//       use crate::rebased::WasmIdentityClient;
//       use crate::rebased::WasmIdentityClientReadOnly;
//       use identity_iota::iota::rebased::transaction_builder::MutGasDataRef;
//       use identity_iota::iota::rebased::transaction_builder::TransactionBuilder;
//       use identity_iota::iota_interaction::types::crypto::Signature;
//       use identity_iota::iota_interaction::types::transaction::TransactionData;
//       use identity_iota::iota_interaction::types::transaction::TransactionDataAPI as _;
//       use iota_interaction_ts::bindings::WasmIotaSignature;
//       use iota_interaction_ts::bindings::WasmTransactionData;
//       use wasm_bindgen::prelude::wasm_bindgen;
//       use wasm_bindgen::JsCast;
//       use wasm_bindgen::JsValue;
//       use wasm_bindgen_futures::JsFuture;

//     }
//     pub use __tx_builder_impl::*;
//   };
// }

// #[wasm_bindgen(js_name = PublishDidDocument)]
// pub struct WasmPublishDidDocument(pub(crate) PublishDidDocument);

// impl_wasm_transaction_builder!(
//   PublishDidDocument,
//   WasmPublishDidDocument,
//   "Readonly<PublishDidDocument>",
//   "[TransactionData, Signature[], PublishDidDocument]"
// );
use std::result::Result as StdResult;

use anyhow::anyhow;
use anyhow::Context as _;
use async_trait::async_trait;
use fastcrypto::traits::EncodeDecodeBase64;
use identity_iota::iota::rebased::client::IdentityClientReadOnly;
use identity_iota::iota::rebased::transaction::TransactionOutputInternal;
use identity_iota::iota::rebased::transaction_builder::MutGasDataRef;
use identity_iota::iota::rebased::transaction_builder::Transaction;
use identity_iota::iota::rebased::transaction_builder::TransactionBuilder;
use identity_iota::iota::rebased::Error as IotaError;
use identity_iota::iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota::iota_interaction::types::crypto::Signature;
use identity_iota::iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota::iota_interaction::types::transaction::TransactionData;
use identity_iota::iota_interaction::types::transaction::TransactionDataAPI as _;
use iota_interaction_ts::bindings::WasmIotaTransactionBlockEffects;
use iota_interaction_ts::bindings::WasmIotaTransactionBlockResponse;
use iota_interaction_ts::bindings::WasmObjectRef;
use iota_interaction_ts::bindings::WasmTransactionDataBuilder;
use js_sys::JsString;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast as _;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use super::WasmIdentityClient;
use super::WasmIdentityClientReadOnly;
use crate::error::Result;
use crate::error::WasmResult as _;

#[wasm_bindgen]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = "Transaction<unknown>")]
  pub type WasmTransaction;

  #[wasm_bindgen(method, catch, js_name = buildProgrammableTransaction)]
  pub async fn build_programmable_transaction(
    this: &WasmTransaction,
    client: WasmIdentityClientReadOnly,
  ) -> Result<js_sys::Uint8Array>;
  #[wasm_bindgen(method, catch)]
  pub async fn apply(
    this: &WasmTransaction,
    effects: WasmIotaTransactionBlockEffects,
    client: WasmIdentityClientReadOnly,
  ) -> Result<JsValue>;
}

#[async_trait(?Send)]
impl Transaction for WasmTransaction {
  type Output = JsValue;
  async fn build_programmable_transaction(
    &self,
    client: &IdentityClientReadOnly,
  ) -> StdResult<ProgrammableTransaction, IotaError> {
    let client = WasmIdentityClientReadOnly(client.clone());
    let pt_bcs = Self::build_programmable_transaction(&self, client)
      .await
      .map_err(|e| IotaError::FfiError(format!("{e:?}")))?
      .to_vec();
    Ok(bcs::from_bytes(&pt_bcs)?)
  }
  async fn apply(
    self,
    effects: &IotaTransactionBlockEffects,
    client: &IdentityClientReadOnly,
  ) -> StdResult<Self::Output, IotaError> {
    let client = WasmIdentityClientReadOnly(client.clone());
    let effects = effects.into();

    Self::apply(&self, effects, client)
      .await
      .map_err(|e| IotaError::FfiError(format!("failed to apply effects from WASM Transaction: {e:?}")))
  }
}

#[wasm_bindgen(js_name = TransactionBuilder, skip_typescript)]
pub struct WasmTransactionBuilder(pub(crate) TransactionBuilder<WasmTransaction>);

#[wasm_bindgen(js_class = TransactionBuilder)]
impl WasmTransactionBuilder {
  #[wasm_bindgen(constructor)]
  pub fn new(tx: WasmTransaction) -> Self {
    Self(TransactionBuilder::new(tx))
  }

  #[wasm_bindgen(getter)]
  pub fn transaction(&self) -> WasmTransaction {
    self.0.as_ref().clone()
  }

  #[wasm_bindgen(js_name = withGasPrice)]
  pub fn with_gas_price(mut self, price: u64) -> Self {
    self.0 = self.0.with_gas_price(price);
    self
  }

  #[wasm_bindgen(js_name = withGasBudget)]
  pub fn with_gas_budget(mut self, budget: u64) -> Self {
    self.0 = self.0.with_gas_budget(budget);
    self
  }

  #[wasm_bindgen(js_name = withGasOwner)]
  pub fn with_gas_owner(mut self, owner: &str) -> Result<Self> {
    let owner = owner.parse().wasm_result()?;
    self.0 = self.0.with_gas_owner(owner);
    Ok(self)
  }

  #[wasm_bindgen(js_name = withGasPayment)]
  pub fn with_gas_payment(mut self, payment: Vec<WasmObjectRef>) -> Result<Self> {
    let payment = payment
      .into_iter()
      .map(TryInto::try_into)
      .collect::<anyhow::Result<Vec<_>>>()
      .wasm_result()?;

    self.0 = self.0.with_gas_payment(payment);
    Ok(self)
  }

  #[wasm_bindgen(js_name = withSender)]
  pub fn with_sender(mut self, sender: &str) -> Result<Self> {
    let sender = sender
      .parse()
      .map_err(|e| anyhow!("failed to parse IotaAddress: {e}"))
      .wasm_result()?;
    self.0 = self.0.with_sender(sender);
    Ok(self)
  }

  #[wasm_bindgen(js_name = withSignature)]
  pub async fn with_signature(mut self, client: &WasmIdentityClient) -> Result<Self> {
    self.0 = self.0.with_signature(client).await.wasm_result()?;
    Ok(self)
  }

  #[wasm_bindgen(js_name = withSponsor)]
  pub async fn with_sponsor(
    mut self,
    client: &WasmIdentityClientReadOnly,
    #[wasm_bindgen(unchecked_param_type = "SponsorFn")] sponsor_fn: &js_sys::Function,
  ) -> Result<Self> {
    let closure = async |mut tx_data_ref: MutGasDataRef<'_>| -> anyhow::Result<Signature> {
      let tx_data_bcs = bcs::to_bytes(&*tx_data_ref)?;
      let wasm_tx = WasmTransactionDataBuilder::from_bcs_bytes(js_sys::Uint8Array::from(tx_data_bcs.as_slice()))
        .map_err(|_| anyhow!("failed to convert TransactionData into JS IotaTransaction"))?;
      let promise: js_sys::Promise = sponsor_fn
        .call1(&JsValue::NULL, &wasm_tx)
        .and_then(|value| value.dyn_into())
        .map_err(|_| anyhow!("failed to call JS closure"))?;
      let sig_str: JsString = JsFuture::from(promise)
        .await
        .and_then(|value| value.dyn_into())
        .map_err(|_| anyhow!("failed to build a Future from a JS Promise"))?;

      let modified_tx_data_bcs = wasm_tx
        .build()
        .map_err(|_| anyhow!("failed to build JS TransactionDataBuilder"))?
        .to_vec();
      let tx_data = bcs::from_bytes::<TransactionData>(&modified_tx_data_bcs)?;

      *tx_data_ref.gas_data_mut() = tx_data.gas_data().clone();
      let signature = Signature::decode_base64(&String::from(sig_str)).context("failed to decode b64 signature")?;

      Ok(signature)
    };

    self.0 = self.0.with_sponsor(&client.0, closure).await.wasm_result()?;
    Ok(self)
  }

  #[wasm_bindgen(unchecked_return_type = "[Uint8Array, string[], Transaction]")]
  pub async fn build(self, client: &WasmIdentityClient) -> Result<JsValue> {
    let (tx_data, signatures, inner_tx) = self.0.build(&client.0).await.wasm_result()?;
    let tx_data_bcs = bcs::to_bytes(&tx_data)
      .wasm_result()
      .map(|bcs_bytes| js_sys::Uint8Array::from(bcs_bytes.as_slice()))?;
    let wasm_signatures = {
      let wasm_signatures = js_sys::Array::new();
      for sig in signatures {
        let b64_sig = sig.encode_base64();
        wasm_signatures.push(&JsValue::from_str(&b64_sig));
      }

      wasm_signatures
    };

    let wasm_triple = js_sys::Array::new();
    wasm_triple.push(&tx_data_bcs);
    wasm_triple.push(wasm_signatures.as_ref());
    wasm_triple.push(inner_tx.as_ref());

    Ok(wasm_triple.into())
  }

  #[wasm_bindgen(js_name = buildAndExecute, unchecked_return_type = "TransactionOutput<unknown>")]
  pub async fn build_and_execute(self, client: &WasmIdentityClient) -> Result<WasmTransactionOutput> {
    self.0.build_and_execute(&client.0).await.wasm_result().map(Into::into)
  }
}

#[wasm_bindgen(
  js_name = TransactionOutput,
  skip_typescript,
  inspectable,
  getter_with_clone
)]
pub struct WasmTransactionOutput {
  pub output: JsValue,
  pub response: WasmIotaTransactionBlockResponse,
}

impl From<TransactionOutputInternal<JsValue>> for WasmTransactionOutput {
  fn from(value: TransactionOutputInternal<JsValue>) -> Self {
    Self {
      output: value.output,
      response: value.response.clone_native_response().response(),
    }
  }
}
