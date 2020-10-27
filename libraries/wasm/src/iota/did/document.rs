use identity_core::{
    did::{DIDDocument, DID},
    key::{KeyData, KeyRelation, KeyType, PublicKey, PublicKeyBuilder},
    utils::encode_b58,
};
use identity_iota::{
    did::{IotaDID, IotaDocument as _IotaDocument},
    error::Error,
    utils::utf8_to_trytes,
};
use multihash::{Blake2b256, MultihashGeneric};
use std::str::FromStr;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct IotaDocument {
    document: _IotaDocument,
}

#[wasm_bindgen]
impl IotaDocument {
    #[wasm_bindgen(constructor)]
    pub fn new(did: String, auth_key: String) -> Self {
        console_error_panic_hook::set_once();
        let did = IotaDID::parse(did).unwrap();
        let key: DID = format!("{}#key-1", did).parse().unwrap();

        let public_key: PublicKey = PublicKeyBuilder::default()
            .id(key)
            .controller(DID::parse(did.clone()).unwrap())
            .key_type(KeyType::Ed25519VerificationKey2018)
            .key_data(KeyData::PublicKeyBase58(auth_key))
            .build()
            .unwrap();
        let iota_document = _IotaDocument::new(did, public_key).unwrap();
        IotaDocument {
            document: iota_document,
        }
    }

    #[wasm_bindgen(js_name = "TryFromDocument")]
    pub fn try_from_document(document_string: String) -> Self {
        console_error_panic_hook::set_once();
        //DIDDocument::from_str(&json_str).unwrap();
        let document = DIDDocument::from_str(&document_string).unwrap();
        let iota_doc = _IotaDocument::try_from_document(document.clone()).unwrap();
        let did: IotaDID = IotaDID::try_from_did(document.did().clone()).unwrap();

        let authentication: &PublicKey = document
            .resolve_key(0, KeyRelation::Authentication)
            .ok_or(Error::InvalidAuthenticationKey)
            .unwrap();
        _IotaDocument::check_authentication_key_id(authentication, &did).unwrap();

        IotaDocument { document: iota_doc }
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> String {
        console_error_panic_hook::set_once();
        self.document.to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn did(&self) -> String {
        self.document.did().to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn authentication_key(&self) -> String {
        let key = self
            .document
            .resolve_key(0, KeyRelation::Authentication)
            .expect("infallible");
        match key.key_data() {
            KeyData::PublicKeyBase58(_key) => _key.to_string(),
            _ => panic!("Can't get key as string"),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn create_diff_address(&self) -> String {
        let hash: MultihashGeneric<_> = Blake2b256::digest(self.authentication_key().as_bytes());
        let hash: MultihashGeneric<_> = Blake2b256::digest(hash.digest());

        let mut trytes: String = utf8_to_trytes(&encode_b58(hash.digest()));

        trytes.truncate(iota_constants::HASH_TRYTES_SIZE);
        trytes
    }
}

/*
TODO

impl IotaDocument {
    pub fn generate_ed25519_keypair() -> KeyPair {
        jcsed25519signature2020::new_keypair()
    }

    pub fn authentication_key_bytes(&self) -> Vec<u8> {
        self.authentication_key()
            .key_data()
            .try_decode()
            .transpose()
            .ok()
            .flatten()
            .unwrap_or_default()
    }

    pub fn sign(&mut self, secret: &SecretKey) -> Result<()> {
        let key: &PublicKey = self.authentication_key();

        let fragment: String = format!("{}", key.id());
        let options: SignatureOptions = SignatureOptions::new(fragment);

        match key.key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::sign_lds(&mut self.document, options, secret)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(())
    }

    pub fn verify(&self) -> Result<()> {
        let key: &PublicKey = self.authentication_key();

        match key.key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::verify_lds(&self.document)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(())
    }

    pub fn diff(&self, mut other: Document, secret: &SecretKey) -> Result<DIDDiff> {
        // Update the `updated` timestamp of the new document
        other.update_time();

        // Get the first authentication key from the document.
        let key: &PublicKey = self.authentication_key();

        let fragment: String = format!("{}", key.id());
        let options: SignatureOptions = SignatureOptions::new(fragment);

        // Create a diff of changes between the two documents.
        let mut diff: DIDDiff = DIDDiff {
            id: self.document.did().clone(),
            diff: self.document.diff(&other)?,
            proof: LdSignature::new("", options.clone()),
        };

        // Wrap the diff/document in a signable type.
        let mut target: LdWrite<DIDDiff> = LdWrite::new(&mut diff, &self.document);

        // Create and apply the signature
        match key.key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::sign_lds(&mut target, options, secret)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(diff)
    }

    pub fn verify_diff(&self, diff: &DIDDiff) -> Result<()> {
        // Wrap the diff/document in a verifiable type.
        let target: LdRead<DIDDiff> = LdRead::new(diff, &self.document);

        match self.authentication_key().key_type() {
            KeyType::Ed25519VerificationKey2018 => {
                jcsed25519signature2020::verify_lds(&target)?;
            }
            _ => {
                return Err(Error::InvalidAuthenticationKey);
            }
        }

        Ok(())
    }


}


*/
