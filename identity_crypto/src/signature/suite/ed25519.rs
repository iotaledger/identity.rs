use ed25519_dalek::{self as ed25519, PublicKey as PubKey, SecretKey as SecKey, Signature, Signer as _, Verifier as _};
use rand::rngs::OsRng;
use sha2::{Digest, Sha512};
use std::convert::TryFrom;

use crate::{
  error::{Error, Result},
  key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
  signature::SignatureSuite,
};

fn to_keypair(secret: &SecretKey) -> Result<ed25519::Keypair> {
  let secret = SecKey::from_bytes(secret.as_ref()).map_err(|error| Error::KeyError(error.into()))?;

  Ok(ed25519::Keypair {
    public: PubKey::from(&secret),
    secret,
  })
}

/// An implementation of the 2018 Ed25519 Signature Suite
///
/// Ref: https://w3c-ccg.github.io/lds-ed25519-2018/
#[derive(Clone, Copy, Debug)]
pub struct Ed25519;

impl SignatureSuite for Ed25519 {
  fn signature_type(&self) -> &'static str {
    "Ed25519Signature2018"
  }

  fn key_type(&self) -> &'static str {
    "Ed25519VerificationKey2018"
  }

  fn keypair(&self, generator: KeyGenerator) -> Result<KeyPair> {
    let keypair = match generator {
      KeyGenerator::Seed(_) => todo!("Handle Ed25519 KeyGenerator::Seed"),
      KeyGenerator::Load(ref secret) => to_keypair(secret)?,
      KeyGenerator::None => ed25519::Keypair::generate(&mut OsRng),
    };

    Ok(KeyPair::new(
      keypair.public.to_bytes().to_vec().into(),
      keypair.secret.to_bytes().to_vec().into(),
    ))
  }

  fn sign(&self, document: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
    secret.check_length(&[ed25519::SECRET_KEY_LENGTH])?;

    let keypair = to_keypair(secret)?;
    let digest = keypair.sign(document).to_bytes();

    Ok(digest.to_vec())
  }

  fn verify(&self, document: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
    public.check_length(&[ed25519::PUBLIC_KEY_LENGTH])?;

    let public = PubKey::from_bytes(public.as_ref()).map_err(|error| Error::KeyError(error.into()))?;
    let signature = Signature::try_from(signature).map_err(|error| Error::VerifyError(error.into()))?;

    Ok(public.verify(document, &signature).is_ok())
  }

  fn digest(&self, message: &[u8]) -> Result<Vec<u8>> {
    Ok(Sha512::digest(message).to_vec())
  }
}
