// Copyright 2020-2024 IOTA Stiftung, Filancore GmbH
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref as _;

use ecdsa::elliptic_curve::sec1::FromEncodedPoint;
use ecdsa::elliptic_curve::sec1::ModulusSize;
use ecdsa::elliptic_curve::sec1::ToEncodedPoint;
use ecdsa::elliptic_curve::CurveArithmetic;
use ecdsa::EncodedPoint;
use ecdsa::PrimeCurve;
use ecdsa::Signature;
use ecdsa::SignatureSize;
use ecdsa::VerifyingKey;
use identity_verification::jwk::Jwk;
use identity_verification::jwk::JwkParamsEc;
use identity_verification::jws::SignatureVerificationError;
use identity_verification::jws::SignatureVerificationErrorKind;
use identity_verification::jws::VerificationInput;
use identity_verification::jwu;
use signature::digest::generic_array::ArrayLength;

fn jwk_to_verifying_key<C>(jwk: &Jwk) -> Result<VerifyingKey<C>, SignatureVerificationError>
where
  C: PrimeCurve + CurveArithmetic,
  C::FieldBytesSize: ModulusSize,
  C::AffinePoint: FromEncodedPoint<C> + ToEncodedPoint<C>,
{
  // Obtain an Elliptic Curve public key.
  let params: &JwkParamsEc = jwk
    .try_ec_params()
    .map_err(|_| SignatureVerificationError::new(SignatureVerificationErrorKind::UnsupportedKeyType))?;

  // Concatenate x and y coordinates as required by
  // EncodedPoint::from_untagged_bytes.
  let public_key_bytes = {
    let x_bytes = jwu::decode_b64(&params.x)
      .map_err(|err| {
        SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure).with_source(err)
      })?
      .into_iter();
    let y_bytes = jwu::decode_b64(&params.y)
      .map_err(|err| {
        SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure).with_source(err)
      })?
      .into_iter();

    x_bytes.chain(y_bytes).collect()
  };

  // The JWK contains the uncompressed x and y coordinates, so we can create the
  // encoded point directly without prefixing an SEC1 tag.
  let encoded_point = EncodedPoint::<C>::from_untagged_bytes(&public_key_bytes);
  let verifying_key = VerifyingKey::<C>::from_encoded_point(&encoded_point)
    .map_err(|e| SignatureVerificationError::new(SignatureVerificationErrorKind::KeyDecodingFailure).with_source(e))?;

  Ok(verifying_key)
}

pub(crate) fn verify_signature<C, F>(
  input: &VerificationInput,
  public_key: &Jwk,
  verifying_fn: F,
) -> Result<(), SignatureVerificationError>
where
  C: PrimeCurve + CurveArithmetic,
  C::FieldBytesSize: ModulusSize,
  C::AffinePoint: FromEncodedPoint<C> + ToEncodedPoint<C>,
  SignatureSize<C>: ArrayLength<u8>,
  F: FnOnce(&VerifyingKey<C>, &[u8], &Signature<C>) -> Result<(), signature::Error>,
{
  let verifying_key = jwk_to_verifying_key(public_key)?;
  let mut signature = Signature::<C>::from_slice(input.decoded_signature.deref()).map_err(|err| {
    SignatureVerificationError::new(SignatureVerificationErrorKind::InvalidSignature).with_source(err)
  })?;

  if let Some(normalized) = signature.normalize_s() {
    signature = normalized;
  }

  verifying_fn(&verifying_key, &input.signing_input, &signature)
    .map_err(|e| SignatureVerificationError::new(SignatureVerificationErrorKind::InvalidSignature).with_source(e))
}
