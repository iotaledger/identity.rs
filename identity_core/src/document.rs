use identity_diff::Diff;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{
    common::Timestamp,
    did::DID,
    utils::{helpers::string_or_list, Authentication, Context, HasId, IdCompare, PublicKey, Service, Subject},
};

/// A struct that represents a DID Document.  Contains the fields `context`, `id`, `created`, `updated`,
/// `public_keys`, services and metadata.  Only `context` and `id` are required to create a DID document.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Diff)]
pub struct DIDDocument {
    #[serde(rename = "@context", deserialize_with = "string_or_list", default)]
    pub context: Context,
    pub id: Subject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(should_ignore)]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "HashSet::is_empty", default)]
    pub public_keys: HashSet<IdCompare<PublicKey>>,
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
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    pub services: HashSet<IdCompare<Service>>,
    #[serde(flatten)]
    pub metadata: HashMap<String, String>,
}

impl DIDDocument {
    /// Initialize the DIDDocument.
    pub fn init(self) -> Self {
        let mut doc = DIDDocument {
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
            ..Default::default()
        };

        self.public_keys.into_iter().for_each(|k| doc.update_public_key(k));
        self.services.into_iter().for_each(|s| doc.update_service(s));

        doc
    }

    /// gets the inner value of the `context` from the `DIDDocument`.
    pub fn context(&self) -> &Vec<String> {
        &self.context.as_inner()
    }

    /// sets a new `service` of type `Service` into the `DIDDocument`.
    pub fn update_service(&mut self, service: IdCompare<Service>) {
        let mut services: HashSet<IdCompare<Service>> = self
            .services
            .clone()
            .into_iter()
            .filter(|s| s.0.id() != service.0.id())
            .collect::<HashSet<IdCompare<Service>>>();

        services.insert(service);

        self.services = services;
    }

    /// remove all of the services from the `DIDDocument`.
    pub fn clear_services(&mut self) {
        self.services.clear();
    }

    /// sets a new `key_pair` of type `PublicKey` into the `DIDDocument`.
    pub fn update_public_key(&mut self, key_pair: IdCompare<PublicKey>) {
        let mut keys: HashSet<IdCompare<PublicKey>> = self
            .public_keys
            .clone()
            .into_iter()
            .filter(|k| k.0.id() != key_pair.0.id())
            .collect::<HashSet<IdCompare<PublicKey>>>();

        keys.insert(key_pair);

        self.public_keys = keys;
    }

    /// remove all of the public keys from the `DIDDocument`.
    pub fn clear_public_keys(&mut self) {
        self.public_keys.clear();
    }

    /// sets in a new `auth` of type `Authentication` into the `DIDDocument`.
    pub fn update_auth(&mut self, auth: Authentication) {
        self.auth.push(auth);
    }

    /// remove all of the authentications from the `DIDDocument`.
    pub fn clear_auth(&mut self) {
        self.auth.clear();
    }

    /// sets in a new `assert` of type `Authentication` into the `DIDDocument`.
    pub fn update_assert(&mut self, assert: Authentication) {
        self.assert.push(assert);
    }

    /// remove all of the assertion methods from the `DIDDocument`.
    pub fn clear_assert(&mut self) {
        self.assert.clear();
    }

    /// sets in a new `verification` of type `Authentication` into the `DIDDocument`.
    pub fn update_verification(&mut self, verification: Authentication) {
        self.verification.push(verification);
    }

    /// remove all of the verification methods from the `DIDDocument`.
    pub fn clear_verification(&mut self) {
        self.verification.clear();
    }

    /// sets in a new `delegation` of type `Authentication` into the `DIDDocument`.
    pub fn update_delegation(&mut self, delegation: Authentication) {
        self.delegation.push(delegation);
    }

    /// remove all of the capability delegations from the `DIDDocument`.
    pub fn clear_delegation(&mut self) {
        self.delegation.clear();
    }

    /// sets in a new `invocation` of type `Authentication` into the `DIDDocument`.
    pub fn update_invocation(&mut self, invocation: Authentication) {
        self.invocation.push(invocation);
    }

    /// remove all of the capability invocations from the `DIDDocument`.
    pub fn clear_invocation(&mut self) {
        self.invocation.clear();
    }

    /// sets in a new `agreement` of type `Authentication` into the `DIDDocument`.
    pub fn update_agreement(&mut self, agreement: Authentication) {
        self.agreement.push(agreement);
    }

    /// remove all of the key agreements from the `DIDDocument`.
    pub fn clear_agreement(&mut self) {
        self.agreement.clear();
    }

    /// get the ID from the Document as a DID.
    pub fn derive_did(&self) -> crate::Result<DID> {
        self.id.to_did()
    }

    /// Updates the `updated` time for the `DIDDocument`.
    pub fn update_time(&mut self) {
        self.updated = Some(Timestamp::now().to_rfc3339());
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
            created: Some(Timestamp::now().to_rfc3339()),
            updated: Some(Timestamp::now().to_rfc3339()),
            ..self
        }
        .init())
    }

    pub fn get_diff_from_str(json: String) -> crate::Result<DiffDIDDocument> {
        Ok(serde_json::from_str(&json)?)
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
