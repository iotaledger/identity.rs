//! non-repudiable authentication
//!
//! To Encrypt a message for a recipient and sign the message
//! using a non-repudiable signature.
//!
//!
//! Run with:
//!
//! ```
//! cargo run --example pack_auth_msg
//! ```

use identity_communication::{did_comm::DIDComm, envelope::pack_auth_msg_non_repudiable};

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

    let packedMsg = pack_auth_msg_non_repudiable(message, "bob_public_key".to_string(), alice);

    println!("packedMsg: {:?}", packedMsg);
}
