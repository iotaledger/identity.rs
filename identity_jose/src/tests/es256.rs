// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

use crate::jwk::EcCurve;
use crate::jwk::Jwk;
use crate::jwk::JwkParamsEc;
use crate::jws::Encoder;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::SignatureVerificationError;
use crate::jws::SignatureVerificationErrorKind;
use crate::jws::VerificationInput;
use crate::jwt::JwtHeaderSet;
use crate::jwu;
use p256::ecdsa::Signature;
use p256::ecdsa::SigningKey;
use p256::ecdsa::VerifyingKey;
use p256::PublicKey;
use p256::SecretKey;

pub(crate) fn expand_p256_jwk(jwk: &Jwk) -> (SecretKey, PublicKey) {
  let params: &JwkParamsEc = jwk.try_ec_params().unwrap();

  if params.try_ec_curve().unwrap() != EcCurve::P256 {
    panic!("expected a P256 curve");
  }

  let sk_bytes = params.d.as_ref().map(jwu::decode_b64).unwrap().unwrap();
  let sk = SecretKey::from_be_bytes(&sk_bytes).unwrap();

  // Transformation according to section 2.3.3 from http://www.secg.org/sec1-v2.pdf.
  let pk_bytes: Vec<u8> = [0x04]
    .into_iter()
    .chain(jwu::decode_b64(&params.x).unwrap().into_iter())
    .chain(jwu::decode_b64(&params.y).unwrap().into_iter())
    .collect();

  let pk = PublicKey::from_sec1_bytes(&pk_bytes).unwrap();

  assert_eq!(sk.public_key(), pk);

  (sk, pk)
}

pub(crate) async fn encode(encoder: &Encoder<'_>, claims: &[u8], jwk: &Jwk) -> String {
  let (secret_key, _) = expand_p256_jwk(jwk);

  let signing_key = SigningKey::from(secret_key);

  let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
    let sk = signing_key.clone();
    async move {
      let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new()
        .with_protected(&protected)
        .with_unprotected(&unprotected);
      if header_set.try_alg().map_err(|_| "missing `alg` parameter".to_owned())? != JwsAlgorithm::ES256 {
        return Err("incompatible `alg` parameter".to_owned());
      }
      let signature: Signature = signature::Signer::sign(&sk, &msg);
      let b64 = jwu::encode_b64(signature.to_bytes());
      Ok(b64)
    }
  };

  encoder.encode(&sign_fn, claims).await.unwrap()
}

pub(crate) fn verify(verification_input: VerificationInput, jwk: &Jwk) -> Result<(), SignatureVerificationError> {
  let (_, public_key) = expand_p256_jwk(jwk);
  let verifying_key = VerifyingKey::from(public_key);

  let signature = Signature::try_from(verification_input.decoded_signature.deref()).unwrap();

  match signature::Verifier::verify(&verifying_key, &verification_input.signing_input, &signature) {
    Ok(()) => Ok(()),
    Err(err) => Err(SignatureVerificationError::new(SignatureVerificationErrorKind::InvalidSignature).with_source(err)),
  }
}
