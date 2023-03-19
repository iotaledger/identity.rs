// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_verification::VerificationMethod;
use seahash::SeaHasher;
use std::hash::Hasher;

use super::KeyIdStorageError;

/// Unique identifier of a [`VerificationMethod`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MethodDigest {
  /// Version of hashing.
  version: u8,
  /// Hash value.
  value: u64,
}

impl MethodDigest {
  /// Creates a new [`MethodDigest`].
  pub fn new(verification_method: &VerificationMethod) -> identity_verification::Result<Self> {
    let mut hasher: SeaHasher = SeaHasher::new();
    let fragment: &str = verification_method
      .id()
      .fragment()
      .ok_or(identity_verification::Error::MissingIdFragment)?;

    let method_data: Vec<u8> = verification_method.data().try_decode()?;
    hasher.write(fragment.as_bytes());
    hasher.write(&method_data);
    let key_hash: u64 = hasher.finish();
    Ok(Self {
      version: 0,
      value: key_hash,
    })
  }

  /// Packs [`MethodDigest`] into bytes.
  pub fn pack(&self) -> Vec<u8> {
    let mut pack: Vec<u8> = vec![self.version];
    pack.append(&mut self.value.to_le_bytes().to_vec());
    pack
  }

  /// Unpacks bytes into [`MethodDigest`].
  pub fn unpack(bytes: Vec<u8>) -> crate::key_id_storage::KeyIdStorageResult<Self> {
    if bytes.len() != 9 {
      return Err(KeyIdStorageError::new(super::KeyIdStorageErrorKind::SerializationError));
    }
    let version: u8 = bytes[0];
    if version != 0 {
      return Err(KeyIdStorageError::new(super::KeyIdStorageErrorKind::SerializationError));
    }
    let value_le_bytes: [u8; 8] = bytes[1..9]
      .try_into()
      .map_err(|_| KeyIdStorageError::new(super::KeyIdStorageErrorKind::SerializationError))?;
    let value: u64 = u64::from_le_bytes(value_le_bytes);
    Ok(Self { version, value })
  }
}

#[cfg(test)]
mod test {
  use crate::key_id_storage::KeyIdStorageError;
  use crate::key_id_storage::KeyIdStorageErrorKind;
  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_core::json;
  use identity_core::utils::BaseEncoding;
  use identity_did::CoreDID;
  use identity_verification::VerificationMethod;
  use serde_json::Value;

  use super::MethodDigest;

  #[test]
  pub fn hash() {
    // These values should be tested in the bindings too.
    let a: Value = json!(
      {
        "id": "did:example:HHoh9NQC9AUsK15Jyyq53VTujxEUizKDXRXd7zbT1B5u#frag_1",
        "controller": "did:example:HHoh9NQC9AUsK15Jyyq53VTujxEUizKDXRXd7zbT1B5u",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "zHHoh9NQC9AUsK15Jyyq53VTujxEUizKDXRXd7zbT1B5u"
      }
    );
    let verification_method: VerificationMethod = VerificationMethod::from_json_value(a).unwrap();
    let method_digest: MethodDigest = MethodDigest::new(&verification_method).unwrap();
    let method_digest_expected: MethodDigest = MethodDigest {
      version: 0,
      value: 9634551232492878922,
    };
    assert_eq!(method_digest, method_digest_expected);

    let packed: Vec<u8> = method_digest.pack();
    let packed_expected: Vec<u8> = vec![0, 74, 60, 10, 199, 76, 205, 180, 133];
    assert_eq!(packed, packed_expected);
  }

  #[test]
  pub fn pack() {
    let verification_method: VerificationMethod = create_verification_method();
    let method_digest: MethodDigest = MethodDigest::new(&verification_method).unwrap();
    let packed: Vec<u8> = method_digest.pack();
    let method_digest_unpacked: MethodDigest = MethodDigest::unpack(packed).unwrap();
    assert_eq!(method_digest, method_digest_unpacked);
  }

  #[test]
  pub fn unpack() {
    let packed: Vec<u8> = vec![0, 255, 212, 82, 63, 57, 19, 134, 193];
    let method_digest_unpacked: MethodDigest = MethodDigest::unpack(packed).unwrap();
    let method_digest_expected: MethodDigest = MethodDigest {
      version: 0,
      value: 13944854432795776255,
    };
    assert_eq!(method_digest_unpacked, method_digest_expected);
  }

  #[test]
  pub fn invalid_unpack() {
    let packed: Vec<u8> = vec![1, 255, 212, 82, 63, 57, 19, 134, 193];
    let method_digest_unpacked = MethodDigest::unpack(packed).unwrap_err();
    let _expected_error = KeyIdStorageError::new(KeyIdStorageErrorKind::SerializationError);
    assert!(matches!(method_digest_unpacked, _expected_error));

    // Vec size > 9.
    let packed: Vec<u8> = vec![1, 255, 212, 82, 63, 57, 19, 134, 193, 200];
    let method_digest_unpacked = MethodDigest::unpack(packed).unwrap_err();
    let _expected_error = KeyIdStorageError::new(KeyIdStorageErrorKind::SerializationError);
    assert!(matches!(method_digest_unpacked, _expected_error));

    // Vec size < 9.
    let packed: Vec<u8> = vec![1, 255, 212, 82, 63, 57, 19, 134];
    let method_digest_unpacked = MethodDigest::unpack(packed).unwrap_err();
    let _expected_error = KeyIdStorageError::new(KeyIdStorageErrorKind::SerializationError);
    assert!(matches!(method_digest_unpacked, _expected_error));

    // Vec size 0;
    let packed: Vec<u8> = vec![];
    let method_digest_unpacked = MethodDigest::unpack(packed).unwrap_err();
    let _expected_error = KeyIdStorageError::new(KeyIdStorageErrorKind::SerializationError);
    assert!(matches!(method_digest_unpacked, _expected_error));
  }

  fn create_verification_method() -> VerificationMethod {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let did: CoreDID =
      CoreDID::parse(format!("did:example:{}", BaseEncoding::encode_base58(keypair.public()))).unwrap();
    VerificationMethod::new(did, KeyType::Ed25519, keypair.public(), "frag_1").unwrap()
  }
}
