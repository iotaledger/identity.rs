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
//   let keypair = KeyPair::new_ed25519().unwrap();
//   let resolution_request = ResolutionRequest::new(
//     "did-discovery/1.0/did-resolution/1.0/resolutionRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("https://example.com").unwrap(),
//   );
//   let resolotion_respone = ResolutionResponse::new(
//     "did-resolution/1.0/resolutionResponse".to_string(),
//     Uuid::new_v4(),
//     Document::from_keypair(&keypair).unwrap(),
//   );

//   let plain_envelope_request = resolution_request.pack_plain().unwrap();
//   let plain_envelope_response = resolotion_respone.pack_plain().unwrap();

//   let request: ResolutionRequest = plain_envelope_request.unpack().unwrap();
//   let response: ResolutionResponse = plain_envelope_response.unpack().unwrap();
//   assert_eq!(format!("{:?}", request), format!("{:?}", resolution_request));
//   assert_eq!(format!("{:?}", response), format!("{:?}", resolotion_respone));
// }

// #[test]
// fn test_signed_roundtrip() {
//   let keypair = KeyPair::new_ed25519().unwrap();

//   let resolution_request = ResolutionRequest::new(
//     "did-discovery/1.0/did-resolution/1.0/resolutionRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("https://example.com").unwrap(),
//   );
//   let resolotion_respone = ResolutionResponse::new(
//     "did-resolution/1.0/resolutionResponse".to_string(),
//     Uuid::new_v4(),
//     Document::from_keypair(&keypair).unwrap(),
//   );
//   let signed_envelope_request = resolution_request
//     .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
//     .unwrap();

//   let signed_envelope_response = resolotion_respone
//     .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
//     .unwrap();

//   let request = signed_envelope_request
//     .unpack::<ResolutionRequest>(SignatureAlgorithm::EdDSA, &keypair.public())
//     .unwrap();

//   let response = signed_envelope_response
//     .unpack::<ResolutionResponse>(SignatureAlgorithm::EdDSA, &keypair.public())
//     .unwrap();

//   assert_eq!(format!("{:?}", request), format!("{:?}", resolution_request));
//   assert_eq!(format!("{:?}", response), format!("{:?}", resolotion_respone));
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
//   let keypair = KeyPair::new_ed25519().unwrap();
//   let key_alice = KeyPair::new_ed25519().unwrap();
//   let key_alice = ed25519_to_x25519_keypair(key_alice).unwrap();

//   let key_bob = KeyPair::new_ed25519().unwrap();
//   let key_bob = ed25519_to_x25519_keypair(key_bob).unwrap();

//   let resolution_request = ResolutionRequest::new(
//     "did-discovery/1.0/did-resolution/1.0/resolutionRequest".to_string(),
//     Uuid::new_v4(),
//     Url::parse("https://example.com").unwrap(),
//   );
//   let resolotion_respone = ResolutionResponse::new(
//     "did-resolution/1.0/resolutionResponse".to_string(),
//     Uuid::new_v4(),
//     Document::from_keypair(&keypair).unwrap(),
//   );

//   let recipients = slice::from_ref(key_alice.public());

//   let encrypted_envelope_request: Encrypted = resolution_request
//     .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
//     .unwrap();

//   let encrypted_envelope_response: Encrypted = resolotion_respone
//     .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
//     .unwrap();

//   let request: ResolutionRequest = encrypted_envelope_request
//     .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
//     .unwrap();

//   let response: ResolutionResponse = encrypted_envelope_response
//     .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
//     .unwrap();

//   assert_eq!(format!("{:?}", request), format!("{:?}", resolution_request));
//   assert_eq!(format!("{:?}", response), format!("{:?}", resolotion_respone));
// }
