// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::marker::PhantomData;

use crate::rebased::client::IdentityClient;
use crate::rebased::client::IotaKeySignature;
use crate::rebased::iota::move_calls;
use crate::rebased::migration::Proposal;
use crate::rebased::transaction::ProtoTransaction;
use crate::rebased::transaction::Transaction;
use crate::rebased::transaction::TransactionOutput;
use crate::rebased::utils::MoveType;
use crate::rebased::Error;
use async_trait::async_trait;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::rpc_types::IotaTransactionBlockResponse;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::TypeTag;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use super::CreateProposalTx;
use super::ExecuteProposalTx;
use super::OnChainIdentity;
use super::ProposalBuilder;
use super::ProposalT;
use super::UserDrivenTx;

pub(crate) type IntentFn = Box<dyn FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) + Send>;

/// Action used to borrow in transaction [OnChainIdentity]'s assets.
#[derive(Default, Deserialize, Serialize)]
pub struct BorrowAction {
  objects: Vec<ObjectID>,
}

/// A [`BorrowAction`] coupled with a user-provided function to describe how
/// the borrowed assets shall be used.
pub struct BorrowActionWithIntent<F>
where
  F: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>),
{
  action: BorrowAction,
  intent_fn: F,
}

impl MoveType for BorrowAction {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::borrow_proposal::Borrow")).expect("valid move type")
  }
}

impl BorrowAction {
  /// Adds an object to the lists of objects that will be borrowed when executing
  /// this action in a proposal.
  pub fn borrow_object(&mut self, object_id: ObjectID) {
    self.objects.push(object_id);
  }

  /// Adds many objects. See [`BorrowAction::borrow_object`] for more details.
  pub fn borrow_objects<I>(&mut self, objects: I)
  where
    I: IntoIterator<Item = ObjectID>,
  {
    objects.into_iter().for_each(|obj_id| self.borrow_object(obj_id));
  }
}

impl ProposalBuilder<'_, BorrowAction> {
  /// Adds an object to the list of objects that will be borrowed when executing this action.
  pub fn borrow(mut self, object_id: ObjectID) -> Self {
    self.borrow_object(object_id);
    self
  }
  /// Adds many objects. See [`BorrowAction::borrow_object`] for more details.
  pub fn borrow_objects<I>(self, objects: I) -> Self
  where
    I: IntoIterator<Item = ObjectID>,
  {
    objects.into_iter().fold(self, |builder, obj| builder.borrow(obj))
  }
}

#[async_trait]
impl ProposalT for Proposal<BorrowAction> {
  type Action = BorrowAction;
  type Output = ();

  async fn create<'i, S>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<CreateProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;
    let tx = move_calls::identity::propose_borrow(
      identity_ref,
      controller_cap_ref,
      action.objects,
      expiration,
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(CreateProposalTx {
      identity,
      tx,
      // Borrow proposals cannot be chain-executed as they have to be driven.
      chained_execution: false,
      _action: PhantomData,
    })
  }

  async fn into_tx<'i, S>(
    self,
    identity: &'i mut OnChainIdentity,
    _: &IdentityClient<S>,
  ) -> Result<UserDrivenTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let proposal_id = self.id();
    let borrow_action = self.into_action();

    Ok(UserDrivenTx {
      identity,
      proposal_id,
      action: borrow_action,
    })
  }

  fn parse_tx_effects(_tx_response: &IotaTransactionBlockResponse) -> Result<Self::Output, Error> {
    Ok(())
  }
}

impl<'i> UserDrivenTx<'i, BorrowAction> {
  /// Defines how the borrowed assets should be used.
  pub fn with_intent<F>(self, intent_fn: F) -> UserDrivenTx<'i, BorrowActionWithIntent<F>>
  where
    F: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>),
  {
    let UserDrivenTx {
      identity,
      action,
      proposal_id,
    } = self;
    UserDrivenTx {
      identity,
      proposal_id,
      action: BorrowActionWithIntent { action, intent_fn },
    }
  }
}

impl<'i> ProtoTransaction for UserDrivenTx<'i, BorrowAction> {
  type Input = IntentFn;
  type Tx = UserDrivenTx<'i, BorrowActionWithIntent<IntentFn>>;

  fn with(self, input: Self::Input) -> Self::Tx {
    self.with_intent(input)
  }
}

#[async_trait]
impl<F> Transaction for UserDrivenTx<'_, BorrowActionWithIntent<F>>
where
  F: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) + Send,
{
  type Output = ();
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutput<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let Self {
      identity,
      action: borrow_action,
      proposal_id,
    } = self;
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;

    // Construct a list of `(ObjectRef, TypeTag)` from the list of objects to send.
    let object_data_list = {
      let mut object_data_list = vec![];
      for obj_id in borrow_action.action.objects {
        let object_data = super::obj_data_for_id(client, obj_id)
          .await
          .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
        object_data_list.push(object_data);
      }
      object_data_list
    };

    let tx = move_calls::identity::execute_borrow(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      object_data_list,
      borrow_action.intent_fn,
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    ExecuteProposalTx {
      identity,
      tx,
      _action: PhantomData::<BorrowAction>,
    }
    .execute_with_opt_gas(gas_budget, client)
    .await
  }
}
