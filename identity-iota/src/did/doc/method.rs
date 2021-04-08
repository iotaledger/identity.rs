// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use identity_core::common::BitSet;
use identity_core::convert::ToJson;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::KeyCollection;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_did::error::Result as DIDResult;
use identity_did::verifiable::Revocation;
use identity_did::verification::Method as CoreMethod;
use identity_did::verification::MethodBuilder;
use identity_did::verification::MethodData;
use identity_did::verification::MethodRef;
use identity_did::verification::MethodType;

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(into = "CoreMethod", try_from = "CoreMethod")]
pub struct Method(CoreMethod);

impl Method {
  /// The default verification method tag.
  pub const TAG: &'static str = "key";

  /// Creates a new Merkle Key Collection Method from the given key collection.
  pub fn create_merkle_key<'a, D, F>(did: DID, keys: &KeyCollection, fragment: F) -> Result<Self>
  where
    F: Into<Option<&'a str>>,
    D: MerkleDigest,
  {
    let tag: String = format!("#{}", fragment.into().unwrap_or(Self::TAG));
    let key: DID = did.join(tag)?;

    MethodBuilder::default()
      .id(key.into())
      .controller(did.into())
      .key_type(MethodType::MerkleKeyCollection2021)
      .key_data(MethodData::new_b58(&keys.encode_merkle_key::<D>()))
      .build()
      .map_err(Into::into)
      .map(Self)
  }

  /// Creates a new [`Method`] object from the given `keypair`.
  pub fn from_keypair<'a, F>(keypair: &KeyPair, fragment: F) -> Result<Self>
  where
    F: Into<Option<&'a str>>,
  {
    let key: &[u8] = keypair.public().as_ref();
    let did: DID = DID::new(key)?;

    Self::from_did(did, keypair, fragment)
  }

  /// Creates a new [`Method`] object from the given `did` and `keypair`.
  pub fn from_did<'a, F>(did: DID, keypair: &KeyPair, fragment: F) -> Result<Self>
  where
    F: Into<Option<&'a str>>,
  {
    let tag: String = format!("#{}", fragment.into().unwrap_or(Self::TAG));
    let key: DID = did.join(tag)?;

    let mut builder: MethodBuilder = MethodBuilder::default().id(key.into()).controller(did.into());

    match keypair.type_() {
      KeyType::Ed25519 => {
        builder = builder.key_type(MethodType::Ed25519VerificationKey2018);
        builder = builder.key_data(MethodData::new_b58(keypair.public()));
      }
    }

    Ok(Self(builder.build()?))
  }

  /// Converts a generic Verification Method to an IOTA Verification Method.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the document is not a valid IOTA Verification Method.
  pub fn try_from_core(method: CoreMethod) -> Result<Self> {
    Self::check_validity(&method)?;

    Ok(Self(method))
  }

  /// Converts a mutable `Method` reference to a mutable  IOTA Verification
  /// Method reference.
  pub fn try_from_mut(method: &mut CoreMethod) -> Result<&mut Self> {
    Self::check_validity(method)?;

    // SAFETY: We just checked the validity of the verification method.
    Ok(unsafe { &mut *(method as *mut CoreMethod as *mut Method) })
  }

  /// Converts a `Method` reference to an IOTA Verification Method reference
  /// without performing validation checks.
  ///
  /// # Safety
  ///
  /// This must be guaranteed safe by the caller.
  pub unsafe fn new_unchecked_ref(method: &CoreMethod) -> &Self {
    // SAFETY: This is guaranteed safe by the caller.
    &*(method as *const CoreMethod as *const Method)
  }

  /// Checks if the given verification method is valid according to the IOTA
  /// DID method specification.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the input is not a valid IOTA verification method.
  pub fn check_validity(method: &CoreMethod) -> Result<()> {
    // Ensure all associated DIDs are IOTA Identity DIDs
    DID::check_validity(method.id())?;
    DID::check_validity(method.controller())?;

    // Ensure the authentication method has an identifying fragment
    if method.id().fragment().is_none() {
      return Err(Error::InvalidDocumentAuthFragment);
    }

    // Ensure the id and controller are the same - we don't support DIDs
    // controlled by 3rd parties - yet.
    if method.id().authority() != method.controller().authority() {
      return Err(Error::InvalidDocumentAuthAuthority);
    }

    Ok(())
  }

  /// Returns a `bool` indicating if the given verification method is valid
  /// according to the IOTA DID method specification.
  pub fn is_valid(method: &CoreMethod) -> bool {
    Self::check_validity(method).is_ok()
  }

  /// Returns the method `id` property.
  pub fn id(&self) -> &DID {
    // SAFETY: We don't create methods with invalid DID's
    unsafe { DID::new_unchecked_ref(self.0.id()) }
  }

  /// Returns the method `controller` property.
  pub fn controller(&self) -> &DID {
    // SAFETY: We don't create methods with invalid DID's
    unsafe { DID::new_unchecked_ref(self.0.controller()) }
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

impl Display for Method {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl Debug for Method {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.0, f)
  }
}

impl Deref for Method {
  type Target = CoreMethod;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<Method> for CoreMethod {
  fn from(other: Method) -> Self {
    other.0
  }
}

impl From<Method> for MethodRef {
  fn from(other: Method) -> Self {
    other.0.into()
  }
}

impl TryFrom<CoreMethod> for Method {
  type Error = Error;

  fn try_from(other: CoreMethod) -> Result<Self, Self::Error> {
    Self::try_from_core(other)
  }
}

impl Revocation for Method {
  fn revocation(&self) -> DIDResult<Option<BitSet>> {
    self.0.properties().revocation()
  }
}
