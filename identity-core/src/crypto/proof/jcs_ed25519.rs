// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use erased_serde::Serialize;

use crate::convert::ToJson;
use crate::crypto::ed25519_sign;
use crate::crypto::ed25519_verify;
use crate::crypto::SignatureName;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::crypto::SignatureVerify;
use crate::error::Error;
use crate::error::Result;
use crate::utils::decode_b58;
use crate::utils::encode_b58;

const SIGNATURE_NAME: &str = "JcsEd25519Signature2020";

/// An implementation of the [JCS Ed25519 Signature 2020][SPEC1] signature suite
/// for [Linked Data Proofs][SPEC2].
///
/// Users should use the [`SignatureSign`]/[`SignatureVerify`] traits to access
/// this implementation.
///
/// [SPEC1]: https://identity.foundation/JcsEd25519Signature2020/
/// [SPEC2]: https://w3c-ccg.github.io/ld-proofs/
#[derive(Clone, Copy, Debug)]
pub struct JcsEd25519Signature2020;

impl SignatureName for JcsEd25519Signature2020 {
  fn name(&self) -> String {
    SIGNATURE_NAME.to_string()
  }
}

impl SignatureSign for JcsEd25519Signature2020 {
  fn sign(&self, data: &dyn Serialize, secret: &[u8]) -> Result<SignatureValue> {
    let signature: _ = ed25519_sign(&data.to_jcs()?, secret)?;
    let signature: String = encode_b58(&signature);

    Ok(SignatureValue::Signature(signature))
  }
}

impl SignatureVerify for JcsEd25519Signature2020 {
  fn verify(&self, data: &dyn Serialize, signature: &SignatureValue, public: &[u8]) -> Result<()> {
    let signature: &str = signature.as_signature().ok_or(Error::InvalidProofValue)?;
    let signature: Vec<u8> = decode_b58(signature)?;

    ed25519_verify(&data.to_jcs()?, &signature, public)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::common::Object;
  use crate::common::Value;
  use crate::convert::FromJson;
  use crate::crypto::JcsEd25519Signature2020 as Ed25519;
  use crate::crypto::KeyPair;
  use crate::crypto::SignatureSign;
  use crate::crypto::SignatureValue;
  use crate::crypto::SignatureVerify;
  use crate::json;
  use crate::utils::decode_b58;

  struct TestVector {
    public: &'static str,
    secret: &'static str,
    input: &'static str,
    output: &'static str,
  }

  const TVS: &[TestVector] = &include!("../../../tests/fixtures/jcs_ed25519.rs");

  #[test]
  fn test_tvs() {
    for tv in TVS {
      let public: Vec<u8> = decode_b58(tv.public).unwrap();
      let secret: Vec<u8> = decode_b58(tv.secret).unwrap();

      let input: Object = Object::from_json(tv.input).unwrap();
      let output: Object = Object::from_json(tv.output).unwrap();

      let signature: SignatureValue = Ed25519.sign(&input, &secret).unwrap();

      assert_eq!(output["proof"]["signatureValue"], signature.as_str());

      assert!(Ed25519.verify(&input, &signature, &public).is_ok());

      // Fails when the key is mutated
      assert!(Ed25519.verify(&input, &signature, b"IOTA").is_err());

      // Fails when the signature is mutated
      let signature: _ = SignatureValue::Signature("IOTA".into());
      assert!(Ed25519.verify(&input, &signature, &public).is_err());
    }
  }

  #[test]
  fn test_sign_hello() {
    const PUBLIC: &[u8] = b"8CpYU3CXo1NEXVi5ZJcGgfmYjMoQ4xpewofpcPnWS5kt";
    const SECRET: &[u8] = b"8gFfcuUTmX7P4DYfpEV7iVWzfSSV6QHQZFZamT6oNjVV";

    const SIG: &[u8] = b"4VjbV3672WRhKqUVn4Cdp6e7AaXYYv2f71dM8ZDHqWexfku4oLUeDVFuxGRXxpkVUwZ924zFHu527Z2ZNiPKZVeF";
    const MSG: &[u8] = b"hello";

    let public: Vec<u8> = decode_b58(PUBLIC).unwrap();
    let secret: Vec<u8> = decode_b58(SECRET).unwrap();

    let signature: _ = Ed25519.sign(&MSG, &secret).unwrap();
    assert_eq!(signature.as_str().as_bytes(), SIG);

    let verified: _ = Ed25519.verify(&MSG, &signature, &public);
    assert!(verified.is_ok());
  }

  #[test]
  fn test_sign_verify() {
    let key1: KeyPair = KeyPair::new_ed25519().unwrap();
    let key2: KeyPair = KeyPair::new_ed25519().unwrap();

    let public1: &[u8] = key1.public().as_ref();
    let secret1: &[u8] = key1.secret().as_ref();
    let public2: &[u8] = key2.public().as_ref();

    let data1: Value = json!({ "msg": "IOTA Identity" });
    let data2: Value = json!({ "msg": "IOTA Identity 2" });

    let signature: _ = Ed25519.sign(&data1, secret1).unwrap();

    // The signature should be valid
    assert!(Ed25519.verify(&data1, &signature, public1).is_ok());

    // Modified data should be invaldid
    assert!(Ed25519.verify(&data2, &signature, public1).is_err());

    // A modified key should be invaldid
    assert!(Ed25519.verify(&data1, &signature, public2).is_err());
  }
}
