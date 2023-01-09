// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::hashes::blake2b::Blake2b256;
use crypto::hashes::Digest;
use identity_core::common::Object;
use identity_data_integrity::verification_material::Multikey;
use identity_data_integrity::verification_material::VerificationMaterial;
use identity_did::did::DID;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
/// An index used to look up metadata stored in [`IdentityStorage`](crate::identity_storage::IdentityStorage) associated
/// with a [`VerificationMethod`].
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MethodId(Repr);

/// Length necessary for verification methods of type `Multikey`.
///
/// The length corresponds to version_byte + Blake2b256 digest length. Due to limitations of const generics in traits
/// this cannot be expressed more elegantly.
const MULTIKEY_METHOD_IDX_V1_LENGTH: usize = 33;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Repr {
  MultiKeyV1([u8; MULTIKEY_METHOD_IDX_V1_LENGTH]),
}

impl AsRef<[u8]> for MethodId {
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
impl MethodId {
  /// Generate the [`MethodIdx`] corresponding to be used with verification methods of type `Multikey`.
  pub(crate) fn new_from_multikey(fragment: &str, multikey: &Multikey) -> Self {
    let hasher = Blake2b256::new();
    let output = hasher
      .chain_update(fragment.as_bytes())
      .chain_update(multikey.as_multibase_str().as_bytes())
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

impl<D: DID, U> TryFrom<&VerificationMethod<D, U>> for MethodId {
  type Error = ();

  fn try_from(method: &VerificationMethod<D, U>) -> Result<Self, Self::Error> {
    // TODO: match doesn't work here, why?
    if method.type_() == &MethodType::MULTIKEY {
      let material = method.material().expect("TODO");

      // TODO: Get rid of clone by changing how `MethodId`s can be constructed.
      let multikey = if let VerificationMaterial::PublicKeyMultibase(multibase_str) = material {
        Multikey::from_multibase_string(multibase_str.as_str().to_owned())
      } else {
        todo!();
      };

      Ok(MethodId::new_from_multikey(
        method.id().fragment().expect("TODO"),
        &multikey,
      ))
    } else {
      todo!("unsupported method type")
    }
  }
}
