// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;

use core::ops::Deref;

use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::common::BitSet;
use identity_core::convert::ToJson;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_did::did::CoreDID;
use identity_did::did::CoreDIDUrl;
use identity_did::did::DID;
use identity_did::error::Result as DIDResult;
use identity_did::verifiable::Revocation;
use identity_did::verification::MethodBuilder;
use identity_did::verification::MethodData;
use identity_did::verification::MethodRef;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;

use crate::did::IotaDID;
use crate::did::IotaDIDUrl;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::NetworkName;

/// A DID Document verification method
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(into = "VerificationMethod", try_from = "VerificationMethod")]
pub struct IotaVerificationMethod(VerificationMethod);

impl IotaVerificationMethod {
  /// Creates a new Merkle Key Collection Method from the given key collection.
  pub fn create_merkle_key<D>(did: IotaDID, keys: &KeyCollection, fragment: &str) -> Result<Self>
  where
    D: MerkleDigest,
  {
    let tag: String = format!("#{}", fragment);
    let key: IotaDIDUrl = did.to_url().join(tag)?;

    MethodBuilder::default()
      .id(CoreDIDUrl::from(key))
      .controller(did.into())
      .key_type(MethodType::MerkleKeyCollection2021)
      .key_data(MethodData::new_multibase(&keys.encode_merkle_key::<D>()))
      .build()
      .map_err(Into::into)
      .map(Self)
  }

  /// Creates a new [`IotaVerificationMethod`] from the given `keypair`.
  ///
  /// WARNING: this derives a new DID from the keypair, which will not match the DID of a document
  /// created with a different keypair. Use [`IotaVerificationMethod::from_did`] instead.
  pub fn from_keypair(keypair: &KeyPair, fragment: &str) -> Result<Self> {
    let key: &[u8] = keypair.public().as_ref();
    let did: IotaDID = IotaDID::new(key)?;

    Self::from_did(did, keypair.type_(), keypair.public(), fragment)
  }

  /// Creates a new [`IotaVerificationMethod`] from the given `keypair` on the specified
  /// `network`.
  pub fn from_keypair_with_network(keypair: &KeyPair, fragment: &str, network: NetworkName) -> Result<Self> {
    let key: &[u8] = keypair.public().as_ref();
    let did: IotaDID = IotaDID::new_with_network(key, network)?;

    Self::from_did(did, keypair.type_(), keypair.public(), fragment)
  }

  /// Creates a new [`IotaVerificationMethod`] from the given `did` and `keypair`.
  pub fn from_did(did: IotaDID, key_type: KeyType, public_key: &PublicKey, fragment: &str) -> Result<Self> {
    let tag: String = format!("#{}", fragment);
    let key: IotaDIDUrl = did.to_url().join(tag)?;

    let mut builder: MethodBuilder = MethodBuilder::default()
      .id(CoreDIDUrl::from(key))
      .controller(did.into());

    match key_type {
      KeyType::Ed25519 => {
        builder = builder.key_type(MethodType::Ed25519VerificationKey2018);
        builder = builder.key_data(MethodData::new_multibase(public_key));
      }
    }

    Ok(Self(builder.build()?))
  }

  /// Converts a generic Verification Method to an IOTA Verification Method.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the document is not a valid IOTA Verification Method.
  pub fn try_from_core(method: VerificationMethod) -> Result<Self> {
    Self::check_validity(&method)?;

    Ok(Self(method))
  }

  /// Converts a mutable `Method` reference to a mutable  IOTA Verification
  /// Method reference.
  pub fn try_from_mut(method: &mut VerificationMethod) -> Result<&mut Self> {
    Self::check_validity(method)?;

    // SAFETY: We just checked the validity of the verification method and the layout of
    //         IotaVerificationMethod is transparent.
    Ok(unsafe { &mut *(method as *mut VerificationMethod as *mut IotaVerificationMethod) })
  }

  /// Converts a `Method` reference to an IOTA Verification Method reference
  /// without performing validation checks.
  ///
  /// # Safety
  ///
  /// This must be guaranteed safe by the caller.
  pub unsafe fn new_unchecked_ref(method: &VerificationMethod) -> &Self {
    // SAFETY: This is guaranteed safe by the caller.
    &*(method as *const VerificationMethod as *const IotaVerificationMethod)
  }

  /// Checks if the given verification method is valid according to the IOTA
  /// DID method specification.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid IOTA verification method.
  pub fn check_validity<T>(method: &VerificationMethod<T>) -> Result<()> {
    // Ensure all associated DIDs are valid IotaDIDs.
    IotaDID::check_validity(method.id().did())?;
    IotaDID::check_validity(method.controller())?;

    // Ensure the verification method has an identifying fragment.
    if method.id().fragment().is_none() || method.id().fragment().unwrap_or_default().is_empty() {
      return Err(Error::InvalidMethodMissingFragment);
    }

    Ok(())
  }

  /// Returns a `bool` indicating if the given verification method is valid
  /// according to the IOTA DID method specification.
  pub fn is_valid(method: &VerificationMethod) -> bool {
    Self::check_validity(method).is_ok()
  }

  /// Returns the method `id` property.
  ///
  /// NOTE: clones the [`DIDUrl`].
  pub fn id(&self) -> IotaDIDUrl {
    // We ensure the validity of the id on creation.
    let did_url: &CoreDIDUrl = self.0.id();
    IotaDIDUrl::try_from(did_url.clone()).expect("invalid IotaDIDUrl")

    // TODO: unable to guarantee the safety of this cast due to layout of generic DIDUrl<T>
    //       possibly differing from DIDUrl<U> even if U is a transparent wrapper of T
    //      (even though it _seems_ fine)
    // unsafe { &*(did_url as *const CoreDIDUrl as *const IotaDIDUrl) }
  }

  /// Returns a reference to the underlying method `id` property.
  pub fn id_core(&self) -> &CoreDIDUrl {
    self.0.id()
  }

  /// Returns the method `controller` property.
  pub fn controller(&self) -> &IotaDID {
    // SAFETY: We don't create methods with invalid DIDs and the layout of IotaDID is transparent.
    unsafe { IotaDID::new_unchecked_ref(self.0.controller()) }
  }

  /// Sets the method `controller` property.
  pub fn set_controller(&mut self, did: IotaDID) {
    *self.0.controller_mut() = CoreDID::from(did);
  }

  /// Revokes the public key of a Merkle Key Collection at the specified `index`.
  pub fn revoke_merkle_key(&mut self, index: usize) -> Result<bool> {
    if !matches!(self.key_type(), MethodType::MerkleKeyCollection2021) {
      return Err(Error::CannotRevokeMethod);
    }

    let mut revocation: BitSet = self.revocation()?.unwrap_or_else(BitSet::new);
    let index: u32 = index.try_into().map_err(|_| Error::CannotRevokeMethod)?;
    let revoked: bool = revocation.insert(index);

    self
      .0
      .properties_mut()
      .insert("revocation".into(), revocation.to_json_value()?);

    Ok(revoked)
  }
}

impl Display for IotaVerificationMethod {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    Display::fmt(&self.0, f)
  }
}

impl Debug for IotaVerificationMethod {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    Debug::fmt(&self.0, f)
  }
}

impl Deref for IotaVerificationMethod {
  type Target = VerificationMethod;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<IotaVerificationMethod> for VerificationMethod {
  fn from(other: IotaVerificationMethod) -> Self {
    other.0
  }
}

impl From<IotaVerificationMethod> for MethodRef {
  fn from(other: IotaVerificationMethod) -> Self {
    other.0.into()
  }
}

impl TryFrom<VerificationMethod> for IotaVerificationMethod {
  type Error = Error;

  fn try_from(other: VerificationMethod) -> Result<Self, Self::Error> {
    Self::try_from_core(other)
  }
}

impl Revocation for IotaVerificationMethod {
  fn revocation(&self) -> DIDResult<Option<BitSet>> {
    self.0.properties().revocation()
  }
}
