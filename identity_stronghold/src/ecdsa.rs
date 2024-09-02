use identity_verification::{jwk::{EcCurve, Jwk, JwkParamsEc}, jws::JwsAlgorithm, jwu};
use stronghold_ext::{Algorithm, Es256, Es256k, VerifyingKey};
use anyhow::Context;

pub fn es256_pk_bytes_to_jwk(pk_bytes: &[u8]) -> anyhow::Result<Jwk> {
    let pk = <Es256 as Algorithm>::VerifyingKey::from_slice(&pk_bytes)?;
    let mut params = JwkParamsEc::new();

    let pk_point = pk.to_encoded_point(false);
    params.x = pk_point.x().context("missing x coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.y = pk_point.y().context("missing y coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.crv = EcCurve::P256.name().to_string();

    let mut jwk = Jwk::from_params(params);
    jwk.set_alg(JwsAlgorithm::ES256.name());
    jwk.set_kid(jwk.thumbprint_sha256_b64());

    Ok(jwk)
}

pub fn es256k_pk_bytes_to_jwk(pk_bytes: &[u8]) -> anyhow::Result<Jwk> {
    let pk = <Es256k as Algorithm>::VerifyingKey::from_slice(&pk_bytes)?;
    let mut params = JwkParamsEc::new();

    let pk_point = pk.to_encoded_point(false);
    params.x = pk_point.x().context("missing x coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.y = pk_point.y().context("missing y coordinate for point-encoded public key").map(jwu::encode_b64)?;
    params.crv = EcCurve::Secp256K1.name().to_string();

    let mut jwk = Jwk::from_params(params);
    jwk.set_alg(JwsAlgorithm::ES256K.name());
    jwk.set_kid(jwk.thumbprint_sha256_b64());

    Ok(jwk)
}