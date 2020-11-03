use identity_core::{
    common::{AsJson as _, Context, Timestamp},
    did::DID,
    object,
    vc::{Credential as CoreCredential, CredentialBuilder, CredentialSubject, CredentialSubjectBuilder},
};
use identity_iota::vc::VerifiableCredential as IotaVC;

use crate::{doc::Doc, js_err, key::Key};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiableCredential(pub(crate) IotaVC);

#[wasm_bindgen]
impl VerifiableCredential {
    #[wasm_bindgen(constructor)]
    pub fn new(document: &Doc, name: String, key: &Key) -> Result<VerifiableCredential, JsValue> {
        let subject: CredentialSubject = CredentialSubjectBuilder::default()
            .id(DID::from(document.0.did().clone()))
            // Get this from JsValue and how?
            .properties(object!(
                name: name,
                degree:
                    object!(
                      name: "Bachelor of Science and Arts",
                      type: "BachelorDegree",
                    )
            ))
            .build()
            .unwrap();

        let mut credential: IotaVC = CredentialBuilder::new()
            .id("http://example.edu/credentials/3732")
            .issuer(DID::from(document.0.did().clone()))
            .context(vec![Context::from(CoreCredential::BASE_CONTEXT)])
            .types(vec![
                CoreCredential::BASE_TYPE.into(),
                "UniversityDegreeCredential".into(),
            ])
            .subject(vec![subject])
            .issuance_date(Timestamp::now())
            .build()
            .map(IotaVC::new)
            .map_err(js_err)?;

        credential.sign(&document.0, key.0.secret()).map_err(js_err)?;

        Ok(Self(credential))
    }
    #[wasm_bindgen]
    pub fn to_json_pretty(&self) -> Result<String, JsValue> {
        Ok(self.0.to_json_pretty().map_err(js_err)?)
    }
}
