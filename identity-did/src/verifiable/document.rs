// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::any::Any;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::merkle_key::MerkleKey;
use identity_core::crypto::merkle_key::MerkleTag;
use identity_core::crypto::merkle_key::Sha256;
use identity_core::crypto::merkle_key::Signer;
use identity_core::crypto::merkle_key::Verifier;
use identity_core::crypto::merkle_tree::Proof;
use identity_core::crypto::JcsEd25519Signature2020 as Ed25519;
use identity_core::crypto::SecretKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::SignatureSign;
use identity_core::crypto::SignatureVerify;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_core::error::Error as CoreError;
use serde::Serialize;

use crate::document::Document;
use crate::error::Error;
use crate::error::Result;
use crate::verifiable::Properties;
use crate::verifiable::Revocation;
use crate::verification::MethodQuery;
use crate::verification::MethodType;
use crate::verification::MethodWrap;

// =============================================================================
// Generic Crypto Extensions
// =============================================================================

impl<T, U, V> Document<T, U, V> {
  pub fn into_verifiable(self) -> Document<Properties<T>, U, V> {
    self.map(Properties::new)
  }

  pub fn into_verifiable2(self, proof: Signature) -> Document<Properties<T>, U, V> {
    self.map(|old| Properties::with_proof(old, proof))
  }
}

impl<T, U, V> Document<Properties<T>, U, V> {
  pub fn proof(&self) -> Option<&Signature> {
    self.properties().proof()
  }

  pub fn proof_mut(&mut self) -> Option<&mut Signature> {
    self.properties_mut().proof_mut()
  }

  pub fn set_proof(&mut self, signature: Signature) {
    self.properties_mut().set_proof(signature);
  }
}

impl<T, U, V> TrySignature for Document<Properties<T>, U, V> {
  fn signature(&self) -> Option<&Signature> {
    self.proof()
  }
}

impl<T, U, V> TrySignatureMut for Document<Properties<T>, U, V> {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof_mut()
  }
}

impl<T, U, V> SetSignature for Document<Properties<T>, U, V> {
  fn set_signature(&mut self, signature: Signature) {
    self.set_proof(signature)
  }
}

// =============================================================================
// Signature Extensions
// =============================================================================

impl<T, U, V> Document<Properties<T>, U, V>
where
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  pub fn sign_this<'a, Q>(&mut self, query: Q, secret: &[u8]) -> Result<()>
  where
    Q: Into<MethodQuery<'a>>,
  {
    let method: MethodWrap<'_, U> = self.try_resolve(query)?;
    let fragment: String = method.try_into_fragment()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        Ed25519.__sign(self, fragment, secret)?;
      }
      MethodType::MerkleKeyCollection2021 => {
        // Documents can't be signed with Merkle Key Collections
        return Err(Error::InvalidMethodType);
      }
    }

    Ok(())
  }

  pub fn verify_this(&self) -> Result<()> {
    let signature: &Signature = self.try_signature()?;
    let method: MethodWrap<'_, U> = self.try_resolve(signature)?;
    let public: Vec<u8> = method.key_data().try_decode()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        Ed25519.__verify(self, &public)?;
      }
      MethodType::MerkleKeyCollection2021 => {
        // Documents can't be signed with Merkle Key Collections
        return Err(Error::InvalidMethodType);
      }
    }

    Ok(())
  }
}

impl<T, U, V> Document<T, U, V> {
  pub fn sign_that<'a, 'b, X, Q, S>(&self, that: &mut X, query: Q, secret: S) -> Result<()>
  where
    X: Serialize + SetSignature,
    Q: Into<MethodQuery<'a>>,
    S: Into<Secret<'b>>,
  {
    let secret: Secret<'_> = secret.into();
    let method: MethodWrap<'_, U> = self.try_resolve(query)?;
    let fragment: String = method.try_into_fragment()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        Ed25519.__sign(that, fragment, secret.secret)?;
      }
      MethodType::MerkleKeyCollection2021 => {
        let data: Vec<u8> = method.key_data().try_decode()?;

        match MerkleKey::extract_tags(&data)? {
          (MerkleTag::ED25519, MerkleTag::SHA256) => {
            let signer: Signer<'_, Ed25519, Sha256> = secret
              .mk_proof
              .and_then(Any::downcast_ref)
              .ok_or(Error::CoreError(CoreError::InvalidKeyFormat))
              .map(|proof| Signer::from_borrowed(proof, Ed25519))?;

            signer.__sign(that, fragment, secret.secret)?;
          }
          (_, _) => {
            return Err(Error::InvalidMethodType);
          }
        }
      }
    }

    Ok(())
  }

  pub fn verify_that<'a, X, P>(&self, that: &X, public: P) -> Result<()>
  where
    X: Serialize + TrySignature,
    P: Into<Public<'a>>,
    U: Revocation,
  {
    let public: Public<'_> = public.into();
    let signature: &Signature = that.try_signature()?;
    let method: MethodWrap<'_, U> = self.try_resolve(signature)?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        Ed25519.__verify(that, &method.key_data().try_decode()?)?;
      }
      MethodType::MerkleKeyCollection2021 => {
        let data: Vec<u8> = method.key_data().try_decode()?;

        match MerkleKey::extract_tags(&data)? {
          (MerkleTag::ED25519, MerkleTag::SHA256) => {
            let verifier: Verifier<'_, Ed25519, Sha256> = {
              let mut this: _ = Verifier::from_borrowed(&data, Ed25519);

              if let Some(revocation) = method.revocation()? {
                this.set_revocation(revocation);
              }

              this
            };

            let target: &[u8] = public.mk_target.ok_or(Error::CoreError(CoreError::InvalidKeyFormat))?;

            verifier.__verify(that, target)?;
          }
          (_, _) => {
            return Err(Error::InvalidMethodType);
          }
        }
      }
    }

    Ok(())
  }
}

// =============================================================================
// Enhanced Public Key
// =============================================================================

#[derive(Clone, Copy, Debug, Default)]
pub struct Public<'a> {
  mk_target: Option<&'a [u8]>,
}

impl<'a> Public<'a> {
  /// Creates a new [`Public`] key object.
  pub fn new() -> Self {
    Self { mk_target: None }
  }

  /// Creates a new [`Public`] with a Merkle Key Collection `target`.
  pub fn with_merkle_target(target: &'a [u8]) -> Self {
    Self {
      mk_target: Some(target),
    }
  }

  /// Sets the `target` value of a Merkle Key Collection public key.
  pub fn set_merkle_target(&mut self, target: &'a [u8]) {
    self.mk_target = Some(target);
  }
}

impl From<()> for Public<'_> {
  fn from(_: ()) -> Self {
    Self { mk_target: None }
  }
}

// =============================================================================
// Enhanced Secret Key
// =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct Secret<'a> {
  secret: &'a [u8],
  mk_proof: Option<&'a dyn Any>,
}

impl<'a> Secret<'a> {
  /// Creates a new [`Secret`] key object from the given slice of bytes.
  pub fn new(secret: &'a [u8]) -> Self {
    Self { secret, mk_proof: None }
  }

  /// Creates a new [`Secret`] with a Merkle Key Collection [`proof`][`Proof`].
  pub fn with_merkle_proof<D>(secret: &'a [u8], proof: &'a Proof<D>) -> Self
  where
    D: MerkleDigest,
  {
    Self {
      secret,
      mk_proof: Some(proof),
    }
  }

  /// Sets the `proof` value of a Merkle Key Collection secret key.
  pub fn set_merkle_proof<D>(&mut self, proof: &'a Proof<D>)
  where
    D: MerkleDigest,
  {
    self.mk_proof = Some(proof);
  }
}

impl<'a> From<&'a [u8]> for Secret<'a> {
  fn from(other: &'a [u8]) -> Self {
    Self::new(other)
  }
}

impl<'a> From<&'a SecretKey> for Secret<'a> {
  fn from(other: &'a SecretKey) -> Self {
    Self::new(other.as_ref())
  }
}
