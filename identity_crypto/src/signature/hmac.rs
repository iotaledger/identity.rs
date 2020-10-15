use crate::{
    error::Result,
    key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
    traits::{KeyGen, Sign, Verify},
};

use ring::{
    constant_time::verify_slices_are_equal,
    hmac::{self, Algorithm, Key},
};

fn hmac_sign(algorithm: Algorithm, message: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    Ok(hmac::sign(&Key::new(algorithm, key), message).as_ref().to_vec())
}

fn hmac_verify(algorithm: Algorithm, message: &[u8], signature: &[u8], key: &[u8]) -> Result<bool> {
    hmac_sign(algorithm, message, key).map(|result| verify_slices_are_equal(result.as_slice(), signature).is_ok())
}

macro_rules! impl_hmac {
    ($ident:ident, $algorithm:expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $ident;

        impl KeyGen for $ident {
            fn generate(&self, _generator: KeyGenerator) -> Result<KeyPair> {
                todo!("Implement {} KeyGen", stringify!($ident))
            }
        }

        impl Sign for $ident {
            fn sign(&self, message: &[u8], secret: &SecretKey) -> Result<Vec<u8>> {
                hmac_sign($algorithm, message, secret.as_ref())
            }
        }

        impl Verify for $ident {
            fn verify(&self, message: &[u8], signature: &[u8], public: &PublicKey) -> Result<bool> {
                hmac_verify($algorithm, message, signature, public.as_ref())
            }
        }
    };
}

impl_hmac!(HmacSha256, hmac::HMAC_SHA256);
impl_hmac!(HmacSha384, hmac::HMAC_SHA384);
impl_hmac!(HmacSha512, hmac::HMAC_SHA512);
