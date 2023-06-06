// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use identity_iota::core::Object;
use identity_iota::core::OneOrMany;
use identity_iota::core::Url;
use identity_iota::credential::vc_jwt_validation::CredentialValidator as JwtCredentialValidator;
use identity_iota::credential::DecodedJwtPresentation;
use identity_iota::credential::StatusCheck;
use identity_iota::did::CoreDID;

use crate::common::ImportedDocumentLock;
use crate::common::ImportedDocumentReadGuard;
use crate::common::WasmTimestamp;
use crate::credential::jwt_presentation::WasmJwtPresentation;
use crate::credential::types::ArrayCoreDID;
use crate::credential::JwtPresentationDids;
use crate::credential::WasmFailFast;
use crate::credential::WasmJwt;
use crate::did::ArrayIToCoreDocument;
use crate::did::IToCoreDocument;
use crate::did::WasmCoreDID;
use crate::error::Result;
use crate::error::WasmResult;
use crate::verification::IJwsVerifier;
use crate::verification::WasmJwsVerifier;

use identity_iota::credential::JwtPresentationValidator;
use wasm_bindgen::prelude::*;

use super::decoded_jwt_presentation::WasmDecodedJwtPresentation;
use super::options::WasmJwtPresentationValidationOptions;

#[wasm_bindgen(js_name = JwtPresentationValidator, inspectable)]
pub struct WasmJwtPresentationValidator(JwtPresentationValidator<WasmJwsVerifier>);

#[wasm_bindgen(js_class = JwtPresentationValidator)]
impl WasmJwtPresentationValidator {
  /// Creates a new `JwtPresentationValidator`. If a `signature_verifier` is provided it will be used when
  /// verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
  /// algorithm will be used.
  #[wasm_bindgen(constructor)]
  pub fn new(signature_verifier: Option<IJwsVerifier>) -> WasmJwtPresentationValidator {
    let signature_verifier = WasmJwsVerifier::new(signature_verifier);
    WasmJwtPresentationValidator(JwtPresentationValidator::with_signature_verifier(signature_verifier))
  }

  #[wasm_bindgen]
  pub fn validate(
    &self,
    presentation_jwt: &WasmJwt,
    holder: &IToCoreDocument,
    issuers: &ArrayIToCoreDocument,
    options: &WasmJwtPresentationValidationOptions,
    fail_fast: WasmFailFast,
  ) -> Result<WasmDecodedJwtPresentation> {
    let issuer_locks: Vec<ImportedDocumentLock> = issuers.into();
    let issuers_guards: Vec<ImportedDocumentReadGuard<'_>> =
      issuer_locks.iter().map(ImportedDocumentLock::blocking_read).collect();

    let holder_lock = ImportedDocumentLock::from(holder);
    let holder_guard = holder_lock.blocking_read();

    self
      .0
      .validate(
        &presentation_jwt.0,
        &holder_guard,
        &issuers_guards,
        &options.0,
        fail_fast.into(),
      )
      .map(WasmDecodedJwtPresentation::from)
      .wasm_result()
  }

  #[wasm_bindgen(js_name = checkStructure)]
  pub fn check_structure(presentation: &WasmJwtPresentation) -> Result<()> {
    JwtPresentationValidator::check_structure(&presentation.0).wasm_result()?;
    Ok(())
  }

  #[wasm_bindgen(js_name = extractDids)]
  pub fn extract_dids(presentation: &WasmJwt) -> Result<JwtPresentationDids> {
    let (holder, issuers) =
      JwtPresentationValidator::extract_dids::<CoreDID, CoreDID, Object, Object>(&presentation.0).wasm_result()?;
    let mut map = BTreeMap::<&str, OneOrMany<CoreDID>>::new();
    map.insert("holder", OneOrMany::One(holder));
    map.insert("issuers", OneOrMany::Many(issuers));

    Ok(JsValue::from_serde(&map).wasm_result()?.unchecked_into())
  }
}
