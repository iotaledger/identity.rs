// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
  use std::sync::Arc;
  use std::time::SystemTime;

  use crypto::signatures::ed25519::SecretKey;

  use identity_jose::jws::Decoder;
  use identity_jose::jws::Encoder;
  use identity_jose::jws::JwsAlgorithm;
  use identity_jose::jws::JwsHeader;
  use identity_jose::jws::Recipient;
  use identity_jose::jwt::JwtClaims;
  use identity_jose::jwu;

  #[tokio::test]
  async fn test_encoder_decoder_roundtrip() {
    let secret_key = Arc::new(SecretKey::generate().unwrap());
    let public_key = secret_key.public_key();

    let sign_fn = move |alg, _key_id, msg: Vec<u8>| {
      let sk = secret_key.clone();
      async move {
        if alg != JwsAlgorithm::EdDSA {
          return Err("incompatible `alg` parameter");
        }
        let sig: _ = sk.sign(msg.as_slice()).to_bytes();
        Ok(jwu::encode_b64(sig))
      }
    };

    let verify_fn = |alg: JwsAlgorithm, _key_id: &str, msg: &[u8], sig: &[u8]| {
      if alg != JwsAlgorithm::EdDSA {
        return Err("incompatible `alg` parameter".to_owned());
      }

      let signature_arr = <[u8; crypto::signatures::ed25519::SIGNATURE_LENGTH]>::try_from(sig)
        .map_err(|err| err.to_string())
        .unwrap();

      let signature = crypto::signatures::ed25519::Signature::from_bytes(signature_arr);
      if public_key.verify(&signature, msg) {
        Ok(())
      } else {
        Err("invalid signature".to_owned())
      }
    };

    let mut header: JwsHeader = JwsHeader::new(JwsAlgorithm::EdDSA);
    header.set_kid("did:iota:0x123#signing-key");

    let mut claims: JwtClaims<serde_json::Value> = JwtClaims::new();
    claims.set_iss("issuer");
    claims.set_iat(
      SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64,
    );
    claims.set_custom(serde_json::json!({"num": 42u64}));

    let token: String = Encoder::new(sign_fn)
      .recipient(Recipient::new().protected(&header))
      .encode_serde(&claims)
      .await
      .unwrap();

    let token: _ = Decoder::new(verify_fn).decode(token.as_bytes()).unwrap();

    let recovered_claims: JwtClaims<serde_json::Value> = serde_json::from_slice(&token.claims).unwrap();

    assert_eq!(claims, recovered_claims);
  }
}
