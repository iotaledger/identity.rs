// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use crypto::hashes::sha::Sha512;
use crypto::hashes::Digest;
use crypto::hashes::Output;
use curve25519_dalek::edwards::CompressedEdwardsY;
use zeroize::Zeroize;

use crate::error::Error;
use crate::error::Result;
use crate::utils::ED25519_PUBLIC_KEY_LEN;
use crate::utils::ED25519_SECRET_KEY_LEN;
use crate::utils::X25519_PUBLIC_KEY_LEN;
use crate::utils::X25519_SECRET_KEY_LEN;

pub fn ed25519_to_x25519_public<T>(public: &T) -> Result<[u8; X25519_PUBLIC_KEY_LEN]>
where
  T: AsRef<[u8]> + ?Sized,
{
  let mut ed25519: [u8; ED25519_PUBLIC_KEY_LEN] = public
    .as_ref()
    .try_into()
    .map_err(|_| Error::KeyError("ed25519_to_x25519_public"))?;

  let x25519: [u8; X25519_PUBLIC_KEY_LEN] = CompressedEdwardsY(ed25519)
    .decompress()
    .map(|edwards| edwards.to_montgomery().0)
    .ok_or(Error::KeyError("ed25519_to_x25519_public"))?;

  ed25519.zeroize();

  Ok(x25519)
}

pub fn ed25519_to_x25519_secret<T>(secret: &T) -> Result<[u8; X25519_SECRET_KEY_LEN]>
where
  T: AsRef<[u8]> + ?Sized,
{
  let mut ed25519: [u8; ED25519_SECRET_KEY_LEN] = secret
    .as_ref()
    .try_into()
    .map_err(|_| Error::KeyError("ed25519_to_x25519_secret"))?;

  let mut x25519: [u8; X25519_SECRET_KEY_LEN] = [0; X25519_SECRET_KEY_LEN];
  let hash: Output<Sha512> = Sha512::digest(&ed25519);

  x25519.copy_from_slice(&hash[..X25519_SECRET_KEY_LEN]);
  x25519[0] &= 248;
  x25519[31] &= 127;
  x25519[31] |= 64;

  ed25519.zeroize();

  Ok(x25519)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::Ed25519PublicKey;
  use crate::utils::Ed25519SecretKey;
  use crate::utils::X25519PublicKey;
  use crate::utils::X25519SecretKey;

  #[test]
  fn test_convert_ed25519() {
    let ed25519_sk: Ed25519SecretKey = Ed25519SecretKey::generate().unwrap();
    let ed25519_pk: Ed25519PublicKey = ed25519_sk.public_key();

    let derived_sk = ed25519_to_x25519_secret(&ed25519_sk.to_le_bytes()).unwrap();
    let derived_pk = ed25519_to_x25519_public(ed25519_pk.as_ref()).unwrap();

    let x25519_sk: X25519SecretKey = X25519SecretKey::from_bytes(&derived_sk).unwrap();
    let x25519_pk: X25519PublicKey = X25519PublicKey::from_bytes(&derived_pk).unwrap();

    assert_eq!(derived_sk, x25519_sk.to_bytes());
    assert_eq!(derived_pk, x25519_pk.to_bytes());
  }
}
