// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::marker::PhantomData;

use crate::iota_interaction_adapter::AdapterError;
use crate::iota_interaction_adapter::AdapterNativeResponse;
use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::iota_interaction_adapter::IotaTransactionBlockResponseAdapter;
use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::IotaTransactionBlockResponseT;
use identity_iota_interaction::TransactionBuilderT;

use crate::rebased::client::IdentityClient;
use crate::rebased::migration::Proposal;
use crate::rebased::transaction::ProtoTransaction;
use crate::rebased::transaction::TransactionInternal;
use crate::rebased::transaction::TransactionOutputInternal;
use crate::rebased::Error;
use async_trait::async_trait;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::transaction::Argument;
use identity_iota_interaction::types::TypeTag;
use identity_iota_interaction::MoveType;
use secret_storage::Signer;
use serde::Deserialize;
use serde::Serialize;

use super::CreateProposalTx;
use super::ExecuteProposalTx;
use super::OnChainIdentity;
use super::ProposalBuilder;
use super::ProposalT;
use super::UserDrivenTx;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
      use iota_interaction_ts::NativeTsCodeBindingWrapper as Ptb;
      /// Instances of BorrowIntentFnT can be used as user-provided function to describe how
      /// a borrowed assets shall be used.
      pub trait BorrowIntentFnT: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) {}
      impl<T> BorrowIntentFnT for T where T: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) {}
      /// Boxed dynamic trait object of {@link BorrowIntentFnT}
      #[allow(unreachable_pub)]
      pub type BorrowIntentFn = Box<dyn BorrowIntentFnT + Send>;
    } else {
      use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
      /// Instances of BorrowIntentFnT can be used as user-provided function to describe how
      /// a borrowed assets shall be used.
      pub trait BorrowIntentFnT: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) {}
      impl<T> BorrowIntentFnT for T where T: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>) {}
      /// Boxed dynamic trait object of {@link BorrowIntentFnT}
      #[allow(unreachable_pub)]
      pub type BorrowIntentFn = Box<dyn BorrowIntentFnT + Send>;
    }
}

/// Action used to borrow in transaction [OnChainIdentity]'s assets.
#[derive(Default, Deserialize, Serialize)]
pub struct BorrowAction {
  objects: Vec<ObjectID>,
}

/// A [`BorrowAction`] coupled with a user-provided function to describe how
/// the borrowed assets shall be used.
pub struct BorrowActionWithIntent<F>
where
  F: BorrowIntentFnT,
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

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ProposalT for Proposal<BorrowAction> {
  type Action = BorrowAction;
  type Output = ();
  type Response = IotaTransactionBlockResponseAdapter;

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
    let tx = IdentityMoveCallsAdapter::propose_borrow(
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

  fn parse_tx_effects_internal(
    _tx_response: &dyn IotaTransactionBlockResponseT<Error = AdapterError, NativeResponse = AdapterNativeResponse>,
  ) -> Result<Self::Output, Error> {
    Ok(())
  }
}

impl<'i> UserDrivenTx<'i, BorrowAction> {
  /// Defines how the borrowed assets should be used.
  pub fn with_intent<F>(self, intent_fn: F) -> UserDrivenTx<'i, BorrowActionWithIntent<F>>
  where
    F: BorrowIntentFnT,
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
  type Input = BorrowIntentFn;
  type Tx = UserDrivenTx<'i, BorrowActionWithIntent<BorrowIntentFn>>;

  fn with(self, input: Self::Input) -> Self::Tx {
    self.with_intent(input)
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<F> TransactionInternal for UserDrivenTx<'_, BorrowActionWithIntent<F>>
where
  F: BorrowIntentFnT + Send,
{
  type Output = ();
  async fn execute_with_opt_gas_internal<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error>
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

    let intent_adapter = move |ptb: &mut dyn TransactionBuilderT<Error = AdapterError, NativeTxBuilder = Ptb>,
                               args: &HashMap<ObjectID, (Argument, IotaObjectData)>| {
      (borrow_action.intent_fn)(ptb.as_native_tx_builder(), args)
    };

    let tx = IdentityMoveCallsAdapter::execute_borrow(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      object_data_list,
      intent_adapter,
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    ExecuteProposalTx {
      identity,
      tx,
      _action: PhantomData::<BorrowAction>,
    }
    .execute_with_opt_gas_internal(gas_budget, client)
    .await
  }
}
