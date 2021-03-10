// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! An implementation of the [JCS Ed25519 Signature 2020][SPEC1] signature suite
//! for [Linked Data Proofs][SPEC2].
//!
//! Users should use the [`SignatureSign`]/[`SignatureVerify`] traits to access
//! this implementation.
//!
//! [SPEC1]: https://identity.foundation/JcsEd25519Signature2020/
//! [SPEC2]: https://w3c-ccg.github.io/ld-proofs/

use serde::Serialize;

use crate::convert::ToJson;
use crate::crypto::ed25519_sign;
use crate::crypto::ed25519_verify;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::crypto::SignatureName;
use crate::crypto::SignatureSign;
use crate::crypto::SignatureValue;
use crate::crypto::SignatureVerify;
use crate::error::Error;
use crate::error::Result;
use crate::utils::decode_b58;
use crate::utils::encode_b58;

const SIGNATURE_NAME: &str = "JcsEd25519Signature2020";

// =============================================================================
// =============================================================================

/// An implementation of [`SignatureSign`] for [JCS Ed25519 Signatures][SPEC].
///
/// [SPEC]: https://identity.foundation/JcsEd25519Signature2020/
#[derive(Clone, Copy, Debug)]
pub struct JcsEd25519Signer<'key>(&'key SecretKey);

impl<'key> JcsEd25519Signer<'key> {
  /// Creates a new [`JcsEd25519Signer`] instance.
  pub const fn new(key: &'key SecretKey) -> Self {
    Self(key)
  }
}

impl SignatureName for JcsEd25519Signer<'_> {
  const NAME: &'static str = SIGNATURE_NAME;
}

impl<'key> SignatureSign<'key> for JcsEd25519Signer<'key> {
  type Actual = Self;
  type Secret = SecretKey;

  fn create(key: &'key Self::Secret) -> Self::Actual {
    Self::new(key)
  }

  fn sign<T>(&self, data: &T) -> Result<SignatureValue>
  where
    T: Serialize,
  {
    let signature: _ = ed25519_sign(&data.to_jcs()?, self.0.as_ref())?;
    let signature: String = encode_b58(&signature);

    Ok(SignatureValue::Signature(signature))
  }
}

// =============================================================================
// =============================================================================

/// An implementation of [`SignatureVerify`] for [JCS Ed25519 Signatures][SPEC].
///
/// [SPEC]: https://identity.foundation/JcsEd25519Signature2020/
#[derive(Clone, Copy, Debug)]
pub struct JcsEd25519Verifier<'key>(&'key PublicKey);

impl<'key> JcsEd25519Verifier<'key> {
  /// Creates a new [`JcsEd25519Verifier`] instance.
  pub const fn new(key: &'key PublicKey) -> Self {
    Self(key)
  }
}

impl SignatureName for JcsEd25519Verifier<'_> {
  const NAME: &'static str = SIGNATURE_NAME;
}

impl<'key> SignatureVerify<'key> for JcsEd25519Verifier<'key> {
  type Actual = Self;
  type Public = PublicKey;

  fn create(key: &'key Self::Public) -> Self::Actual {
    Self::new(key)
  }

  fn verify<T>(&self, data: &T, signature: &SignatureValue) -> Result<()>
  where
    T: Serialize,
  {
    let signature: &str = signature.as_signature().ok_or(Error::InvalidProofValue)?;
    let signature: Vec<u8> = decode_b58(signature)?;

    ed25519_verify(&data.to_jcs()?, &signature, self.0.as_ref())?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::common::Object;
  use crate::common::Value;
  use crate::convert::FromJson;
  use crate::crypto::JcsEd25519Signer;
  use crate::crypto::JcsEd25519Verifier;
  use crate::crypto::KeyPair;
  use crate::crypto::PublicKey;
  use crate::crypto::SecretKey;
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
      let public: PublicKey = decode_b58(tv.public).unwrap().into();
      let secret: SecretKey = decode_b58(tv.secret).unwrap().into();
      let badkey: PublicKey = b"IOTA".to_vec().into();

      let input: Object = Object::from_json(tv.input).unwrap();
      let output: Object = Object::from_json(tv.output).unwrap();

      let signer: JcsEd25519Signer<'_> = JcsEd25519Signer::new(&secret);
      let verifier: JcsEd25519Verifier<'_> = JcsEd25519Verifier::new(&public);
      let verifier2: JcsEd25519Verifier<'_> = JcsEd25519Verifier::new(&badkey);

      let signature: SignatureValue = signer.sign(&input).unwrap();

      assert_eq!(output["proof"]["signatureValue"], signature.as_str());

      assert!(verifier.verify(&input, &signature).is_ok());

      // Fails when the key is mutated
      assert!(verifier2.verify(&input, &signature).is_err());

      // Fails when the signature is mutated
      let signature: _ = SignatureValue::Signature("IOTA".into());
      assert!(verifier.verify(&input, &signature).is_err());
    }
  }

  #[test]
  fn test_sign_hello() {
    const PUBLIC: &[u8] = b"8CpYU3CXo1NEXVi5ZJcGgfmYjMoQ4xpewofpcPnWS5kt";
    const SECRET: &[u8] = b"8gFfcuUTmX7P4DYfpEV7iVWzfSSV6QHQZFZamT6oNjVV";

    const SIG: &[u8] = b"4VjbV3672WRhKqUVn4Cdp6e7AaXYYv2f71dM8ZDHqWexfku4oLUeDVFuxGRXxpkVUwZ924zFHu527Z2ZNiPKZVeF";
    const MSG: &[u8] = b"hello";

    let public: PublicKey = decode_b58(PUBLIC).unwrap().into();
    let secret: SecretKey = decode_b58(SECRET).unwrap().into();

    let signer: JcsEd25519Signer<'_> = JcsEd25519Signer::new(&secret);
    let verifier: JcsEd25519Verifier<'_> = JcsEd25519Verifier::new(&public);

    let signature: _ = signer.sign(&MSG).unwrap();

    assert_eq!(signature.as_str().as_bytes(), SIG);
    assert_eq!(verifier.verify(&MSG, &signature).is_ok(), true);
  }

  #[test]
  fn test_sign_verify() {
    let key1: KeyPair = KeyPair::new_ed25519().unwrap();
    let key2: KeyPair = KeyPair::new_ed25519().unwrap();

    let signer: JcsEd25519Signer<'_> = JcsEd25519Signer::new(key1.secret());
    let verifier: JcsEd25519Verifier<'_> = JcsEd25519Verifier::new(key1.public());

    let data1: Value = json!({ "msg": "IOTA Identity" });
    let data2: Value = json!({ "msg": "IOTA Identity 2" });

    let signature: _ = signer.sign(&data1).unwrap();

    // The signature should be valid
    assert!(verifier.verify(&data1, &signature).is_ok());

    // Modified data should be invaldid
    assert!(verifier.verify(&data2, &signature).is_err());

    let verifier: JcsEd25519Verifier<'_> = JcsEd25519Verifier::new(key2.public());

    // A modified key should be invaldid
    assert!(verifier.verify(&data1, &signature).is_err());
  }
}
