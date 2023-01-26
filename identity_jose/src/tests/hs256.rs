use crate::jwk::Jwk;
use crate::jwk::JwkParamsOct;
use crate::jws;
use crate::jws::JwsAlgorithm;
use crate::jws::JwsHeader;
use crate::jws::Recipient;
use crate::jws::Token;
use crate::jwt::JwtHeaderSet;
use crate::jwu;
use crypto::hashes::sha::SHA256_LEN;

pub fn expand_hmac_jwk(jwk: &Jwk, key_len: usize) -> Vec<u8> {
  let params: &JwkParamsOct = jwk.try_oct_params().unwrap();
  let k: Vec<u8> = jwu::decode_b64(&params.k).unwrap();

  if k.len() >= key_len {
    k
  } else {
    panic!("expected different key length");
  }
}

pub async fn encode(claims: &[u8], header: &JwsHeader, jwk: &Jwk) -> String {
  let shared_secret: Vec<u8> = expand_hmac_jwk(jwk, SHA256_LEN);

  let sign_fn = move |protected: Option<JwsHeader>, unprotected: Option<JwsHeader>, msg: Vec<u8>| {
    let sk = shared_secret.clone();
    async move {
      let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new().protected(&protected).unprotected(&unprotected);
      if header_set.try_alg().map_err(|_| "missing `alg` parameter".to_owned())? != JwsAlgorithm::HS256 {
        return Err("incompatible `alg` parameter".to_owned());
      }
      let mut mac: [u8; SHA256_LEN] = Default::default();
      crypto::macs::hmac::HMAC_SHA256(&msg, &sk, &mut mac);
      Ok(jwu::encode_b64(mac))
    }
  };

  jws::Encoder::new(sign_fn)
    .recipient(Recipient::new().protected(header))
    .encode(claims)
    .await
    .unwrap()
}

pub fn decode<'a>(encoded: &'a [u8], jwk: &Jwk) -> Token<'a> {
  let shared_secret: Vec<u8> = expand_hmac_jwk(jwk, SHA256_LEN);

  let verify_fn = move |protected: Option<&JwsHeader>, unprotected: Option<&JwsHeader>, msg: &[u8], sig: &[u8]| {
    let header_set: JwtHeaderSet<JwsHeader> = JwtHeaderSet::new().protected(protected).unprotected(unprotected);
    let alg: JwsAlgorithm = header_set.try_alg().map_err(|_| "missing `alg` parameter")?;
    if alg != JwsAlgorithm::HS256 {
      return Err("incompatible `alg` parameter");
    }

    let mut mac: [u8; SHA256_LEN] = Default::default();
    crypto::macs::hmac::HMAC_SHA256(msg, &shared_secret, &mut mac);

    if sig == mac {
      Ok(())
    } else {
      Err("invalid signature")
    }
  };

  jws::Decoder::new(verify_fn).decode(encoded).unwrap()
}
