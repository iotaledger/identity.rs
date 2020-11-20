//! simple message
//!
//!
//! Run with:
//!
//! ```
//! cargo run --example simple_message
//! ```

use identity_comm::{did_comm::DIDComm, types::MessageType};
use identity_crypto::KeyPair;
use identity_iota::did::IotaDocument;

use identity_core::common::Timestamp;

fn main() {
    let alice = Account::new("alice".to_string());
    let bob = Account::new("bob".to_string());

    let message = DIDComm {
        id: "".into(),
        comm_type: MessageType::TrustPing,
        from: Some(alice.document.did().to_string()),
        to: Some(vec![bob.document.did().to_string()]),
        created_at: Some(Timestamp::now()),
        expires_at: Some(Timestamp::now()),
        body: Some("".into()),
    };

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
