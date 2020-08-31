use rand::rngs::OsRng;
use secp256k1::{Message, PublicKey as PubKey, SecretKey as SecKey, Signature};
use sha2::{Digest, Sha256};

use crate::{
  error::{Error, Result},
  key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
  signature::SignatureSuite,
};

#[derive(Clone, Copy, Debug)]
pub struct EcdsaSecp256k1;

impl SignatureSuite for EcdsaSecp256k1 {
  fn signature_type(&self) -> &'static str {
    "EcdsaSecp256k1Signature2019"
  }

  fn key_type(&self) -> &'static str {
    "EcdsaSecp256k1VerificationKey2019"
  }

  fn keypair(&self, generator: KeyGenerator) -> Result<KeyPair> {
    let secret = match generator {
      KeyGenerator::Seed(_) => todo!("Handle EcdsaSecp256k1 KeyGenerator::Seed"),
      KeyGenerator::Load(ref secret) => {
        SecKey::parse_slice(secret.as_ref()).map_err(|error| Error::KeyError(error.into()))?
      }
      KeyGenerator::None => SecKey::random(&mut OsRng),
    };

    let public = PubKey::from_secret_key(&secret);

    Ok(KeyPair::new(
      public.serialize_compressed().to_vec().into(),
      secret.serialize().to_vec().into(),
    ))
  }

  fn sign(&self, document: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
    secret.check_length(&[secp256k1::util::SECRET_KEY_SIZE])?;

    let message = Message::parse_slice(&Sha256::digest(document)).map_err(|error| Error::SignError(error.into()))?;
    let secret = SecKey::parse_slice(secret.as_ref()).map_err(|error| Error::KeyError(error.into()))?;

    let (signature, _recovery_id) = secp256k1::sign(&message, &secret);

    Ok(signature.serialize().to_vec())
  }

  fn verify(&self, document: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
    public.check_length(&[
      secp256k1::util::COMPRESSED_PUBLIC_KEY_SIZE,
      secp256k1::util::RAW_PUBLIC_KEY_SIZE,
    ])?;

    let message = Message::parse_slice(&Sha256::digest(document)).map_err(|error| Error::VerifyError(error.into()))?;
    let signature = Signature::parse_slice(signature).map_err(|error| Error::VerifyError(error.into()))?;
    let public = PubKey::parse_slice(public.as_ref(), None).map_err(|error| Error::KeyError(error.into()))?;

    Ok(secp256k1::verify(&message, &signature, &public))
  }

  fn digest(&self, message: &[u8]) -> Result<Vec<u8>> {
    Ok(Sha256::digest(message).to_vec())
  }
}
