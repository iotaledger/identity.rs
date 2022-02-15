// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::iter::once;

use serde::Deserialize;
use serde::Serialize;

use identity_core::common::BitSet;
use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::convert::FmtJson;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;

use crate::did::CoreDID;
use crate::did::DIDUrl;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::utils::check_fragment_non_empty;
use crate::verifiable::Revocation;
use crate::verification::MethodBuilder;
use crate::verification::MethodData;
use crate::verification::MethodRef;
use crate::verification::MethodType;

/// A DID Document Verification Method.
///
/// [Specification](https://www.w3.org/TR/did-core/#verification-method-properties)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerificationMethod<D = CoreDID, T = Object>
where
  D: DID,
{
  #[serde(deserialize_with = "crate::utils::deserialize_did_url_with_fragment")]
  pub(crate) id: DIDUrl<D>,
  pub(crate) controller: D,
  #[serde(rename = "type")]
  pub(crate) key_type: MethodType,
  #[serde(flatten)]
  pub(crate) key_data: MethodData,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<D, T> VerificationMethod<D, T>
where
  D: DID,
{
  // ===========================================================================
  // Builder
  // ===========================================================================

  /// Creates a `MethodBuilder` to configure a new `Method`.
  ///
  /// This is the same as `MethodBuilder::new()`.
  pub fn builder(properties: T) -> MethodBuilder<D, T> {
    MethodBuilder::new(properties)
  }

  /// Returns a new `Method` based on the `MethodBuilder` configuration.
  pub fn from_builder(builder: MethodBuilder<D, T>) -> Result<Self> {
    let id: DIDUrl<D> = builder.id.ok_or(Error::BuilderInvalidMethodId)?;
    check_fragment_non_empty(&id)?;

    Ok(VerificationMethod {
      id,
      controller: builder.controller.ok_or(Error::BuilderInvalidMethodController)?,
      key_type: builder.key_type.ok_or(Error::BuilderInvalidMethodType)?,
      key_data: builder.key_data.ok_or(Error::BuilderInvalidMethodData)?,
      properties: builder.properties,
    })
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns a reference to the `VerificationMethod` id.
  pub fn id(&self) -> &DIDUrl<D> {
    &self.id
  }

  /// Sets the `VerificationMethod` id.
  ///
  /// # Errors
  /// [`Error::InvalidMethodFragment`] if there is no fragment on the [`DIDUrl`].
  pub fn set_id(&mut self, id: DIDUrl<D>) -> Result<()> {
    check_fragment_non_empty(&id)?;
    self.id = id;
    Ok(())
  }

  /// Returns a reference to the verification `Method` controller.
  pub fn controller(&self) -> &D {
    &self.controller
  }

  /// Returns a mutable reference to the verification `Method` controller.
  pub fn controller_mut(&mut self) -> &mut D {
    &mut self.controller
  }

  /// Returns a reference to the verification `Method` type.
  pub fn key_type(&self) -> MethodType {
    self.key_type
  }

  /// Returns a mutable reference to the verification `Method` type.
  pub fn key_type_mut(&mut self) -> &mut MethodType {
    &mut self.key_type
  }

  /// Returns a reference to the verification `Method` data.
  pub fn key_data(&self) -> &MethodData {
    &self.key_data
  }

  /// Returns a mutable reference to the verification `Method` data.
  pub fn key_data_mut(&mut self) -> &mut MethodData {
    &mut self.key_data
  }

  /// Returns a reference to the custom verification `Method` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom verification `Method` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  /// Returns the fragment of the `VerificationMethod` id field.
  pub fn try_into_fragment(&self) -> Result<String> {
    self
      .id
      .fragment()
      .ok_or(Error::InvalidMethodFragment)
      .map(|fragment| once('#').chain(fragment.chars()).collect())
  }

  /// Creates a new [`MethodRef`] from `self`.
  pub fn into_ref(self) -> MethodRef<D, T> {
    MethodRef::Embed(self)
  }

  /// Maps `VerificationMethod<D,T>` to `VerificationMethod<C,T>` by applying a function `f` to
  /// the id and controller.
  pub fn map<C, F>(self, mut f: F) -> VerificationMethod<C, T>
  where
    C: DID,
    F: FnMut(D) -> C,
  {
    VerificationMethod {
      id: self.id.map(&mut f),
      controller: f(self.controller),
      key_type: self.key_type,
      key_data: self.key_data,
      properties: self.properties,
    }
  }

  /// Fallible version of [`VerificationMethod::map`].
  pub fn try_map<C, F, E>(self, mut f: F) -> Result<VerificationMethod<C, T>, E>
  where
    C: DID,
    F: FnMut(D) -> Result<C, E>,
  {
    Ok(VerificationMethod {
      id: self.id.try_map(&mut f)?,
      controller: f(self.controller)?,
      key_type: self.key_type,
      key_data: self.key_data,
      properties: self.properties,
    })
  }
}

impl<D, T> VerificationMethod<D, T>
where
  D: DID,
  T: Revocation + Default,
{
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Creates a new [`IotaVerificationMethod`] from the given `did` and `keypair`.
  pub fn new(did: D, key_type: KeyType, public_key: &PublicKey, fragment: &str) -> Result<Self> {
    let method_fragment: String = if !fragment.starts_with('#') {
      format!("#{}", fragment)
    } else {
      fragment.to_owned()
    };
    let id: DIDUrl<D> = did.to_url().join(method_fragment)?;

    let mut builder: MethodBuilder<D, T> = MethodBuilder::default().id(id).controller(did);
    match key_type {
      KeyType::Ed25519 => {
        builder = builder.key_type(MethodType::Ed25519VerificationKey2018);
        builder = builder.key_data(MethodData::new_multibase(public_key));
      }
    }
    builder.build()
  }

  /// Creates a new [`MerkleKeyCollection2021`](MethodType::MerkleKeyCollection2021) method from
  /// the given key collection.
  pub fn new_merkle_key<M>(did: D, keys: &KeyCollection, fragment: &str) -> Result<Self>
  where
    M: MerkleDigest,
  {
    let method_fragment: String = if fragment.starts_with('#') {
      fragment.to_owned()
    } else {
      format!("#{}", fragment)
    };
    let id: DIDUrl<D> = did.to_url().join(method_fragment)?;

    MethodBuilder::default()
      .id(id)
      .controller(did)
      .key_type(MethodType::MerkleKeyCollection2021)
      .key_data(MethodData::new_multibase(&keys.encode_merkle_key::<M>()))
      .build()
      .map_err(Into::into)
  }

  /// Revokes the public key of a Merkle Key Collection at the specified `index`.
  ///
  /// # Errors
  ///
  /// - if this is called on a `VerificationMethod` with a type other than [`MethodType::MerkleKeyCollection2021`].
  /// - if the index exceeds u32.
  pub fn revoke_merkle_key(&mut self, index: usize) -> Result<bool> {
    if self.key_type() != MethodType::MerkleKeyCollection2021 {
      return Err(Error::InvalidMethodRevocation);
    }

    // TODO: map this to a proper error?
    let index: u32 = index.try_into().map_err(|_| Error::InvalidMethodRevocation)?;
    let mut revocation: BitSet = self.revocation()?.unwrap_or_else(BitSet::new);
    let revoked: bool = revocation.insert(index);

    self.set_revocation(Some(revocation))?;

    Ok(revoked)
  }
}

impl<D, T> Display for VerificationMethod<D, T>
where
  D: DID + Serialize,
  T: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl<D, T> AsRef<DIDUrl<D>> for VerificationMethod<D, T>
where
  D: DID,
{
  fn as_ref(&self) -> &DIDUrl<D> {
    self.id()
  }
}

impl<D, T> KeyComparable for VerificationMethod<D, T>
where
  D: DID,
{
  type Key = DIDUrl<D>;

  #[inline]
  fn key(&self) -> &Self::Key {
    self.id()
  }
}
