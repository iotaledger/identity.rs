//! non-repudiable authentication
//!
//! To Encrypt a message for a recipient and sign the message
//! using a non-repudiable signature.
//!
//!
//! Run with:
//!
//! ```
//! cargo run --example pack_auth_msg_non_repudiable
//! ```

use identity_comm::{
    did_comm_builder::DIDCommBuilder,
    envelope::{pack_auth_msg_non_repudiable, EncryptionType},
    messages::MessageType,
};

fn main() {
    let alice_key = b"LbNeQyMtf2HF1D6oQWabsrd6wPX1CUhg";
    let alice = DIDCommBuilder::new()
        .id("123456")
        .comm_type(MessageType::TrustPing)
        .build()
        .unwrap();

    let bob = DIDCommBuilder::new()
        .id("789012")
        .comm_type(MessageType::TrustPing)
        .build()
        .unwrap();

    println!("alice: {:?}", alice);
    println!("bob: {:?}", bob);

    let message = "I AM A PRIVATE SIGNED MESSAGE";

    let packed_msg = pack_auth_msg_non_repudiable(
        message.to_string(),
        vec!["bob_public_key".to_string()],
        alice_key.to_vec(),
        EncryptionType::XC20P,
    );

    println!("packedMsg: {:?}", packed_msg);
}
