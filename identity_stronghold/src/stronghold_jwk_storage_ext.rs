// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wrapper around [`StrongholdSecretManager`](StrongholdSecretManager).

use anyhow::Context;
use async_trait::async_trait;
use identity_storage::key_storage::JwkStorage;
use identity_storage::JwkGenOutput;
use identity_storage::JwkStorageExt;
use identity_storage::KeyId;
use identity_storage::KeyStorageError;
use identity_storage::KeyStorageErrorKind;
use identity_storage::KeyStorageResult;
use identity_storage::KeyType;
use identity_storage::ProofUpdateCtx;
use identity_verification::jwk::BlsCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParamsEc;
use identity_verification::jwu;
use iota_stronghold::procedures::FatalProcedureError;
use iota_stronghold::procedures::Products;
use iota_stronghold::procedures::Runner as _;
use iota_stronghold::Location;
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use std::str::FromStr;
use zeroize::Zeroizing;
use zkryptium::bbsplus::keys::BBSplusPublicKey;
use zkryptium::bbsplus::keys::BBSplusSecretKey;
use zkryptium::bbsplus::signature::BBSplusSignature;
use zkryptium::keys::pair::KeyPair;
use zkryptium::schemes::algorithms::BbsBls12381Sha256;
use zkryptium::schemes::algorithms::BbsBls12381Shake256;
use zkryptium::schemes::generics::Signature;

use crate::stronghold_key_type::*;
use crate::utils::*;
use crate::StrongholdStorage;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwkStorageExt for StrongholdStorage {
  async fn generate_bbs(&self, key_type: KeyType, alg: ProofAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    let key_type = StrongholdKeyType::try_from(&key_type)?;

    if !matches!(key_type, StrongholdKeyType::BLS12381G2) {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
          .with_custom_message(format!("{key_type} is not supported")),
      );
    }

    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;

    let kid: KeyId = random_key_id();
    let target_key_location = Location::generic(
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      kid.to_string().as_bytes().to_vec(),
    );
    let jwk = client
      .exec_proc([], &target_key_location, |_| {
        let (sk, pk) = generate_bbs_plus_key_pair(alg).map_err(|e| FatalProcedureError::from(e.to_string()))?;
        let mut jwk = encode_bls_jwk(&sk, &pk);
        jwk.set_alg(alg.to_string());
        jwk.set_kid(jwk.thumbprint_sha256_b64());
        // Safety: jkw.kty can only be "ec".
        let public_jwk = jwk.to_public().expect("should only panic if kty == oct");

        Ok(Products {
          output: public_jwk,
          secret: Zeroizing::new(sk.to_bytes().to_vec()),
        })
      })
      .map_err(|e| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("Failed to execute stronghold procedure")
          .with_source(e)
      })?;

    Ok(JwkGenOutput::new(kid, jwk))
  }

  async fn sign_bbs(
    &self,
    key_id: &KeyId,
    data: &[Vec<u8>],
    header: &[u8],
    public_key: &Jwk,
  ) -> KeyStorageResult<Vec<u8>> {
    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;

    // Extract the required alg from the given public key
    let alg = public_key
      .alg()
      .ok_or(KeyStorageErrorKind::UnsupportedProofAlgorithm)
      .and_then(|alg_str| {
        ProofAlgorithm::from_str(alg_str).map_err(|_| KeyStorageErrorKind::UnsupportedProofAlgorithm)
      })?;

    if matches!(alg, ProofAlgorithm::BLS12381_SHA256 | ProofAlgorithm::BLS12381_SHAKE256) {
      let ec_params = public_key.try_ec_params().map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message(format!("expected a Jwk with EC params in order to sign with {alg}"))
          .with_source(err)
      })?;
      if ec_params.crv != BlsCurve::BLS12381G2.to_string() {
        return Err(
          KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message(format!(
            "expected Jwk with EC {} crv in order to generate the proof with {alg}",
            BlsCurve::BLS12381G2
          )),
        );
      }
    } else {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::UnsupportedProofAlgorithm)
          .with_custom_message(format!("{alg} is not supported")),
      );
    }

    // Check `key_id` exists in store.
    if !self.exists(key_id).await? {
      return Err(KeyStorageError::new(KeyStorageErrorKind::KeyNotFound));
    }

    let pk = jwk_to_bbs_plus_pk(public_key)
      .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(e))?;

    let sk_location = Location::Generic {
      vault_path: IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      record_path: key_id.to_string().as_bytes().to_vec(),
    };

    client
      .get_guards([sk_location], |[sk]| {
        let sk = BBSplusSecretKey::from_bytes(&sk.borrow()).map_err(|e| FatalProcedureError::from(e.to_string()))?;
        let signature_result = match alg {
          ProofAlgorithm::BLS12381_SHA256 => {
            Signature::<BbsBls12381Sha256>::sign(Some(data), &sk, &pk, Some(header)).map(|s| s.to_bytes())
          }
          ProofAlgorithm::BLS12381_SHAKE256 => {
            Signature::<BbsBls12381Shake256>::sign(Some(data), &sk, &pk, Some(header)).map(|s| s.to_bytes())
          }
          // Safety: Already checked it's either of the two handled variants
          _ => unreachable!(),
        }
        .map_err(|e| FatalProcedureError::from(e.to_string()))?;
        Ok(signature_result)
      })
      .map(|sig| sig.to_vec())
      .map_err(|e| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("Signature failed")
          .with_source(e)
      })
  }

  async fn update_signature(
    &self,
    key_id: &KeyId,
    public_key: &Jwk,
    signature: &[u8; BBSplusSignature::BYTES],
    ctx: ProofUpdateCtx,
  ) -> KeyStorageResult<[u8; BBSplusSignature::BYTES]> {
    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;

    let ProofUpdateCtx {
      old_start_validity_timeframe,
      new_start_validity_timeframe,
      old_end_validity_timeframe,
      new_end_validity_timeframe,
      index_start_validity_timeframe,
      index_end_validity_timeframe,
      number_of_signed_messages,
    } = ctx;

    // Extract the required alg from the given public key
    let alg = public_key
      .alg()
      .ok_or(KeyStorageErrorKind::UnsupportedProofAlgorithm)
      .and_then(|alg_str| {
        ProofAlgorithm::from_str(alg_str).map_err(|_| KeyStorageErrorKind::UnsupportedProofAlgorithm)
      })?;

    if matches!(alg, ProofAlgorithm::BLS12381_SHA256 | ProofAlgorithm::BLS12381_SHAKE256) {
      let ec_params = public_key.try_ec_params().map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message(format!("expected a Jwk with EC params in order to sign with {alg}"))
          .with_source(err)
      })?;
      if ec_params.crv != BlsCurve::BLS12381G2.to_string() {
        return Err(
          KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message(format!(
            "expected Jwk with EC {} crv in order to generate the proof with {alg}",
            BlsCurve::BLS12381G2
          )),
        );
      }
    } else {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::UnsupportedProofAlgorithm)
          .with_custom_message(format!("{alg} is not supported")),
      );
    }

    // Check `key_id` exists in store.
    if !self.exists(key_id).await? {
      return Err(KeyStorageError::new(KeyStorageErrorKind::KeyNotFound));
    }

    let sk_location = Location::Generic {
      vault_path: IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      record_path: key_id.to_string().as_bytes().to_vec(),
    };

    client
      .get_guards([sk_location], |[sk]| {
        let sk = BBSplusSecretKey::from_bytes(&sk.borrow()).map_err(|e| FatalProcedureError::from(e.to_string()))?;
        match alg {
          ProofAlgorithm::BLS12381_SHA256 => Signature::<BbsBls12381Sha256>::from_bytes(signature)
            .and_then(|sig| {
              sig.update_signature(
                &sk,
                &old_start_validity_timeframe,
                &new_start_validity_timeframe,
                index_start_validity_timeframe,
                number_of_signed_messages,
              )
            })
            .and_then(|sig| {
              sig.update_signature(
                &sk,
                &old_end_validity_timeframe,
                &new_end_validity_timeframe,
                index_end_validity_timeframe,
                number_of_signed_messages,
              )
            })
            .map_err(|e| FatalProcedureError::from(e.to_string()))
            .map(|sig| sig.to_bytes()),
          ProofAlgorithm::BLS12381_SHAKE256 => Signature::<BbsBls12381Shake256>::from_bytes(signature)
            .and_then(|sig| {
              sig.update_signature(
                &sk,
                &old_start_validity_timeframe,
                &new_start_validity_timeframe,
                index_start_validity_timeframe,
                number_of_signed_messages,
              )
            })
            .and_then(|sig| {
              sig.update_signature(
                &sk,
                &old_end_validity_timeframe,
                &new_end_validity_timeframe,
                index_end_validity_timeframe,
                number_of_signed_messages,
              )
            })
            .map_err(|e| FatalProcedureError::from(e.to_string()))
            .map(|sig| sig.to_bytes()),
          _ => unreachable!(),
        }
      })
      .map_err(|e| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("Signature update failed")
          .with_source(e)
      })
  }
}

fn jwk_to_bbs_plus_pk(jwk: &Jwk) -> anyhow::Result<BBSplusPublicKey> {
  // Safety: only called after checking `jwk`.
  let params = jwk.try_ec_params().unwrap();
  let x = jwu::decode_b64(params.x.as_bytes())?
    .try_into()
    .map_err(|_| anyhow::anyhow!("Invalid coordinate length"))?;
  let y = jwu::decode_b64(params.y.as_bytes())?
    .try_into()
    .map_err(|_| anyhow::anyhow!("Invalid coordinate length"))?;

  BBSplusPublicKey::from_coordinates(&x, &y).context("Failed to create BBS+ public key with the given coordinates")
}

fn generate_bbs_plus_key_pair(alg: ProofAlgorithm) -> KeyStorageResult<(BBSplusSecretKey, BBSplusPublicKey)> {
  match alg {
    ProofAlgorithm::BLS12381_SHA256 => {
      let keypair = KeyPair::<BbsBls12381Sha256>::random()
        .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(err))?;
      let sk = keypair.private_key().clone();
      let pk = keypair.public_key().clone();

      Ok((sk, pk))
    }
    ProofAlgorithm::BLS12381_SHAKE256 => {
      let keypair = KeyPair::<BbsBls12381Shake256>::random()
        .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(err))?;
      let sk = keypair.private_key().clone();
      let pk = keypair.public_key().clone();

      Ok((sk, pk))
    }
    other => Err(
      KeyStorageError::new(KeyStorageErrorKind::UnsupportedProofAlgorithm).with_custom_message(format!(
        "`{other}` is not supported with key type `{}`",
        StrongholdKeyType::BLS12381G2
      )),
    ),
  }
}

fn encode_bls_jwk(private_key: &BBSplusSecretKey, public_key: &BBSplusPublicKey) -> Jwk {
  let (x, y) = public_key.to_coordinates();
  let x = jwu::encode_b64(x);
  let y = jwu::encode_b64(y);

  let d = jwu::encode_b64(private_key.to_bytes());
  let mut params = JwkParamsEc::new();
  params.x = x;
  params.y = y;
  params.d = Some(d);
  params.crv = BlsCurve::BLS12381G2.name().to_owned();
  Jwk::from_params(params)
}
