use identity_core::crypto::KeyPair;
use identity_iota::did::IotaDocument;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const AUTH_MESSAGE: &str = "https://didcomm.org/iota/v1/message_types/auth";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AuthMessage {
    pub response_requested: bool,
    pub challenge: IotaDocument,
}

impl AuthMessage {
    pub fn create_random() -> Self {
        let (doc, _keypair) = IotaDocument::generate_ed25519("random", "main", None).unwrap();
        Self {
            response_requested: false,
            challenge: doc,
        }
    }
    pub fn create_with_doc(document: IotaDocument) -> Self {
        Self {
            response_requested: false,
            challenge: document,
        }
    }

    pub fn challenge(&self) -> IotaDocument {
        self.challenge.clone()
    }

    pub fn set_challenge(&mut self, document: IotaDocument) {
        self.challenge = document;
    }

    pub fn request_response(&mut self) {
        self.response_requested = true;
    }

    pub fn sign(&mut self, keypair: KeyPair, document: IotaDocument) {
        document.sign_data(&mut self.challenge, &keypair.secret()).unwrap();
    }

    pub fn verify(&self, document: IotaDocument) -> crate::Result<()> {
        Ok(document.verify_data(&self.challenge).unwrap())
    }
}

/// converts a `AuthMessage` into a string using the `to_string()` method.
impl ToString for AuthMessage {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize AuthMessage")
    }
}

/// takes a &str and converts it into a `AuthMessage` given the proper format.
impl FromStr for AuthMessage {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let did_comm = serde_json::from_str(s).expect("Unable to build AuthMessage");
        Ok(did_comm)
    }
}
