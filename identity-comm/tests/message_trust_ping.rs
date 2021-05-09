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
//   let ping = TrustPing::new("trust-ping/1.0/ping".to_string());

//   let plain_envelope_ping = ping.pack_plain().unwrap();

//   let tp: TrustPing = plain_envelope_ping.unpack().unwrap();

//   assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
// }

// #[test]
// fn test_signed_roundtrip() {
//   let keypair = KeyPair::new_ed25519().unwrap();

//   let ping = TrustPing::new("trust-ping/1.0/ping".to_string());

//   let signed_envelope_ping = ping.pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair).unwrap();

//   let tp = signed_envelope_ping
//     .unpack::<TrustPing>(SignatureAlgorithm::EdDSA, &keypair.public())
//     .unwrap();

//   assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
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

//   let ping = TrustPing::new("trust-ping/1.0/ping".to_string());

//   let recipients = slice::from_ref(key_alice.public());

//   let encoded_envelope_ping: Encrypted = ping
//     .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
//     .unwrap();

//   let tp: TrustPing = encoded_envelope_ping
//     .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
//     .unwrap();

//   assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
// }
