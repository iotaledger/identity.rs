// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::jws::JwsVerifier;
use identity_iota::verification::jws::VerificationInput;
use wasm_bindgen::prelude::*;

use crate::error::WasmResult;
use crate::jose::WasmJwk;
use crate::jose::WasmJwsAlgorithm;

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
  EdDSAJwsVerifier::default().verify(input, &publicKey.0).wasm_result()
}
