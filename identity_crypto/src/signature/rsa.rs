use anyhow::anyhow;
use ring::{rand, signature};

use crate::{
    error::{Error, Result},
    key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
    traits::{KeyGen, Sign, Verify},
};

macro_rules! impl_rsa {
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
                let keypair =
                    signature::RsaKeyPair::from_pkcs8(secret.as_ref()).map_err(|_| Error::KeyError(anyhow!("RSA")))?;
                let mut signature = vec![0; keypair.public_modulus_len()];

                keypair
                    .sign(&$signing, &rand::SystemRandom::new(), message, &mut signature)
                    .map_err(|_| Error::SignError(anyhow!("RSA")))?;

                Ok(signature)
            }
        }

        impl Verify for $ident {
            fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
                let pubkey = signature::UnparsedPublicKey::new(&$verification, public.as_ref());
                let result = pubkey.verify(message, signature);

                Ok(result.is_ok())
            }
        }
    };
}

impl_rsa!(
    RsaPkcs1Sha256,
    signature::RSA_PKCS1_SHA256,
    signature::RSA_PKCS1_2048_8192_SHA256
);
impl_rsa!(
    RsaPkcs1Sha384,
    signature::RSA_PKCS1_SHA384,
    signature::RSA_PKCS1_2048_8192_SHA384
);
impl_rsa!(
    RsaPkcs1Sha512,
    signature::RSA_PKCS1_SHA512,
    signature::RSA_PKCS1_2048_8192_SHA512
);
impl_rsa!(
    RsaPssSha256,
    signature::RSA_PSS_SHA256,
    signature::RSA_PSS_2048_8192_SHA256
);
impl_rsa!(
    RsaPssSha384,
    signature::RSA_PSS_SHA384,
    signature::RSA_PSS_2048_8192_SHA384
);
impl_rsa!(
    RsaPssSha512,
    signature::RSA_PSS_SHA512,
    signature::RSA_PSS_2048_8192_SHA512
);
