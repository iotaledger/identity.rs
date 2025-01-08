// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
use identity_iota_interaction::rpc_types::IotaObjectRef;
use identity_iota_interaction::rpc_types::OwnedObjectRef;
use identity_iota_interaction::types::base_types::IotaAddress;
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
use super::ProposalT;
use super::UserDrivenTx;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
      use iota_interaction_ts::NativeTsCodeBindingWrapper as Ptb;
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
pub struct ControllerExecution {
  controller_cap: ObjectID,
  identity: IotaAddress,
}

/// A [`ControllerExecution`] action coupled with a user-provided function to describe how
/// the borrowed identity's controller capability will be used.
pub struct ControllerExecutionWithIntent<F>
where
  F: ControllerIntentFnT,
{
  action: ControllerExecution,
  intent_fn: F,
}

impl ControllerExecution {
  /// Creates a new [`ControllerExecution`] action, allowing a controller of `identity` to
  /// borrow `identity`'s controller cap for a transaction.
  pub fn new(controller_cap: ObjectID, identity: &OnChainIdentity) -> Self {
    Self {
      controller_cap,
      identity: identity.id().into(),
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
impl ProposalT for Proposal<ControllerExecution> {
  type Action = ControllerExecution;
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

    let tx = IdentityMoveCallsAdapter::propose_controller_execution(
      identity_ref,
      controller_cap_ref,
      action.controller_cap,
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
    let controller_execution_action = self.into_action();

    Ok(UserDrivenTx {
      identity,
      proposal_id,
      action: controller_execution_action,
    })
  }

  fn parse_tx_effects_internal(
    _tx_response: &dyn IotaTransactionBlockResponseT<Error = AdapterError, NativeResponse = AdapterNativeResponse>,
  ) -> Result<Self::Output, Error> {
    Ok(())
  }
}

impl<'i> UserDrivenTx<'i, ControllerExecution> {
  /// Defines how the borrowed assets should be used.
  pub fn with_intent<F>(self, intent_fn: F) -> UserDrivenTx<'i, ControllerExecutionWithIntent<F>>
  where
    F: ControllerIntentFnT,
  {
    let UserDrivenTx {
      identity,
      action,
      proposal_id,
    } = self;
    UserDrivenTx {
      identity,
      proposal_id,
      action: ControllerExecutionWithIntent { action, intent_fn },
    }
  }
}

impl<'i> ProtoTransaction for UserDrivenTx<'i, ControllerExecution> {
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
    S: Signer<IotaKeySignature> + Sync,
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

    let borrowing_cap_id = action.action.controller_cap;
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

    let intent_adapter = move |ptb: &mut dyn TransactionBuilderT<Error = AdapterError, NativeTxBuilder = Ptb>,
                               arg: &Argument| {
      (action.intent_fn)(ptb.as_native_tx_builder(), arg)
    };

    let tx = IdentityMoveCallsAdapter::execute_controller_execution(
      identity_ref,
      controller_cap_ref,
      proposal_id,
      borrowing_controller_cap_ref,
      intent_adapter,
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
