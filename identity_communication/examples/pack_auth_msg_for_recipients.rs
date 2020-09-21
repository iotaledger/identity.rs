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

use identity_communication::{did_comm::DIDComm, envelope::pack_auth_msg};

fn main() {
    let alice = DIDComm {
        id: "123456".into(),
        comm_type: "https://didcomm.org/iota".into(),
        ..Default::default()
    }
    .init()
    .unwrap();

    let bob = DIDComm {
        id: "789012".into(),
        comm_type: "https://didcomm.org/iota".into(),
        ..Default::default()
    }
    .init()
    .unwrap();

    println!("alice: {:?}", alice);
    println!("bob: {:?}", bob);

    let message = "I AM A PRIVATE SIGNED MESSAGE";

    let packedMsg = pack_auth_msg(message, "bob_public_key".to_string(), alice);

    println!("packedMsg: {:?}", packedMsg);
}
