use rand::rngs::OsRng;
use ring::constant_time::verify_slices_are_equal;
use ring::digest;
use ring::hmac;
use ring::rand::SecureRandom as _;
use ring::rand::SystemRandom;
use ring::signature;
use ring::signature::EcdsaKeyPair;
use ring::signature::EcdsaSigningAlgorithm;
use ring::signature::Ed25519KeyPair;
use ring::signature::RsaEncoding;
use ring::signature::RsaKeyPair;
use ring::signature::Signature;
use ring::signature::UnparsedPublicKey;
use ring::signature::VerificationAlgorithm;

use crate::alloc::Vec;
use crate::crypto::KeyPair;
use crate::crypto::PKey;
use crate::crypto::Public;
use crate::crypto::Secret;
use crate::error::CryptoError;
use crate::error::Result;
use crate::jwa::EcCurve;
use crate::jwa::EcdsaAlgorithm;
use crate::jwa::EdCurve;
use crate::jwa::EddsaAlgorithm;
use crate::jwa::HashAlgorithm;
use crate::jwa::HmacAlgorithm;
use crate::jwa::RsaBits;
use crate::jwa::RsassaAlgorithm;
use crate::jwa::RsassaPssAlgorithm;

lazy_static::lazy_static! {
  static ref RANDOM: SystemRandom = SystemRandom::new();
}

#[inline(always)]
fn rng() -> &'static SystemRandom {
  &*RANDOM
}

pub(crate) fn message_digest(
  algorithm: HashAlgorithm,
  message: impl AsRef<[u8]>,
) -> Result<Vec<u8>> {
  let digest: digest::Digest = match algorithm {
    HashAlgorithm::Sha256 => digest::digest(&digest::SHA256, message.as_ref()),
    HashAlgorithm::Sha384 => digest::digest(&digest::SHA384, message.as_ref()),
    HashAlgorithm::Sha512 => digest::digest(&digest::SHA512, message.as_ref()),
  };

  Ok(digest.as_ref().to_vec())
}

pub(crate) fn ecdsa_generate(curve: EcCurve) -> Result<KeyPair> {
  match curve {
    EcCurve::P256 => todo!("ecdsa_generate(P256)"),
    EcCurve::P384 => todo!("ecdsa_generate(P384)"),
    EcCurve::P521 => todo!("ecdsa_generate(P521)"),
    EcCurve::Secp256K1 => {
      let secret: secp256k1::SecretKey = secp256k1::SecretKey::random(&mut OsRng);
      let public: secp256k1::PublicKey = secp256k1::PublicKey::from_secret_key(&secret);

      Ok((
        public.serialize_compressed()[..].into(),
        secret.serialize()[..].into(),
      ))
    }
  }
}

pub(crate) fn ecdsa_sign(
  algorithm: EcdsaAlgorithm,
  message: &[u8],
  key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  match algorithm {
    EcdsaAlgorithm::ES256 => sign_ecdsa(&signature::ECDSA_P256_SHA256_FIXED_SIGNING, message, key),
    EcdsaAlgorithm::ES384 => sign_ecdsa(&signature::ECDSA_P384_SHA384_FIXED_SIGNING, message, key),
    EcdsaAlgorithm::ES512 => todo!("ecdsa_sign(ES512)"),
    EcdsaAlgorithm::ES256K => sign_es256k(message, key),
  }
}

pub(crate) fn ecdsa_verify(
  algorithm: EcdsaAlgorithm,
  message: &[u8],
  signature: &[u8],
  key: &PKey<Public>,
) -> Result<()> {
  match algorithm {
    EcdsaAlgorithm::ES256 => {
      verify_asymmetric(&signature::ECDSA_P256_SHA256_FIXED, message, signature, key)
    }
    EcdsaAlgorithm::ES384 => {
      verify_asymmetric(&signature::ECDSA_P384_SHA384_FIXED, message, signature, key)
    }
    EcdsaAlgorithm::ES512 => todo!("ecdsa_verify(ES512)"),
    EcdsaAlgorithm::ES256K => verify_es256k(message, signature, key),
  }
}

pub(crate) fn eddsa_generate(curve: EdCurve) -> Result<KeyPair> {
  match curve {
    EdCurve::Ed25519 => todo!("eddsa_generate(Ed25519)"),
    EdCurve::Ed448 => todo!("eddsa_generate(Ed448)"),
  }
}

pub(crate) fn eddsa_sign(
  algorithm: EddsaAlgorithm,
  message: &[u8],
  key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  match algorithm {
    EddsaAlgorithm::EdDSA => sign_ed25519(message, key),
  }
}

pub(crate) fn eddsa_verify(
  algorithm: EddsaAlgorithm,
  message: &[u8],
  signature: &[u8],
  key: &PKey<Public>,
) -> Result<()> {
  match algorithm {
    EddsaAlgorithm::EdDSA => verify_asymmetric(&signature::ED25519, message, signature, key),
  }
}

pub(crate) fn hmac_generate(algorithm: HmacAlgorithm) -> Result<PKey<Secret>> {
  let mut key: Vec<u8> = match algorithm {
    HmacAlgorithm::HS256 => vec![0; hmac::HMAC_SHA256.digest_algorithm().chaining_len],
    HmacAlgorithm::HS384 => vec![0; hmac::HMAC_SHA384.digest_algorithm().chaining_len],
    HmacAlgorithm::HS512 => vec![0; hmac::HMAC_SHA512.digest_algorithm().chaining_len],
  };

  rng().fill(&mut key)?;

  Ok(key.into())
}

pub(crate) fn hmac_sign(
  algorithm: HmacAlgorithm,
  message: &[u8],
  key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  match algorithm {
    HmacAlgorithm::HS256 => sign_hmac(hmac::HMAC_SHA256, message, key),
    HmacAlgorithm::HS384 => sign_hmac(hmac::HMAC_SHA384, message, key),
    HmacAlgorithm::HS512 => sign_hmac(hmac::HMAC_SHA512, message, key),
  }
}

pub(crate) fn hmac_verify(
  algorithm: HmacAlgorithm,
  message: &[u8],
  signature: &[u8],
  key: &PKey<Public>,
) -> Result<()> {
  match algorithm {
    HmacAlgorithm::HS256 => verify_hmac(hmac::HMAC_SHA256, message, signature, key),
    HmacAlgorithm::HS384 => verify_hmac(hmac::HMAC_SHA384, message, signature, key),
    HmacAlgorithm::HS512 => verify_hmac(hmac::HMAC_SHA512, message, signature, key),
  }
}

pub(crate) fn rsa_generate(_bits: RsaBits) -> Result<KeyPair> {
  todo!("rsa_generate(RsaBits)")
}

pub(crate) fn rsassa_sign(
  algorithm: RsassaAlgorithm,
  message: &[u8],
  key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  match algorithm {
    RsassaAlgorithm::RS256 => sign_rsa(&signature::RSA_PKCS1_SHA256, message, key),
    RsassaAlgorithm::RS384 => sign_rsa(&signature::RSA_PKCS1_SHA384, message, key),
    RsassaAlgorithm::RS512 => sign_rsa(&signature::RSA_PKCS1_SHA512, message, key),
  }
}

pub(crate) fn rsassa_verify(
  algorithm: RsassaAlgorithm,
  message: &[u8],
  signature: &[u8],
  key: &PKey<Public>,
) -> Result<()> {
  match algorithm {
    RsassaAlgorithm::RS256 => verify_asymmetric(
      &signature::RSA_PKCS1_2048_8192_SHA256,
      message,
      signature,
      key,
    ),
    RsassaAlgorithm::RS384 => verify_asymmetric(
      &signature::RSA_PKCS1_2048_8192_SHA384,
      message,
      signature,
      key,
    ),
    RsassaAlgorithm::RS512 => verify_asymmetric(
      &signature::RSA_PKCS1_2048_8192_SHA512,
      message,
      signature,
      key,
    ),
  }
}

pub(crate) fn rsassa_pss_sign(
  algorithm: RsassaPssAlgorithm,
  message: &[u8],
  key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  match algorithm {
    RsassaPssAlgorithm::PS256 => sign_rsa(&signature::RSA_PSS_SHA256, message, key),
    RsassaPssAlgorithm::PS384 => sign_rsa(&signature::RSA_PSS_SHA384, message, key),
    RsassaPssAlgorithm::PS512 => sign_rsa(&signature::RSA_PSS_SHA512, message, key),
  }
}

pub(crate) fn rsassa_pss_verify(
  algorithm: RsassaPssAlgorithm,
  message: &[u8],
  signature: &[u8],
  key: &PKey<Public>,
) -> Result<()> {
  match algorithm {
    RsassaPssAlgorithm::PS256 => verify_asymmetric(
      &signature::RSA_PSS_2048_8192_SHA256,
      message,
      signature,
      key,
    ),
    RsassaPssAlgorithm::PS384 => verify_asymmetric(
      &signature::RSA_PSS_2048_8192_SHA384,
      message,
      signature,
      key,
    ),
    RsassaPssAlgorithm::PS512 => verify_asymmetric(
      &signature::RSA_PSS_2048_8192_SHA512,
      message,
      signature,
      key,
    ),
  }
}

fn sign_ed25519(message: &[u8], key: &PKey<Secret>) -> Result<Vec<u8>> {
  let key: Ed25519KeyPair = Ed25519KeyPair::from_pkcs8_maybe_unchecked(key.as_ref())?;
  let sig: Signature = key.sign(message);

  Ok(sig.as_ref().to_vec())
}

fn sign_es256k(message: &[u8], key: &PKey<Secret>) -> Result<Vec<u8>> {
  let digest: digest::Digest = digest::digest(&digest::SHA256, message);
  let msg: secp256k1::Message = secp256k1::Message::parse_slice(digest.as_ref())?;
  let key: secp256k1::SecretKey = secp256k1::SecretKey::parse_slice(key.as_ref())?;

  let (sig, _): (secp256k1::Signature, _) = secp256k1::sign(&msg, &key);

  Ok(sig.serialize().to_vec())
}

fn sign_hmac<T>(algorithm: hmac::Algorithm, message: &[u8], key: &PKey<T>) -> Result<Vec<u8>> {
  let key: hmac::Key = hmac::Key::new(algorithm, key.as_ref());
  let sig: hmac::Tag = hmac::sign(&key, message);

  Ok(sig.as_ref().to_vec())
}

fn sign_ecdsa(
  algorithm: &'static EcdsaSigningAlgorithm,
  message: &[u8],
  key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  let key: EcdsaKeyPair = EcdsaKeyPair::from_pkcs8(algorithm, key.as_ref())?;
  let sig: Signature = key.sign(rng(), message)?;

  Ok(sig.as_ref().to_vec())
}

fn sign_rsa(
  padding: &'static dyn RsaEncoding,
  message: &[u8],
  key: &PKey<Secret>,
) -> Result<Vec<u8>> {
  let key: RsaKeyPair = RsaKeyPair::from_pkcs8(key.as_ref())?;
  let mut sig: Vec<u8> = vec![0; key.public_modulus_len()];

  key.sign(padding, rng(), message, &mut sig)?;

  Ok(sig)
}

fn verify_asymmetric(
  algorithm: &'static dyn VerificationAlgorithm,
  message: &[u8],
  signature: &[u8],
  key: &PKey<Public>,
) -> Result<()> {
  let key: UnparsedPublicKey<_> = UnparsedPublicKey::new(algorithm, key.as_ref());

  key.verify(message, signature)?;

  Ok(())
}

fn verify_es256k(message: &[u8], signature: &[u8], key: &PKey<Public>) -> Result<()> {
  let digest: digest::Digest = digest::digest(&digest::SHA256, message);
  let msg: secp256k1::Message = secp256k1::Message::parse_slice(digest.as_ref())?;
  let sig: secp256k1::Signature = secp256k1::Signature::parse_slice(signature)?;
  let key: secp256k1::PublicKey = secp256k1::PublicKey::parse_slice(key.as_ref(), None)?;

  if secp256k1::verify(&msg, &sig, &key) {
    Ok(())
  } else {
    Err(CryptoError::InvalidSignature("ES256K").into())
  }
}

fn verify_hmac<T>(
  algorithm: hmac::Algorithm,
  message: &[u8],
  signature: &[u8],
  key: &PKey<T>,
) -> Result<()> {
  let sig: Vec<u8> = sign_hmac(algorithm, message, key)?;

  verify_slices_are_equal(sig.as_slice(), signature)?;

  Ok(())
}
