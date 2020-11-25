use crate::{doc::Doc, js_err, key::Key};
use identity_account::{
    error,
    identity_state::{Key as Keypair, State as identity_state},
};
use identity_iota::did::IotaDocument;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
#[wasm_bindgen(inspectable)]
#[derive(Debug, PartialEq)]
pub struct State(pub(crate) identity_state);

#[wasm_bindgen]
impl State {
    /// Generates a new `State`.
    #[wasm_bindgen(constructor)]
    pub fn new(keypair: Key, document: Doc) -> Result<State, JsValue> {
        Ok(Self(identity_state::new(keypair.0, document.0).map_err(js_err)?))
    }
    /// Write State to localStorage
    #[wasm_bindgen]
    pub fn to_localstorage(&self) -> Result<(), JsValue> {
        let window = web_sys::window()
            .ok_or_else(|| error::Error::StateError("Can't get window".into()))
            .map_err(js_err)?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            local_storage.set_item(&"identity_state", &self.0.to_string())?;
            Ok(())
        } else {
            Err("Can't write to localStorage".into())
        }
    }
    /// Read State from localStorage
    #[wasm_bindgen]
    pub fn from_localstorage(&self) -> Result<State, JsValue> {
        let window = web_sys::window()
            .ok_or_else(|| error::Error::StateError("Can't get window".into()))
            .map_err(js_err)?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            let state = local_storage
                .get(&"identity_state")?
                .ok_or_else(|| error::Error::StateError("Can't get localStorage".into()))
                .map_err(js_err)?;
            Ok(Self(identity_state::from_str(&state).map_err(js_err)?))
        } else {
            Err("Can't read from localStorage".into())
        }
    }
    /// Get latest document with diffs applied
    #[wasm_bindgen(getter)]
    pub fn latest_doc(&self) -> Doc {
        Doc(self.0.latest_doc())
    }
    /// Get all documents
    #[wasm_bindgen(getter)]
    pub fn documents(&self) -> Result<JsValue, JsValue> {
        let documents: Vec<IotaDocument> = self.0.documents().iter().map(|i| i.document.clone()).collect();
        Ok(JsValue::from_serde(&documents).map_err(js_err)?)
    }
    /// Add a document to the state
    #[wasm_bindgen]
    pub fn add_document(&mut self, document: Doc) {
        self.0.add_document(document.0)
    }
    /// Add a keypair to the state
    #[wasm_bindgen]
    pub fn update_keypair(&mut self, keypair: Key) {
        self.0.update_keypair(&keypair.0)
    }
    /// Generates a new Ed25519 keypair and stores it
    #[wasm_bindgen]
    pub fn new_keypair(&mut self) -> Key {
        Key(self.0.new_keypair())
    }
    /// Get the latest keypair
    #[wasm_bindgen(getter)]
    pub fn keypair(&self) -> Result<Key, JsValue> {
        Ok(Key(self.0.keypair().map_err(js_err)?))
    }
    /// Get all stored keypairs
    #[wasm_bindgen(getter)]
    pub fn keypairs(&self) -> Result<JsValue, JsValue> {
        let keypairs: Vec<Keypair> = self.0.keypairs_as_string();
        Ok(JsValue::from_serde(&keypairs).map_err(js_err)?)
    }
    /// Converts the State to a String
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
    /// Converts a String to a State
    #[wasm_bindgen]
    pub fn from_string(string: &str) -> Result<State, JsValue> {
        Ok(State(identity_state::from_str(string).map_err(js_err)?))
    }
}
