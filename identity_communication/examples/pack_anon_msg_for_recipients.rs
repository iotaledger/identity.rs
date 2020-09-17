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
//! cargo run --example pack_anon_msg_for_recipients
//! ```

use identity_communication::did_comm::{DIDComm};
use identity_communication::envelope::pack_anon_msg_for_recipients;

fn main() {
    let alice = DIDComm {
        id: "123456".into(),
        comm_type: "https://didcomm.org/iota".into(),
        ..Default::default()
    }
    .init()
    .unwrap();

    println!("alice: {:?}", alice);
    
    let message = "I AM A PRIVATE ANONYM MESSAGE";
    
    let packedMsg = pack_anon_msg_for_recipients(message, "bob_public_key".to_string());
    
    println!("packedMsg: {:?}", packedMsg);
}