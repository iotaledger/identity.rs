// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use crate::iota_interaction_adapter::AdapterError;
use crate::iota_interaction_adapter::NativeTransactionBlockResponse;
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
use identity_iota_interaction::rpc_types::IotaObjectRef;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
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
      /// Instances of ControllerIntentFnT can be used as user-provided function to describe how
      /// a borrowed identity's controller capability will be used.
      pub trait ControllerIntentFnT: FnOnce(&mut Ptb, &Argument) {}
      impl<T> ControllerIntentFnT for T where T: FnOnce(&mut Ptb, &Argument) {}
      #[allow(unreachable_pub)]
      /// Boxed dynamic trait object of {@link ControllerIntentFnT}
      pub type ControllerIntentFn = Box<dyn ControllerIntentFnT + Send>;
    } else {
      use identity_iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder as Ptb;
      /// Instances of ControllerIntentFnT can be used as user-provided function to describe how
      /// a borrowed identity's controller capability will be used.
      pub trait ControllerIntentFnT: FnOnce(&mut Ptb, &Argument) {}
      impl<T> ControllerIntentFnT for T where T: FnOnce(&mut Ptb, &Argument) {}
      #[allow(unreachable_pub)]
      /// Boxed dynamic trait object of {@link ControllerIntentFnT}
      pub type ControllerIntentFn = Box<dyn ControllerIntentFnT + Send>;
    }
}

/// Borrow an [`OnChainIdentity`]'s controller capability to exert control on
/// a sub-owned identity.
#[derive(Debug, Deserialize, Serialize)]
pub struct ControllerExecution<F = ControllerIntentFn> {
  controller_cap: ObjectID,
  identity: IotaAddress,
  #[serde(skip, default = "Option::default")]
  intent_fn: Option<F>,
}

/// A [`ControllerExecution`] action coupled with a user-provided function to describe how
/// the borrowed identity's controller capability will be used.
pub struct ControllerExecutionWithIntent<F>(ControllerExecution<F>)
where
  F: FnOnce(&mut Ptb, &Argument);

impl<F> ControllerExecutionWithIntent<F>
where
  F: ControllerIntentFnT,
{
  fn new(action: ControllerExecution<F>) -> Self {
    debug_assert!(action.intent_fn.is_some());
    Self(action)
  }
}

impl<F> ControllerExecution<F> {
  /// Creates a new [`ControllerExecution`] action, allowing a controller of `identity` to
  /// borrow `identity`'s controller cap for a transaction.
  pub fn new(controller_cap: ObjectID, identity: &OnChainIdentity) -> Self {
    Self {
      controller_cap,
      identity: identity.id().into(),
      intent_fn: None,
    }
  }

  /// Specifies how the borrowed `ControllerCap` should be used in the transaction.
  /// This is only useful if the controller creating this proposal has enough voting
  /// power to carry out it out immediately.
  pub fn with_intent<F1>(self, intent_fn: F1) -> ControllerExecution<F1>
  where
    F1: FnOnce(&mut Ptb, &Argument),
  {
    let Self {
      controller_cap,
      identity,
      ..
    } = self;
    ControllerExecution {
      controller_cap,
      identity,
      intent_fn: Some(intent_fn),
    }
  }
}

impl<'i, F> ProposalBuilder<'i, ControllerExecution<F>> {
  /// Specifies how the borrowed `ControllerCap` should be used in the transaction.
  /// This is only useful if the controller creating this proposal has enough voting
  /// power to carry out it out immediately.
  pub fn with_intent<F1>(self, intent_fn: F1) -> ProposalBuilder<'i, ControllerExecution<F1>>
  where
    F1: FnOnce(&mut Ptb, &Argument),
  {
    let ProposalBuilder {
      identity,
      expiration,
      action,
    } = self;
    ProposalBuilder {
      identity,
      expiration,
      action: action.with_intent(intent_fn),
    }
  }
}

impl MoveType for ControllerExecution {
  fn move_type(package: ObjectID) -> TypeTag {
    use std::str::FromStr;

    TypeTag::from_str(&format!("{package}::controller_proposal::ControllerExecution")).expect("valid move type")
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<F> ProposalT for Proposal<ControllerExecution<F>>
where
  F: ControllerIntentFnT + Send,
{
  type Action = ControllerExecution<F>;
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
    let chained_execution = action.intent_fn.is_some()
      && identity
        .controller_voting_power(controller_cap_ref.0)
        .expect("is an identity's controller")
        >= identity.threshold();

    let tx = if chained_execution {
      let borrowing_controller_cap_ref = client
        .get_object_ref_by_id(action.controller_cap)
        .await?
        .map(|OwnedObjectRef { reference, .. }| {
          let IotaObjectRef {
            object_id,
            version,
            digest,
          } = reference;
          (object_id, version, digest)
        })
        .ok_or_else(|| Error::ObjectLookup(format!("object {} doesn't exist", action.controller_cap)))?;

      IdentityMoveCallsAdapter::create_and_execute_controller_execution(
        identity_ref,
        controller_cap_ref,
        expiration,
        borrowing_controller_cap_ref,
        action.intent_fn.unwrap(),
        client.package_id(),
      )
    } else {
      IdentityMoveCallsAdapter::propose_controller_execution(
        identity_ref,
        controller_cap_ref,
        action.controller_cap,
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
    let controller_execution_action = self.into_action();

    Ok(UserDrivenTx {
      identity,
      proposal_id,
      action: controller_execution_action,
    })
  }

  fn parse_tx_effects_internal(
    _tx_response: &dyn IotaTransactionBlockResponseT<Error = AdapterError, NativeResponse = NativeTransactionBlockResponse>,
  ) -> Result<Self::Output, Error> {
    Ok(())
  }
}

impl<'i, F> UserDrivenTx<'i, ControllerExecution<F>> {
  /// Defines how the borrowed assets should be used.
  pub fn with_intent<F1>(self, intent_fn: F1) -> UserDrivenTx<'i, ControllerExecutionWithIntent<F1>>
  where
    F1: ControllerIntentFnT,
  {
    let UserDrivenTx {
      identity,
      action,
      proposal_id,
    } = self;

    UserDrivenTx {
      identity,
      proposal_id,
      action: ControllerExecutionWithIntent::new(action.with_intent(intent_fn)),
    }
  }
}

impl<'i, F> ProtoTransaction for UserDrivenTx<'i, ControllerExecution<F>> {
  type Input = ControllerIntentFn;
  type Tx = UserDrivenTx<'i, ControllerExecutionWithIntent<ControllerIntentFn>>;

  fn with(self, input: Self::Input) -> Self::Tx {
    self.with_intent(input)
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<F> TransactionInternal for UserDrivenTx<'_, ControllerExecutionWithIntent<F>>
where
  F: ControllerIntentFnT + Send,
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
      action,
      proposal_id,
    } = self;
    let identity_ref = client
      .get_object_ref_by_id(identity.id())
      .await?
      .expect("identity exists on-chain");
    let controller_cap_ref = identity.get_controller_cap(client).await?;

    let borrowing_cap_id = action.0.controller_cap;
    let borrowing_controller_cap_ref = client
      .get_object_ref_by_id(borrowing_cap_id)
      .await?
      .map(|OwnedObjectRef { reference, .. }| {
        let IotaObjectRef {
          object_id,
          version,
          digest,
        } = reference;
        (object_id, version, digest)
      })
      .ok_or_else(|| Error::ObjectLookup(format!("object {borrowing_cap_id} doesn't exist")))?;

    let tx = IdentityMoveCallsAdapter::execute_controller_execution(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      borrowing_controller_cap_ref,
      action
        .0
        .intent_fn
        .expect("BorrowActionWithIntent makes sure intent_fn is present"),
      client.package_id(),
    )
    .map_err(|e| Error::TransactionBuildingFailed(e.to_string()))?;

    ExecuteProposalTx {
      identity,
      tx,
      _action: PhantomData::<ControllerExecution>,
    }
    .execute_with_opt_gas_internal(gas_budget, client)
    .await
  }
}
