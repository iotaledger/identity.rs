use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

use crate::utils::{helpers::string_or_list, Context, PublicKey, Service, Subject};

#[cfg_attr(not(test), derive(PartialEq))]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DIDDocument {
    #[serde(rename = "@context", deserialize_with = "string_or_list")]
    pub context: Context,
    pub id: Subject,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub created: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub updated: String,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    pub public_key: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub services: Vec<Service>,
    #[serde(flatten)]
    pub metadata: HashMap<String, String>,
}

impl DIDDocument {
    pub fn new(context: String, id: String) -> crate::Result<Self> {
        Ok(DIDDocument {
            context: Context::from_str(&context)?,
            id: Subject::from_str(&id)?,
            created: Utc::now().to_string(),
            updated: Utc::now().to_string(),
            ..Default::default()
        })
    }

    pub fn context(&self) -> &Vec<String> {
        &self.context.as_inner()
    }

    pub fn add_service(&mut self, service: Service) {
        self.services.push(service);

        self.update_time();
    }

    pub fn add_key_pair(&mut self, key_pair: PublicKey) {
        self.public_key.push(key_pair);

        self.update_time();
    }

    pub fn update_time(&mut self) {
        self.updated = Utc::now().to_string();
    }
}

impl ToString for DIDDocument {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize document")
    }
}

impl FromStr for DIDDocument {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let doc = serde_json::from_str(s)?;
        Ok(doc)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl PartialEq for DIDDocument {
        fn eq(&self, other: &Self) -> bool {
            self.context == other.context
                && self.id == other.id
                && self.public_key == other.public_key
                && self.services == other.services
        }
    }

    #[test]
    fn test_doc_creation() {
        let mut did_doc =
            DIDDocument::new("https://w3id.org/did/v1".into(), "did:iota:123456789abcdefghi".into()).unwrap();
        let service = Service::new(
            vec!["https://w3id.org/did/v1".into()],
            "did:into:123#edv".into(),
            "EncryptedDataVault".into(),
            "https://edv.example.com/".into(),
        )
        .unwrap();
        did_doc.add_service(service.clone());
        let public_key = PublicKey::new(
            "did:iota:123456789abcdefghi#keys-1".into(),
            "RsaVerificationKey2018".into(),
            "did:iota:123456789abcdefghi".into(),
            "publicKeyBase58".into(),
            "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV".into(),
        )
        .unwrap();
        did_doc.add_key_pair(public_key.clone());

        let mut did_doc_2 =
            DIDDocument::new("https://w3id.org/did/v1".into(), "did:iota:123456789abcdefghi".into()).unwrap();
        did_doc_2.add_service(service);
        did_doc_2.add_key_pair(public_key);

        // timestamps will not be equal but partialeq will ignore them for testing.
        assert_eq!(did_doc, did_doc_2);
    }
}
