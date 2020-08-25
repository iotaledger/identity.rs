use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use std::{collections::HashMap, str::FromStr};

use crate::{
    did::DID,
    utils::{helpers::string_or_list, Context, PublicKey, Service, Subject},
};

/// A struct that represents a DID Document.  Contains the fields `context`, `id`, `created`, `updated`,
/// `public_key`, services and metadata.  Only `context` and `id` are required to create a DID document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, SerdeDiff)]
pub struct DIDDocument {
    #[serde(rename = "@context", deserialize_with = "string_or_list", default)]
    pub context: Context,
    pub id: Subject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_diff(skip)]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    pub public_key: Vec<PublicKey>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub services: Vec<Service>,
    #[serde(flatten)]
    pub metadata: HashMap<String, String>,
}

impl DIDDocument {
    /// Takes in the `context` and `id` as strings and creates a new `DIDDocument` struct.  The `context` field may be
    /// an empty string in which case it will default to "https://www.w3.org/ns/did/v1"
    pub fn new(context: String, id: String) -> crate::Result<Self> {
        if context == String::new() {
            Ok(DIDDocument {
                context: Context::default(),
                id: Subject::from_str(&id)?,
                ..Default::default()
            })
        } else {
            Ok(DIDDocument {
                context: Context::from_str(&context)?,
                id: Subject::from_str(&id)?,
                ..Default::default()
            })
        }
    }

    /// gets the inner value of the `context` from the `DIDDocument`.
    pub fn context(&self) -> &Vec<String> {
        &self.context.as_inner()
    }

    /// sets a new `service` of type `Service` into the `DIDDocument`.
    pub fn add_service(&mut self, service: Service) {
        self.services.push(service);
    }

    /// sets a new `key_pair` of type `PublicKey` into the `DIDDocument`.
    pub fn add_key_pair(&mut self, key_pair: PublicKey) {
        self.public_key.push(key_pair);
    }

    /// derive the did from the document.
    pub fn derive_did(&self) -> crate::Result<DID> {
        self.id.to_did()
    }

    /// initialize the `created` and `updated` timestamps to publish the did document.  Returns the did document with
    /// these timestamps.
    pub fn init_timestamps(self) -> crate::Result<Self> {
        Ok(DIDDocument {
            created: Some(Utc::now().to_string()),
            updated: Some(Utc::now().to_string()),
            ..self
        })
    }
}

/// converts a `DIDDocument` into a string using the `to_string()` method.
impl ToString for DIDDocument {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize document")
    }
}

/// takes a &str and converts it into a `DIDDocument` given the proper format.
impl FromStr for DIDDocument {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let doc = serde_json::from_str(s)?;
        Ok(doc)
    }
}
