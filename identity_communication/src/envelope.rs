use crate::did_comm;

/// repudiable authentication
pub fn pack_auth_msg_for_recipients(message: &str, recipientKeys: String, senderKeys: did_comm::DIDComm) -> String {
    packMessage(message, recipientKeys , Some(senderKeys))
}

/// Non-repudiable authentication
pub fn pack_auth_msg_for_recipients_non_repudiable(message: &str, recipientKeys: String, senderKeys: did_comm::DIDComm) -> String {
    let signedMsg = sign(message, senderKeys.clone());
    packMessage(&*signedMsg, recipientKeys, Some(senderKeys))
}


/// Encrypt with no authentication
pub fn pack_anon_msg_for_recipients(message: &str, pub_key: String) -> String {
    packMessage(message, pub_key, None)
}

/// Non-repudiable signature with no encryption

pub fn pack_nonrepudiable_msg_for_anyone(message: &str, did_comm: did_comm::DIDComm) -> String {
    sign(message, did_comm)
}

// senderKeys = keypair
fn sign(msg: &str, senderKeys: did_comm::DIDComm) -> String{
    println!("signed");
    "will be implemented soon".to_string()
}

// senderKeys = keypair
fn packMessage(msg: &str, recipientKeys: String, fromKeys: Option<did_comm::DIDComm>) -> String{
    match fromKeys {
        Some(p) => println!("encrypt and sign message"),
        None => println!("encrypt message"),
    }
    "will be implemented soon".to_string()
}