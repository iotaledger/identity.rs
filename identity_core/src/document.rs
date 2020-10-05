use identity_diff::Diff;

use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{
    common::Timestamp,
    did::DID,
    utils::{
        add_unique_to_vec, helpers::string_or_list, Authentication, Context, IdCompare, PublicKey, Service, Subject,
    },
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
    pub auth: Vec<IdCompare<Authentication>>,
    #[serde(rename = "assertionMethod", skip_serializing_if = "Vec::is_empty", default)]
    pub assert: Vec<IdCompare<Authentication>>,
    #[serde(rename = "verificationMethod", skip_serializing_if = "Vec::is_empty", default)]
    pub verification: Vec<IdCompare<Authentication>>,
    #[serde(rename = "capabilityDelegation", skip_serializing_if = "Vec::is_empty", default)]
    pub delegation: Vec<IdCompare<Authentication>>,
    #[serde(rename = "capabilityInvocation", skip_serializing_if = "Vec::is_empty", default)]
    pub invocation: Vec<IdCompare<Authentication>>,
    #[serde(rename = "keyAgreement", skip_serializing_if = "Vec::is_empty", default)]
    pub agreement: Vec<IdCompare<Authentication>>,
    #[serde(skip_serializing_if = "HashSet::is_empty", default)]
    pub services: HashSet<IdCompare<Service>>,
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
    pub fn context(&self) -> &Vec<String> {
        &self.context.as_inner()
    }

    /// sets a new `service` of type `Service` into the `DIDDocument`.
    pub fn update_service(&mut self, service: Service) {
        let service = IdCompare::new(service);

        self.services.insert(service);
    }

    /// remove all of the services from the `DIDDocument`.
    pub fn clear_services(&mut self) {
        self.services.clear();
    }

    /// sets a new `key_pair` of type `PublicKey` into the `DIDDocument`.
    pub fn update_public_key(&mut self, key_pair: PublicKey) {
        let key_pair = IdCompare::new(key_pair);

        self.public_keys.insert(key_pair);
    }

    /// remove all of the public keys from the `DIDDocument`.
    pub fn clear_public_keys(&mut self) {
        self.public_keys.clear();
    }

    /// sets in a new `auth` of type `Authentication` into the `DIDDocument`.
    pub fn update_auth(&mut self, auth: Authentication) {
        let auth = IdCompare::new(auth);

        let collection = add_unique_to_vec(auth, self.auth.clone());

        self.auth = collection;
    }

    /// remove all of the authentications from the `DIDDocument`.
    pub fn clear_auth(&mut self) {
        self.auth.clear();
    }

    /// sets in a new `assert` of type `Authentication` into the `DIDDocument`.
    pub fn update_assert(&mut self, assert: Authentication) {
        let assert = IdCompare::new(assert);

        let collection = add_unique_to_vec(assert, self.assert.clone());

        self.assert = collection;
    }

    /// remove all of the assertion methods from the `DIDDocument`.
    pub fn clear_assert(&mut self) {
        self.assert.clear();
    }

    /// sets in a new `verification` of type `Authentication` into the `DIDDocument`.
    pub fn update_verification(&mut self, verification: Authentication) {
        let verification = IdCompare::new(verification);

        let collection = add_unique_to_vec(verification, self.verification.clone());

        self.verification = collection;
    }

    /// remove all of the verification methods from the `DIDDocument`.
    pub fn clear_verification(&mut self) {
        self.verification.clear();
    }

    /// sets in a new `delegation` of type `Authentication` into the `DIDDocument`.
    pub fn update_delegation(&mut self, delegation: Authentication) {
        let delegation = IdCompare::new(delegation);

        let collection = add_unique_to_vec(delegation, self.delegation.clone());

        self.delegation = collection;
    }

    /// remove all of the capability delegations from the `DIDDocument`.
    pub fn clear_delegation(&mut self) {
        self.delegation.clear();
    }

    /// sets in a new `invocation` of type `Authentication` into the `DIDDocument`.
    pub fn update_invocation(&mut self, invocation: Authentication) {
        let invocation = IdCompare::new(invocation);

        let collection = add_unique_to_vec(invocation, self.invocation.clone());

        self.invocation = collection;
    }

    /// remove all of the capability invocations from the `DIDDocument`.
    pub fn clear_invocation(&mut self) {
        self.invocation.clear();
    }

    /// sets in a new `agreement` of type `Authentication` into the `DIDDocument`.
    pub fn update_agreement(&mut self, agreement: Authentication) {
        let agreement = IdCompare::new(agreement);

        let collection = add_unique_to_vec(agreement, self.agreement.clone());

        self.agreement = collection;
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
        self.updated = Some(Timestamp::now().to_string());
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
            created: Some(Timestamp::now().to_string()),
            updated: Some(Timestamp::now().to_string()),
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
