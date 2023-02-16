use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const JWK_TYPE: &'static str = r#"
/** Supported types for the JSON Web Key `kty` property.
 * 
 * [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-types) */
export declare type JwkType = "EC" | "RSA" | "oct" | "OKP"
"#;

