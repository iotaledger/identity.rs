use std::str::FromStr;

use crate::error::Result as WasmResult;
use crate::error::WasmResult as _;
use crate::jose::WasmJwk;
use crate::jpt::WasmProofAlgorithm;

use super::WasmJwkGenOutput;
use super::WasmJwkStorage;
use super::WasmProofUpdateCtx;

use identity_iota::storage::JwkGenOutput;
use identity_iota::storage::JwkStorage;
use identity_iota::storage::JwkStorageExt;
use identity_iota::storage::KeyId;
use identity_iota::storage::KeyStorageError;
use identity_iota::storage::KeyStorageErrorKind;
use identity_iota::storage::KeyStorageResult;
use identity_iota::storage::KeyType;
use identity_iota::storage::ProofUpdateCtx;
use identity_iota::verification::jwk::BlsCurve;
use identity_iota::verification::jwk::Jwk;
use identity_iota::verification::jwk::JwkParamsEc;
use identity_iota::verification::jwu;
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use wasm_bindgen::prelude::*;
use zkryptium::bbsplus::ciphersuites::BbsCiphersuite;
use zkryptium::bbsplus::ciphersuites::Bls12381Sha256;
use zkryptium::bbsplus::ciphersuites::Bls12381Shake256;
use zkryptium::bbsplus::keys::BBSplusPublicKey;
use zkryptium::bbsplus::keys::BBSplusSecretKey;
use zkryptium::bbsplus::signature::BBSplusSignature;
use zkryptium::keys::pair::KeyPair;
use zkryptium::schemes::algorithms::BBSplus;
use zkryptium::schemes::algorithms::BbsBls12381Sha256;
use zkryptium::schemes::algorithms::BbsBls12381Shake256;
use zkryptium::schemes::generics::Signature;

fn generate_bbs_keypair<A: BbsCiphersuite>() -> Result<(BBSplusSecretKey, BBSplusPublicKey), KeyStorageError> {
  let keypair = KeyPair::<BBSplus<A>>::random()
    .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(err))?;
  let sk = keypair.private_key().clone();
  let pk = keypair.public_key().clone();
  Ok((sk, pk))
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

fn expand_bls_jwk(jwk: &Jwk) -> Result<(BBSplusSecretKey, BBSplusPublicKey), KeyStorageError> {
  let params: &JwkParamsEc = jwk.try_ec_params().unwrap();

  if params
    .try_bls_curve()
    .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType).with_source(err))?
    != BlsCurve::BLS12381G2
  {
    return Err(
      KeyStorageError::new(KeyStorageErrorKind::UnsupportedKeyType)
        .with_custom_message(format!("expected an {} key", BlsCurve::BLS12381G2.name())),
    );
  }

  let sk: BBSplusSecretKey = params
    .d
    .as_deref()
    .map(jwu::decode_b64)
    .ok_or_else(|| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("expected Jwk `d` param to be present")
    })?
    .map(|v| BBSplusSecretKey::from_bytes(&v))
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("unable to decode `d` param")
        .with_source(err)
    })?
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("invalid BBS+ secret key".to_owned())
    })?;

  let x: [u8; BBSplusPublicKey::COORDINATE_LEN] = jwu::decode_b64(&params.x)
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("unable to decode `x` param")
        .with_source(err)
    })?
    .try_into()
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message(format!("expected key of length {}", BBSplusPublicKey::COORDINATE_LEN))
    })?;

  let y: [u8; BBSplusPublicKey::COORDINATE_LEN] = jwu::decode_b64(&params.y)
    .map_err(|err| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message("unable to decode `y` param")
        .with_source(err)
    })?
    .try_into()
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified)
        .with_custom_message(format!("expected key of length {}", BBSplusPublicKey::COORDINATE_LEN))
    })?;

  let pk = BBSplusPublicKey::from_coordinates(&x, &y).map_err(|_| {
    KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("invalid BBS+ public key".to_owned())
  })?;

  Ok((sk, pk))
}

fn update_bbs_signature<A>(
  sig: &[u8; 80],
  sk: &BBSplusSecretKey,
  update_ctx: &ProofUpdateCtx,
) -> Result<[u8; 80], KeyStorageError>
where
  A: BbsCiphersuite,
{
  let sig = Signature::<BBSplus<A>>::from_bytes(sig)
    .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(e))?;
  let ProofUpdateCtx {
    old_start_validity_timeframe,
    new_start_validity_timeframe,
    old_end_validity_timeframe,
    new_end_validity_timeframe,
    index_start_validity_timeframe,
    index_end_validity_timeframe,
    number_of_signed_messages,
  } = update_ctx;
  let half_updated = sig
    .update_signature(
      sk,
      old_start_validity_timeframe,
      new_start_validity_timeframe,
      *index_start_validity_timeframe,
      *number_of_signed_messages,
    )
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("Signature update failed")
    })?;
  half_updated
    .update_signature(
      sk,
      old_end_validity_timeframe,
      new_end_validity_timeframe,
      *index_end_validity_timeframe,
      *number_of_signed_messages,
    )
    .map(|sig| sig.to_bytes())
    .map_err(|_| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("Signature update failed"))
}

fn decode_sk_jwt(jwk: &Jwk) -> Result<BBSplusSecretKey, KeyStorageError> {
  let params = jwk.try_ec_params().map_err(|_| KeyStorageErrorKind::Unspecified)?;
  BBSplusSecretKey::from_bytes(
    &params
      .d
      .as_deref()
      .map(jwu::decode_b64)
      .ok_or_else(|| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("expected Jwk `d` param to be present")
      })?
      .map_err(|err| {
        KeyStorageError::new(KeyStorageErrorKind::Unspecified)
          .with_custom_message("unable to decode `d` param")
          .with_source(err)
      })?,
  )
  .map_err(|_| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("key not valid"))
}

#[wasm_bindgen(js_class = JwkStorage)]
impl WasmJwkStorage {
  #[wasm_bindgen(js_name = generateBBS)]
  /// Generates a new BBS+ keypair.
  pub async fn _generate_bbs(&self, alg: WasmProofAlgorithm) -> WasmResult<WasmJwkGenOutput> {
    self
      .generate_bbs(KeyType::from_static_str("BLS12381"), alg.into())
      .await
      .map(WasmJwkGenOutput::from)
      .wasm_result()
  }

  #[wasm_bindgen(js_name = signBBS)]
  pub async fn _sign_bbs(
    &self,
    key_id: String,
    data: Vec<js_sys::Uint8Array>,
    public_key: WasmJwk,
    header: Option<Vec<u8>>,
  ) -> WasmResult<js_sys::Uint8Array> {
    let key_id = KeyId::new(key_id);
    let data = data.into_iter().map(|arr| arr.to_vec()).collect::<Vec<_>>();
    let header = header.unwrap_or_default();
    self
      .sign_bbs(&key_id, &data, header.as_slice(), &public_key.into())
      .await
      .map(|v| js_sys::Uint8Array::from(v.as_slice()))
      .wasm_result()
  }

  #[wasm_bindgen(js_name = updateBBSSignature)]
  pub async fn _update_signature(
    &self,
    key_id: String,
    public_key: &WasmJwk,
    signature: Vec<u8>,
    ctx: WasmProofUpdateCtx,
  ) -> WasmResult<js_sys::Uint8Array> {
    let key_id = KeyId::new(key_id);
    let signature = signature
      .try_into()
      .map_err(|_| JsError::new("Invalid signature length"))?;
    self
      .update_signature(&key_id, &public_key.0, &signature, ctx.into())
      .await
      .map(|sig| js_sys::Uint8Array::from(sig.as_slice()))
      .wasm_result()
  }
}

#[async_trait::async_trait(?Send)]
impl JwkStorageExt for WasmJwkStorage {
  async fn generate_bbs(&self, _key_type: KeyType, alg: ProofAlgorithm) -> KeyStorageResult<JwkGenOutput> {
    let (sk, pk) = match alg {
      ProofAlgorithm::BLS12381_SHA256 => generate_bbs_keypair::<Bls12381Sha256>(),
      ProofAlgorithm::BLS12381_SHAKE256 => generate_bbs_keypair::<Bls12381Shake256>(),
      other => Err(
        KeyStorageError::new(KeyStorageErrorKind::KeyAlgorithmMismatch)
          .with_custom_message(format!("cannot validate proof with {}", other)),
      ),
    }?;

    let mut jwk = encode_bls_jwk(&sk, &pk);
    jwk.set_alg(alg.to_string());
    jwk.set_kid(jwk.thumbprint_sha256_b64());

    let public_jwk = jwk.to_public().expect("kty != oct");
    let kid = <Self as JwkStorage>::insert(self, jwk).await?;

    Ok(JwkGenOutput::new(kid, public_jwk))
  }
  async fn sign_bbs(
    &self,
    key_id: &KeyId,
    data: &[Vec<u8>],
    header: &[u8],
    public_key: &Jwk,
  ) -> KeyStorageResult<Vec<u8>> {
    let Some(private_jwk) = WasmJwkStorage::_get_key(self, key_id.as_str()).map(Jwk::from) else {
      return Err(KeyStorageError::new(KeyStorageErrorKind::KeyNotFound));
    };
    // Extract the required alg from the given public key
    let alg = public_key
      .alg()
      .ok_or(KeyStorageErrorKind::UnsupportedProofAlgorithm)
      .and_then(|alg_str| ProofAlgorithm::from_str(alg_str).map_err(|_| KeyStorageErrorKind::UnsupportedProofAlgorithm))
      .map_err(KeyStorageError::new)?;

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
    let (sk, pk) = expand_bls_jwk(&private_jwk)?;
    match alg {
      ProofAlgorithm::BLS12381_SHA256 => {
        Signature::<BbsBls12381Sha256>::sign(Some(data), &sk, &pk, Some(header)).map(|s| s.to_bytes())
      }
      ProofAlgorithm::BLS12381_SHAKE256 => {
        Signature::<BbsBls12381Shake256>::sign(Some(data), &sk, &pk, Some(header)).map(|s| s.to_bytes())
      }
      other => {
        return Err(
          KeyStorageError::new(KeyStorageErrorKind::UnsupportedProofAlgorithm)
            .with_custom_message(format!("{other} is not supported")),
        );
      }
    }
    .map(|bytes| bytes.to_vec())
    .map_err(|_| {
      KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_custom_message("signature failed".to_owned())
    })
  }
  async fn update_signature(
    &self,
    key_id: &KeyId,
    public_key: &Jwk,
    signature: &[u8; BBSplusSignature::BYTES],
    ctx: ProofUpdateCtx,
  ) -> KeyStorageResult<[u8; BBSplusSignature::BYTES]> {
    // Extract the required alg from the given public key
    let alg = public_key
      .alg()
      .ok_or(KeyStorageErrorKind::UnsupportedProofAlgorithm)
      .and_then(|alg_str| ProofAlgorithm::from_str(alg_str).map_err(|_| KeyStorageErrorKind::UnsupportedProofAlgorithm))
      .map_err(KeyStorageError::new)?;

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
    let Some(private_jwk) = WasmJwkStorage::_get_key(self, key_id.as_str()).map(Jwk::from) else {
      return Err(KeyStorageError::new(KeyStorageErrorKind::KeyNotFound));
    };
    let sk = decode_sk_jwt(&private_jwk)?;
    match alg {
      ProofAlgorithm::BLS12381_SHA256 => update_bbs_signature::<Bls12381Sha256>(signature, &sk, &ctx),
      ProofAlgorithm::BLS12381_SHAKE256 => update_bbs_signature::<Bls12381Shake256>(signature, &sk, &ctx),
      _ => unreachable!(),
    }
  }
}
