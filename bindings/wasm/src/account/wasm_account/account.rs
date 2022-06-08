// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use identity::account::Account;
use identity::account::AccountBuilder;
use identity::account::PublishOptions;
use identity::account_storage::CekAlgorithm;
use identity::account_storage::EncryptedData;
use identity::account_storage::EncryptionAlgorithm;
use identity::account_storage::Storage;
use identity::credential::Credential;
use identity::credential::Presentation;
use identity::crypto::ProofOptions;
use identity::crypto::PublicKey;
use identity::did::verifiable::VerifiableProperties;
use identity::iota::Client;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaDocument;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;

use crate::account::types::WasmAutoSave;
use crate::account::types::WasmCekAlgorithm;
use crate::account::types::WasmEncryptedData;
use crate::account::types::WasmEncryptionAlgorithm;
use crate::common::PromiseVoid;
use crate::credential::WasmCredential;
use crate::credential::WasmPresentation;
use crate::crypto::WasmProofOptions;
use crate::did::PromiseResolvedDocument;
use crate::did::WasmDID;
use crate::did::WasmDocument;
use crate::did::WasmResolvedDocument;
use crate::error::Result;
use crate::error::WasmResult;

pub(crate) type AccountRc = Account<Rc<Client>>;

/// An account manages one identity.
///
/// It handles private keys, writing to storage and
/// publishing to the Tangle.
#[wasm_bindgen(js_name = Account)]
pub struct WasmAccount(pub(crate) Rc<RefCell<AccountRc>>);

#[wasm_bindgen(js_class = Account)]
impl WasmAccount {
  /// Returns the {@link DID} of the managed identity.
  #[wasm_bindgen(js_name = did)]
  pub fn did(&self) -> WasmDID {
    WasmDID::from(self.0.borrow().did().clone())
  }

  /// Returns whether auto-publish is enabled.
  #[wasm_bindgen]
  pub fn autopublish(&self) -> bool {
    self.0.borrow().autopublish()
  }

  /// Returns the auto-save configuration value.
  #[wasm_bindgen]
  pub fn autosave(&self) -> WasmAutoSave {
    WasmAutoSave(self.0.borrow().autosave())
  }

  /// Returns a copy of the document managed by the `Account`.
  ///
  /// Note: the returned document only has a valid signature after publishing an integration chain update.
  /// In general, for use cases where the signature is required, it is advisable to resolve the
  /// document from the Tangle.
  #[wasm_bindgen]
  pub fn document(&self) -> WasmDocument {
    let document: IotaDocument = self.0.borrow().document().clone();
    WasmDocument::from(document)
  }

  /// Resolves the DID Document associated with this `Account` from the Tangle.
  #[wasm_bindgen(js_name = resolveIdentity)]
  pub fn resolve_identity(&self) -> PromiseResolvedDocument {
    let account: Rc<RefCell<AccountRc>> = self.0.clone();

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

  /// Removes the identity from the local storage entirely.
  ///
  /// Note: This will remove all associated document updates and key material - recovery is NOT POSSIBLE!
  #[wasm_bindgen(js_name = deleteIdentity)]
  pub fn delete_identity(self) -> PromiseVoid {
    // Get IotaDID and storage from the account.
    let did: IotaDID = self.0.borrow().did().to_owned();
    let storage: Arc<dyn Storage> = Arc::clone(self.0.borrow().storage());

    future_to_promise(async move {
      // Create a new account since `delete_identity` consumes it.
      let account: Result<AccountRc> = AccountBuilder::new()
        .storage_shared(storage)
        .load_identity(did)
        .await
        .wasm_result();

      match account {
        Ok(a) => a.delete_identity().await.wasm_result().map(|_| JsValue::undefined()),
        Err(e) => Err(e),
      }
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Push all unpublished changes to the tangle in a single message.
  #[wasm_bindgen]
  pub fn publish(&mut self, publish_options: Option<WasmPublishOptions>) -> PromiseVoid {
    let options: PublishOptions = publish_options.map(PublishOptions::from).unwrap_or_default();
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .publish_with_options(options)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Signs a {@link Credential} with the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedCredential)]
  pub fn create_signed_credential(
    &self,
    fragment: String,
    credential: &WasmCredential,
    options: &WasmProofOptions,
  ) -> PromiseCredential {
    let account = self.0.clone();
    let options: ProofOptions = options.0.clone();

    let mut credential: Credential = credential.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut credential, options)
        .await
        .wasm_result()?;
      Ok(JsValue::from(WasmCredential::from(credential)))
    })
    .unchecked_into::<PromiseCredential>()
  }

  /// Signs a {@link Document} with the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedDocument)]
  pub fn create_signed_document(
    &self,
    fragment: String,
    document: &WasmDocument,
    options: &WasmProofOptions,
  ) -> PromiseDocument {
    let account = self.0.clone();
    let options: ProofOptions = options.0.clone();

    let mut document: IotaDocument = document.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut document, options)
        .await
        .wasm_result()?;
      Ok(JsValue::from(WasmDocument::from(document)))
    })
    .unchecked_into::<PromiseDocument>()
  }

  /// Signs a {@link Presentation} the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedPresentation)]
  pub fn create_signed_presentation(
    &self,
    fragment: String,
    presentation: &WasmPresentation,
    options: &WasmProofOptions,
  ) -> PromisePresentation {
    let account = self.0.clone();
    let options: ProofOptions = options.0.clone();

    let mut presentation: Presentation = presentation.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .sign(fragment.as_str(), &mut presentation, options)
        .await
        .wasm_result()?;
      Ok(JsValue::from(WasmPresentation::from(presentation)))
    })
    .unchecked_into::<PromisePresentation>()
  }

  /// Signs arbitrary `data` with the key specified by `fragment`.
  #[wasm_bindgen(js_name = createSignedData)]
  pub fn create_signed_data(&self, fragment: String, data: &JsValue, options: &WasmProofOptions) -> Result<Promise> {
    let mut verifiable_properties: VerifiableProperties = data.into_serde().wasm_result()?;
    let account = self.0.clone();
    let options: ProofOptions = options.0.clone();

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

  /// Overwrites the {@link Document} this account manages, **without doing any validation**.
  ///
  /// ### WARNING
  ///
  /// This method is dangerous and can easily corrupt the internal state,
  /// potentially making the identity unusable. Only call this if you fully
  /// understand the implications!
  #[wasm_bindgen(js_name = updateDocumentUnchecked)]
  pub fn update_document_unchecked(&mut self, document: &WasmDocument) -> PromiseVoid {
    let account = self.0.clone();
    let document_copy: IotaDocument = document.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .update_document_unchecked(document_copy)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Fetches the latest changes from the tangle and **overwrites** the local document.
  ///
  /// If a DID is managed from distributed accounts, this should be called before making changes
  /// to the identity, to avoid publishing updates that would be ignored.
  #[wasm_bindgen(js_name = fetchDocument)]
  pub fn fetch_document(&mut self) -> PromiseVoid {
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .fetch_document()
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// If the document has an `EmbeddedRevocationService` identified by `fragment`, revokes all given `credentials`.
  #[wasm_bindgen(js_name = revokeCredentials)]
  pub fn revoke_credentials(&mut self, fragment: String, credentials: Vec<u32>) -> PromiseVoid {
    let account = self.0.clone();
    future_to_promise(async move {
      account
        .as_ref()
        .borrow_mut()
        .revoke_credentials(&fragment, &credentials)
        .await
        .map(|_| JsValue::undefined())
        .wasm_result()
    })
    .unchecked_into::<PromiseVoid>()
  }

  /// Encrypts the given `plaintext` with the specified `encryption_algorithm` and `cek_algorithm`.
  ///
  /// Returns an [`EncryptedData`] instance.
  #[wasm_bindgen(js_name = encryptData)]
  pub fn encrypt_data(
    &self,
    plaintext: Vec<u8>,
    associated_data: Vec<u8>,
    encryption_algorithm: &WasmEncryptionAlgorithm,
    cek_algorithm: &WasmCekAlgorithm,
    public_key: Vec<u8>,
  ) -> PromiseEncryptedData {
    let account = self.0.clone();
    let encryption_algorithm: EncryptionAlgorithm = encryption_algorithm.clone().into();
    let cek_algorithm: CekAlgorithm = cek_algorithm.clone().into();
    let public_key: PublicKey = public_key.to_vec().into();

    future_to_promise(async move {
      let encrypted_data: EncryptedData = account
        .as_ref()
        .borrow()
        .encrypt_data(
          &plaintext,
          &associated_data,
          &encryption_algorithm,
          &cek_algorithm,
          public_key,
        )
        .await
        .wasm_result()?;
      Ok(JsValue::from(WasmEncryptedData::from(encrypted_data)))
    })
    .unchecked_into::<PromiseEncryptedData>()
  }

  /// Decrypts the given `data` with the key identified by `fragment` using the given `encryption_algorithm` and
  /// `cek_algorithm`.
  ///
  /// Returns the decrypted text.
  #[wasm_bindgen(js_name = decryptData)]
  pub fn decrypt_data(
    &self,
    data: &WasmEncryptedData,
    encryption_algorithm: &WasmEncryptionAlgorithm,
    cek_algorithm: &WasmCekAlgorithm,
    fragment: String,
  ) -> PromiseData {
    let account = self.0.clone();
    let data: EncryptedData = data.0.clone();
    let encryption_algorithm: EncryptionAlgorithm = encryption_algorithm.clone().into();
    let cek_algorithm: CekAlgorithm = cek_algorithm.clone().into();

    future_to_promise(async move {
      let data: Vec<u8> = account
        .as_ref()
        .borrow()
        .decrypt_data(data, &encryption_algorithm, &cek_algorithm, &fragment)
        .await
        .wasm_result()?;
      Ok(JsValue::from(js_sys::Uint8Array::from(data.as_ref())))
    })
    .unchecked_into::<PromiseData>()
  }
}

impl From<AccountRc> for WasmAccount {
  fn from(account: AccountRc) -> WasmAccount {
    WasmAccount(Rc::new(RefCell::new(account)))
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Credential>")]
  pub type PromiseCredential;

  #[wasm_bindgen(typescript_type = "Promise<Presentation>")]
  pub type PromisePresentation;

  #[wasm_bindgen(typescript_type = "Promise<Document>")]
  pub type PromiseDocument;

  #[wasm_bindgen(typescript_type = "Promise<EncryptedData>")]
  pub type PromiseEncryptedData;

  #[wasm_bindgen(typescript_type = "Promise<Uint8Array>")]
  pub type PromiseData;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "PublishOptions")]
  pub type WasmPublishOptions;

  #[wasm_bindgen(getter, method)]
  pub fn forceIntegrationUpdate(this: &WasmPublishOptions) -> Option<bool>;

  #[wasm_bindgen(getter, method)]
  pub fn signWith(this: &WasmPublishOptions) -> Option<String>;
}

#[wasm_bindgen(typescript_custom_section)]
const TS_PUBLISH_OPTIONS: &'static str = r#"
/**
 * Options to customize how identities are published to the Tangle.
**/
export type PublishOptions = {
    /**
     * Whether to force the publication to be an integration update.
     * If this option is not set, the account automatically determines whether
     * an update needs to be published as an integration or a diff update.
     * Publishing as an integration update is always valid, but not recommended
     * for identities with many updates.
     *
     * See the IOTA DID method specification for more details.
     *
     * @deprecated since 0.5.0, diff chain features are slated for removal.
     */
     forceIntegrationUpdate?: boolean,


    /**
     * Set the fragment of a verification method with which to sign the update.
     * This must point to an Ed25519 method with a capability invocation
     * verification relationship.
     *
     *  If omitted, the default signing method on the Document will be used.
     */
     signWith?: string
 }
"#;

impl From<WasmPublishOptions> for PublishOptions {
  fn from(publish_options: WasmPublishOptions) -> Self {
    let mut options: PublishOptions = PublishOptions::new();

    if let Some(force_integration) = publish_options.forceIntegrationUpdate() {
      options = options.force_integration_update(force_integration);
    }

    if let Some(sign_with) = publish_options.signWith() {
      options = options.sign_with(sign_with);
    };
    options
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Promise<Account>")]
  pub type PromiseAccount;
}
