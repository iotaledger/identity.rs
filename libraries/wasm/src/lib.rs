use wasm_bindgen::prelude::*;

mod core;
mod iota;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[wasm_bindgen(js_name = "Greet")]
pub fn greet() -> Result<String, JsValue> {
    console_error_panic_hook::set_once();

    Ok("Hello World!".to_owned())
}
