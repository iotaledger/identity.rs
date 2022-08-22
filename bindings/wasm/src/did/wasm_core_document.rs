use identity_iota::did::CoreDocument;
use wasm_bindgen::prelude::*; 

#[wasm_bindgen(js_name = CoreDocument, inspectable)]
pub struct WasmCoreDocument(pub(crate) CoreDocument);

#[wasm_bindgen(js_class = CoreDocument)]
impl WasmCoreDocument {
    //TODO!
}

impl_wasm_json!(WasmCoreDocument, CoreDocument);
impl_wasm_clone!(WasmCoreDocument, CoreDocument);

impl From<CoreDocument> for WasmCoreDocument {
    fn from(document: CoreDocument) -> Self {
      Self(document)
    }
  }
  
  impl From<WasmCoreDocument> for CoreDocument {
    fn from(wasm_document: WasmCoreDocument) -> Self {
      wasm_document.0
    }
  }