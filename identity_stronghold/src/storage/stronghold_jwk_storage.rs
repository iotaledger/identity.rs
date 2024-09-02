// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Wrapper around [`StrongholdSecretManager`](StrongholdSecretManager).

use anyhow::Context;
use async_trait::async_trait;
use identity_storage::key_storage::JwkStorage;
use identity_storage::JwkGenOutput;
use identity_storage::KeyId;
use identity_storage::KeyStorageError;
use identity_storage::KeyStorageErrorKind;
use identity_storage::KeyStorageResult;
use identity_storage::KeyType;
use identity_verification::jwk::EcCurve;
use identity_verification::jwk::EdCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParams;
use identity_verification::jwk::JwkParamsEc;
use identity_verification::jwk::JwkParamsOkp;
use identity_verification::jws::JwsAlgorithm;
use identity_verification::jwu;
use iota_stronghold::procedures::Ed25519Sign;
use iota_stronghold::procedures::GenerateKey;
use iota_stronghold::procedures::KeyType as ProceduresKeyType;
use iota_stronghold::procedures::StrongholdProcedure;
use iota_stronghold::Client;
use iota_stronghold::Location;
use stronghold_ext::Algorithm;
use stronghold_ext::Es256k;
use std::str::FromStr;
use stronghold_ext::execute_procedure_ext;
use stronghold_ext::procs::es256::Es256Procs;
use stronghold_ext::procs::es256::GenerateKey as Es256GenKey;
use stronghold_ext::procs::es256::PublicKey as Es256PK;
use stronghold_ext::procs::es256::Sign as Es256Sign;
use stronghold_ext::procs::es256k::Es256kProcs;
use stronghold_ext::procs::es256k::GenerateKey as Es256kGenKey;
use stronghold_ext::procs::es256k::PublicKey as Es256kPK;
use stronghold_ext::procs::es256k::Sign as Es256kSign;
use stronghold_ext::Es256;
use stronghold_ext::VerifyingKey;

use crate::ed25519;
use crate::stronghold_key_type::StrongholdKeyType;
use crate::utils::*;
use crate::StrongholdStorage;

fn gen_ed25519(client: &Client, location: Location) -> KeyStorageResult<JwkParams> {
  let generate_key_procedure = GenerateKey {
    ty: ProceduresKeyType::Ed25519,
    output: location.clone(),
  };

  client
    .execute_procedure(StrongholdProcedure::GenerateKey(generate_key_procedure))
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("stronghold generate key procedure failed")
        .with_source(err)
    })?;

  let public_key_procedure = iota_stronghold::procedures::PublicKey {
    ty: ProceduresKeyType::Ed25519,
    private_key: location,
  };

  let procedure_result = client
    .execute_procedure(StrongholdProcedure::PublicKey(public_key_procedure))
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("stronghold public key procedure failed")
        .with_source(err)
    })?;

  let public_key: Vec<u8> = procedure_result.into();
  let mut params = JwkParamsOkp::new();
  params.x = jwu::encode_b64(public_key);
  params.crv = EdCurve::Ed25519.name().to_string();

  Ok(params.into())
}

fn gen_es256(client: &Client, location: Location) -> KeyStorageResult<JwkParams> {
  execute_procedure_ext(
    client,
    Es256Procs::GenerateKey(Es256GenKey {
      output: location.clone(),
    }),
  )
  .and_then(|_| execute_procedure_ext(client, Es256Procs::PublicKey(Es256PK { private_key: location })))
  .context("stronghold's procedure execution failed")
  .and_then(|output| {
    let pk_bytes: Vec<u8> = output.into();
    let pk = <Es256 as Algorithm>::VerifyingKey::from_slice(&pk_bytes)?;
    let mut params = JwkParamsEc::new();

    let pk_point = pk.to_encoded_point(false);
    params.x = pk_point.x().context("missing x coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.y = pk_point.y().context("missing y coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.crv = EcCurve::P256.name().to_string();
    Ok(params.into())
  })
  .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(e))
}

fn gen_es256k(client: &Client, location: Location) -> KeyStorageResult<JwkParams> {
  execute_procedure_ext(
    client,
    Es256kProcs::GenerateKey(Es256kGenKey {
      output: location.clone(),
    }),
  )
  .and_then(|_| execute_procedure_ext(client, Es256kProcs::PublicKey(Es256kPK { private_key: location })))
  .context("stronghold's procedure execution failed")
  .and_then(|output| {
    let pk_bytes: Vec<u8> = output.into();
    let pk = <Es256k as Algorithm>::VerifyingKey::from_slice(&pk_bytes)?;
    let mut params = JwkParamsEc::new();

    let pk_point = pk.to_encoded_point(false);
    params.x = pk_point.x().context("missing x coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.y = pk_point.y().context("missing y coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.crv = EcCurve::Secp256K1.name().to_string();
    Ok(params.into())
  })
  .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(e))
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwkStorage for StrongholdStorage {
  async fn generate(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    let stronghold = self.get_stronghold().await;

    let client = get_client(&stronghold)?;
    let key_type = StrongholdKeyType::try_from(&key_type)?;
    check_key_alg_compatibility(key_type, alg)?;

    let key_id: KeyId = random_key_id();
    let location = Location::generic(
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );

    let params = match key_type {
      StrongholdKeyType::Ed25519 => gen_ed25519(&client, location)?,
      StrongholdKeyType::Es256 => gen_es256(&client, location)?,
      StrongholdKeyType::Es256k => gen_es256k(&client, location)?,
      _ => return Err(KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)),
    };
    persist_changes(self.as_secret_manager(), stronghold).await?;

    let mut jwk: Jwk = Jwk::from_params(params);
    jwk.set_alg(alg.name());
    jwk.set_kid(jwk.thumbprint_sha256_b64());

    Ok(JwkGenOutput::new(key_id, jwk))
  }

  async fn insert(&self, jwk: Jwk) -> KeyStorageResult<KeyId> {
    let key_type = StrongholdKeyType::try_from(&jwk)?;
    if !jwk.is_private() {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("expected a Jwk with all private key components set"),
      );
    }

    match jwk.alg() {
      Some(alg) => {
        let alg: JwsAlgorithm = JwsAlgorithm::from_str(alg)
          .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedSignatureAlgorithm).with_source(err))?;
        check_key_alg_compatibility(key_type, alg)?;
      }
      None => {
        return Err(
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedSignatureAlgorithm)
            .with_custom_message("expected a Jwk with an `alg` parameter"),
        );
      }
    }
    let secret_key = ed25519::expand_secret_jwk(&jwk)?;
    let key_id: KeyId = random_key_id();

    let location = Location::generic(
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );
    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;
    client
      .vault(IDENTITY_VAULT_PATH.as_bytes())
      .write_secret(location, zeroize::Zeroizing::from(secret_key.to_bytes().to_vec()))
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("stronghold write secret failed")
          .with_source(err)
      })?;

    persist_changes(self.as_secret_manager(), stronghold).await?;

    Ok(key_id)
  }

  async fn sign(&self, key_id: &KeyId, data: &[u8], public_key: &Jwk) -> KeyStorageResult<Vec<u8>> {
    // Extract the required alg from the given public key
    let alg = public_key
      .alg()
      .ok_or(KeyStorageErrorKind::UnsupportedSignatureAlgorithm)
      .and_then(|alg_str| {
        JwsAlgorithm::from_str(alg_str).map_err(|_| KeyStorageErrorKind::UnsupportedSignatureAlgorithm)
      })?;

    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;

    let location = Location::generic(
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );

    // Check that `kty` is `Okp` and `crv = Ed25519`.
    match alg {
      JwsAlgorithm::EdDSA => {
        let okp_params = public_key.try_okp_params().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::Unspecified)
            .with_custom_message(format!("expected a Jwk with Okp params in order to sign with {alg}"))
            .with_source(err)
        })?;
        if okp_params.crv != EdCurve::Ed25519.name() {
          return Err(
            KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message(format!(
              "expected Jwk with Okp {} crv in order to sign with {alg}",
              EdCurve::Ed25519
            )),
          );
        }
      }
      JwsAlgorithm::ES256 => {
        return execute_procedure_ext(
          &client,
          Es256Procs::Sign(Es256Sign {
            msg: data.to_vec(),
            private_key: location,
          }),
        )
        .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(e))
        .map(Into::into);
      }

      JwsAlgorithm::ES256K => {
        return execute_procedure_ext(
          &client,
          Es256kProcs::Sign(Es256kSign {
            msg: data.to_vec(),
            private_key: location,
          }),
        )
        .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(e))
        .map(Into::into);
      }
      other => {
        return Err(
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedSignatureAlgorithm)
            .with_custom_message(format!("{other} is not supported")),
        );
      }
    };

    let procedure: Ed25519Sign = Ed25519Sign {
      private_key: location,
      msg: data.to_vec(),
    };

    let signature: [u8; 64] = client.execute_procedure(procedure).map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("stronghold Ed25519Sign procedure failed")
        .with_source(err)
    })?;

    Ok(signature.to_vec())
  }

  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<()> {
    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;
    let deleted = client
      .vault(IDENTITY_VAULT_PATH.as_bytes())
      .delete_secret(key_id.to_string().as_bytes())
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("stronghold client error")
          .with_source(err)
      })?;

    if !deleted {
      return Err(KeyStorageError::new(KeyStorageErrorKind::KeyNotFound));
    }

    persist_changes(self.as_secret_manager(), stronghold).await?;

    Ok(())
  }

  async fn exists(&self, key_id: &KeyId) -> KeyStorageResult<bool> {
    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;
    let location = Location::generic(
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );
    let exists = client.record_exists(&location).map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("stronghold client error")
        .with_source(err)
    })?;
    Ok(exists)
  }
}
