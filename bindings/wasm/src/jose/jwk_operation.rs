use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const JWK_OPERATION: &'static str = r#"
/** Supported algorithms for the JSON Web Key `key_ops` property.
 * 
 * [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-operations) */
export declare type JwkOperation = "sign" | "verify" | "encrypt" | "decrypt" |
    "wrapKey" | "unwrapKey" | "deriveKey" | "deriveBits"
"#;
