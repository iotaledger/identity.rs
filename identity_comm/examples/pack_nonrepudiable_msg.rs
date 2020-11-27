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
    did_comm_builder::DIDCommBuilder,
    envelope::{pack_nonrepudiable_msg, EncryptionType},
    messages::MessageType,
};

fn main() {
    let key = b"LbNeQyMtf2HF1D6oQWabsrd6wPX1CUhg";
    let alice = DIDCommBuilder::new()
        .id("123456")
        .comm_type(MessageType::TrustPing)
        .body("I AM A PUBLIC SIGNED MESSAGE".to_string())
        .build()
        .unwrap();
    println!("alice: {:?}", alice);

    let packed_msg = pack_nonrepudiable_msg(alice.to_string(), key.to_vec(), EncryptionType::XC20P);

    println!("packedMsg: {}", packed_msg);
}
