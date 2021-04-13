// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
        let public: _ = public.to_p256_public()?;
        let secret: _ = secret.to_p256_secret()?;
        let shared: _ = secret.diffie_hellman(&public);

        Ok(shared.as_bytes().to_vec())
      }
      EcCurve::P384 => Err(Error::AlgError("Diffie-Hellman (P384)")),
      EcCurve::P521 => Err(Error::AlgError("Diffie-Hellman (P521)")),
      EcCurve::Secp256K1 => {
        let public: _ = public.to_k256_public()?;
        let secret: _ = secret.to_k256_secret()?;
        let shared: _ = secret.diffie_hellman(&public);

        Ok(shared.as_bytes().to_vec())
      }
    },
    EcdhCurve::Ecx(curve) => match curve {
      EcxCurve::X25519 => {
        let public: _ = public.to_x25519_public()?;
        let secret: _ = secret.to_x25519_secret()?;
        let shared: _ = secret.diffie_hellman(&public);

        Ok(shared.as_bytes().to_vec())
      }
      EcxCurve::X448 => {
        let public: _ = public.to_x448_public()?;
        let secret: _ = secret.to_x448_secret()?;
        let shared: _ = secret.diffie_hellman(&public);

        Ok(shared.as_bytes().to_vec())
      }
    },
  }
}
