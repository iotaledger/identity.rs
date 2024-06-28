// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Provides JWK and JWS types and functionality.  

// Re-export necessary types from `identity_jose`.

pub mod jwk {
  //! Reexport of [identity_jose::jwk].

  pub use identity_jose::jwk::*;
}

pub mod jws {
  //! Reexport of [identity_jose::jwk].

  pub use identity_jose::jws::*;
}

pub mod jwu {
  //! Reexport of [identity_jose::jwu].

  pub use identity_jose::jwu::*;
}

pub mod error {
  //! Reexport of [identity_jose::error].

  pub use identity_jose::error::*;
}

use error::Error;
use identity_core::convert::BaseEncoding;
use identity_did::{DIDKey, DID as _};
use identity_jose::{
  jwk::{EdCurve, JwkParamsOkp},
  jwu::encode_b64,
};
use jwk::Jwk;

/// Transcode the public key in `did_key` to `JWK`.
pub fn did_key_to_jwk(did_key: &DIDKey) -> Result<Jwk, Error> {
  let decoded =
    BaseEncoding::decode_multibase(did_key.method_id()).map_err(|_| Error::KeyError("key is not multibase encoded"))?;
  let (key_type, pk_bytes) = decoded.split_at(2);

  // Make sure `did_key` encodes an ED25519 public key.
  if key_type != &[0xed, 0x01] || pk_bytes.len() != 32 {
    return Err(Error::KeyError("invalid ED25519 key"));
  }

  let mut params = JwkParamsOkp::new();
  params.crv = EdCurve::Ed25519.name().to_string();
  params.x = encode_b64(pk_bytes);

  Ok(Jwk::from_params(params))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_did_key_to_jwk() {
    let target_jwk = serde_json::from_value(serde_json::json!({
      "kty": "OKP",
      "crv": "Ed25519",
      "x": "O2onvM62pC1io6jQKm8Nc2UyFXcd4kOmOsBIoYtZ2ik"
    }))
    .unwrap();

    let did_key = "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp"
      .parse::<DIDKey>()
      .unwrap();
    let jwk = did_key_to_jwk(&did_key).unwrap();
    assert_eq!(jwk, target_jwk);
  }
}
