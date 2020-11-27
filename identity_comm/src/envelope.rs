use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};

pub enum EncryptionType {
    XC20P,
    A256GCM,
}

/// Non-repudiable authentication
pub fn pack_auth_msg_non_repudiable(
    message: String,
    _recipient_keys: Vec<String>,
    sender_keys: Vec<u8>,
    _encryption_type: EncryptionType,
) -> crate::Result<String> {
    let _signed_msg = sign(message, sender_keys);
    // packMessage(signedMsg, recipient_keys, Some(sender_keys), encryption_type)
    Ok(_signed_msg)
}

/// repudiable authentication
pub fn pack_auth_msg(
    _message: String,
    _recipient_keys: Vec<String>,
    _sender_keys: Option<Vec<u8>>,
    _encryption_type: EncryptionType,
) -> crate::Result<String> {
    // packMessage(message, recipientKeys, sender_keys, encryption_type)
    Ok("TODO".to_string())
}

/// Encrypt without authentication
pub fn pack_anon_msg(
    message: String,
    pub_key: Vec<u8>,
    nonce: Vec<u8>,
    encryption_type: EncryptionType,
) -> crate::Result<String> {
    pack_message(message, pub_key, nonce, None, encryption_type)
}

/// Non-repudiable signature with no encryption
pub fn pack_nonrepudiable_msg(message: String, sender_keys: Vec<u8>, _encryption_type: EncryptionType) -> String {
    sign(message, sender_keys)
}

// senderKeys = keypair
fn sign(msg: String, _sender_keys: Vec<u8>) -> String {
    println!("sign is not implemented, return raw msg");
    let signature = "Not implemented";
    json!({"message":msg,"signature":signature}).to_string()
}

// senderKeys = keypair
fn pack_message(
    msg: String,
    recipient_key: Vec<u8>,
    nonce: Vec<u8>,
    from_keys: Option<Vec<u8>>,
    _encryption_type: EncryptionType,
) -> crate::Result<String> {
    match from_keys {
        Some(_p) => {
            println!("encrypt and sign message");

            // Sender Authenticated Encryption
            // https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption
            //
            // For content encryption of the message, the following algorithms MUST be supported.
            // XC20P   -> XChaCha20Poly1305
            // A256GCM -> AES-GCM with a 256 bit key

            // XChaChaPoly::aead_cipher().seal(buf: &mut [u8], plain_len: usize, ad: &[u8], key: &[u8], nonce: &[u8])
            // example here: https://github.com/iotaledger/stronghold.rs/blob/42913086a86259ae32d99a1702592932bdaec03f/engine/crypto/tests/xchachapoly.rs#L52
            Ok("Result".into())
        }
        None => {
            let key = Key::from_slice(&recipient_key); // 32-bytes
            let aead = XChaCha20Poly1305::new(key);
            let nonce = XNonce::from_slice(&nonce); // 24-bytes; unique
            let ciphertext = aead
                .encrypt(nonce, msg.as_bytes().as_ref())
                .expect("encryption failure!");
            Ok(base64::encode(&ciphertext))
        }
    }
}

pub fn unpack_message(message: String, private_key: Vec<u8>, nonce: Vec<u8>) -> crate::Result<String> {
    let cipertext = base64::decode(&message)?;
    let key = Key::from_slice(&private_key); // 32-bytes
    let aead = XChaCha20Poly1305::new(key);
    let nonce = XNonce::from_slice(&nonce); // 24-bytes; unique
    let plaintext = aead.decrypt(nonce, cipertext.as_ref()).expect("decryption failure!");
    Ok(plaintext.iter().map(|&c| c as char).collect::<String>())
}
