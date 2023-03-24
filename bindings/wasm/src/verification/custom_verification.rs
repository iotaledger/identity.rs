// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::verification::jws::EdDSAJwsSignatureVerifier;
use identity_iota::verification::jws::JwsSignatureVerifier;
use identity_iota::verification::jws::SignatureVerificationError;
use identity_iota::verification::jws::SignatureVerificationErrorKind;
use identity_iota::verification::jws::VerificationInput;
use wasm_bindgen::prelude::*;

use crate::jose::WasmJwk;

/// Wrapper that enables custom TS JWS signature verification plugins to be used where the
/// JwsSignatureVerifier trait is required. Falls back to the default implementation if a custom
/// implementation was not passed.
pub(crate) struct WasmJwsSignatureVerifier(Option<IJwsSignatureVerifier>);

impl WasmJwsSignatureVerifier {
  pub(crate) fn new(verifier: Option<IJwsSignatureVerifier>) -> Self {
    Self(verifier)
  }
}

impl JwsSignatureVerifier for WasmJwsSignatureVerifier {
  fn verify(
    &self,
    input: identity_iota::verification::jws::VerificationInput,
    public_key: &identity_iota::verification::jwk::Jwk,
  ) -> Result<(), identity_iota::verification::jws::SignatureVerificationError> {
    match &self.0 {
      None => EdDSAJwsSignatureVerifier::default().verify(input, public_key),
      Some(custom) => {
        let VerificationInput {
          alg,
          signing_input,
          decoded_signature,
        } = input;
        let verification_result = custom.verify(
          alg.name().to_owned(),
          signing_input.into(),
          decoded_signature.into(),
          WasmJwk(public_key.to_owned()),
        );
        // Convert error
        crate::error::stringify_js_error(verification_result).map_err(|error_string| {
          SignatureVerificationError::new(SignatureVerificationErrorKind::Unspecified).with_custom_message(error_string)
        })
      }
    }
  }
}
#[wasm_bindgen(typescript_custom_section)]
const JWS_SIGNATURE_VERIFIER: &'static str = r#"
/** Interface for cryptographically verifying a JWS signature. 
 * 
 * Implementers are expected to provide a procedure for step 8 of [RFC 7515 section 5.2](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) for 
 * the JWS signature algorithms they want to support.
 * 
*/
interface IJwsSignatureVerifier {
  /** Validate the `decodedSignature` against the `signingInput` in the manner defined by `alg` using the `publicKey`.
   * 
   *  See [RFC 7515: section 5.2 part 8.](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) and
   *  [RFC 7797 section 3](https://www.rfc-editor.org/rfc/rfc7797#section-3).
   * 
   * ### Failures
   * Upon verification failure an error must be thrown.
  */
   verify: (alg: JwsAlgorithm, signingInput: Uint8Array, decodedSignature: Uint8Array, publicKey: Jwk) => void;
}"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwsSignatureVerifier")]
  pub type IJwsSignatureVerifier;

  #[wasm_bindgen(catch, method)]
  #[allow(non_snake_case)]
  pub fn verify(
    this: &IJwsSignatureVerifier,
    alg: String,
    signingInput: Vec<u8>,
    decodedSignature: Vec<u8>,
    publicKey: WasmJwk,
  ) -> Result<(), JsValue>;
}
