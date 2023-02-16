use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const JWS_ALGORITHM: &'static str = r#"
/** Supported algorithms for the JSON Web Signatures `alg` claim.
 * 
 * [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-signature-encryption-algorithms) */
export declare type JwsAlgorithm = "HS256" | "HS384" | "HS512" | "RS256" | "RS384" | "RS512" |
    "PS256" | "PS384" | "PS512" | "ES256" | "ES384" | "ES512" | "ES256K" | "NONE" | "EdDSA"
"#;
