// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// use core::slice;
// use identity_core::crypto::KeyPair;
// use identity_core::crypto::PublicKey;
// use identity_core::crypto::SecretKey;
// use libjose::utils::ed25519_to_x25519_public;
// use libjose::utils::ed25519_to_x25519_secret;

// use super::*;
// use crate::envelope::Encrypted;
// use crate::envelope::EncryptionAlgorithm;
// use crate::envelope::SignatureAlgorithm;
// use crate::error::Result;
// use crate::message::Message;

// #[test]
// fn test_plaintext_roundtrip() {
//   let authentication_request = AuthenticationRequest::new(
//     "authentication/1.0/authenticationRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("htpps://example.com").unwrap(),
//     "please sign this".to_string(),
//   );
//   let plain_envelope_request = authentication_request.pack_plain().unwrap();
//   let request: AuthenticationRequest = plain_envelope_request.unpack().unwrap();
//   assert_eq!(format!("{:?}", request), format!("{:?}", authentication_request));
// }

// #[test]
// fn test_signed_roundtrip() {
//   let keypair = KeyPair::new_ed25519().unwrap();

//   let authentication_request = AuthenticationRequest::new(
//     "authentication/1.0/authenticationRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("htpps://example.com").unwrap(),
//     "please sign this".to_string(),
//   );
//   let signed_request = authentication_request
//     .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
//     .unwrap();

//   let request = signed_request
//     .unpack::<AuthenticationRequest>(SignatureAlgorithm::EdDSA, &keypair.public())
//     .unwrap();

//   assert_eq!(format!("{:?}", request), format!("{:?}", authentication_request));
// }

// fn ed25519_to_x25519(keypair: KeyPair) -> Result<(PublicKey, SecretKey)> {
//   Ok((
//     ed25519_to_x25519_public(keypair.public())?.to_vec().into(),
//     ed25519_to_x25519_secret(keypair.secret())?.to_vec().into(),
//   ))
// }

// fn ed25519_to_x25519_keypair(keypair: KeyPair) -> Result<KeyPair> {
//   // This is completely wrong but `type_` is never used around here
//   let type_ = keypair.type_();
//   let (public, secret) = ed25519_to_x25519(keypair)?;
//   Ok((type_, public, secret).into())
// }

// #[test]
// fn test_encrypted_roundtrip() {
//   let key_alice = KeyPair::new_ed25519().unwrap();
//   let key_alice = ed25519_to_x25519_keypair(key_alice).unwrap();

//   let key_bob = KeyPair::new_ed25519().unwrap();
//   let key_bob = ed25519_to_x25519_keypair(key_bob).unwrap();

//   let authentication_request = AuthenticationRequest::new(
//     "authentication/1.0/authenticationRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("htpps://example.com").unwrap(),
//     "please sign this".to_string(),
//   );
//   let recipients = slice::from_ref(key_alice.public());

//   let encoded_request: Encrypted = authentication_request
//     .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
//     .unwrap();

//   let decoded_request: AuthenticationRequest = encoded_request
//     .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
//     .unwrap();

//   assert_eq!(
//     format!("{:?}", decoded_request),
//     format!("{:?}", authentication_request)
//   );
// }
