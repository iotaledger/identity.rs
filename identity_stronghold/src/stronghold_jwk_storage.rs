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
use identity_verification::jwk::EdCurve;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParamsEc;
use identity_verification::jwk::JwkParamsOkp;
use identity_verification::jwk::JwkType;
use identity_verification::jws::JwsAlgorithm;
use identity_verification::jwu;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_stronghold::procedures::Ed25519Sign;
use iota_stronghold::procedures::FatalProcedureError;
use iota_stronghold::procedures::GenerateKey;
use iota_stronghold::procedures::KeyType as ProceduresKeyType;
use iota_stronghold::procedures::Products;
use iota_stronghold::procedures::Runner;
use iota_stronghold::procedures::StrongholdProcedure;
use iota_stronghold::Client;
use iota_stronghold::ClientError;
use iota_stronghold::Location;
use iota_stronghold::Stronghold;
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use rand::distributions::DistString;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::MutexGuard;
use zeroize::Zeroizing;
use zkryptium::bbsplus::keys::BBSplusPublicKey;
use zkryptium::bbsplus::keys::BBSplusSecretKey;
use zkryptium::bbsplus::signature::BBSplusSignature;
use zkryptium::keys::pair::KeyPair;
use zkryptium::schemes::algorithms::BbsBls12381Sha256;
use zkryptium::schemes::algorithms::BbsBls12381Shake256;
use zkryptium::schemes::generics::Signature;

use crate::ed25519;

const ED25519_KEY_TYPE_STR: &str = "Ed25519";
const BLS12381G2_KEY_TYPE_STR: &str = "BLS12381G2";
/// The BLS12381G2 key type
pub const BLS12381G2_KEY_TYPE: KeyType = KeyType::from_static_str(BLS12381G2_KEY_TYPE_STR);

static IDENTITY_VAULT_PATH: &str = "iota_identity_vault";
pub(crate) static IDENTITY_CLIENT_PATH: &[u8] = b"iota_identity_client";

/// The Ed25519 key type.
pub const ED25519_KEY_TYPE: &KeyType = &KeyType::from_static_str(ED25519_KEY_TYPE_STR);

/// Wrapper around a [`StrongholdSecretManager`] that implements the [`KeyIdStorage`](crate::KeyIdStorage)
/// and [`JwkStorage`](crate::JwkStorage) interfaces.
#[derive(Clone, Debug)]
pub struct StrongholdStorage(Arc<SecretManager>);

impl StrongholdStorage {
  /// Creates a new [`StrongholdStorage`].
  pub fn new(stronghold_secret_manager: StrongholdSecretManager) -> Self {
    Self(Arc::new(SecretManager::Stronghold(stronghold_secret_manager)))
  }

  /// Shared reference to the inner [`SecretManager`].
  pub fn as_secret_manager(&self) -> &SecretManager {
    self.0.as_ref()
  }

  /// Acquire lock of the inner [`Stronghold`].
  pub(crate) async fn get_stronghold(&self) -> MutexGuard<'_, Stronghold> {
    match *self.0 {
      SecretManager::Stronghold(ref stronghold) => stronghold.inner().await,
      _ => unreachable!("secret manager can be only constructed from stronghold"),
    }
  }

  /// Retrieve the public key corresponding to `key_id`.
  pub async fn get_public_key(&self, key_id: &KeyId) -> KeyStorageResult<Jwk> {
    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;

    let location = Location::generic(
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );

    let public_key_procedure = iota_stronghold::procedures::PublicKey {
      ty: ProceduresKeyType::Ed25519,
      private_key: location,
    };

    let procedure_result = client
      .execute_procedure(StrongholdProcedure::PublicKey(public_key_procedure))
      .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::KeyNotFound).with_source(err))?;

    let public_key: Vec<u8> = procedure_result.into();

    let mut params = JwkParamsOkp::new();
    params.x = jwu::encode_b64(public_key);
    params.crv = EdCurve::Ed25519.name().to_owned();
    let mut jwk: Jwk = Jwk::from_params(params);
    jwk.set_alg(JwsAlgorithm::EdDSA.name());
    jwk.set_kid(jwk.thumbprint_sha256_b64());

    Ok(jwk)
  }
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl JwkStorage for StrongholdStorage {
  async fn generate(&self, key_type: KeyType, alg: JwsAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    let stronghold = self.get_stronghold().await;

    let client = get_client(&stronghold)?;
    let key_type = StrongholdKeyType::try_from(&key_type)?;
    check_key_alg_compatibility(key_type, alg)?;

    let keytype: ProceduresKeyType = match key_type {
      StrongholdKeyType::Ed25519 => ProceduresKeyType::Ed25519,
      StrongholdKeyType::BLS12381G2 => {
        todo!("return an error that instruct the user to call the BBS+ flavor for this function.")
      }
    };

    let key_id: KeyId = random_key_id();
    let location = Location::generic(
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );

    let generate_key_procedure = GenerateKey {
      ty: keytype.clone(),
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
      ty: keytype,
      private_key: location,
    };

    let procedure_result = client
      .execute_procedure(StrongholdProcedure::PublicKey(public_key_procedure))
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("stronghold public key procedure failed")
          .with_source(err)
      })?;
    persist_changes(self, stronghold).await?;
    let public_key: Vec<u8> = procedure_result.into();

    let mut params = JwkParamsOkp::new();
    params.x = jwu::encode_b64(public_key);
    params.crv = EdCurve::Ed25519.name().to_owned();
    let mut jwk: Jwk = Jwk::from_params(params);
    jwk.set_alg(alg.name());
    jwk.set_kid(jwk.thumbprint_sha256_b64());

    Ok(JwkGenOutput::new(key_id, jwk))
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
    persist_changes(self, stronghold).await?;

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
      IDENTITY_VAULT_PATH.as_bytes().to_vec(),
      key_id.to_string().as_bytes().to_vec(),
    );
    let procedure: Ed25519Sign = Ed25519Sign {
      private_key: location,
      msg: data.to_vec(),
    };

    let stronghold = self.get_stronghold().await;
    let client = get_client(&stronghold)?;

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
    persist_changes(self, stronghold).await?;

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

/// Generate a random alphanumeric string of len 32.
fn random_key_id() -> KeyId {
  KeyId::new(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32))
}

/// Check that the key type can be used with the algorithm.
fn check_key_alg_compatibility(key_type: StrongholdKeyType, alg: JwsAlgorithm) -> KeyStorageResult<()> {
  match (key_type, alg) {
    (StrongholdKeyType::Ed25519, JwsAlgorithm::EdDSA) => Ok(()),
    (key_type, alg) => Err(
      KeyStorageError::new(identity_storage::KeyStorageErrorKind::KeyAlgorithmMismatch)
        .with_custom_message(format!("cannot use key type `{key_type}` with algorithm `{alg}`")),
    ),
  }
}

fn get_client(stronghold: &Stronghold) -> KeyStorageResult<Client> {
  let client = stronghold.get_client(IDENTITY_CLIENT_PATH);
  match client {
    Ok(client) => Ok(client),
    Err(ClientError::ClientDataNotPresent) => load_or_create_client(stronghold),
    Err(err) => Err(KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(err)),
  }
}

fn load_or_create_client(stronghold: &Stronghold) -> KeyStorageResult<Client> {
  match stronghold.load_client(IDENTITY_CLIENT_PATH) {
    Ok(client) => Ok(client),
    Err(ClientError::ClientDataNotPresent) => stronghold
      .create_client(IDENTITY_CLIENT_PATH)
      .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(err)),
    Err(err) => Err(KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(err)),
  }
}

async fn persist_changes(
  secret_manager: &StrongholdStorage,
  stronghold: MutexGuard<'_, Stronghold>,
) -> KeyStorageResult<()> {
  stronghold.write_client(IDENTITY_CLIENT_PATH).map_err(|err| {
    KeyStorageError::new(KeyStorageErrorKind::Unspecified)
      .with_custom_message("stronghold write client error")
      .with_source(err)
  })?;
  // Must be dropped since `write_stronghold_snapshot` needs to acquire the stronghold lock.
  drop(stronghold);

  match secret_manager.as_secret_manager() {
    iota_sdk::client::secret::SecretManager::Stronghold(stronghold_manager) => {
      stronghold_manager
        .write_stronghold_snapshot(None)
        .await
        .map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::Unspecified)
            .with_custom_message("writing to stronghold snapshot failed")
            .with_source(err)
        })?;
    }
    _ => {
      return Err(
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("secret manager is not of type stronghold"),
      )
    }
  };
  Ok(())
}

/// Key Types supported by the stronghold storage implementation.
#[derive(Debug, Copy, Clone)]
enum StrongholdKeyType {
  Ed25519,
  BLS12381G2,
}

impl StrongholdKeyType {
  /// String representation of the key type.
  const fn name(&self) -> &'static str {
    match self {
      StrongholdKeyType::Ed25519 => "Ed25519",
      StrongholdKeyType::BLS12381G2 => "BLS12381G2",
    }
  }
}

impl Display for StrongholdKeyType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name())
  }
}

impl TryFrom<&KeyType> for StrongholdKeyType {
  type Error = KeyStorageError;

  fn try_from(value: &KeyType) -> Result<Self, Self::Error> {
    match value.as_str() {
      ED25519_KEY_TYPE_STR => Ok(StrongholdKeyType::Ed25519),
      BLS12381G2_KEY_TYPE_STR => Ok(StrongholdKeyType::BLS12381G2),
      _ => Err(KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)),
    }
  }
}

impl TryFrom<&Jwk> for StrongholdKeyType {
  type Error = KeyStorageError;

  fn try_from(jwk: &Jwk) -> Result<Self, Self::Error> {
    match jwk.kty() {
      JwkType::Okp => {
        let okp_params = jwk.try_okp_params().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
            .with_custom_message("expected Okp parameters for a JWK with `kty` Okp")
            .with_source(err)
        })?;
        match okp_params.try_ed_curve().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
            .with_custom_message("only Ed curves are supported for signing")
            .with_source(err)
        })? {
          EdCurve::Ed25519 => Ok(StrongholdKeyType::Ed25519),
          curve => Err(
            KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
              .with_custom_message(format!("{curve} not supported")),
          ),
        }
      }
      JwkType::Ec => {
        let ec_params = jwk.try_ec_params().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
            .with_custom_message("expected EC parameters for a JWK with `kty` Ec")
            .with_source(err)
        })?;
        match ec_params.try_bls_curve().map_err(|err| {
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
            .with_custom_message("only Ed curves are supported for signing")
            .with_source(err)
        })? {
          BlsCurve::BLS12381G2 => Ok(StrongholdKeyType::BLS12381G2),
          curve => Err(
            KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
              .with_custom_message(format!("{curve} not supported")),
          ),
        }
      }
      other => Err(
        KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
          .with_custom_message(format!("Jwk `kty` {other} not supported")),
      ),
    }
  }
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
