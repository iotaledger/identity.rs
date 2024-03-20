use crate::common::ImportedDocumentLock;
use crate::credential::WasmDecodedJptCredential;
use crate::credential::WasmFailFast;
use crate::credential::WasmJpt;
use crate::credential::WasmJptCredentialValidationOptions;
use crate::did::WasmCoreDocument;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::credential::JptCredentialValidator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JptCredentialValidator)]
pub struct WasmJptCredentialValidator;

#[wasm_bindgen(js_class = JptCredentialValidator)]
impl WasmJptCredentialValidator {
  #[wasm_bindgen]
  pub fn validate(
    credential_jpt: &WasmJpt,
    issuer: WasmCoreDocument,
    options: &WasmJptCredentialValidationOptions,
    fail_fast: WasmFailFast,
  ) -> Result<WasmDecodedJptCredential> {
    let issuer_doc = ImportedDocumentLock::Core(issuer.0);
    let doc = issuer_doc.try_read()?;
    JptCredentialValidator::validate(&credential_jpt.0, &doc, &options.0, fail_fast.into())
      .wasm_result()
      .map(WasmDecodedJptCredential)
  }
}
