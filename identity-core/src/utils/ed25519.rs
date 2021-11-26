// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;

use crate::crypto::PrivateKey;
use crate::crypto::PublicKey;

/// Generates a new pair of public/private ed25519 keys.
///
/// The returned [`PrivateKey`] instance represents what the paper [High-speed high-security signatures](http://ed25519.cr.yp.to/ed25519-20110926.pdf)
/// calls an *EdDSA secret key*. The following [blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) gives a nice overview of
/// how Ed25519 keys work and the different conventions for public/private key formats used in various implementations.
/// What the original paper calls an *EdDSA secret key* is called a *seed* in the aforementioned blog post.  
pub fn generate_ed25519_keypair() -> Result<(PublicKey, PrivateKey), Ed25519KeyPairGenerationError> {
  let secret: ed25519::SecretKey =
    ed25519::SecretKey::generate().map_err(|inner| Ed25519KeyPairGenerationError { inner })?;
  let public: ed25519::PublicKey = secret.public_key();

  let private: PrivateKey = secret.to_bytes().to_vec().into();
  let public: PublicKey = public.to_bytes().to_vec().into();

  Ok((public, private))
}

/// Reconstructs the ed25519 public key given the private key.
pub(crate) fn keypair_from_ed25519_private_key(private_key: ed25519::SecretKey) -> (PublicKey, PrivateKey) {
  let public: ed25519::PublicKey = private_key.public_key();

  let private: PrivateKey = private_key.to_bytes().to_vec().into();
  let public: PublicKey = public.to_bytes().to_vec().into();

  (public, private)
}

/// Generates a list of public/private ed25519 keys.
/// The [`PrivateKey`] instances represent what the paper [High-speed high-security signatures](http://ed25519.cr.yp.to/ed25519-20110926.pdf)
/// calls *EdDSA secret keys*. The following [blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) gives a nice overview of
/// how Ed25519 keys work and the different conventions for public/private key formats used in various implementations.
/// What the original paper calls an *EdDSA secret key* is called a *seed* in the aforementioned blog post.  
pub fn generate_ed25519_keypairs(count: usize) -> Result<Vec<(PublicKey, PrivateKey)>, Ed25519KeyPairGenerationError> {
  (0..count).map(|_| generate_ed25519_keypair()).collect()
}

/// Caused by a failure to generate an ED25519 Keypair
#[derive(Debug, thiserror::Error)]
#[error("failed to generate a ed25519 key-pair: {inner}")]
pub struct Ed25519KeyPairGenerationError {
  #[source]
  pub(crate) inner: crypto::Error,
}
