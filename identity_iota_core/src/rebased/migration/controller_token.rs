// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::id::UID;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::MoveType;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

/// A token that proves ownership over an object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControllerToken {
  /// A Controller Capability.
  Controller(ControllerCap),
  /// A Delegation Token.
  Delegate(DelegationToken),
}

impl ControllerToken {
  /// Returns the ID of this [ControllerToken].
  pub fn id(&self) -> ObjectID {
    match self {
      Self::Controller(controller) => controller.id,
      Self::Delegate(delegate) => delegate.id,
    }
  }

  /// Returns the ID of the object this token controls.
  pub fn controller_of(&self) -> ObjectID {
    match self {
      Self::Controller(controller) => controller.controller_of,
      Self::Delegate(delegate) => delegate.controller_of,
    }
  }

  /// Returns a reference to [ControllerCap], if this token is a [ControllerCap].
  pub fn as_controller(&self) -> Option<&ControllerCap> {
    match self {
      Self::Controller(controller) => Some(controller),
      Self::Delegate(_) => None,
    }
  }

  /// Attepts to return the [ControllerToken::Controller] variant of this [ControllerToken].
  pub fn try_controller(self) -> Option<ControllerCap> {
    match self {
      Self::Controller(controller) => Some(controller),
      Self::Delegate(_) => None,
    }
  }

  /// Returns a reference to [DelegationToken], if this token is a [DelegationToken].
  pub fn as_delegate(&self) -> Option<&DelegationToken> {
    match self {
      Self::Controller(_) => None,
      Self::Delegate(delegate) => Some(delegate),
    }
  }

  /// Attepts to return the [ControllerToken::Delegate] variant of this [ControllerToken].
  pub fn try_delegate(self) -> Option<DelegationToken> {
    match self {
      Self::Controller(_) => None,
      Self::Delegate(delegate) => Some(delegate),
    }
  }

  /// Returns the Move type of this token.
  pub fn move_type(&self, package: ObjectID) -> TypeTag {
    match self {
      Self::Controller(_) => ControllerCap::move_type(package),
      Self::Delegate(_) => DelegationToken::move_type(package),
    }
  }
}

/// An object that authenticates the actor presenting it
/// as a controller of shared object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerCap {
  #[serde(deserialize_with = "deserialize_from_uid")]
  id: ObjectID,
  controller_of: ObjectID,
  can_delegate: bool,
}

fn deserialize_from_uid<'de, D>(deserializer: D) -> Result<ObjectID, D::Error>
where
  D: Deserializer<'de>,
{
  UID::deserialize(deserializer).map(|uid| *uid.object_id())
}

impl MoveType for ControllerCap {
  fn move_type(package: ObjectID) -> TypeTag {
    format!("{package}::controller::ControllerCap")
      .parse()
      .expect("valid Move type")
  }
}

impl ControllerCap {
  /// Returns the ID of this [ControllerCap].
  pub fn id(&self) -> ObjectID {
    self.id
  }

  /// Returns the ID of the object this token controls.
  pub fn controller_of(&self) -> ObjectID {
    self.controller_of
  }

  /// Returns whether this controller is allowed to delegate
  /// its access to the controlled object.
  pub fn can_delegate(&self) -> bool {
    self.can_delegate
  }
}

impl From<ControllerCap> for ControllerToken {
  fn from(cap: ControllerCap) -> Self {
    Self::Controller(cap)
  }
}

/// A token minted by a controller that allows another to act in
/// its stead - with full or reduced permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationToken {
  #[serde(deserialize_with = "deserialize_from_uid")]
  id: ObjectID,
  #[serde(rename = "permissions")]
  _permissions: u32,
  controller: ObjectID,
  controller_of: ObjectID,
}

impl DelegationToken {
  /// Returns the ID of this [DelegationToken].
  pub fn id(&self) -> ObjectID {
    self.id
  }

  /// Returns the ID of the [ControllerCap] that minted
  /// this [DelegationToken].
  pub fn controller(&self) -> ObjectID {
    self.controller
  }

  /// Returns the ID of the object this token controls.
  pub fn controller_of(&self) -> ObjectID {
    self.controller_of
  }
}

impl From<DelegationToken> for ControllerToken {
  fn from(value: DelegationToken) -> Self {
    Self::Delegate(value)
  }
}

impl MoveType for DelegationToken {
  fn move_type(package: ObjectID) -> TypeTag {
    format!("{package}::controller::DelegationToken")
      .parse()
      .expect("valid Move type")
  }
}
