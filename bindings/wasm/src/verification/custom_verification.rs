// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::verification::jws::EdDSAJwsVerifier;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::jws::JwsVerifier;
use identity_iota::verification::jws::SignatureVerificationError;
use identity_iota::verification::jws::SignatureVerificationErrorKind;
use identity_iota::verification::jws::VerificationInput;
use wasm_bindgen::prelude::*;

use crate::error::WasmResult;
use crate::jose::WasmJwk;
use crate::jose::WasmJwsAlgorithm;

/// Wrapper that enables custom TS JWS signature verification plugins to be used where the
/// JwsVerifier trait is required. Falls back to the default implementation if a custom
/// implementation was not passed.
pub(crate) struct WasmJwsVerifier(Option<IJwsVerifier>);

impl WasmJwsVerifier {
  pub(crate) fn new(verifier: Option<IJwsVerifier>) -> Self {
    Self(verifier)
  }
}

impl JwsVerifier for WasmJwsVerifier {
  fn verify(
    &self,
    input: identity_iota::verification::jws::VerificationInput,
    public_key: &identity_iota::verification::jwk::Jwk,
  ) -> Result<(), identity_iota::verification::jws::SignatureVerificationError> {
    match &self.0 {
      None => EdDSAJwsVerifier::default().verify(input, public_key),
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
*/
interface IJwsVerifier {
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
  #[wasm_bindgen(typescript_type = "IJwsVerifier")]
  pub type IJwsVerifier;

  #[wasm_bindgen(catch, method)]
  #[allow(non_snake_case)]
  pub fn verify(
    this: &IJwsVerifier,
    alg: String,
    signingInput: Vec<u8>,
    decodedSignature: Vec<u8>,
    publicKey: WasmJwk,
  ) -> Result<(), JsValue>;
}

/// Verify a JWS signature secured with the `JwsAlgorithm::EdDSA` algorithm.
/// Only the `EdCurve::Ed25519` variant is supported for now.
///
/// This function is useful when one is building an `IJwsVerifier` that extends the default provided by
/// the IOTA Identity Framework.
///
/// # Warning
/// This function does not check whether `alg = EdDSA` in the protected header. Callers are expected to assert this
/// prior to calling the function.
#[wasm_bindgen(js_name = verifyEdDSA)]
#[allow(non_snake_case)]
pub fn verify_eddsa(
  alg: WasmJwsAlgorithm,
  signingInput: &[u8],
  decodedSignature: &[u8],
  publicKey: &WasmJwk,
) -> Result<(), JsValue> {
  let alg: JwsAlgorithm = JwsAlgorithm::try_from(alg)?;
  let input: VerificationInput = VerificationInput {
    alg,
    signing_input: Box::from(signingInput),
    decoded_signature: Box::from(decodedSignature),
  };
  identity_iota::verification::jose::jws::EdDSAJwsVerifier::verify_eddsa(input, &publicKey.0).wasm_result()
}
