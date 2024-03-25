use crate::common::ImportedDocumentLock;
use crate::credential::WasmDecodedJptPresentation;
use crate::credential::WasmFailFast;
use crate::credential::WasmJpt;
use crate::credential::WasmJptPresentationValidationOptions;
use crate::did::WasmCoreDocument;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::credential::JptPresentationValidator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JptPresentationValidator)]
pub struct WasmJptPresentationValidator;

#[wasm_bindgen(js_class = JptPresentationValidator)]
impl WasmJptPresentationValidator {
  /// Decodes and validates a Presented {@link Credential} issued as a JPT (JWP Presented Form). A
  /// {@link DecodedJptPresentation} is returned upon success.
  ///
  /// The following properties are validated according to `options`:
  /// - the holder's proof on the JWP,
  /// - the expiration date,
  /// - the issuance date,
  /// - the semantic structure.
  #[wasm_bindgen]
  pub fn validate(
    presentation_jpt: &WasmJpt,
    issuer: WasmCoreDocument,
    options: &WasmJptPresentationValidationOptions,
    fail_fast: WasmFailFast,
  ) -> Result<WasmDecodedJptPresentation> {
    let issuer_doc = ImportedDocumentLock::Core(issuer.0);
    let doc = issuer_doc.try_read()?;
    JptPresentationValidator::validate(&presentation_jpt.0, &doc, &options.0, fail_fast.into())
      .wasm_result()
      .map(WasmDecodedJptPresentation)
  }
}
