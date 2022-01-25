// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::account_builder::WasmAutoSave;
use crate::common::WasmTimestamp;
use crate::credential::{WasmCredential, WasmPresentation};
use crate::crypto::{WasmProofPurpose, WasmSignatureOptions};
use crate::did::{PromiseResolvedDocument, WasmDID, WasmDocument, WasmResolvedDocument};
use crate::error::{Result, WasmResult};
use crate::tangle::Client;
use identity::account::{Account, AccountBuilder, AccountStorage};
use identity::core::{Timestamp, ToJson};
use identity::credential::{Credential, Presentation};
use identity::crypto::{ProofPurpose, SetSignature, Signature, SignatureOptions, TrySignature, TrySignatureMut};
use identity::did::verifiable::VerifiableProperties;
use identity::iota::IotaDocument;
use js_sys::Promise;
use serde::ser::Error;
use serde::{Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen(js_name = Account)]
pub struct WasmAccount(pub(crate) Rc<WasmRefCell<Account>>);

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  //ToDo remove test method
  #[wasm_bindgen(js_name = testAccount)]
  pub fn test_account(&self) -> String {
    return String::from("test success");
  }

  #[wasm_bindgen(js_name = did)]
  pub fn did(&self) -> WasmDID {
    let x = self.0.as_ref().borrow();
    WasmDID::from(x.document().id().clone())
  }

  #[wasm_bindgen]
  pub fn autopublish(&self) -> bool {
    self.0.as_ref().borrow().autopublish()
  }

  #[wasm_bindgen]
  pub fn autosave(&self) -> WasmAutoSave {
    unimplemented!() //ToDo
  }

  #[wasm_bindgen]
  pub fn actions(&self) -> usize {
    self.0.as_ref().borrow().actions()
  }

  pub fn set_client(&self, _client: Client) {
    todo!();
  }

  pub fn state(&self) {
    unimplemented!() //ToDo
  }

  #[wasm_bindgen]
  pub fn document(&self) -> WasmDocument {
    let document: IotaDocument = self.0.as_ref().borrow().document().clone();
    WasmDocument::from(document)
  }

  #[wasm_bindgen(js_name = resolveIdentity)]
  pub fn resolve_identity(&self) -> PromiseResolvedDocument {
    let account = self.0.clone();

    let promise: Promise = future_to_promise(async move {
      account
        .as_ref()
        .borrow()
        .resolve_identity()
        .await
        .map(WasmResolvedDocument::from)
        .map(Into::into)
        .wasm_result()
    });
    promise.unchecked_into::<PromiseResolvedDocument>()
  }

  #[wasm_bindgen(js_name = deleteIdentity)]
  pub fn delete_identity(self) -> Promise {
    let account = self.0.clone();
    let did = account.as_ref().borrow().did().to_owned();
    let storage = account.as_ref().borrow().storage_arc();
    std::mem::drop(account);

    let promise: Promise = future_to_promise(async move {
      let account = AccountBuilder::new()
        .storage(AccountStorage::Custom(storage))
        .load_identity(did)
        .await
        .wasm_result();

      match account {
        Ok(a) => a.delete_identity().await.wasm_result().map(|_| JsValue::undefined()),
        Err(e) => Err(e),
      }
    });
    promise
  }

  #[wasm_bindgen]
  pub fn publish(&mut self) -> Promise {
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .publish()
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
  }

  #[wasm_bindgen(js_name = createSignedCredential)]
  pub fn create_signed_credential(
    &self,
    fragment: String,
    credential: &WasmCredential,
    signature_options: WasmSignatureOptions,
  ) -> PromiseCredential {
    let account = self.0.clone();
    let mut cred: Credential = credential.0.clone();
    let options: SignatureOptions = SignatureOptions::from(signature_options);

    let promise: Promise = future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut cred, options)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()?;
      JsValue::from_serde(&cred).wasm_result()
    });
    promise.unchecked_into::<PromiseCredential>()
  }

  #[wasm_bindgen(js_name = createSignedDocument)]
  pub fn create_signed_document(
    &self,
    fragment: String,
    document: &WasmDocument,
    signature_options: WasmSignatureOptions,
  ) -> PromiseDocument {
    let account = self.0.clone();
    let mut doc: IotaDocument = document.0.clone();
    let options: SignatureOptions = SignatureOptions::from(signature_options);

    let promise: Promise = future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut doc, options)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()?;
      JsValue::from_serde(&doc).wasm_result()
    });
    promise.unchecked_into::<PromiseDocument>()
  }

  #[wasm_bindgen(js_name = createSignedPresentation)]
  pub fn create_signed_presentation(
    &self,
    fragment: String,
    presentation: &WasmPresentation,
    signature_options: WasmSignatureOptions,
  ) -> PromisePresentation {
    let account = self.0.clone();
    let mut pres: Presentation = presentation.0.clone();
    let options: SignatureOptions = SignatureOptions::from(signature_options);

    wasm_logger::init(wasm_logger::Config::default());
    log::info!("{:?}", options);

    let promise: Promise = future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut pres, options)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()?;
      JsValue::from_serde(&pres).wasm_result()
    });
    promise.unchecked_into::<PromisePresentation>()
  }

  #[wasm_bindgen(js_name = createSignedData)]
  pub fn create_signed_data(
    &self,
    fragment: String,
    data: &JsValue,
    signature_options: WasmSignatureOptions,
  ) -> Result<Promise> {
    let account = self.0.clone();
    let mut verifiable_properties: VerifiableProperties = data.into_serde().wasm_result()?;
    let options: SignatureOptions = SignatureOptions::from(signature_options);

    let promise = future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut verifiable_properties, options)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()?;
      JsValue::from_serde(&verifiable_properties).wasm_result()
    });
    Ok(promise)
  }

  #[wasm_bindgen(js_name = updateDocumentUnchecked)]
  pub fn update_document_unchecked(&mut self, document: WasmDocument) {
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .update_document_unchecked(document.0)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    });
  }
}

impl From<Account> for WasmAccount {
  fn from(account: Account) -> WasmAccount {
    WasmAccount(Rc::new(WasmRefCell::new(account)))
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Credential>")]
  pub type PromiseCredential;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Presentation>")]
  pub type PromisePresentation;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Document>")]
  pub type PromiseDocument;
}