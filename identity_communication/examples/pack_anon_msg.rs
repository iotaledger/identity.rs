//! Encrypt with no authentication
//!
//! For privacy reasons or to meet the principle of least information,
//! it may be necessary to encrypt a message,
//! but does not provide authentication guarantees.
//!
//!
//! Run with:
//!
//! ```
//! cargo run --example pack_anon_msg
//! ```

use identity_communication::{
    did_comm::DIDComm,
    envelope::{pack_anon_msg, unpack_message, EncryptionType},
};

fn main() {
    let key = b"LbNeQyMtf2HF1D6oQWabsrd6wPX1CUhg";
    let nonce = b"extra long unique nonce!";
    let message = "I AM A MESSAGE FROM A ANONYM SENDER AND JUST ALICE CAN READ IT";

    println!("plaintext message: {:?}", message);
    let packed_msg = pack_anon_msg(message.into(), key.to_vec(), nonce.to_vec(), EncryptionType::XC20P).unwrap();

    println!("packedMsg: {:?}", packed_msg);

    let unpacked_message = unpack_message(packed_msg, key.to_vec(), nonce.to_vec()).unwrap();
    println!("unpacked_message: {:?}", unpacked_message);
}
