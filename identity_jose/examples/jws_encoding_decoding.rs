use crypto::signatures::ed25519;
use crypto::signatures::ed25519::PublicKey;
use crypto::signatures::ed25519::SecretKey;
use identity_jose::jwk::EdCurve;
use identity_jose::jwk::Jwk;
use identity_jose::jwk::JwkParamsOkp;
use identity_jose::jwk::JwkType;
use identity_jose::jws::Decoder;
use identity_jose::jws::Encoder;
use identity_jose::jws::JwsAlgorithm;
use identity_jose::jws::JwsHeader;
use identity_jose::jws::JwsSignatureVerifierFn;
use identity_jose::jws::Recipient;
use identity_jose::jws::SignatureVerificationError;
use identity_jose::jws::SignatureVerificationErrorKind;
use identity_jose::jws::VerificationInput;
use identity_jose::jwt::JwtClaims;
use identity_jose::jwt::JwtHeaderSet;
use identity_jose::jwu;
use std::sync::Arc;
use std::time::SystemTime;

/// Example code demonstrating how to use the [`Encoder`] and [`Decoder`] APIs.
async fn encode_then_decode() -> Result<JwtClaims<serde_json::Value>, Box<dyn std::error::Error>> {
  // =============================
  // Generate an Ed25519 key pair
  // =============================
  let secret_key = SecretKey::generate()?;
  let public_key = secret_key.public_key();

  // ====================================
  // Create the header for the recipient
  // ====================================
  let mut header: JwsHeader = JwsHeader::new();
  header.set_alg(JwsAlgorithm::EdDSA);
  let kid = "did:iota:0x123#signing-key";
  header.set_kid(kid);

  // ==================================
  // Create the claims we want to sign
  // ==================================
  let mut claims: JwtClaims<serde_json::Value> = JwtClaims::new();
  claims.set_iss("issuer");
  claims.set_iat(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64);
  claims.set_custom(serde_json::json!({"num": 42u64}));

  // ==================
  // Encode the claims
  // ==================
  let encoder: Encoder = Encoder::new().recipient(Recipient::new().protected(&header));
  let claims_bytes: Vec<u8> = serde_json::to_vec(&claims)?;
  let secret_key: Arc<SecretKey> = Arc::new(secret_key);
  let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
    let sk: Arc<SecretKey> = secret_key.clone();
    async move {
      let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new()
        .with_protected(&protected)
        .with_unprotected(&unprotected);
      if header_set.try_alg().map_err(|_| "missing `alg` parameter")? != JwsAlgorithm::EdDSA {
        return Err("incompatible `alg` parameter");
      }
      let sig: [u8; ed25519::SIGNATURE_LENGTH] = sk.sign(msg.as_slice()).to_bytes();
      Ok(jwu::encode_b64(sig))
    }
  };
  let token: String = encoder.encode(&sign_fn, &claims_bytes).await?;

  // ============
  // Create Public key for verification
  // =============
  let mut public_key_jwk = Jwk::new(JwkType::Okp);
  public_key_jwk.set_kid(kid);
  public_key_jwk
    .set_params(JwkParamsOkp {
      crv: "Ed25519".into(),
      x: identity_jose::jwu::encode_b64(public_key.as_slice()),
      d: None,
    })
    .unwrap();
  public_key_jwk.set_alg(JwsAlgorithm::EdDSA.to_string());

  // ==================
  // Decode the claims
  // ==================

  // Set up a verifier that verifies JWS signatures secured with the Ed25519 algorithm
  let verify_fn = JwsSignatureVerifierFn::from(
    |verification_input: &VerificationInput, jwk: &Jwk| -> Result<(), SignatureVerificationError> {
      if verification_input
        .jose_header()
        .alg()
        .filter(|value| *value == JwsAlgorithm::EdDSA)
        .is_none()
      {
        return Err(SignatureVerificationErrorKind::UnsupportedAlg.into());
      }

      let params: &JwkParamsOkp = jwk
        .try_okp_params()
        .map_err(|_| SignatureVerificationErrorKind::UnsupportedKeyType)?;

      if params.try_ed_curve().unwrap() != EdCurve::Ed25519 {
        return Err(SignatureVerificationErrorKind::UnsupportedKeyParams.into());
      }

      let pk: [u8; ed25519::PUBLIC_KEY_LENGTH] = jwu::decode_b64(params.x.as_str()).unwrap().try_into().unwrap();

      let public_key = PublicKey::try_from(pk).map_err(|_| SignatureVerificationErrorKind::KeyDecodingFailure)?;
      let signature_arr =
        <[u8; ed25519::SIGNATURE_LENGTH]>::try_from(verification_input.signature()).map_err(|err| {
          SignatureVerificationError::new(SignatureVerificationErrorKind::InvalidSignature).with_source(err)
        })?;
      let signature = ed25519::Signature::from_bytes(signature_arr);
      if public_key.verify(&signature, verification_input.signing_input()) {
        Ok(())
      } else {
        Err(SignatureVerificationErrorKind::InvalidSignature.into())
      }
    },
  );
  let decoder = Decoder::new();
  // We don't use a detached payload.
  let detached_payload: Option<&[u8]> = None;
  let token = decoder.decode(
    token.as_bytes(),
    &verify_fn,
    |_| Some(&public_key_jwk),
    detached_payload,
  )?;

  // ==================================
  // Assert the claims are as expected
  // ==================================
  let recovered_claims: JwtClaims<serde_json::Value> = serde_json::from_slice(&token.claims)?;
  assert_eq!(claims, recovered_claims);
  Ok(recovered_claims)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let recovered_claims = encode_then_decode().await?;
  println!("{:#?}", recovered_claims);
  Ok(())
}

#[tokio::test]
async fn test_example_code() {
  assert!(encode_then_decode().await.is_ok());
}
