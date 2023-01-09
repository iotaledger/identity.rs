// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
// use crypto::hashes::sha::Sha256;
use identity_core::common::Timestamp;
use identity_core::convert::ToJson;
use identity_data_integrity::proof::ProofOptions;

use super::cryptosuite::CryptoSuite;
use super::Signable;
use crate::identifiers::KeyId;
use crate::key_storage::Ed25519SignatureAlgorithm;
use crate::key_storage::KeyStorage;
use crate::signature::Signature;
use crate::storage_error::SimpleStorageError;

pub struct EdDSA2020Suite {}

#[async_trait(?Send)]
impl<K> CryptoSuite<K> for EdDSA2020Suite
where
  K: KeyStorage,
  K::SigningAlgorithm: From<Ed25519SignatureAlgorithm>,
{
  async fn sign_data_integrity<'signable>(
    &self,
    key_id: &KeyId,
    data: Signable<'signable>,
    proof_options: ProofOptions,
    key_storage: &K,
  ) -> Result<Signature, SimpleStorageError>
  where
    'signable: 'async_trait,
  {
    // TODO: RDF transform the input instead.
    let serialized_data: Vec<u8> = data.to_jcs().unwrap();

    // TODO: Transform proof_options into a ProofConfig, and hash it.
    let _proof_config = ProofConfig::from(proof_options);

    // let _hasher = Sha256::default();

    let algorithm: K::SigningAlgorithm = Ed25519SignatureAlgorithm.into();

    let signature = key_storage
      .sign(key_id, &algorithm, serialized_data)
      .await
      .expect("TODO");

    // TODO: Should be errors. Also, we might want to check this in a separate method of this trait?
    // if method.type_() == &MethodType::MULTIKEY {
    //   let multikey = Multikey::from_multibase_string(
    //     match method.material().expect("should not return an option eventually") {
    //       VerificationMaterial::PublicKeyMultibase(multibase_str) => multibase_str.as_str().to_owned(),
    //       _ => todo!("unsupported verification material"),
    //     },
    //   );

    //   let (multicodec, public_key) = multikey.decode().expect("TODO");

    //   if multicodec != Multicodec::ED25519_PUB {
    //     todo!("unsupported key type for EdDSA 2020");
    //   }

    // }

    // assert_eq!(method.type_(), &MethodType::ED25519_VERIFICATION_KEY_2018);

    Ok(signature)
  }
}

/// A ProofConfig according to https://w3c-ccg.github.io/di-eddsa-2020/#proof-configuration.
struct ProofConfig {
  type_: &'static str,
  cryptosuite: Option<&'static str>,
  created: Option<Timestamp>,
}

impl From<ProofOptions> for ProofConfig {
  fn from(value: ProofOptions) -> Self {
    todo!()
  }
}
