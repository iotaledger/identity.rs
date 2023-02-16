use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const JWK_USE: &'static str = r#"
/** Supported algorithms for the JSON Web Key `use` property.
 * 
 * [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-use) */
export declare type JwkUse = "sig" | "enc"
"#;
