//! Non-repudiable signature with no encryption
//!
//! In very specific use cases like the invitation protocol or
//! incredibly short lived connection (1 round trip only)
//! it’s necessary to provide data in a plaintext format to provide a key.
//! In these cases we will sign the data, but leave it unencrypted.
//!
//!
//! Run with:
//!
//! ```
//! cargo run --example pack_nonrepudiable_msg
//! ```

use identity_communication::{did_comm::DIDComm, envelope::pack_nonrepudiable_msg};

fn main() {
    let alice = DIDComm {
        id: "123456".into(),
        comm_type: "https://didcomm.org/iota".into(),
        ..Default::default()
    }
    .init()
    .unwrap();

    println!("alice: {:?}", alice);

    let message = "I AM A PUBLIC SIGNED MESSAGE";

    let packedMsg = pack_nonrepudiable_msg(message, alice);

    println!("packedMsg: {:?}", packedMsg);
}
