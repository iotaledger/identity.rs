// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use serde::Serialize;

use crate::convert::ToJson;
use crate::crypto::Ed25519;
use crate::crypto::Named;
use crate::crypto::ProofValue;
use crate::crypto::Sign;
use crate::crypto::Signer;
use crate::crypto::Verifier;
use crate::crypto::Verify;
use crate::error::Error;
use crate::error::Result;
use crate::utils::BaseEncoding;

// TODO: Marker trait for Ed25519 implementations (?)

/// An implementation of the [JCS Ed25519 Signature 2020][SPEC1] signature suite
/// for [Linked Data Proofs][SPEC2] modified to pass the official test vectors.
///
/// Users should use the [`Sign`]/[`Verify`] traits to access
/// this implementation.
///
/// ## Deviation from the [JCS Ed25519 Signature 2020 specification][SPEC1]
/// This implementation follows the specification with the single exception that the SHA-256 pre-hash of the input, the third step of the [proof generation algorithm](https://identity.foundation/JcsEd25519Signature2020/#ProofGeneration), is skipped.
///
/// We deviate from the specification to satisfy the [official test vectors](https://github.com/decentralized-identity/JcsEd25519Signature2020/tree/master/signature-suite-impls/test-vectors), as well as to achieve compatibility with the official reference implementations for [Go](https://github.com/decentralized-identity/JcsEd25519Signature2020/tree/master/signature-suite-impls/golang)
/// and [Java](https://github.com/decentralized-identity/JcsEd25519Signature2020/tree/master/signature-suite-impls/java).
///
/// See [this GitHub issue](https://github.com/decentralized-identity/JcsEd25519Signature2020/issues/22) for further discussions.
///
///
/// [SPEC1]: https://identity.foundation/JcsEd25519Signature2020/
/// [SPEC2]: https://w3c-ccg.github.io/ld-proofs/
pub struct JcsEd25519<T = Ed25519>(PhantomData<T>);

impl<T> Named for JcsEd25519<T> {
  const NAME: &'static str = "JcsEd25519Signature2020";
}

impl<T> Signer<T::Private> for JcsEd25519<T>
where
  T: Sign,
  T::Output: AsRef<[u8]>,
{
  fn sign<X>(data: &X, private: &T::Private) -> Result<ProofValue>
  where
    X: Serialize,
  {
    let message: Vec<u8> = data.to_jcs()?;
    let signature: T::Output = T::sign(&message, private)?;
    let signature: String = BaseEncoding::encode_base58(signature.as_ref());

    Ok(ProofValue::Signature(signature))
  }
}

impl<T> Verifier<T::Public> for JcsEd25519<T>
where
  T: Verify,
{
  fn verify<X>(data: &X, signature: &ProofValue, public: &T::Public) -> Result<()>
  where
    X: Serialize + ?Sized,
  {
    let signature: &str = signature
      .as_signature()
      .ok_or(Error::InvalidProofValue("jcs ed25519"))?;

    let signature: Vec<u8> = BaseEncoding::decode_base58(signature)?;
    let message: Vec<u8> = data.to_jcs()?;

    T::verify(&message, &signature, public)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::common::Object;
  use crate::common::Value;
  use crate::convert::FromJson;
  use crate::crypto::Ed25519;
  use crate::crypto::JcsEd25519;
  use crate::crypto::KeyPair;
  use crate::crypto::KeyType;
  use crate::crypto::PrivateKey;
  use crate::crypto::ProofValue;
  use crate::crypto::PublicKey;
  use crate::crypto::Signer as _;
  use crate::crypto::Verifier as _;
  use crate::json;
  use crate::utils::BaseEncoding;
  use serde::Deserialize;

  type Signer = JcsEd25519<Ed25519<PrivateKey>>;

  type Verifier = JcsEd25519<Ed25519<PublicKey>>;

  #[test]
  fn test_tvs() {
    // Represents a test vector from [JcsEd25519Signature2020](https://github.com/decentralized-identity/JcsEd25519Signature2020/tree/master/signature-suite-impls/test-vectors),
    #[derive(Deserialize)]
    struct TestVector {
      public: String,
      private: String,
      input: Object,
      output: Object,
    }

    const TV_1_BYTES: &[u8] = include_bytes!("../../../tests/fixtures/jcs_ed25519/test_vector_1.json");
    const TV_2_BYTES: &[u8] = include_bytes!("../../../tests/fixtures/jcs_ed25519/test_vector_2.json");
    const TEST_VECTOR_BYTES: [&[u8]; 2] = [TV_1_BYTES, TV_2_BYTES];

    for tv_bytes in TEST_VECTOR_BYTES {
      let TestVector {
        public,
        private,
        input,
        output,
      } = TestVector::from_json_slice(tv_bytes).unwrap();

      // The test vectors use [Go crypto/ed25519](https://pkg.go.dev/crypto/ed25519#pkg-types)'s convention of representing an Ed25519 private key as: 32-byte seed concatenated with the 32-byte public key (computed from the seed).
      // We follow the convention from [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2) and extract the 32-byte seed as the private key.
      let public: PublicKey = BaseEncoding::decode_base58(public.as_str()).unwrap().into();
      let private: PrivateKey = (BaseEncoding::decode_base58(private.as_str()).unwrap()[..32])
        .to_vec()
        .into();
      let badkey: PublicKey = b"IOTA".to_vec().into();

      let signature: ProofValue = Signer::sign(&input, &private).unwrap();

      assert_eq!(output["proof"]["signatureValue"], signature.as_str());

      assert!(Verifier::verify(&input, &signature, &public).is_ok());

      // Fails when the key is mutated
      assert!(Verifier::verify(&input, &signature, &badkey).is_err());

      // Fails when the signature is mutated
      let signature = ProofValue::Signature("IOTA".into());
      assert!(Verifier::verify(&input, &signature, &public).is_err());
    }
  }

  #[test]
  fn test_sign_hello() {
    const PUBLIC: &str = "8CpYU3CXo1NEXVi5ZJcGgfmYjMoQ4xpewofpcPnWS5kt";
    const SECRET: &str = "8gFfcuUTmX7P4DYfpEV7iVWzfSSV6QHQZFZamT6oNjVV";

    const SIG: &[u8] = b"4VjbV3672WRhKqUVn4Cdp6e7AaXYYv2f71dM8ZDHqWexfku4oLUeDVFuxGRXxpkVUwZ924zFHu527Z2ZNiPKZVeF";
    const MSG: &[u8] = b"hello";

    let public: PublicKey = BaseEncoding::decode_base58(PUBLIC).unwrap().into();
    let private: PrivateKey = BaseEncoding::decode_base58(SECRET).unwrap().into();

    let signature: ProofValue = Signer::sign(&MSG, &private).unwrap();

    assert_eq!(signature.as_str().as_bytes(), SIG);
    assert!(Verifier::verify(&MSG, &signature, &public).is_ok());
  }

  #[test]
  fn test_sign_verify() {
    let key1: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let key2: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();

    let data1: Value = json!({ "msg": "IOTA Identity" });
    let data2: Value = json!({ "msg": "IOTA Identity 2" });

    let signature = Signer::sign(&data1, key1.private()).unwrap();

    // The signature should be valid
    assert!(Verifier::verify(&data1, &signature, key1.public()).is_ok());

    // Modified data should be invaldid
    assert!(Verifier::verify(&data2, &signature, key1.public()).is_err());

    // A modified key should be invaldid
    assert!(Verifier::verify(&data1, &signature, key2.public()).is_err());
  }
}
