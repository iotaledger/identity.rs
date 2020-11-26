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

use identity_comm::{
    did_comm_builder::DIDCommBuilder, envelope::pack_nonrepudiable_msg, envelope::EncryptionType, messages::MessageType,
};

fn main() {
    let alice = DIDCommBuilder::new()
        .id("123456")
        .comm_type(MessageType::TrustPing)
        .build()
        .unwrap();

    println!("alice: {:?}", alice);

    let message = "I AM A PUBLIC SIGNED MESSAGE";

    let packed_msg = pack_nonrepudiable_msg(message.to_string(), alice, EncryptionType::XC20P);

    println!("packedMsg: {:?}", packed_msg);
}
