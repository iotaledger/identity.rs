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
//   let did_request = FeaturesRequest::new(
//     "did-discovery/1.0/didRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("https://example.com").unwrap(),
//   );
//   let did_response = FeaturesResponse::new(
//     "did-discovery/1.0/didResponse".to_string(),
//     Uuid::new_v4(),
//     vec!["trust-ping/1.0".to_string(), "did-discovery/1.0".to_string()],
//   );

//   let plain_envelope_request = did_request.pack_plain().unwrap();
//   let plain_envelope_response = did_response.pack_plain().unwrap();

//   let request: FeaturesRequest = plain_envelope_request.unpack().unwrap();
//   let response: FeaturesResponse = plain_envelope_response.unpack().unwrap();

//   assert_eq!(format!("{:?}", request), format!("{:?}", did_request));
//   assert_eq!(format!("{:?}", response), format!("{:?}", did_response));
// }

// #[test]
// fn test_signed_roundtrip() {
//   let keypair = KeyPair::new_ed25519().unwrap();

//   let did_request = FeaturesRequest::new(
//     "did-discovery/1.0/didRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("https://example.com").unwrap(),
//   );
//   let did_response = FeaturesResponse::new(
//     "did-discovery/1.0/didResponse".to_string(),
//     Uuid::new_v4(),
//     vec!["trust-ping/1.0".to_string(), "did-discovery/1.0".to_string()],
//   );
//   let signed_request = did_request
//     .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
//     .unwrap();

//   let signed_response = did_response
//     .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
//     .unwrap();

//   let request = signed_request
//     .unpack::<FeaturesRequest>(SignatureAlgorithm::EdDSA, &keypair.public())
//     .unwrap();

//   let response = signed_response
//     .unpack::<FeaturesResponse>(SignatureAlgorithm::EdDSA, &keypair.public())
//     .unwrap();

//   assert_eq!(format!("{:?}", request), format!("{:?}", did_request));
//   assert_eq!(format!("{:?}", response), format!("{:?}", did_response));
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

//   let did_request = FeaturesRequest::new(
//     "did-discovery/1.0/didRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("https://example.com").unwrap(),
//   );
//   let did_response = FeaturesResponse::new(
//     "did-discovery/1.0/didResponse".to_string(),
//     Uuid::new_v4(),
//     vec!["trust-ping/1.0".to_string(), "did-discovery/1.0".to_string()],
//   );
//   let recipients = slice::from_ref(key_alice.public());

//   let encoded_request: Encrypted = did_request
//     .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
//     .unwrap();

//   let encoded_response: Encrypted = did_response
//     .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
//     .unwrap();

//   let decoded_request: FeaturesRequest = encoded_request
//     .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
//     .unwrap();

//   let decoded_response: FeaturesResponse = encoded_response
//     .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
//     .unwrap();

//   assert_eq!(format!("{:?}", decoded_request), format!("{:?}", did_request));
//   assert_eq!(format!("{:?}", decoded_response), format!("{:?}", did_response));
// }
