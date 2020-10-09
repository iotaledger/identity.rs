use identity_diff::Diff;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

use crate::{
    common::{Context, OneOrMany, Timestamp},
    did::{Authentication, PublicKey, Service, DID},
    utils::AddUnique as _,
};

/// A struct that represents a DID Document.  Contains the fields `context`, `id`, `created`, `updated`,
/// `public_keys`, services and metadata.  Only `context` and `id` are required to create a DID document.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Diff)]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    pub context: OneOrMany<Context>,
    pub id: DID,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(should_ignore)]
    pub created: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<Timestamp>,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    pub public_keys: Vec<PublicKey>,
    #[serde(rename = "authentication", skip_serializing_if = "Vec::is_empty", default)]
    pub auth: Vec<Authentication>,
    #[serde(rename = "assertionMethod", skip_serializing_if = "Vec::is_empty", default)]
    pub assert: Vec<Authentication>,
    #[serde(rename = "verificationMethod", skip_serializing_if = "Vec::is_empty", default)]
    pub verification: Vec<Authentication>,
    #[serde(rename = "capabilityDelegation", skip_serializing_if = "Vec::is_empty", default)]
    pub delegation: Vec<Authentication>,
    #[serde(rename = "capabilityInvocation", skip_serializing_if = "Vec::is_empty", default)]
    pub invocation: Vec<Authentication>,
    #[serde(rename = "keyAgreement", skip_serializing_if = "Vec::is_empty", default)]
    pub agreement: Vec<Authentication>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub services: Vec<Service>,
    #[serde(flatten)]
    pub metadata: HashMap<String, String>,
}

impl DIDDocument {
    /// Initialize the DIDDocument.
    pub fn init(self) -> Self {
        DIDDocument {
            context: self.context,
            id: self.id,
            created: self.created,
            updated: self.updated,
            auth: self.auth,
            assert: self.assert,
            verification: self.verification,
            delegation: self.delegation,
            invocation: self.invocation,
            agreement: self.agreement,
            metadata: self.metadata,
            public_keys: self.public_keys,
            services: self.services,
        }
    }

    /// gets the inner value of the `context` from the `DIDDocument`.
    pub fn context(&self) -> &[Context] {
        self.context.as_slice()
    }

    /// sets a new `service` of type `Service` into the `DIDDocument`.
    pub fn update_service(&mut self, service: Service) {
        self.services.set_unique(service);
    }

    /// remove all of the services from the `DIDDocument`.
    pub fn clear_services(&mut self) {
        self.services.clear();
    }

    /// sets a new `key_pair` of type `PublicKey` into the `DIDDocument`.
    pub fn update_public_key(&mut self, key_pair: PublicKey) {
        self.public_keys.set_unique(key_pair);
    }

    /// remove all of the public keys from the `DIDDocument`.
    pub fn clear_public_keys(&mut self) {
        self.public_keys.clear();
    }

    /// sets in a new `auth` of type `Authentication` into the `DIDDocument`.
    pub fn update_auth(&mut self, auth: Authentication) {
        self.auth.set_unique(auth);
    }

    /// remove all of the authentications from the `DIDDocument`.
    pub fn clear_auth(&mut self) {
        self.auth.clear();
    }

    /// sets in a new `assert` of type `Authentication` into the `DIDDocument`.
    pub fn update_assert(&mut self, assert: Authentication) {
        self.assert.set_unique(assert);
    }

    /// remove all of the assertion methods from the `DIDDocument`.
    pub fn clear_assert(&mut self) {
        self.assert.clear();
    }

    /// sets in a new `verification` of type `Authentication` into the `DIDDocument`.
    pub fn update_verification(&mut self, verification: Authentication) {
        self.verification.set_unique(verification);
    }

    /// remove all of the verification methods from the `DIDDocument`.
    pub fn clear_verification(&mut self) {
        self.verification.clear();
    }

    /// sets in a new `delegation` of type `Authentication` into the `DIDDocument`.
    pub fn update_delegation(&mut self, delegation: Authentication) {
        self.delegation.set_unique(delegation);
    }

    /// remove all of the capability delegations from the `DIDDocument`.
    pub fn clear_delegation(&mut self) {
        self.delegation.clear();
    }

    /// sets in a new `invocation` of type `Authentication` into the `DIDDocument`.
    pub fn update_invocation(&mut self, invocation: Authentication) {
        self.invocation.set_unique(invocation);
    }

    /// remove all of the capability invocations from the `DIDDocument`.
    pub fn clear_invocation(&mut self) {
        self.invocation.clear();
    }

    /// sets in a new `agreement` of type `Authentication` into the `DIDDocument`.
    pub fn update_agreement(&mut self, agreement: Authentication) {
        self.agreement.set_unique(agreement);
    }

    /// remove all of the key agreements from the `DIDDocument`.
    pub fn clear_agreement(&mut self) {
        self.agreement.clear();
    }

    /// get the ID from the Document as a DID.
    pub fn derive_did(&self) -> &DID {
        &self.id
    }

    /// Updates the `updated` time for the `DIDDocument`.
    pub fn update_time(&mut self) {
        self.updated = Some(Timestamp::now());
    }

    /// Inserts `metadata` into the `DIDDocument` body.  The metadata must be a HashMap<String, String> where the keys
    /// are json keys and values are the json values.
    pub fn supply_metadata(self, metadata: HashMap<String, String>) -> crate::Result<Self> {
        Ok(DIDDocument { metadata, ..self }.init())
    }

    /// initialize the `created` and `updated` timestamps to publish the did document.  Returns the did document with
    /// these timestamps.
    pub fn init_timestamps(self) -> crate::Result<Self> {
        Ok(DIDDocument {
            created: Some(Timestamp::now()),
            updated: Some(Timestamp::now()),
            ..self
        }
        .init())
    }

    pub fn get_diff_from_str(json: impl AsRef<str>) -> crate::Result<DiffDIDDocument> {
        serde_json::from_str(json.as_ref()).map_err(crate::Error::DecodeJSON)
    }
}

impl Default for DIDDocument {
    fn default() -> Self {
        Self {
            context: OneOrMany::One(DID::BASE_CONTEXT.into()),
            id: Default::default(),
            created: None,
            updated: None,
            public_keys: Vec::new(),
            auth: Vec::new(),
            assert: Vec::new(),
            verification: Vec::new(),
            delegation: Vec::new(),
            invocation: Vec::new(),
            agreement: Vec::new(),
            services: Vec::new(),
            metadata: HashMap::new(),
        }
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
        serde_json::from_str(s).map_err(crate::Error::DecodeJSON)
    }
}
