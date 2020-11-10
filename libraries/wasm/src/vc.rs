use identity_core::{
    common::{Context, OneOrMany, Timestamp},
    vc::{Credential, CredentialBuilder, CredentialSubject},
};
use identity_iota::vc::VerifiableCredential as IotaVC;
use wasm_bindgen::prelude::*;

use crate::{doc::Doc, js_err, key::Key};

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiableCredential(pub(crate) IotaVC);

#[wasm_bindgen]
impl VerifiableCredential {
    #[wasm_bindgen(constructor)]
    pub fn new(
        issuer_doc: &Doc,
        issuer_key: &Key,
        subject_data: JsValue,
        credential_type: Option<String>,
        credential_id: Option<String>,
    ) -> Result<VerifiableCredential, JsValue> {
        let subjects: OneOrMany<CredentialSubject> = subject_data.into_serde().map_err(js_err)?;

        let types: Vec<String> = {
            let mut types = vec![Credential::BASE_TYPE.into()];
            types.extend(credential_type.into_iter());
            types
        };

        let mut builder: CredentialBuilder = CredentialBuilder::new()
            .issuer(issuer_doc.did().0.into_inner())
            .context(vec![Context::from(Credential::BASE_CONTEXT)])
            .types(types)
            .subject(subjects)
            .issuance_date(Timestamp::now());

        if let Some(credential_id) = credential_id {
            builder = builder.id(credential_id);
        }

        let mut this: Self = builder
            .build()
            .map(IotaVC::new)
            .map_err(js_err)
            .map(Self)?;

        this.sign(issuer_doc, issuer_key)?;

        Ok(this)
    }

    /// Signs the credential with the given issuer `Doc` and `Key` object.
    #[wasm_bindgen]
    pub fn sign(&mut self, issuer: &Doc, key: &Key) -> Result<(), JsValue> {
        self.0.sign(&issuer.0, key.0.secret()).map_err(js_err)
    }

    /// Verifies the credential signature against the issuer `Doc`.
    #[wasm_bindgen]
    pub fn verify(&self, issuer: &Doc) -> Result<bool, JsValue> {
        self.0.verify(&issuer.0).map_err(js_err).map(|_| true)
    }

    /// Serializes a `VerifiableCredential` object as a JSON object.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.0).map_err(js_err)
    }

    /// Deserializes a `VerifiableCredential` object from a JSON object.
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &JsValue) -> Result<VerifiableCredential, JsValue> {
        json.into_serde().map_err(js_err).map(Self)
    }
}
