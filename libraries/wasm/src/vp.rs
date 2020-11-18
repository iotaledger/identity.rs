use identity_core::{
    common::{Context, OneOrMany},
    vc::{Presentation, PresentationBuilder, VerifiableCredential},
};
use identity_iota::vc::VerifiablePresentation as IotaVP;
use wasm_bindgen::prelude::*;

use crate::{doc::Doc, js_err, key::Key};

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiablePresentation(pub(crate) IotaVP);

#[wasm_bindgen]
impl VerifiablePresentation {
    #[wasm_bindgen(constructor)]
    pub fn new(
        holder_doc: &Doc,
        holder_key: &Key,
        credential_data: JsValue,
        presentation_type: Option<String>,
        presentation_id: Option<String>,
    ) -> Result<VerifiablePresentation, JsValue> {
        let credentials: OneOrMany<VerifiableCredential> = credential_data.into_serde().map_err(js_err)?;

        let types: Vec<String> = {
            let mut types = vec![Presentation::BASE_TYPE.into()];
            types.extend(presentation_type.into_iter());
            types
        };

        let mut builder: PresentationBuilder = PresentationBuilder::new()
            .holder(holder_doc.did().0.into_inner())
            .context(vec![Context::from(Presentation::BASE_CONTEXT)])
            .credential(credentials)
            .types(types);

        if let Some(presentation_id) = presentation_id {
            builder = builder.id(presentation_id);
        }

        let mut this: Self = builder.build().map(IotaVP::new).map_err(js_err).map(Self)?;

        this.sign(holder_doc, holder_key)?;

        Ok(this)
    }

    /// Signs the credential with the given holder `Doc` and `Key` object.
    #[wasm_bindgen]
    pub fn sign(&mut self, holder: &Doc, key: &Key) -> Result<(), JsValue> {
        self.0.sign(&holder.0, key.0.secret()).map_err(js_err)
    }

    /// Verifies the credential signature against the holder `Doc`.
    #[wasm_bindgen]
    pub fn verify(&self, holder: &Doc) -> Result<bool, JsValue> {
        self.0.verify(&holder.0).map_err(js_err).map(|_| true)
    }

    /// Serializes a `VerifiablePresentation` object as a JSON object.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(js_err)
    }

    /// Deserializes a `VerifiablePresentation` object from a JSON object.
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &JsValue) -> Result<VerifiablePresentation, JsValue> {
        json.into_serde().map_err(js_err).map(Self)
    }
}
