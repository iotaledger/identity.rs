// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::ed25519;
use super::JwkGenOutput;
use super::KeyId;
use super::KeyStorageError;
use super::KeyStorageErrorKind;
use super::KeyStorageResult;
use super::KeyType;
use crate::stronghold::Stronghold;
use crate::stronghold::StrongholdKeyType;
use crate::JwkStorage;
use async_trait::async_trait;
use identity_verification::jwk::EdCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParamsOkp;
use identity_verification::jws::JwsAlgorithm;
use identity_verification::jwu;
use iota_stronghold::procedures::Ed25519Sign;
use iota_stronghold::procedures::GenerateKey;
use iota_stronghold::procedures::KeyType as ProceduresKeyType;
use iota_stronghold::procedures::StrongholdProcedure;
use iota_stronghold::Location;
use rand::distributions::DistString;
use std::str::FromStr;

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwkStorage for Stronghold {
  async fn generate(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    let key_type = StrongholdKeyType::try_from(&key_type)?;
    check_key_alg_compatibility(key_type, alg)?;

    let keytype: ProceduresKeyType = match key_type.to_string().to_lowercase().as_str() {
      "ed25519" => Ok(ProceduresKeyType::Ed25519),
      _ => Err(KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)),
    }?;

    let key_id: KeyId = random_key_id();
    let location = Location::generic(
      self.vault_path.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );

    let generate_key_procedure = GenerateKey {
      ty: keytype.clone(),
      output: location.clone(),
    };

    self
      .client
      .execute_procedure(StrongholdProcedure::GenerateKey(generate_key_procedure))
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("stronghold procedure failed")
          .with_source(err)
      })?;

    let public_key_procedure = iota_stronghold::procedures::PublicKey {
      ty: keytype,
      private_key: location,
    };

    let procedure_result = self
      .client
      .execute_procedure(StrongholdProcedure::PublicKey(public_key_procedure))
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("stronghold procedure failed")
          .with_source(err)
      })?;
    let public_key: Vec<u8> = procedure_result.into();

    let mut params = JwkParamsOkp::new();
    params.x = jwu::encode_b64(public_key);
    params.crv = EdCurve::Ed25519.name().to_owned();
    let mut jwk: Jwk = Jwk::from_params(params);
    jwk.set_alg(alg.name());
    jwk.set_kid(key_id.clone());

    Ok(JwkGenOutput { key_id, jwk })
  }

  async fn insert(&self, jwk: Jwk) -> KeyStorageResult<KeyId> {
    let key_type: StrongholdKeyType = StrongholdKeyType::try_from(&jwk)?;
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
      self.vault_path.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );
    self
      .client
      .vault(self.vault_path.as_bytes())
      .write_secret(location, secret_key.to_bytes().to_vec())
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("stronghold client error")
          .with_source(err)
      })?;

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
      other => {
        return Err(
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedSignatureAlgorithm)
            .with_custom_message(format!("{other} is not supported")),
        );
      }
    };

    let location = Location::generic(
      self.vault_path.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );
    let procedure: Ed25519Sign = Ed25519Sign {
      private_key: location,
      msg: data.to_vec(),
    };

    let signature: [u8; 64] = self.client.execute_procedure(procedure).map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("stronghold procedure failed")
        .with_source(err)
    })?;

    Ok(signature.to_vec())
  }

  async fn delete(&self, key_id: &KeyId) -> KeyStorageResult<()> {
    let deleted = self
      .client
      .vault(self.vault_path.as_bytes())
      .delete_secret(key_id.to_string().as_bytes())
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("stronghold client error")
          .with_source(err)
      })?;

    if !deleted {
      return Err(KeyStorageError::new(KeyStorageErrorKind::KeyNotFound));
    }

    Ok(())
  }

  async fn exists(&self, key_id: &KeyId) -> KeyStorageResult<bool> {
    let location = Location::generic(
      self.vault_path.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );
    Ok(self.client.record_exists(&location).map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("stronghold client error")
        .with_source(err)
    })?)
  }
}

/// Check that the key type can be used with the algorithm.
fn check_key_alg_compatibility(key_type: StrongholdKeyType, alg: JwsAlgorithm) -> KeyStorageResult<()> {
  match (key_type, alg) {
    (StrongholdKeyType::Ed25519, JwsAlgorithm::EdDSA) => Ok(()),
    (key_type, alg) => Err(
      KeyStorageError::new(crate::key_storage::KeyStorageErrorKind::KeyAlgorithmMismatch)
        .with_custom_message(format!("`cannot use key type `{key_type}` with algorithm `{alg}`")),
    ),
  }
}

/// Generate a random alphanumeric string of len 32.
fn random_key_id() -> KeyId {
  KeyId::new(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32))
}

#[cfg(test)]
mod test {
  use crypto::signatures::ed25519::PublicKey;
  use crypto::signatures::ed25519::Signature;
  use identity_verification::jwk::EcCurve;
  use identity_verification::jwk::Jwk;
  use identity_verification::jwk::JwkParamsEc;
  use identity_verification::jws::JwsAlgorithm;

  use crate::key_storage::ed25519::expand_public_jwk;
  use crate::key_storage::ed25519::generate_ed25519;

  use crate::key_storage::KeyType;
  use crate::stronghold::Stronghold;
  use crate::JwkStorage;
  use crate::KeyId;
  use crate::KeyStorageErrorKind;

  #[tokio::test]
  pub async fn test_generate_and_sign() {
    let test_msg: &[u8] = b"test";
    let stronghold: Stronghold = Stronghold::new("stronghold.hodl", "test-password".to_owned(), None, None)
      .await
      .unwrap();

    let generate = stronghold
      .generate(KeyType::new("Ed25519"), JwsAlgorithm::EdDSA)
      .await
      .unwrap();

    let signature = stronghold
      .sign(&generate.key_id, test_msg, &generate.jwk)
      .await
      .unwrap();

    let signature: Signature = Signature::from_bytes(signature.try_into().unwrap());
    let public_key: PublicKey = expand_public_jwk(&generate.jwk);
    assert!(public_key.verify(&signature, test_msg));

    let key_id: KeyId = generate.key_id;
    assert!(stronghold.exists(&key_id).await.unwrap());
    stronghold.delete(&key_id).await.unwrap();
  }

  #[tokio::test]
  pub async fn test_exists() {
    let stronghold: Stronghold = Stronghold::new("stronghold.hodl", "test-password".to_owned(), None, None)
      .await
      .unwrap();
    assert!(!stronghold.exists(&KeyId::new("non-existent-id")).await.unwrap());
  }

  #[tokio::test]
  async fn incompatible_key_type() {
    let stronghold: Stronghold = Stronghold::new("stronghold.hodl", "test-password".to_owned(), None, None)
      .await
      .unwrap();

    let mut ec_params = JwkParamsEc::new();
    ec_params.crv = EcCurve::P256.name().to_owned();
    ec_params.x = "".to_owned();
    ec_params.y = "".to_owned();
    ec_params.d = Some("".to_owned());
    let jwk_ec = Jwk::from_params(ec_params);

    let err: _ = stronghold.insert(jwk_ec).await.unwrap_err();
    assert!(matches!(err.kind(), KeyStorageErrorKind::UnsupportedKeyType));
  }

  #[tokio::test]
  async fn incompatible_key_alg() {
    let stronghold: Stronghold = Stronghold::new("stronghold.hodl", "test-password".to_owned(), None, None)
      .await
      .unwrap();

    let (private_key, public_key) = generate_ed25519();
    let mut jwk: Jwk = crate::key_storage::ed25519::encode_jwk(&private_key, &public_key);
    jwk.set_alg(JwsAlgorithm::ES256.name());

    // INVALID: Inserting an Ed25519 key with the ES256 alg is not compatible.
    let err = stronghold.insert(jwk.clone()).await.unwrap_err();
    assert!(matches!(err.kind(), KeyStorageErrorKind::KeyAlgorithmMismatch));
  }
}
