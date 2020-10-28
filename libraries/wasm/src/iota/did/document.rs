use identity_core::{
    did::{DIDDocument, DID},
    key::{KeyData, KeyType, PublicKey, PublicKeyBuilder},
};
use identity_iota::did::{IotaDID, IotaDocument as _IotaDocument};

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
        let document = DIDDocument::from_str(&document_string).unwrap();
        let iota_doc = _IotaDocument::try_from_document(document).unwrap();
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
        let key = self.document.authentication_key();
        match key.key_data() {
            KeyData::PublicKeyBase58(_key) => _key.to_string(),
            _ => panic!("Can't get key as string"),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn create_diff_address(&self) -> String {
        _IotaDocument::create_diff_address_hash(self.authentication_key().as_bytes())
    }

    //How can we convert the key?
    // #[wasm_bindgen(js_name = "sign")]
    // pub fn sign(&mut self, secret: String) -> String {
    //     // let keypair = identity_crypto::key::KeyPair { public: b"", secret };
    //     use identity_crypto::key::KeyPair;
    //     // use identity_proof::signature::jcsed25519signature2020;
    //     // let key = jcsed25519signature2020::seckey(secret.as_bytes()).unwrap();
    //     // let keypair: KeyPair = jcsed25519signature2020::new_keypair();
    //     // self.document.sign(key.as_ref().to_vec().into()).unwrap();
    //     self.document.sign(secret.as_bytes().as_ref().to_vec().into()).unwrap();
    //     self.document.to_string()
    //     // self.document.sign(secret.as_bytes()).unwrap()
    // }
    #[wasm_bindgen(js_name = "verify")]
    pub fn verify(&self) -> bool {
        matches!(self.document.verify(), Ok(_))
    }
}

/*
TODO

impl IotaDocument {

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
