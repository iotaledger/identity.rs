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
use identity_iota_interaction::OptionalSync;
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
      use iota_interaction_ts::NativeTsTransactionBuilderBindingWrapper as Ptb;
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
#[derive(Deserialize, Serialize)]
pub struct BorrowAction<F = BorrowIntentFn> {
  objects: Vec<ObjectID>,
  #[serde(skip, default = "Option::default")]
  intent_fn: Option<F>,
}

impl<F> Default for BorrowAction<F> {
  fn default() -> Self {
    Self {
      objects: vec![],
      intent_fn: None,
    }
  }
}

/// A [`BorrowAction`] coupled with a user-provided function to describe how
/// the borrowed assets shall be used.
pub struct BorrowActionWithIntent<F>(BorrowAction<F>)
where
  F: BorrowIntentFnT;

impl MoveType for BorrowAction {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::borrow_proposal::Borrow")).expect("valid move type")
  }
}

impl<F> BorrowAction<F> {
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

impl<'i, F> ProposalBuilder<'i, BorrowAction<F>> {
  /// Adds an object to the list of objects that will be borrowed when executing this action.
  pub fn borrow(mut self, object_id: ObjectID) -> Self {
    self.action.borrow_object(object_id);
    self
  }
  /// Adds many objects. See [`BorrowAction::borrow_object`] for more details.
  pub fn borrow_objects<I>(self, objects: I) -> Self
  where
    I: IntoIterator<Item = ObjectID>,
  {
    objects.into_iter().fold(self, |builder, obj| builder.borrow(obj))
  }

  /// Specifies how to use the borrowed assets. This is only useful if the sender of this
  /// transaction has enough voting power to execute this proposal right-away.
  pub fn with_intent<F1>(self, intent_fn: F1) -> ProposalBuilder<'i, BorrowAction<F1>>
  where
    F1: FnOnce(&mut Ptb, &HashMap<ObjectID, (Argument, IotaObjectData)>),
  {
    let ProposalBuilder {
      identity,
      expiration,
      action: BorrowAction { objects, .. },
    } = self;
    let intent_fn = Some(intent_fn);
    ProposalBuilder {
      identity,
      expiration,
      action: BorrowAction { objects, intent_fn },
    }
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<F> ProposalT for Proposal<BorrowAction<F>>
where
  F: BorrowIntentFnT + Send,
{
  type Action = BorrowAction<F>;
  type Output = ();
  type Response = IotaTransactionBlockResponseAdapter;

  async fn create<'i, S>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<CreateProposalTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;
    let can_execute = identity
      .controller_voting_power(controller_cap_ref.0)
      .expect("is a controller of identity")
      >= identity.threshold();
    let chained_execution = can_execute && action.intent_fn.is_some();
    let tx = if chained_execution {
      // Construct a list of `(ObjectRef, TypeTag)` from the list of objects to send.
      let object_data_list = {
        let mut object_data_list = vec![];
        for obj_id in action.objects {
          let object_data = super::obj_data_for_id(client, obj_id)
            .await
            .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
          object_data_list.push(object_data);
        }
        object_data_list
      };
      IdentityMoveCallsAdapter::create_and_execute_borrow(
        identity_ref,
        controller_cap_ref,
        object_data_list,
        action.intent_fn.unwrap(),
        expiration,
        client.package_id(),
      )
    } else {
      IdentityMoveCallsAdapter::propose_borrow(
        identity_ref,
        controller_cap_ref,
        action.objects,
        expiration,
        client.package_id(),
      )
    }
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    Ok(CreateProposalTx {
      identity,
      tx,
      chained_execution,
      _action: PhantomData,
    })
  }

  async fn into_tx<'i, S>(
    self,
    identity: &'i mut OnChainIdentity,
    _: &IdentityClient<S>,
  ) -> Result<UserDrivenTx<'i, Self::Action>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
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

impl<'i, F> UserDrivenTx<'i, BorrowAction<F>> {
  /// Defines how the borrowed assets should be used.
  pub fn with_intent<F1>(self, intent_fn: F1) -> UserDrivenTx<'i, BorrowActionWithIntent<F1>>
  where
    F1: BorrowIntentFnT,
  {
    let UserDrivenTx {
      identity,
      action: BorrowAction { objects, .. },
      proposal_id,
    } = self;
    let intent_fn = Some(intent_fn);
    UserDrivenTx {
      identity,
      proposal_id,
      action: BorrowActionWithIntent(BorrowAction { objects, intent_fn }),
    }
  }
}

impl<'i, F> ProtoTransaction for UserDrivenTx<'i, BorrowAction<F>> {
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
    S: Signer<IotaKeySignature> + OptionalSync,
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
      for obj_id in borrow_action.0.objects {
        let object_data = super::obj_data_for_id(client, obj_id)
          .await
          .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;
        object_data_list.push(object_data);
      }
      object_data_list
    };

    let tx = IdentityMoveCallsAdapter::execute_borrow(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      object_data_list,
      borrow_action
        .0
        .intent_fn
        .expect("BorrowActionWithIntent makes sure intent_fn is there"),
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
