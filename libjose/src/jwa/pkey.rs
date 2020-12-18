use core::any::Any;
use core::convert::TryFrom as _;
use core::convert::TryInto as _;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::marker::PhantomData;
use crypto::hashes::sha::SHA256;
use crypto::hashes::sha::SHA256_LEN;
use crypto::hashes::sha::SHA384;
use crypto::hashes::sha::SHA384_LEN;
use crypto::hashes::sha::SHA512;
use crypto::hashes::sha::SHA512_LEN;
use curve25519_dalek::edwards;
use rsa::PublicKey as _;
use rsa::PublicKeyParts as _;

use crate::error::Error;
use crate::error::Result;
use crate::jwa::EcCurve;
use crate::jwa::EcxCurve;
use crate::jwa::EdCurve;
use crate::jwa::RsaAlgorithm;
use crate::jwa::RsaBits;
use crate::lib::*;
use crate::utils::random_bytes;
use crate::utils::OsRng;

macro_rules! rsa_padding {
  (@PKCS1_SHA256) => {
    rsa::PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256))
  };
  (@PKCS1_SHA384) => {
    rsa::PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_384))
  };
  (@PKCS1_SHA512) => {
    rsa::PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_512))
  };
  (@PSS_SHA256) => {
    rsa::PaddingScheme::new_pss::<::sha2::Sha256, _>(OsRng)
  };
  (@PSS_SHA384) => {
    rsa::PaddingScheme::new_pss::<::sha2::Sha384, _>(OsRng)
  };
  (@PSS_SHA512) => {
    rsa::PaddingScheme::new_pss::<::sha2::Sha512, _>(OsRng)
  };
}

macro_rules! SHA_256 {
  ($message:expr) => {{
    let mut output: [u8; SHA256_LEN] = [0; SHA256_LEN];
    SHA256($message, &mut output);
    output
  }};
}

macro_rules! SHA_384 {
  ($message:expr) => {{
    let mut output: [u8; SHA384_LEN] = [0; SHA384_LEN];
    SHA384($message, &mut output);
    output
  }};
}

macro_rules! SHA_512 {
  ($message:expr) => {{
    let mut output: [u8; SHA512_LEN] = [0; SHA512_LEN];
    SHA512($message, &mut output);
    output
  }};
}

const ED25519_SECRET_LEN: usize = 32;

const X25519_PUBLIC_LEN: usize = 32;
const X25519_SECRET_LEN: usize = 32;

type P256PublicKey = p256::ecdsa::VerifyingKey;
type P256SecretKey = p256::ecdsa::SigningKey;

type K256PublicKey = k256::ecdsa::VerifyingKey;
type K256SecretKey = k256::ecdsa::SigningKey;

type Ed25519PublicKey = crypto::ed25519::PublicKey;
type Ed25519SecretKey = crypto::ed25519::SecretKey;

type X25519PublicKey = x25519_dalek::PublicKey;
type X25519SecretKey = x25519_dalek::StaticSecret;

type P256Point = p256::EncodedPoint;
type P256Signature = p256::ecdsa::Signature;

type K256Point = k256::EncodedPoint;
type K256Signature = k256::ecdsa::Signature;

type RsaPublicKey = rsa::RSAPublicKey;
type RsaPrivateKey = rsa::RSAPrivateKey;

pub enum Public {}

pub enum Secret {}

pub struct PKey<T> {
  key: Box<dyn Any>,
  vis: PhantomData<T>,
}

impl<T> PKey<T> {
  fn downcast_ref<U: 'static>(&self, identifier: &'static str) -> Result<&U> {
    self
      .key
      .downcast_ref::<U>()
      .ok_or(Error::KeyError(identifier))
  }

  fn from_key<A>(key: A) -> Self
  where
    A: Any,
  {
    Self {
      key: Box::new(key),
      vis: PhantomData,
    }
  }
}

impl<T> Debug for PKey<T> {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_str("PKey {{ .. }}")
  }
}

// =============================================================================
// Public Key Methods
// =============================================================================

impl PKey<Public> {
  pub fn from_rsa_public_components(n: rsa::BigUint, e: rsa::BigUint) -> Result<Self> {
    RsaPublicKey::new(n, e)
      .map_err(|_| Error::KeyError("Rsa"))
      .map(Self::from_key)
  }

  pub fn to_rsa_public_components(&self) -> Result<(&rsa::BigUint, &rsa::BigUint)> {
    self
      .downcast_ref::<RsaPublicKey>("Rsa")
      .map(|key| (key.n(), key.e()))
  }

  pub fn from_ec_coord(curve: EcCurve, x: &[u8], y: &[u8]) -> Result<Self> {
    match curve {
      EcCurve::P256 => {
        let p: _ = P256Point::from_affine_coordinates(x.into(), y.into(), false);

        P256PublicKey::from_encoded_point(&p)
          .map_err(|_| Error::KeyError(curve.name()))
          .map(Self::from_key)
      }
      EcCurve::P384 => todo!("Public::from_coord(P384)"),
      EcCurve::P521 => todo!("Public::from_coord(P521)"),
      EcCurve::Secp256K1 => {
        let p: _ = K256Point::from_affine_coordinates(x.into(), y.into(), false);

        K256PublicKey::from_encoded_point(&p)
          .map_err(|_| Error::KeyError(curve.name()))
          .map(Self::from_key)
      }
    }
  }

  pub fn to_ec_coord(&self, curve: EcCurve) -> Result<(Vec<u8>, Vec<u8>)> {
    match curve {
      EcCurve::P256 => {
        let point: P256Point = self
          .downcast_ref::<P256PublicKey>(curve.name())
          .map(|key| key.to_encoded_point(false))?;

        debug_assert!(!point.is_compressed());

        Ok((
          point.x().expect("Invalid Point: Identity").to_vec(),
          point.y().expect("Invalid Point: Compressed").to_vec(),
        ))
      }
      EcCurve::P384 => todo!("Public::to_ec_coord(P384)"),
      EcCurve::P521 => todo!("Public::to_ec_coord(P521)"),
      EcCurve::Secp256K1 => {
        use k256::elliptic_curve::sec1::ToEncodedPoint as _;

        let point: K256Point = self
          .downcast_ref::<K256PublicKey>(curve.name())
          .map(|key| key.to_encoded_point(false))?;

        debug_assert!(!point.is_compressed());

        Ok((
          point.x().expect("Invalid Point: Identity").to_vec(),
          point.y().expect("Invalid Point: Compressed").to_vec(),
        ))
      }
    }
  }

  pub fn try_ec_verify(&self, curve: EcCurve, message: &[u8], signature: &[u8]) -> Result<()> {
    match curve {
      EcCurve::P256 => {
        use p256::ecdsa::signature::Verifier as _;

        let signature: P256Signature = signature
          .try_into()
          .map_err(|_| Error::SigError(curve.name()))?;

        self
          .downcast_ref::<P256PublicKey>(curve.name())?
          .verify(message, &signature)
          .map_err(|_| Error::SigError(curve.name()))?;
      }
      EcCurve::P384 => todo!("Public::try_ec_verify(P384)"),
      EcCurve::P521 => todo!("Public::try_ec_verify(P521)"),
      EcCurve::Secp256K1 => {
        use k256::ecdsa::signature::Verifier as _;

        let signature: K256Signature = signature
          .try_into()
          .map_err(|_| Error::SigError(curve.name()))?;

        self
          .downcast_ref::<K256PublicKey>(curve.name())?
          .verify(message, &signature)
          .map_err(|_| Error::SigError(curve.name()))?;
      }
    }

    Ok(())
  }

  pub fn try_ed_verify(&self, curve: EdCurve, message: &[u8], signature: &[u8]) -> Result<()> {
    match curve {
      EdCurve::Ed25519 => {
        self
          .downcast_ref::<Ed25519PublicKey>(curve.name())?
          .verify(&signature.try_into()?, message)?;
      }
      EdCurve::Ed448 => todo!("Public::try_ed_verify(Ed448)"),
    }

    Ok(())
  }

  pub fn try_rsa_verify(
    &self,
    algorithm: RsaAlgorithm,
    message: &[u8],
    signature: &[u8],
  ) -> Result<()> {
    let key: &RsaPublicKey = self.downcast_ref(algorithm.name())?;

    let out: Result<(), _> = match algorithm {
      RsaAlgorithm::RS256 => key.verify(rsa_padding!(@PKCS1_SHA256), &SHA_256!(message), signature),
      RsaAlgorithm::RS384 => key.verify(rsa_padding!(@PKCS1_SHA384), &SHA_384!(message), signature),
      RsaAlgorithm::RS512 => key.verify(rsa_padding!(@PKCS1_SHA512), &SHA_512!(message), signature),
      RsaAlgorithm::PS256 => key.verify(rsa_padding!(@PSS_SHA256), &SHA_256!(message), signature),
      RsaAlgorithm::PS384 => key.verify(rsa_padding!(@PSS_SHA384), &SHA_384!(message), signature),
      RsaAlgorithm::PS512 => key.verify(rsa_padding!(@PSS_SHA512), &SHA_512!(message), signature),
    };

    out.map_err(|_| Error::SigError(algorithm.name()))
  }
}

// =============================================================================
// Secret Key Methods
// =============================================================================

impl PKey<Secret> {
  pub fn generate_ec(curve: EcCurve) -> Result<Self> {
    match curve {
      EcCurve::P256 => Ok(Self::from_key(P256SecretKey::random(OsRng))),
      EcCurve::P384 => todo!("Secret::generate_ec"),
      EcCurve::P521 => todo!("Secret::generate_ec"),
      EcCurve::Secp256K1 => Ok(Self::from_key(K256SecretKey::random(OsRng))),
    }
  }

  pub fn generate_ed(curve: EdCurve) -> Result<Self> {
    match curve {
      EdCurve::Ed25519 => Ed25519SecretKey::generate()
        .map_err(Into::into)
        .map(Self::from_key),
      EdCurve::Ed448 => todo!("Secret::generate_ed(Ed448)"),
    }
  }

  pub fn generate_ecx(curve: EcxCurve) -> Result<Self> {
    match curve {
      EcxCurve::X25519 => Ok(Self::from_key(X25519SecretKey::new(OsRng))),
      EcxCurve::X448 => todo!("Secret::generate_ecx"),
    }
  }

  pub fn generate_rsa(bits: RsaBits) -> Result<Self> {
    RsaPrivateKey::new(&mut OsRng, bits.bits())
      .map_err(|_| Error::KeyError("Rsa"))
      .map(Self::from_key)
  }

  pub fn generate_raw(size: usize) -> Result<Self> {
    Ok(Self::from_key(random_bytes(size)?))
  }

  pub fn from_raw_bytes(data: impl AsRef<[u8]>) -> Self {
    Self::from_key(data.as_ref().to_vec())
  }

  pub fn to_raw_bytes(&self) -> Result<&[u8]> {
    self.downcast_ref::<Vec<u8>>("Raw").map(|bytes| &**bytes)
  }

  pub fn from_rsa_secret_components(
    n: rsa::BigUint,
    e: rsa::BigUint,
    d: rsa::BigUint,
    primes: Vec<rsa::BigUint>,
  ) -> Result<Self> {
    let key: RsaPrivateKey = RsaPrivateKey::from_components(n, e, d, primes);

    key.validate().map_err(|_| Error::KeyError("Rsa"))?;

    Ok(Self::from_key(key))
  }

  pub fn to_rsa_secret_components(&self) -> Result<(&rsa::BigUint, &[rsa::BigUint])> {
    self
      .downcast_ref::<RsaPrivateKey>("Rsa")
      .map(|key| (key.d(), key.primes()))
  }

  pub fn ec_public_key(&self, curve: EcCurve) -> Result<PKey<Public>> {
    match curve {
      EcCurve::P256 => self
        .downcast_ref::<P256SecretKey>(curve.name())
        .map(|key| PKey::from_key(key.verify_key())),
      EcCurve::P384 => todo!("Secret::ec_public_key(P384)"),
      EcCurve::P521 => todo!("Secret::ec_public_key(P521)"),
      EcCurve::Secp256K1 => self
        .downcast_ref::<K256SecretKey>(curve.name())
        .map(|key| PKey::from_key(key.verify_key())),
    }
  }

  pub fn ed_public_key(&self, curve: EdCurve) -> Result<PKey<Public>> {
    match curve {
      EdCurve::Ed25519 => self
        .downcast_ref::<Ed25519SecretKey>(curve.name())
        .map(|key| PKey::from_key(key.public_key())),
      EdCurve::Ed448 => todo!("Secret::ed_public_key(Ed448)"),
    }
  }

  pub fn ecx_public_key(&self, curve: EcxCurve) -> Result<PKey<Public>> {
    match curve {
      EcxCurve::X25519 => self
        .downcast_ref::<X25519SecretKey>(curve.name())
        .map(|key| PKey::from_key(X25519PublicKey::from(key))),
      EcxCurve::X448 => todo!("Secret::ecx_public_key(X448)"),
    }
  }

  pub fn rsa_public_key(&self) -> Result<PKey<Public>> {
    self
      .downcast_ref::<RsaPrivateKey>("Rsa")
      .map(|key| PKey::from_key(key.to_public_key()))
  }

  pub fn try_ec_sign(&self, curve: EcCurve, message: &[u8]) -> Result<Vec<u8>> {
    match curve {
      EcCurve::P256 => {
        use p256::ecdsa::signature::Signer as _;

        self
          .downcast_ref::<P256SecretKey>(curve.name())?
          .try_sign(message)
          .map_err(|_| Error::SigError(curve.name()))
          .map(|signature: P256Signature| signature.as_ref().to_vec())
      }
      EcCurve::P384 => todo!("Secret::try_ec_sign(P384)"),
      EcCurve::P521 => todo!("Secret::try_ec_sign(P521)"),
      EcCurve::Secp256K1 => {
        use k256::ecdsa::signature::Signer as _;

        self
          .downcast_ref::<K256SecretKey>(curve.name())?
          .try_sign(message)
          .map_err(|_| Error::SigError(curve.name()))
          .map(|signature: K256Signature| signature.as_ref().to_vec())
      }
    }
  }

  pub fn try_ed_sign(&self, curve: EdCurve, message: &[u8]) -> Result<Vec<u8>> {
    match curve {
      EdCurve::Ed25519 => self
        .downcast_ref::<Ed25519SecretKey>(curve.name())
        .map(|key| key.sign(message).to_bytes().to_vec()),
      EdCurve::Ed448 => todo!("Secret::try_ed_sign(Ed448)"),
    }
  }

  pub fn try_rsa_sign(&self, algorithm: RsaAlgorithm, message: &[u8]) -> Result<Vec<u8>> {
    let key: &RsaPrivateKey = self.downcast_ref(algorithm.name())?;

    let out: Result<Vec<u8>, _> = match algorithm {
      RsaAlgorithm::RS256 => key.sign(rsa_padding!(@PKCS1_SHA256), &SHA_256!(message)),
      RsaAlgorithm::RS384 => key.sign(rsa_padding!(@PKCS1_SHA384), &SHA_384!(message)),
      RsaAlgorithm::RS512 => key.sign(rsa_padding!(@PKCS1_SHA512), &SHA_512!(message)),
      RsaAlgorithm::PS256 => key.sign(rsa_padding!(@PSS_SHA256), &SHA_256!(message)),
      RsaAlgorithm::PS384 => key.sign(rsa_padding!(@PSS_SHA384), &SHA_384!(message)),
      RsaAlgorithm::PS512 => key.sign(rsa_padding!(@PSS_SHA512), &SHA_512!(message)),
    };

    out.map_err(|_| Error::SigError(algorithm.name()))
  }

  pub fn ec_diffie_hellman(&self, curve: EcCurve, _public: &PKey<Public>) -> Result<Vec<u8>> {
    match curve {
      EcCurve::P256 => todo!("Secret::ec_diffie_hellman(P256)"),
      EcCurve::P384 => todo!("Secret::ec_diffie_hellman(P384)"),
      EcCurve::P521 => todo!("Secret::ec_diffie_hellman(P521)"),
      EcCurve::Secp256K1 => todo!("Secret::ec_diffie_hellman(Secp256K1)"),
    }
  }

  pub fn ecx_diffie_hellman(&self, curve: EcxCurve, public: &PKey<Public>) -> Result<Vec<u8>> {
    match curve {
      EcxCurve::X25519 => {
        let public: X25519PublicKey = public
          .to_ecx_bytes(curve)?
          .try_into()
          .map_err(|_| Error::KeyError(curve.name()))
          .map(|bytes: [u8; X25519_PUBLIC_LEN]| bytes.into())?;

        let secret: X25519SecretKey = self
          .to_ecx_bytes(curve)?
          .try_into()
          .map_err(|_| Error::KeyError(curve.name()))
          .map(|bytes: [u8; X25519_SECRET_LEN]| bytes.into())?;

        Ok(secret.diffie_hellman(&public).as_bytes().to_vec())
      }
      EcxCurve::X448 => {
        todo!("Secret::ecx_diffie_hellman(X448)")
      }
    }
  }
}

// =============================================================================
// Key Ext Trait
// =============================================================================

pub trait PKeyExt: Sized {
  fn from_ec_bytes(curve: EcCurve, data: impl AsRef<[u8]>) -> Result<Self>;

  fn from_ed_bytes(curve: EdCurve, data: impl AsRef<[u8]>) -> Result<Self>;

  fn from_ecx_bytes(curve: EcxCurve, data: impl AsRef<[u8]>) -> Result<Self>;

  fn from_rsa_pkcs1(data: impl AsRef<[u8]>) -> Result<Self>;

  fn from_rsa_pkcs8(data: impl AsRef<[u8]>) -> Result<Self>;

  #[cfg(feature = "pem")]
  fn from_rsa_pem(data: impl AsRef<[u8]>) -> Result<Self>;

  fn to_ec_bytes(&self, curve: EcCurve) -> Result<Vec<u8>>;

  fn to_ed_bytes(&self, curve: EdCurve) -> Result<&[u8]>;

  fn to_ecx_bytes(&self, curve: EcxCurve) -> Result<Vec<u8>>;

  fn derive_ecx(&self, curve: EdCurve) -> Result<Self>;
}

// =============================================================================
// Public Key Ext
// =============================================================================

impl PKeyExt for PKey<Public> {
  fn from_ec_bytes(curve: EcCurve, data: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      EcCurve::P256 => P256PublicKey::from_sec1_bytes(data.as_ref())
        .map_err(|_| Error::KeyError(curve.name()))
        .map(Self::from_key),
      EcCurve::P384 => todo!("Public::from_ec_bytes(P384)"),
      EcCurve::P521 => todo!("Public::from_ec_bytes(P521)"),
      EcCurve::Secp256K1 => K256PublicKey::from_sec1_bytes(data.as_ref())
        .map_err(|_| Error::KeyError(curve.name()))
        .map(Self::from_key),
    }
  }

  fn from_ed_bytes(curve: EdCurve, data: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      EdCurve::Ed25519 => Ed25519PublicKey::try_from(data.as_ref())
        .map_err(Into::into)
        .map(Self::from_key),
      EdCurve::Ed448 => todo!("Public::from_ed_bytes(Ed448)"),
    }
  }

  fn from_ecx_bytes(curve: EcxCurve, data: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      EcxCurve::X25519 => data
        .as_ref()
        .try_into()
        .map_err(|_| Error::KeyError(curve.name()))
        .map(|key: [u8; X25519_PUBLIC_LEN]| Self::from_key(X25519PublicKey::from(key))),
      EcxCurve::X448 => todo!("Public::from_ecx_bytes(X448)"),
    }
  }

  fn from_rsa_pkcs1(data: impl AsRef<[u8]>) -> Result<Self> {
    RsaPublicKey::from_pkcs1(data.as_ref())
      .map_err(|_| Error::KeyError("Rsa"))
      .map(Self::from_key)
  }

  fn from_rsa_pkcs8(data: impl AsRef<[u8]>) -> Result<Self> {
    RsaPublicKey::from_pkcs8(data.as_ref())
      .map_err(|_| Error::KeyError("Rsa"))
      .map(Self::from_key)
  }

  #[cfg(feature = "pem")]
  #[allow(clippy::redundant_closure)]
  fn from_rsa_pem(data: impl AsRef<[u8]>) -> Result<Self> {
    rsa::pem::parse(&data)
      .map_err(|_| Error::KeyError("Rsa"))
      .and_then(|pem| pem.try_into().map_err(|_| Error::KeyError("Rsa")))
      .map(|key: RsaPublicKey| Self::from_key(key))
  }

  fn to_ec_bytes(&self, curve: EcCurve) -> Result<Vec<u8>> {
    match curve {
      EcCurve::P256 => todo!("Public::to_ec_bytes(P256)"),
      EcCurve::P384 => todo!("Public::to_ec_bytes(P384)"),
      EcCurve::P521 => todo!("Public::to_ec_bytes(P521)"),
      EcCurve::Secp256K1 => todo!("Public::to_ec_bytes(Secp256K1)"),
    }
  }

  fn to_ed_bytes(&self, curve: EdCurve) -> Result<&[u8]> {
    match curve {
      EdCurve::Ed25519 => self
        .downcast_ref::<Ed25519PublicKey>(curve.name())
        .map(|key| key.as_ref()),
      EdCurve::Ed448 => todo!("Public::to_ed_bytes(Ed448)"),
    }
  }

  fn to_ecx_bytes(&self, curve: EcxCurve) -> Result<Vec<u8>> {
    match curve {
      EcxCurve::X25519 => self
        .downcast_ref::<X25519PublicKey>(curve.name())
        .map(|key| key.as_bytes().to_vec()),
      EcxCurve::X448 => todo!("Public::to_ecx_bytes(X448)"),
    }
  }

  #[allow(clippy::redundant_closure)]
  fn derive_ecx(&self, curve: EdCurve) -> Result<Self> {
    match curve {
      EdCurve::Ed25519 => edwards::CompressedEdwardsY::from_slice(self.to_ed_bytes(curve)?)
        .decompress()
        .ok_or_else(|| Error::KeyError(curve.name()))
        .map(|edwards| edwards.to_montgomery())
        .map(|montgomery| montgomery.to_bytes().into())
        .map(|key: X25519PublicKey| Self::from_key(key)),
      EdCurve::Ed448 => todo!("Public::derive_ecx(Ed448)"),
    }
  }
}

// =============================================================================
// Secret Key Ext
// =============================================================================

impl PKeyExt for PKey<Secret> {
  fn from_ec_bytes(curve: EcCurve, data: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      EcCurve::P256 => P256SecretKey::from_bytes(data.as_ref())
        .map_err(|_| Error::KeyError(curve.name()))
        .map(Self::from_key),
      EcCurve::P384 => todo!("Secret::from_ec_bytes(P384)"),
      EcCurve::P521 => todo!("Secret::from_ec_bytes(P521)"),
      EcCurve::Secp256K1 => K256SecretKey::from_bytes(data.as_ref())
        .map_err(|_| Error::KeyError(curve.name()))
        .map(Self::from_key),
    }
  }

  fn from_ed_bytes(curve: EdCurve, data: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      EdCurve::Ed25519 => Ed25519SecretKey::try_from(data.as_ref())
        .map_err(Into::into)
        .map(Self::from_key),
      EdCurve::Ed448 => todo!("Secret::from_ed_bytes(Ed448)"),
    }
  }

  #[allow(clippy::redundant_closure)]
  fn from_ecx_bytes(curve: EcxCurve, data: impl AsRef<[u8]>) -> Result<Self> {
    match curve {
      EcxCurve::X25519 => data
        .as_ref()
        .try_into()
        .map_err(|_| Error::KeyError(curve.name()))
        .map(|key: [u8; X25519_SECRET_LEN]| X25519SecretKey::from(key))
        .map(Self::from_key),
      EcxCurve::X448 => todo!("Secret::from_ecx_bytes(X448)"),
    }
  }

  fn from_rsa_pkcs1(data: impl AsRef<[u8]>) -> Result<Self> {
    RsaPrivateKey::from_pkcs1(data.as_ref())
      .map_err(|_| Error::KeyError("Rsa"))
      .map(Self::from_key)
  }

  fn from_rsa_pkcs8(data: impl AsRef<[u8]>) -> Result<Self> {
    RsaPrivateKey::from_pkcs8(data.as_ref())
      .map_err(|_| Error::KeyError("Rsa"))
      .map(Self::from_key)
  }

  #[cfg(feature = "pem")]
  #[allow(clippy::redundant_closure)]
  fn from_rsa_pem(data: impl AsRef<[u8]>) -> Result<Self> {
    rsa::pem::parse(&data)
      .map_err(|_| Error::KeyError("Rsa"))
      .and_then(|pem| pem.try_into().map_err(|_| Error::KeyError("Rsa")))
      .map(|key: RsaPrivateKey| Self::from_key(key))
  }

  fn to_ec_bytes(&self, curve: EcCurve) -> Result<Vec<u8>> {
    match curve {
      EcCurve::P256 => self
        .downcast_ref::<P256SecretKey>(curve.name())
        .map(|key| key.to_bytes().to_vec()),
      EcCurve::P384 => todo!("Secret::to_ec_bytes(P384)"),
      EcCurve::P521 => todo!("Secret::to_ec_bytes(P521)"),
      EcCurve::Secp256K1 => self
        .downcast_ref::<K256SecretKey>(curve.name())
        .map(|key| key.to_bytes().to_vec()),
    }
  }

  fn to_ed_bytes(&self, curve: EdCurve) -> Result<&[u8]> {
    match curve {
      EdCurve::Ed25519 => self
        .downcast_ref::<Ed25519SecretKey>(curve.name())
        .map(|key| key.as_ref()),
      EdCurve::Ed448 => todo!("Secret::to_ed_bytes(Ed448)"),
    }
  }

  fn to_ecx_bytes(&self, curve: EcxCurve) -> Result<Vec<u8>> {
    match curve {
      EcxCurve::X25519 => self
        .downcast_ref::<X25519SecretKey>(curve.name())
        .map(|key| key.to_bytes().to_vec()),
      EcxCurve::X448 => todo!("Secret::to_ecx_bytes(X448)"),
    }
  }

  fn derive_ecx(&self, curve: EdCurve) -> Result<Self> {
    match curve {
      EdCurve::Ed25519 => {
        let bytes: &[u8] = self.to_ed_bytes(curve)?;
        assert!(bytes.len() >= ED25519_SECRET_LEN);
        let slice: &[u8] = &bytes[..ED25519_SECRET_LEN];

        let mut x25519: [u8; X25519_SECRET_LEN] = [0; X25519_SECRET_LEN];

        x25519.copy_from_slice(&SHA_512!(slice)[..ED25519_SECRET_LEN]);
        x25519[0] &= 248;
        x25519[31] &= 127;
        x25519[31] |= 64;

        Ok(Self::from_key(X25519SecretKey::from(x25519)))
      }
      EdCurve::Ed448 => todo!("Secret::derive_ecx(Ed448)"),
    }
  }
}
