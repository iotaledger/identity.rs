use anyhow::anyhow;
use sodiumoxide::crypto::sign::ed25519::{self, PublicKey as PubKey, SecretKey as SecKey, Seed, Signature};

use crate::{
    error::{Error, Result},
    key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
    traits::{KeyGen, Sign, Verify},
};

#[derive(Clone, Copy, Debug)]
pub struct Ed25519;

impl KeyGen for Ed25519 {
    fn generate(&self, generator: KeyGenerator) -> Result<KeyPair> {
        let (public, secret) = match generator {
            KeyGenerator::Seed(ref seed) => Seed::from_slice(seed)
                .map(|seed| ed25519::keypair_from_seed(&seed))
                .ok_or_else(|| Error::KeyError(anyhow!("Ed25519")))?,
            KeyGenerator::Load(ref secret) => SecKey::from_slice(secret.as_ref())
                .map(|secret| (secret.public_key(), secret))
                .ok_or_else(|| Error::KeyError(anyhow!("Ed25519")))?,
            KeyGenerator::None => ed25519::gen_keypair(),
        };

        Ok(KeyPair::new(
            public.as_ref().to_vec().into(),
            secret.as_ref().to_vec().into(),
        ))
    }
}

impl Sign for Ed25519 {
    fn sign(&self, message: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
        let seckey = SecKey::from_slice(secret.as_ref()).ok_or_else(|| Error::SignError(anyhow!("Ed25519")))?;

        Ok(ed25519::sign_detached(message, &seckey).as_ref().to_vec())
    }
}

impl Verify for Ed25519 {
    fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
        let pubkey = PubKey::from_slice(public.as_ref()).ok_or_else(|| Error::VerifyError(anyhow!("Ed25519")))?;
        let signature = Signature::from_slice(signature).ok_or_else(|| Error::VerifyError(anyhow!("Ed25519")))?;

        Ok(ed25519::verify_detached(&signature, message, &pubkey))
    }
}
