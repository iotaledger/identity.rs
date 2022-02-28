// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::WasmFailFast;
use crate::credential::WasmPresentation;
use crate::credential::WasmPresentationValidationOptions;
use crate::did::ArrayResolvedDocument;
use crate::did::WasmResolvedDocument;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use identity::iota::PresentationValidator;
use identity::iota::ResolvedIotaDocument;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = PresentationValidator, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmPresentationValidator;

#[wasm_bindgen(js_class = PresentationValidator)]
impl WasmPresentationValidator {
  /// Validate a `Presentation`.
  ///
  /// The following properties are validated according to `options`:
  /// - The holder's signature,
  /// - The relationship between the subject and the holder,
  /// - The semantic structure
  /// - Some properties of the credentials  (see
  /// `CredentialValidator::validate` for more information).
  ///
  /// # Warning
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated.
  ///  Examples of properties **not** validated by this method includes: credentialStatus, types, credentialSchema,
  /// refreshService **and more**.
  ///
  /// # Errors
  /// Fails on the first encountered validation error if `fail_fast` is "Yes", otherwise all
  /// errors will be accumulated in the returned error.
  pub fn validate(
    presentation: &WasmPresentation,
    options: &WasmPresentationValidationOptions,
    holder: &WasmResolvedDocument,
    issuers: &ArrayResolvedDocument,
    fail_fast: WasmFailFast,
  ) -> Result<()> {
    let issuers: Vec<ResolvedIotaDocument> = issuers.into_serde().wasm_result()?;
    PresentationValidator::validate(&presentation.0, &options.0, &holder.0, &issuers, fail_fast.into()).wasm_result()
  }

  /// Verify the presentation's signature using the resolved document of the holder
  ///
  /// # Errors
  /// Fails immediately if the supplied `holder` cannot be identified with the URL of the `presentation`'s holder
  /// property. Otherwise signature verification will be attempted and an error is returned upon failure.
  pub fn verify_presentation_signature(
    presentation: &WasmPresentation,
    holder: &WasmResolvedDocument,
    options: &WasmVerifierOptions,
  ) -> Result<()> {
    PresentationValidator::verify_presentation_signature(&presentation.0, &holder.0, &options.0).wasm_result()
  }

  /// Validates the semantic structure of the [Presentation].
  pub fn check_structure(presentation: &WasmPresentation) -> Result<()> {
    PresentationValidator::check_structure(&presentation.0).wasm_result()
  }

  /// Validates that the nonTransferable property is met.
  ///
  /// # Errors
  /// Returns an error at the first credential requiring a nonTransferable property that is not met.
  pub fn check_non_transferable(presentation: &WasmPresentation) -> Result<()> {
    PresentationValidator::check_non_transferable(&presentation.0).wasm_result()
  }

  /// Validates that the presentation only contains credentials where the credential subject is the holder.
  ///
  /// # Errors
  /// Returns an error at the first credential with a credential subject not corresponding to the holder.
  pub fn check_holder_is_always_subject(presentation: &WasmPresentation) -> Result<()> {
    PresentationValidator::check_holder_is_always_subject(&presentation.0).wasm_result()
  }
}
