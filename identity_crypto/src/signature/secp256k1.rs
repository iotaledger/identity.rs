use rand::{
    rngs::{OsRng, StdRng},
    SeedableRng,
};
use secp256k1::{Message, PublicKey as PubKey, SecretKey as SecKey, Signature};
use zeroize::Zeroize;

use crate::{
    error::{Error, Result},
    key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
    sha2::{Digest, Sha256},
    traits::{KeyGen, Sign, Verify},
};

fn create_message(data: &[u8]) -> Result<Message> {
    Message::parse_slice(&Sha256::digest(data)).map_err(|error| Error::VerifyError(error.into()))
}

#[derive(Clone, Copy, Debug)]
pub struct Secp256k1;

impl KeyGen for Secp256k1 {
    fn generate(&self, generator: KeyGenerator) -> Result<KeyPair> {
        let secret = match generator {
            KeyGenerator::Seed(ref seed) => {
                // TODO: Clean this up
                let mut bytes: [u8; 32] = [0; 32];

                bytes[..seed.len()].copy_from_slice(seed.as_slice());

                let seckey = SecKey::random(&mut StdRng::from_seed(bytes));

                bytes.zeroize();

                seckey
            }
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
}

impl Sign for Secp256k1 {
    fn sign(&self, message: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
        secret.check_length(&[secp256k1::util::SECRET_KEY_SIZE])?;

        let message = create_message(message)?;
        let secret = SecKey::parse_slice(secret.as_ref()).map_err(|error| Error::KeyError(error.into()))?;

        let (signature, _recovery_id) = secp256k1::sign(&message, &secret);

        Ok(signature.serialize().to_vec())
    }
}

impl Verify for Secp256k1 {
    fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
        public.check_length(&[
            secp256k1::util::COMPRESSED_PUBLIC_KEY_SIZE,
            secp256k1::util::RAW_PUBLIC_KEY_SIZE,
        ])?;

        let message = create_message(message)?;
        let signature = Signature::parse_slice(signature).map_err(|error| Error::VerifyError(error.into()))?;
        let public = PubKey::parse_slice(public.as_ref(), None).map_err(|error| Error::KeyError(error.into()))?;

        Ok(secp256k1::verify(&message, &signature, &public))
    }
}
