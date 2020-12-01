//! Alice sends his own DIDDocument as challenge (could also be anything else)
//! which Bob returns signed with his own key
//!
//! Run with:
//!
//! ```
//! cargo run --example auth_message
//! ```

use did_url::DID;
use identity_comm::{
    did_comm_builder::DIDCommBuilder,
    messages::{auth_message::AuthMessage, MessageType},
};
use identity_core::{common::Timestamp, crypto::KeyPair};
use identity_iota::did::IotaDocument;
use std::str::FromStr;

fn main() {
    let alice = Account::new("alice".to_string());
    let bob = Account::new("bob".to_string());
    println!("{}", alice.name);
    println!("{:?}", alice.keypair);
    let mut auth_payload = AuthMessage::create_with_doc(alice.document.clone());
    auth_payload.request_response();
    let request = DIDCommBuilder::new()
        .id("123")
        .comm_type(MessageType::AuthMessage)
        .from(DID::parse(alice.document.id()).unwrap())
        .to(vec![DID::parse(bob.document.id()).unwrap()])
        .created_at(Timestamp::now())
        .expires_at(Timestamp::now())
        .body(auth_payload.to_string())
        .build()
        .unwrap();
    println!("Alice request message: {}", request.to_string());
    println!("Alice send auth request to bob");
    let mut bob_auth_payload =
        AuthMessage::create_with_doc(AuthMessage::from_str(&request.body.unwrap()).unwrap().challenge);
    bob_auth_payload.sign(bob.keypair, bob.document.clone());
    let response = DIDCommBuilder::new()
        .id("456")
        .comm_type(MessageType::AuthMessage)
        .from(DID::parse(bob.document.id()).unwrap())
        .to(vec![DID::parse(request.from.unwrap()).unwrap()])
        .created_at(Timestamp::now())
        .expires_at(Timestamp::now())
        .body(bob_auth_payload.to_string())
        .build()
        .unwrap();
    println!("Bob response message: {}", response.to_string());
    println!("Bob response to the request with a signed auth message");

    //Let Alice verify the signature from Bob
    println!("{}", response.body.clone().unwrap());
    let bob_response_auth = AuthMessage::from_str(&response.body.unwrap()).unwrap();
    // We need bobs document to verify the signature with his public key
    // In a real world app, you need resolve bobs did document to get the current key
    println!(
        "Bobs signature is valid {}",
        bob_response_auth.verify(bob.document).is_ok()
    );
}

struct Account {
    name: String,
    keypair: KeyPair,
    document: IotaDocument,
}

impl Account {
    fn new(name: String) -> Self {
        // Create keypair/DID document
        let (mut document, keypair): (IotaDocument, KeyPair) =
            IotaDocument::generate_ed25519("key-1", "main", None).unwrap();

        // Sign the document with the authentication method secret
        document.sign(keypair.secret()).unwrap();

        // Ensure the document proof is valid
        assert!(document.verify().is_ok());
        println!("Created {}s DID: {}", name, document.id());

        Self {
            name,
            keypair,
            document,
        }
    }
}
