// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::decoded_jwt_presentation::WasmDecodedJwtPresentation;
use super::options::WasmJwtPresentationValidationOptions;
use crate::common::ImportedDocumentLock;
use crate::common::ImportedDocumentReadGuard;
use crate::credential::jwt_presentation::WasmJwtPresentation;
use crate::credential::JwtPresentationDids;
use crate::credential::WasmFailFast;
use crate::credential::WasmJwt;
use crate::did::ArrayIToCoreDocument;
use crate::did::IToCoreDocument;
use crate::error::Result;
use crate::error::WasmResult;
use crate::verification::IJwsVerifier;
use crate::verification::WasmJwsVerifier;
use identity_iota::core::Object;
use identity_iota::core::OneOrMany;
use identity_iota::credential::JwtPresentationValidator;
use identity_iota::did::CoreDID;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

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

  /// Validates a `JwtPresentation`.
  ///
  /// The following properties are validated according to `options`:
  /// - the JWT can be decoded into semantically valid presentation.
  /// - the expiration and issuance date contained in the JWT claims.
  /// - the holder's signature.
  /// - the relationship between the holder and the credential subjects.
  /// - the signatures and some properties of the constituent credentials (see `CredentialValidator`).
  ///
  /// Validation is done with respect to the properties set in `options`.
  ///
  /// # Warning
  /// The lack of an error returned from this method is in of itself not enough to conclude that the presentation can be
  /// trusted. This section contains more information on additional checks that should be carried out before and after
  /// calling this method.
  ///
  /// ## The state of the supplied DID Documents.
  /// The caller must ensure that the DID Documents in `holder` and `issuers` are up-to-date.
  ///
  /// ## Properties that are not validated
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// # Errors
  /// An error is returned whenever a validated condition is not satisfied or when decoding fails.
  #[wasm_bindgen]
  pub fn validate(
    &self,
    presentation_jwt: &WasmJwt,
    holder: &IToCoreDocument,
    issuers: &ArrayIToCoreDocument,
    validation_options: &WasmJwtPresentationValidationOptions,
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
        &validation_options.0,
        fail_fast.into(),
      )
      .map(WasmDecodedJwtPresentation::from)
      .wasm_result()
  }

  /// Validates the semantic structure of the `JwtPresentation`.
  #[wasm_bindgen(js_name = checkStructure)]
  pub fn check_structure(presentation: &WasmJwtPresentation) -> Result<()> {
    JwtPresentationValidator::check_structure(&presentation.0).wasm_result()?;
    Ok(())
  }

  /// Attempt to extract the holder of the presentation and the issuers of the included
  /// credentials.
  ///
  /// # Errors:
  /// * If deserialization/decoding of the presentation or any of the constituent credentials
  /// fails.
  /// * If the holder or any of the issuers can't be parsed as DIDs.
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
