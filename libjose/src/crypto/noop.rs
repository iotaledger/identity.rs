use crate::crypto::KeyPair;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::Result;
use crate::jwa::EcdsaAlgorithm;
use crate::jwa::EddsaAlgorithm;
use crate::jwa::HmacAlgorithm;
use crate::jwa::RsaAlgorithm;
use crate::jwk::EcCurve;
use crate::jwk::EdCurve;
use crate::jwk::HashAlgorithm;
use crate::jwk::RsaBits;

pub(crate) fn message_digest(
  _algorithm: HashAlgorithm,
  _message: impl AsRef<[u8]>,
) -> Result<Vec<u8>> {
  todo!("[noop] message_digest")
}

pub(crate) fn ecdsa_generate(_curve: EcCurve) -> Result<KeyPair> {
  todo!("[noop] ecdsa_generate")
}

pub(crate) fn ecdsa_sign(
  _algorithm: EcdsaAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("[noop] ecdsa_sign")
}

pub(crate) fn ecdsa_verify(
  _algorithm: EcdsaAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("[noop] ecdsa_verify")
}

pub(crate) fn eddsa_generate(_curve: EdCurve) -> Result<KeyPair> {
  todo!("[noop] eddsa_generate")
}

pub(crate) fn eddsa_sign(
  _algorithm: EddsaAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("[noop] eddsa_sign")
}

pub(crate) fn eddsa_verify(
  _algorithm: EddsaAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("[noop] eddsa_verify")
}

pub(crate) fn hmac_generate(_algorithm: HmacAlgorithm) -> Result<PKey<Secret>> {
  todo!("[noop] hmac_generate")
}

pub(crate) fn hmac_sign(
  _algorithm: HmacAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("[noop] hmac_sign")
}

pub(crate) fn hmac_verify(
  _algorithm: HmacAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("[noop] hmac_verify")
}

pub(crate) fn rsa_generate(_bits: RsaBits) -> Result<KeyPair> {
  todo!("[noop] rsa_generate")
}

pub(crate) fn rsa_sign(
  _algorithm: RsaAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("[noop] rsa_sign")
}

pub(crate) fn rsa_verify(
  _algorithm: RsaAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("[noop] rsa_verify")
}
