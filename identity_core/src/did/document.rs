use derive_builder::Builder;
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Diff, Builder)]
#[builder(pattern = "owned")]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    #[builder(setter(into))]
    pub context: OneOrMany<Context>,
    #[builder(try_setter)]
    pub id: DID,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(should_ignore)]
    #[builder(default, setter(into, strip_option))]
    pub created: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub updated: Option<Timestamp>,
    #[serde(rename = "publicKey", skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub public_keys: Vec<PublicKey>,
    #[serde(rename = "authentication", skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub auth: Vec<Authentication>,
    #[serde(rename = "assertionMethod", skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub assert: Vec<Authentication>,
    #[serde(rename = "verificationMethod", skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub verification: Vec<Authentication>,
    #[serde(rename = "capabilityDelegation", skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub delegation: Vec<Authentication>,
    #[serde(rename = "capabilityInvocation", skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub invocation: Vec<Authentication>,
    #[serde(rename = "keyAgreement", skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub agreement: Vec<Authentication>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    #[builder(default)]
    pub services: Vec<Service>,
    #[serde(flatten)]
    #[builder(default)]
    pub metadata: HashMap<String, String>,
}

impl DIDDocument {
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
    pub fn update_auth(&mut self, auth: impl Into<Authentication>) {
        self.auth.set_unique(auth.into());
    }

    /// remove all of the authentications from the `DIDDocument`.
    pub fn clear_auth(&mut self) {
        self.auth.clear();
    }

    /// sets in a new `assert` of type `Authentication` into the `DIDDocument`.
    pub fn update_assert(&mut self, assert: impl Into<Authentication>) {
        self.assert.set_unique(assert.into());
    }

    /// remove all of the assertion methods from the `DIDDocument`.
    pub fn clear_assert(&mut self) {
        self.assert.clear();
    }

    /// sets in a new `verification` of type `Authentication` into the `DIDDocument`.
    pub fn update_verification(&mut self, verification: impl Into<Authentication>) {
        self.verification.set_unique(verification.into());
    }

    /// remove all of the verification methods from the `DIDDocument`.
    pub fn clear_verification(&mut self) {
        self.verification.clear();
    }

    /// sets in a new `delegation` of type `Authentication` into the `DIDDocument`.
    pub fn update_delegation(&mut self, delegation: impl Into<Authentication>) {
        self.delegation.set_unique(delegation.into());
    }

    /// remove all of the capability delegations from the `DIDDocument`.
    pub fn clear_delegation(&mut self) {
        self.delegation.clear();
    }

    /// sets in a new `invocation` of type `Authentication` into the `DIDDocument`.
    pub fn update_invocation(&mut self, invocation: impl Into<Authentication>) {
        self.invocation.set_unique(invocation.into());
    }

    /// remove all of the capability invocations from the `DIDDocument`.
    pub fn clear_invocation(&mut self) {
        self.invocation.clear();
    }

    /// sets in a new `agreement` of type `Authentication` into the `DIDDocument`.
    pub fn update_agreement(&mut self, agreement: impl Into<Authentication>) {
        self.agreement.set_unique(agreement.into());
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

    pub fn set_metadata<T, U>(&mut self, key: T, value: U)
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn remove_metadata(&mut self, key: &str) {
        self.metadata.remove(key);
    }

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    /// initialize the `created` and `updated` timestamps to publish the did document.
    pub fn init_timestamps(&mut self) {
        self.created = Some(Timestamp::now());
        self.updated = Some(Timestamp::now());
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
