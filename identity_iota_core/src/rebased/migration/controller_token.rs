// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Not;

use async_trait::async_trait;
use identity_iota_interaction::rpc_types::IotaExecutionStatus;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffects;
use identity_iota_interaction::rpc_types::IotaTransactionBlockEffectsAPI as _;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::id::UID;
use identity_iota_interaction::types::object::Owner;
use identity_iota_interaction::types::transaction::ProgrammableTransaction;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::ControllerTokenRef;
use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::IotaTransactionBlockEffectsMutAPI as _;
use identity_iota_interaction::MoveType;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use crate::iota_interaction_rust::IdentityMoveCallsAdapter;
use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::transaction_builder::Transaction;
use crate::rebased::transaction_builder::TransactionBuilder;
use crate::rebased::Error;

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

  /// Returns the ID of the this token's controller.
  /// For [ControllerToken::Controller] this is the same as its ID, but
  /// for [ControllerToken::Delegate] this is [DelegationToken::controller].
  pub fn controller_id(&self) -> ObjectID {
    match self {
      Self::Controller(controller) => controller.id,
      Self::Delegate(delegate) => delegate.controller,
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

  pub(crate) async fn controller_ref(&self, client: &IdentityClientReadOnly) -> Result<ControllerTokenRef, Error> {
    let obj_ref = client
      .get_object_ref_by_id(self.id())
      .await?
      .expect("token exists on-chain")
      .reference
      .to_object_ref();

    Ok(match self {
      Self::Controller(_) => ControllerTokenRef::Controller(obj_ref),
      Self::Delegate(_) => ControllerTokenRef::Delegate(obj_ref),
    })
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

  /// If this token can be delegated, this function will return
  /// a [DelegateTransaction] that will mint a new [DelegationToken]
  /// and send it to `recipient`.
  pub fn delegate(
    &self,
    recipient: IotaAddress,
    permissions: Option<DelegatePermissions>,
  ) -> Option<TransactionBuilder<DelegateToken>> {
    if !self.can_delegate {
      return None;
    }

    let tx = {
      let permissions = permissions.unwrap_or_default();
      DelegateToken::new_with_permissions(self, recipient, permissions)
    };

    Some(TransactionBuilder::new(tx))
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
  permissions: DelegatePermissions,
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

/// Permissions of a [DelegationToken].
///
/// Permissions can be operated on as if they were bit vectors:
/// ```
/// use identity_iota_core::rebased::migration::DelegatePermissions;
///
/// let permissions = DelegatePermissions::CREATE_PROPOSAL | DelegatePermissions::APPROVE_PROPOSAL;
/// assert!(permissions & DelegatePermissions::DELETE_PROPOSAL == DelegatePermissions::NONE);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct DelegatePermissions(u32);

impl Default for DelegatePermissions {
  fn default() -> Self {
    Self(u32::MAX)
  }
}

impl DelegatePermissions {
  /// No permissions.
  pub const NONE: Self = Self(0);
  /// Permission that enables the creation of new proposals.
  pub const CREATE_PROPOSAL: Self = Self(1);
  /// Permission that enables the approval of existing proposals.
  pub const APPROVE_PROPOSAL: Self = Self(1 << 1);
  /// Permission that enables the execution of existing proposals.
  pub const EXECUTE_PROPOSAL: Self = Self(1 << 2);
  /// Permission that enables the deletion of existing proposals.
  pub const DELETE_PROPOSAL: Self = Self(1 << 3);
  /// Permission that enables the remove of one's approval for an existing proposal.
  pub const REMOVE_APPROVAL: Self = Self(1 << 4);
  /// All permissions.
  pub const ALL: Self = Self(u32::MAX);

  /// Returns whether this set of permissions contains `permission`.
  /// ```
  /// use identity_iota_core::rebased::migration::DelegatePermissions;
  ///
  /// let all_permissions = DelegatePermissions::ALL;
  /// assert_eq!(
  ///   all_permissions.has(DelegatePermissions::CREATE_PROPOSAL),
  ///   true
  /// );
  /// ```
  pub fn has(&self, permission: Self) -> bool {
    *self | permission != Self::NONE
  }
}

impl Not for DelegatePermissions {
  type Output = Self;
  fn not(self) -> Self::Output {
    Self(!self.0)
  }
}
impl BitOr for DelegatePermissions {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self::Output {
    Self(self.0 | rhs.0)
  }
}
impl BitOrAssign for DelegatePermissions {
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 |= rhs.0;
  }
}
impl BitAnd for DelegatePermissions {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self::Output {
    Self(self.0 & rhs.0)
  }
}
impl BitAndAssign for DelegatePermissions {
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 &= rhs.0;
  }
}
impl BitXor for DelegatePermissions {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self::Output {
    Self(self.0 ^ rhs.0)
  }
}
impl BitXorAssign for DelegatePermissions {
  fn bitxor_assign(&mut self, rhs: Self) {
    self.0 ^= rhs.0;
  }
}

/// A [Transaction] that creates a new [DelegationToken]
/// for a given [ControllerCap].
#[derive(Debug, Clone)]
pub struct DelegateToken {
  cap_id: ObjectID,
  permissions: DelegatePermissions,
  recipient: IotaAddress,
}

impl DelegateToken {
  /// Creates a new [DelegateToken] transaction that will create a new [DelegationToken] with all permissions
  /// for `controller_cap` and send it to `recipient`.
  pub fn new(controller_cap: &ControllerCap, recipient: IotaAddress) -> Self {
    Self::new_with_permissions(controller_cap, recipient, DelegatePermissions::default())
  }

  /// Same as [DelegateToken::new] but permissions for the new token can be specified.
  pub fn new_with_permissions(
    controller_cap: &ControllerCap,
    recipient: IotaAddress,
    permissions: DelegatePermissions,
  ) -> Self {
    Self {
      cap_id: controller_cap.id(),
      permissions,
      recipient,
    }
  }
}

#[cfg_attr(feature = "send-sync", async_trait)]
#[cfg_attr(not(feature = "send-sync"), async_trait(?Send))]
impl Transaction for DelegateToken {
  type Output = DelegationToken;

  async fn build_programmable_transaction(
    &self,
    client: &IdentityClientReadOnly,
  ) -> Result<ProgrammableTransaction, Error> {
    let controller_cap_ref = client
      .get_object_ref_by_id(self.cap_id)
      .await?
      .expect("ControllerCap exists on-chain")
      .reference
      .to_object_ref();

    let ptb_bcs = IdentityMoveCallsAdapter::delegate_controller_cap(
      controller_cap_ref,
      self.recipient,
      self.permissions.0,
      client.package_id(),
    )
    .await?;
    Ok(bcs::from_bytes(&ptb_bcs)?)
  }

  async fn apply(
    self,
    mut effects: IotaTransactionBlockEffects,
    client: &IdentityClientReadOnly,
  ) -> (Result<Self::Output, Error>, IotaTransactionBlockEffects) {
    if let IotaExecutionStatus::Failure { error } = effects.status() {
      return (Err(Error::TransactionUnexpectedResponse(error.clone())), effects);
    }

    let created_objects = effects
      .created()
      .iter()
      .enumerate()
      .filter(|(_, elem)| matches!(elem.owner, Owner::AddressOwner(addr) if addr == self.recipient))
      .map(|(i, obj)| (i, obj.object_id()));

    let is_target_token = |delegation_token: &DelegationToken| -> bool {
      delegation_token.controller == self.cap_id && delegation_token.permissions == self.permissions
    };
    let mut target_token_pos = None;
    let mut target_token = None;
    for (i, obj_id) in created_objects {
      match client.get_object_by_id(obj_id).await {
        Ok(token) if is_target_token(&token) => {
          target_token_pos = Some(i);
          target_token = Some(token);
          break;
        }
        _ => continue,
      }
    }

    let (Some(i), Some(token)) = (target_token_pos, target_token) else {
      return (
        Err(Error::TransactionUnexpectedResponse(
          "failed to find the correct identity in this transaction's effects".to_owned(),
        )),
        effects,
      );
    };

    effects.created_mut().swap_remove(i);

    (Ok(token), effects)
  }
}
