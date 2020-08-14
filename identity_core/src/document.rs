use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

use crate::utils::{helpers::string_or_list, Context, PublicKey, Service, Subject};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DIDDocument {
    #[serde(rename = "@context", deserialize_with = "string_or_list")]
    context: Context,
    id: Subject,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    created: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    updated: String,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    public_key: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    services: Vec<Service>,
    #[serde(flatten)]
    metadata: HashMap<String, String>,
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

    #[test]
    fn parse_document() {
        let jstr = r#"
        {
            "@context": ["https://w3id.org/did/v1", "https://w3id.org/security/v1"],
            "id": "did:example:123456789abcdefghi",
            "publicKey": [{
                "id": "did:example:123456789abcdefghi#keys-1",
                "type": "RsaVerificationKey2018",
                "controller": "did:example:123456789abcdefghi",
                "publicKeyPem": "-----BEGIN PUBLIC KEY...END PUBLIC KEY-----"
            }, {
                "id": "did:example:123456789abcdefghi#keys-2",
                "type": "Ed25519VerificationKey2018",
                "controller": "did:example:pqrstuvwxyz0987654321",
                "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
            }, {
                "id": "did:example:123456789abcdefghi#keys-3",
                "type": "EcdsaSecp256k1VerificationKey2019",
                "controller": "did:example:123456789abcdefghi",
                "publicKeyHex": "02b97c30de767f084ce3080168ee293053ba33b235d7116a3263d29f1450936b71"
            }]
        }
        "#;

        let doc = DIDDocument::from_str(jstr).unwrap();

        println!("{}", doc.to_string())
    }
}
