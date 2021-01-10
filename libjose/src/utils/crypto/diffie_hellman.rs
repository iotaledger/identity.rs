use core::convert::TryInto as _;
use k256::Secp256k1;
use p256::elliptic_curve::ecdh;
use p256::elliptic_curve::sec1::ToEncodedPoint as _;
use p256::elliptic_curve::PublicKey;
use p256::elliptic_curve::SecretKey;
use p256::NistP256;

use crate::error::Error;
use crate::error::Result;
use crate::jwk::EcCurve;
use crate::jwk::EcdhCurve;
use crate::jwk::EcxCurve;
use crate::lib::*;
use crate::utils::Secret;

pub fn diffie_hellman<'a, 'b>(
  curve: impl Into<EcdhCurve>,
  public: impl Into<Secret<'a>>,
  secret: impl Into<Secret<'b>>,
) -> Result<Vec<u8>> {
  let public: Secret<'a> = public.into();
  let secret: Secret<'b> = secret.into();

  match curve.into() {
    EcdhCurve::Ec(curve) => match curve {
      EcCurve::P256 => {
        let public: PublicKey<NistP256> = public.to_p256_public()?.into();
        let secret: SecretKey<NistP256> = SecretKey::from_bytes(secret.to_p256_secret()?.to_bytes())?;
        let shared: _ = ecdh::diffie_hellman(secret.secret_scalar(), public.as_affine());

        Ok(shared.as_bytes().to_vec())
      }
      EcCurve::P384 => {
        todo!("diffie_hellman(P384)")
      }
      EcCurve::P521 => {
        todo!("diffie_hellman(P521)")
      }
      EcCurve::Secp256K1 => {
        let public: PublicKey<Secp256k1> = public.to_k256_public()?.to_encoded_point(false).try_into()?;
        let secret: SecretKey<Secp256k1> = secret.to_k256_secret()?.into();
        let shared: _ = ecdh::diffie_hellman(secret.secret_scalar(), public.as_affine());

        Ok(shared.as_bytes().to_vec())
      }
    },
    EcdhCurve::Ecx(curve) => match curve {
      EcxCurve::X25519 => {
        let shared: _ = secret.to_x25519_secret()?.diffie_hellman(&public.to_x25519_public()?);

        Ok(shared.as_bytes().to_vec())
      }
      EcxCurve::X448 => secret
        .to_x448_secret()?
        .to_diffie_hellman(&public.to_x448_public()?)
        .ok_or_else(|| Error::KeyError(curve.name()))
        .map(|shared| shared.as_bytes().to_vec()),
    },
  }
}
