use crate::{doc::Doc, js_err, key::Key};
use identity_core::{
    common::{AsJson as _, Context, Timestamp, Value},
    did::DID,
    vc::{Credential as CoreCredential, CredentialBuilder, CredentialSubject, CredentialSubjectBuilder},
};
use identity_iota::vc::VerifiableCredential as IotaVC;
use serde_json::Map;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiableCredential(pub(crate) IotaVC);

#[wasm_bindgen]
impl VerifiableCredential {
    #[wasm_bindgen(constructor)]
    pub fn new(
        issuer_document: &Doc,
        key: &Key,
        subject_document: &Doc,
        credential_type: String,
        credential_url: String,
        properties: String,
    ) -> Result<VerifiableCredential, JsValue> {
        let json_properties: serde_json::Value = serde_json::from_str(&properties).map_err(js_err)?;
        let properties_obj: Map<String, Value> = json_properties.as_object().unwrap().clone();

        let subject: CredentialSubject = CredentialSubjectBuilder::default()
            .id(DID::from(subject_document.0.did().clone()))
            .properties(properties_obj)
            .build()
            .unwrap();

        let mut credential: IotaVC = CredentialBuilder::new()
            .id(credential_url)
            .issuer(DID::from(issuer_document.0.did().clone()))
            .context(vec![Context::from(CoreCredential::BASE_CONTEXT)])
            .types(vec![CoreCredential::BASE_TYPE.into(), credential_type])
            .subject(vec![subject])
            .issuance_date(Timestamp::now())
            .build()
            .map(IotaVC::new)
            .map_err(js_err)?;

        credential.sign(&issuer_document.0, key.0.secret()).map_err(js_err)?;

        Ok(Self(credential))
    }
    #[wasm_bindgen]
    pub fn to_json_pretty(&self) -> Result<String, JsValue> {
        Ok(self.0.to_json_pretty().map_err(js_err)?)
    }
}
