//! simple message
//!
//!
//! Run with:
//!
//! ```
//! cargo run --example simple_message
//! ```

use identity_comm::{did_comm_builder::DIDCommBuilder, messages::MessageType};
use identity_core::did::DID;
use identity_crypto::KeyPair;
use identity_iota::did::IotaDocument;

use identity_core::common::Timestamp;

fn main() {
    let alice = Account::new("alice".to_string());
    println!("{}", alice.name);
    println!("{:?}", alice.keypair);
    let bob = Account::new("bob".to_string());

    let message = DIDCommBuilder::new()
        .id("123456")
        .comm_type(MessageType::TrustPing)
        .from(DID::parse(alice.document.did()).unwrap())
        .to(vec![DID::parse(bob.document.did()).unwrap()])
        .created_at(Timestamp::now())
        .expires_at(Timestamp::now())
        .body("".into())
        .build()
        .unwrap();

    println!("Created message: {}", message.to_string());

    println!("Send trust ping message")
}

struct Account {
    name: String,
    keypair: KeyPair,
    document: IotaDocument,
}

impl Account {
    fn new(name: String) -> Self {
        // Create keypair/DID document
        let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::generate_ed25519("key-1", None).unwrap();

        // Sign the document with the authentication method secret
        document.sign(keypair.secret()).unwrap();

        // Ensure the document proof is valid
        assert!(document.verify().is_ok());
        println!("Created DID: {}", document.did());

        Self {
            name,
            keypair,
            document,
        }
    }
}
