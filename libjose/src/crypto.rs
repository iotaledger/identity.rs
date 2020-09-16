use core::marker::PhantomData;

use crate::error::Result;
use crate::jwa::EcdsaAlgorithm;
use crate::jwa::EddsaAlgorithm;
use crate::jwa::HmacAlgorithm;
use crate::jwa::RsaAlgorithm;
use crate::jwk::EcCurve;
use crate::jwk::EdCurve;
use crate::jwk::RsaBits;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Public {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Secret {}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PKey<T>(PhantomData<T>);

pub type KeyPair = (PKey<Public>, PKey<Secret>);

pub(crate) fn ecdsa_generate(_curve: EcCurve) -> Result<KeyPair> {
  todo!("ecdsa_generate")
}

pub(crate) fn ecdsa_sign(
  _algorithm: EcdsaAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("ecdsa_sign")
}

pub(crate) fn ecdsa_verify(
  _algorithm: EcdsaAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("ecdsa_verify")
}

pub(crate) fn eddsa_generate(_curve: EdCurve) -> Result<KeyPair> {
  todo!("eddsa_generate")
}

pub(crate) fn eddsa_sign(
  _algorithm: EddsaAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("eddsa_sign")
}

pub(crate) fn eddsa_verify(
  _algorithm: EddsaAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("eddsa_verify")
}

pub(crate) fn hmac_generate(_algorithm: HmacAlgorithm) -> Result<PKey<Secret>> {
  todo!("hmac_generate")
}

pub(crate) fn hmac_sign(
  _algorithm: HmacAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("hmac_sign")
}

pub(crate) fn hmac_verify(
  _algorithm: HmacAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("hmac_verify")
}

pub(crate) fn rsa_generate(_bits: RsaBits) -> Result<KeyPair> {
  todo!("rsa_generate")
}

pub(crate) fn rsa_sign(
  _algorithm: RsaAlgorithm,
  _message: &[u8],
  _key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  todo!("rsa_sign")
}

pub(crate) fn rsa_verify(
  _algorithm: RsaAlgorithm,
  _message: &[u8],
  _signature: &[u8],
  _key: &PKey<Public>,
) -> Result<()> {
  todo!("rsa_verify")
}
