use crate::{
    storage::{CacheFile, HuffmanCodec},
    Error, Result,
};
use core::convert::TryFrom;
use identity_core::{resolver::resolve, utils::decode_hex};
use identity_crypto::{KeyPair, PublicKey, SecretKey};
use identity_diff::Diff;
use identity_iota::{
    client::Client,
    did::{DIDDiff, IotaDocument},
};
use identity_proof::signature::jcsed25519signature2020;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A struct to store the state of an identity with all updates
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    keys: Vec<Key>,
    documents: Vec<DocState>,
    latest_doc: IotaDocument,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DocState {
    pub document: IotaDocument,
    pub diffs: Vec<DIDDiff>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Key {
    pub public_key: String,
    pub private_key: String,
}

impl State {
    /// Creates a new state and writes it to file
    pub fn new(keypair: KeyPair, document: IotaDocument) -> Result<Self> {
        let state = Self {
            keys: vec![Key {
                public_key: keypair.public().to_string(),
                private_key: keypair.secret().to_string(),
            }],
            documents: vec![DocState {
                document: document.clone(),
                diffs: vec![],
            }],
            latest_doc: document,
        };
        Ok(state)
    }
    /// Write state to file
    pub fn write_to_file(&self, path: &str) -> Result<()> {
        let file = CacheFile::new(path.to_string());
        let data = serde_json::to_string_pretty(&self).map_err(crate::Error::EncodeError)?;
        // .as_bytes()
        // .to_vec();
        // Comment out for debugging, also in read_from_file
        let data = HuffmanCodec::compress(data)?;
        file.write_cache_file(data)?;
        Ok(())
    }
    /// Create state from file
    pub fn read_from_file(path: &str) -> Result<Self> {
        let c = CacheFile::new(path.into());
        let data = c.read_cache_file()?;
        //Comment out for debugging
        let data = HuffmanCodec::decompress(&data)?;
        let state: Self = Self::from_str(&data)?;
        // let state: Self = serde_json::from_str(&String::from_utf8(data)?)?;
        Ok(state)
    }
    /// Get the latest keypair
    pub fn keypair(&self) -> Result<KeyPair> {
        let public: PublicKey = decode_hex(
            &self
                .keys
                .last()
                .ok_or_else(|| Error::StateError("Can't get last keypairs".into()))?
                .public_key,
        )?
        .into();
        let private: SecretKey = decode_hex(
            &self
                .keys
                .last()
                .ok_or_else(|| Error::StateError("Can't get last keypairs".into()))?
                .private_key,
        )?
        .into();
        Ok(KeyPair::new(public, private))
    }
    /// Get all stored keypairs
    pub fn keypairs(&self) -> Result<Vec<KeyPair>> {
        let mut r = Vec::new();
        for key in self.keys.clone() {
            let pu = decode_hex(&key.public_key)?.into();
            let pr = decode_hex(&key.private_key)?.into();
            r.push(KeyPair::new(pu, pr));
        }
        Ok(r)
    }
    /// Get all stored keypairs
    pub fn keypairs_as_string(&self) -> Vec<Key> {
        self.keys.clone()
    }
    /// Generates a new Ed25519 keypair and stores it
    pub fn new_keypair(&mut self) -> KeyPair {
        let keypair: KeyPair = jcsed25519signature2020::new_keypair();
        self.update_keypair(&keypair);
        keypair
    }
    /// Updates the latest keypair
    pub fn update_keypair(&mut self, keypair: &KeyPair) {
        self.keys.push(Key {
            public_key: keypair.public().to_string(),
            private_key: keypair.secret().to_string(),
        });
    }
    /// Get latest document with diffs applied
    pub fn latest_doc(&self) -> IotaDocument {
        self.latest_doc.clone()
    }
    /// Get all documents with their diffs
    pub fn documents(&self) -> Vec<DocState> {
        self.documents.clone()
    }
    /// Add a document to the state
    pub fn add_document(&mut self, document: IotaDocument) {
        self.latest_doc = document.clone();
        self.documents.push(DocState {
            document,
            diffs: vec![],
        })
    }
    /// Add a diff to the state
    pub fn add_diff(&mut self, diff: DIDDiff) -> Result<()> {
        // update latest doc
        self.latest_doc = IotaDocument::try_from(self.latest_doc.merge(diff.diff.clone())?)?;
        let len = self.documents.len() - 1;
        self.documents[len].diffs.push(diff);
        Ok(())
    }
    /// Get the length from the latest diff chain
    pub fn latest_diffchain_len(&self) -> Result<usize> {
        Ok(self
            .documents
            .last()
            .ok_or_else(|| Error::StateError("Unable to get document".into()))?
            .diffs
            .len())
    }
    /// Check if state is synced with the Tangle
    pub async fn is_synced(&self, client: Client) -> Result<bool> {
        let res = resolve(&self.latest_doc().did().to_string(), Default::default(), &client).await?;
        Ok(IotaDocument::try_from(
            res.document
                .ok_or_else(|| Error::StateError("Resolving failed".into()))?,
        )? == self.latest_doc())
    }
}

/// Converts a `State` into a string using the `to_string()` method.
impl ToString for State {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize state")
    }
}

/// Takes a &str and converts it into a `State` given the proper format.
impl FromStr for State {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(crate::Error::DecodeJSON)
    }
}
