use anyhow::anyhow;
use ring::{
    rand::SystemRandom,
    signature::{self, EcdsaKeyPair, UnparsedPublicKey},
};

use crate::{
    error::{Error, Result},
    key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
    traits::{KeyGen, Sign, Verify},
};

macro_rules! impl_ecdsa {
    ($ident:ident, $signing:expr, $verification:expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $ident;

        impl KeyGen for $ident {
            fn generate(&self, _generator: KeyGenerator) -> Result<KeyPair> {
                todo!("Implement {} KeyGen", stringify!($ident))
            }
        }

        impl Sign for $ident {
            fn sign(&self, message: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
                let keypair = EcdsaKeyPair::from_pkcs8(&$signing, secret.as_ref())
                    .map_err(|_| Error::KeyError(anyhow!("ECDSA")))?;

                keypair
                    .sign(&SystemRandom::new(), message)
                    .map_err(|_| Error::SignError(anyhow!("ECDSA")))
                    .map(|signature| signature.as_ref().to_vec())
            }
        }

        impl Verify for $ident {
            fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
                let pubkey = UnparsedPublicKey::new(&$verification, public.as_ref());
                let result = pubkey.verify(message, signature);

                Ok(result.is_ok())
            }
        }
    };
}

impl_ecdsa!(
    EcdsaP256Sha256,
    signature::ECDSA_P256_SHA256_FIXED_SIGNING,
    signature::ECDSA_P256_SHA256_FIXED
);
impl_ecdsa!(
    EcdsaP384Sha384,
    signature::ECDSA_P384_SHA384_FIXED_SIGNING,
    signature::ECDSA_P384_SHA384_FIXED
);
