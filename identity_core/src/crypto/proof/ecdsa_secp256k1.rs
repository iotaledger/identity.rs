// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use serde::Serialize;

use crate::convert::ToJson;
use crate::crypto::key::Secp256k1;
use crate::crypto::Named;
use crate::crypto::ProofValue;
use crate::crypto::Sign;
use crate::crypto::Signer;
use crate::crypto::Verifier;
use crate::crypto::Verify;
use crate::error::Error;
use crate::error::Result;
use crate::utils::BaseEncoding;

/// An implementation of the [Ecdsa Secp256k1 Signature 2019][SPEC1] signature suite
/// for [Linked Data Proofs][SPEC2].
///
/// [SPEC1]: https://w3c-ccg.github.io/lds-ecdsa-secp256k1-2019/#signature-format/
/// [SPEC2]: https://w3c-ccg.github.io/ld-proofs/
pub struct EcdsaSecp256k1<T = Secp256k1>(PhantomData<T>);

impl<T> Named for EcdsaSecp256k1<T> {
  const NAME: &'static str = "EcdsaSecp256k1Signature2019";
}

impl<T> Signer<T::Private> for EcdsaSecp256k1<T>
where
  T: Sign,
  T::Output: AsRef<[u8]>,
{
  fn sign<X>(data: &X, private: &T::Private) -> Result<ProofValue>
  where
    X: Serialize,
  {
    // TODO: change JCS to RDF!
    let message: Vec<u8> = data.to_jcs()?;
    let signature: T::Output = T::sign(&message, private)?;
    // TODO: change this to a JWS!
    let signature: String = BaseEncoding::encode_base58(signature.as_ref());
    // TODO: change this to a JWS!
    Ok(ProofValue::Signature(signature))
  }
}

impl<T> Verifier<T::Public> for EcdsaSecp256k1<T>
where
  T: Verify,
{
  fn verify<X>(data: &X, signature: &ProofValue, public: &T::Public) -> Result<()>
  where
    X: Serialize + ?Sized,
  {
    // TODO: change this to JWS!
    let signature: &str = signature
      .as_signature()
      .ok_or(Error::InvalidProofValue("EcdsaSecp256k1"))?;

    // TODO: change this to decode the JWS!
    let signature: Vec<u8> = BaseEncoding::decode_base58(signature)?;
    // TODO: change JCS to RDF!
    let message: Vec<u8> = data.to_jcs()?;

    T::verify(&message, &signature, public)?;

    Ok(())
  }
}
