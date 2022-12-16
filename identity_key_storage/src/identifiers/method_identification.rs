// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use identity_data_integrity_types::verification_material::PublicKeyMultibase;
/// An index used to look up metadata stored in [`IdentityStorage`](crate::identity_storage::IdentityStorage) associated
/// with a [`VerificationMethod`].
pub struct MethodIdx(Repr);

/// Length necessary for verification methods of type `Multikey`.
///
/// The length corresponds to version_byte + Blake2b256 digest length. Due to limitations of const generics in traits
/// this cannot be expressed more elegantly.
const MULTIKEY_METHOD_IDX_V1_LENGTH: usize = 33;
enum Repr {
  MultiKeyV1([u8; MULTIKEY_METHOD_IDX_V1_LENGTH]),
}

impl AsRef<[u8]> for MethodIdx {
  fn as_ref(&self) -> &[u8] {
    let Repr::MultiKeyV1(ref bytes) = self.0;
    bytes.as_slice()
  }
}

#[derive(Clone, Copy)]
/// For now this is not necessary, but may become useful if we want to switch out the hashing function at some point.
/// If this becomes necessary at some point it will have to be made public, with an option to set it in [`Storage`].
#[repr(u8)]
enum MethodIdxVersion {
  One = 1,
}

impl MethodIdxVersion {
  const CURRENT: Self = Self::One;
}
impl MethodIdx {
  /// Generate the [`MethodIdx`] corresponding to be used with verification methods of type `Multikey`.
  pub(crate) fn new_from_multikey(fragment: &str, material: &PublicKeyMultibase) -> Self {
    let mut hasher = Blake2b256::new();
    let output = hasher
      .chain_update(fragment.as_bytes())
      .chain_update(material.as_bytes())
      .finalize();
    let arr: [u8; MULTIKEY_METHOD_IDX_V1_LENGTH - 1] = output.into();
    let mut repr_inner = [0_u8; MULTIKEY_METHOD_IDX_V1_LENGTH];
    repr_inner[1..].copy_from_slice(&arr);
    repr_inner[0] = MethodIdxVersion::CURRENT as u8;
    Self(Repr::MultiKeyV1(repr_inner))
  }

  // TODO: Would it be useful for implementers to know some more representation details,
  // i.e. that the length of self.as_ref() is 33?
  // TODO: Do we need some public constructor available under cfg(test) for implementers?
  // Going by the "only test public methods" sentiment we may not need a public constructor
  // As one should then test against `CoreDocumentExt::create_multikey`.
}
