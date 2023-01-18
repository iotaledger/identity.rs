pub mod error;
pub mod jose;
pub mod jwk;
pub mod jws;
pub mod jwt;
pub mod jwu;

#[cfg(test)]
mod tests {
  use std::sync::Arc;
  use std::time::SystemTime;

  use crypto::signatures::ed25519::SecretKey;

  use crate::jws::Encoder;
  use crate::jws::JwsAlgorithm;
  use crate::jws::JwsHeader;
  use crate::jws::Recipient;
  use crate::jwt::JwtClaims;
  use crate::jwu::{self};

  #[tokio::test]
  async fn test_encoder_api_jwt() -> anyhow::Result<()> {
    let secret_key = Arc::new(SecretKey::generate().unwrap());

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

    let token = Encoder::new(sign_fn)
      .format(crate::jws::JwsFormat::General)
      .recipient(Recipient::new().protected(&header).unprotected(&header))
      .encode_serde(&claims)
      .await?;

    println!("{token}");

    Ok(())
  }
}
