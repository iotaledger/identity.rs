// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0


#[wasm_bindgen(typescript_custom_section)]
const JWK_STORAGE: &'static str = r#"
/** Interface for cryptographically verifying a JWS signature. 
 * 
 * Implementers are expected to provide a procedure for step 8 of [RFC 7515 section 5.2](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) for 
 * the JWS signature algorithms they want to support.
 * 
*/
interface IJwsSignatureVerifier {
  /** Validate the `decodedSignature` against the `signingInput` in the manner defined by `alg` using the `publicKey`. Returns `true` if the 
   *  implementation considers the verification successful.
   * 
   *  See [RFC 7515: section 5.2 part 8.](https://www.rfc-editor.org/rfc/rfc7515#section-5.2) and
   *  [RFC 7797 section 3](https://www.rfc-editor.org/rfc/rfc7797#section-3).
   * 
   * ### Failures 
  */
   verify: (alg: JwsAlgorithm, signingInput: Uint8Array, decodedSignature: Uint8Array, publicKey: Jwk) => bool;
}"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "JwkStorage")]
  pub type WasmJwkStorage;

  #[wasm_bindgen(method)]
  pub fn generate(this: &WasmJwkStorage, key_type: String, algorithm: String) -> PromiseJwkGenOutput;

  #[wasm_bindgen(method)]
  pub fn insert(this: &WasmJwkStorage, jwk: WasmJwk) -> PromiseString;

  #[wasm_bindgen(method)]
  pub fn sign(this: &WasmJwkStorage, key_id: String, data: Vec<u8>, public_key: WasmJwk) -> PromiseUint8Array;

  #[wasm_bindgen(method)]
  pub fn delete(this: &WasmJwkStorage, key_id: String) -> PromiseVoid;

  #[wasm_bindgen(method)]
  pub fn exists(this: &WasmJwkStorage, key_id: String) -> PromiseBool;
}