//! repudiable authentication
//!
//! pack_auth_msg_for_recipients(message, recipientKeyList, senderKeyPair)
//! should be the default method used.
//! This example shows how to use repudiable authentication
//! to pack a message for the recipient.
//!
//!
//! Run with:
//!
//! ```
//! cargo run --example pack_auth_msg_for_recipients
//! ```

use identity_comm::{
    did_comm_builder::DIDCommBuilder,
    envelope::{pack_auth_msg, EncryptionType},
    messages::MessageType,
};

fn main() {
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

    let message = "I AM A PRIVATE SIGNED MESSAGE".to_string();

    let packedMsg = pack_auth_msg(
        message,
        vec!["bob_public_key".to_string()],
        Some(alice),
        EncryptionType::XC20P,
    );

    println!("packedMsg: {:?}", packedMsg);
}
