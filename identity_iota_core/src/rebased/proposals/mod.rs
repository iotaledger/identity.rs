// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod borrow;
mod config_change;
mod controller;
mod deactivate_did;
mod send;
mod update_did_doc;
mod upgrade;

use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

cfg_if::cfg_if! {
  if #[cfg(not(target_arch = "wasm32"))] {
    use identity_iota_interaction::rpc_types::IotaTransactionBlockResponse;
    use crate::rebased::transaction::Transaction;
  }
}
use crate::iota_interaction_adapter::AdapterError;
use crate::iota_interaction_adapter::NativeTransactionBlockResponse;
use crate::iota_interaction_adapter::IdentityMoveCallsAdapter;
use crate::iota_interaction_adapter::IotaTransactionBlockResponseAdapter;

use identity_iota_interaction::IdentityMoveCalls;
use identity_iota_interaction::IotaClientTrait;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::IotaTransactionBlockResponseT;
use identity_iota_interaction::OptionalSend;
use identity_iota_interaction::OptionalSync;
use identity_iota_interaction::ProgrammableTransactionBcs;

use crate::rebased::client::IdentityClientReadOnly;
use crate::rebased::migration::get_identity;
use crate::rebased::transaction::ProtoTransaction;
use crate::rebased::transaction::TransactionInternal;
use crate::rebased::transaction::TransactionOutputInternal;
use async_trait::async_trait;
pub use borrow::*;
pub use config_change::*;
pub use controller::*;
pub use deactivate_did::*;
use identity_iota_interaction::rpc_types::IotaExecutionStatus;
use identity_iota_interaction::rpc_types::IotaObjectData;
use identity_iota_interaction::rpc_types::IotaObjectDataOptions;
use identity_iota_interaction::types::base_types::ObjectID;
use identity_iota_interaction::types::base_types::ObjectRef;
use identity_iota_interaction::types::base_types::ObjectType;
use identity_iota_interaction::types::TypeTag;
use secret_storage::Signer;
pub use send::*;
use serde::de::DeserializeOwned;
pub use update_did_doc::*;
pub use upgrade::*;

use crate::rebased::client::IdentityClient;
use crate::rebased::migration::OnChainIdentity;
use crate::rebased::migration::Proposal;
use crate::rebased::Error;
use identity_iota_interaction::MoveType;

cfg_if::cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
    /// The internally used [`Transaction`] resulting from a proposal
    pub trait ResultingTransactionT: TransactionInternal {}
    impl<T> ResultingTransactionT for T where T: TransactionInternal {}
  } else {
    /// The [`Transaction`] resulting from a proposal
    pub trait ResultingTransactionT: Transaction {}
    impl<T> ResultingTransactionT for T where T: Transaction {}
  }
}

/// Interface that allows the creation and execution of an [`OnChainIdentity`]'s [`Proposal`]s.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ProposalT: Sized {
  /// The [`Proposal`] action's type.
  type Action;
  /// The output of the [`Proposal`]
  type Output;
  /// Platform-agnostic type of the IotaTransactionBlockResponse
  type Response: IotaTransactionBlockResponseT<Error = AdapterError, NativeResponse = NativeTransactionBlockResponse>;

  /// Creates a new [`Proposal`] with the provided action and expiration.
  async fn create<'i, S>(
    action: Self::Action,
    expiration: Option<u64>,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<impl ResultingTransactionT<Output = ProposalResult<Self>>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync;

  /// Converts the [`Proposal`] into a transaction that can be executed.
  async fn into_tx<'i, S>(
    self,
    identity: &'i mut OnChainIdentity,
    client: &IdentityClient<S>,
  ) -> Result<impl ProtoTransaction, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync;

  #[cfg(not(target_arch = "wasm32"))]
  /// Parses the transaction's effects and returns the output of the [`Proposal`].
  fn parse_tx_effects(tx_response: &IotaTransactionBlockResponse) -> Result<Self::Output, Error> {
    let adapter = IotaTransactionBlockResponseAdapter::new(tx_response.clone());
    Self::parse_tx_effects_internal(&adapter)
  }

  /// For internal platform-agnostic usage only.
  fn parse_tx_effects_internal(
    tx_response: &dyn IotaTransactionBlockResponseT<Error = AdapterError, NativeResponse = NativeTransactionBlockResponse>,
  ) -> Result<Self::Output, Error>;
}

impl<A> Proposal<A> {
  /// Creates a new [`ApproveProposalTx`] for the provided [`Proposal`]
  pub fn approve<'i>(&mut self, identity: &'i OnChainIdentity) -> ApproveProposalTx<'_, 'i, A> {
    ApproveProposalTx {
      proposal: self,
      identity,
    }
  }
}

/// A builder for creating a [`Proposal`].
#[derive(Debug)]
pub struct ProposalBuilder<'i, A> {
  identity: &'i mut OnChainIdentity,
  expiration: Option<u64>,
  action: A,
}

impl<A> Deref for ProposalBuilder<'_, A> {
  type Target = A;
  fn deref(&self) -> &Self::Target {
    &self.action
  }
}

impl<A> DerefMut for ProposalBuilder<'_, A> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.action
  }
}

impl<'i, A> ProposalBuilder<'i, A> {
  pub(crate) fn new(identity: &'i mut OnChainIdentity, action: A) -> Self {
    Self {
      identity,
      expiration: None,
      action,
    }
  }

  /// Sets the expiration epoch for the [`Proposal`].
  pub fn expiration_epoch(mut self, exp: u64) -> Self {
    self.expiration = Some(exp);
    self
  }

  /// Creates a [`Proposal`] with the provided arguments. If `forbid_chained_execution` is set to `true`,
  /// the [`Proposal`] won't be executed even if creator alone has enough voting power.
  pub async fn finish<'c, S>(
    self,
    client: &'c IdentityClient<S>,
  ) -> Result<impl ResultingTransactionT<Output = ProposalResult<Proposal<A>>> + use<'i, 'c, S, A>, Error>
  where
    Proposal<A>: ProposalT<Action = A>,
    S: Signer<IotaKeySignature> + OptionalSync,
    A: 'c,
    'i: 'c,
  {
    let Self {
      action,
      expiration,
      identity,
    } = self;
    Proposal::<A>::create(action, expiration, identity, client).await
  }
}

#[derive(Debug)]
/// The result of creating a [`Proposal`]. When a [`Proposal`] is executed
/// in the same transaction as its creation, a [`ProposalResult::Executed`] is
/// returned. [`ProposalResult::Pending`] otherwise.
pub enum ProposalResult<P: ProposalT> {
  /// A [`Proposal`] that has yet to be executed.
  Pending(P),
  /// A [`Proposal`]'s execution output.
  Executed(P::Output),
}

/// A transaction to create a [`Proposal`].
#[derive(Debug)]
pub struct CreateProposalTx<'i, A> {
  identity: &'i mut OnChainIdentity,
  tx: ProgrammableTransactionBcs,
  chained_execution: bool,
  _action: PhantomData<A>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<A> TransactionInternal for CreateProposalTx<'_, A>
where
  Proposal<A>: ProposalT<Action = A> + DeserializeOwned,
  A: Send,
{
  type Output = ProposalResult<Proposal<A>>;

  async fn execute_with_opt_gas_internal<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<ProposalResult<Proposal<A>>>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let Self {
      identity,
      tx,
      chained_execution,
      ..
    } = self;
    let tx_response = client.execute_transaction(tx, gas_budget).await?;
    let tx_effects_execution_status = tx_response
      .effects_execution_status()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing transaction's effects".to_string()))?;

    if let IotaExecutionStatus::Failure { error } = tx_effects_execution_status {
      return Err(Error::TransactionUnexpectedResponse(error.clone()));
    }

    // Identity has been changed regardless of whether the proposal has been executed
    // or simply created. Refetch it, to sync it with its on-chain state.
    *identity = get_identity(client, identity.id())
      .await?
      .expect("identity exists on-chain");

    if chained_execution {
      // The proposal has been created and executed right-away. Parse its effects.
      Proposal::<A>::parse_tx_effects_internal(tx_response.as_ref()).map(ProposalResult::Executed)
    } else {
      // 2 objects are created, one is the Bag's Field and the other is our Proposal. Proposal is not owned by the bag,
      // but the field is.
      let proposals_bag_id = identity.multicontroller().proposals_bag_id();
      let proposal_id = tx_response
        .effects_created()
        .ok_or_else(|| Error::TransactionUnexpectedResponse("transaction had no effects".to_string()))?
        .iter()
        .find(|obj_ref| obj_ref.owner != proposals_bag_id)
        .expect("tx was successful")
        .object_id();

      client.get_object_by_id(proposal_id).await.map(ProposalResult::Pending)
    }
    .map(move |output| TransactionOutputInternal {
      output,
      response: tx_response,
    })
  }
}

/// A transaction to execute a [`Proposal`].
#[derive(Debug)]
pub struct ExecuteProposalTx<'i, A> {
  tx: ProgrammableTransactionBcs,
  identity: &'i mut OnChainIdentity,
  _action: PhantomData<A>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<A> TransactionInternal for ExecuteProposalTx<'_, A>
where
  Proposal<A>: ProposalT<Action = A>,
  A: OptionalSend,
{
  type Output = <Proposal<A> as ProposalT>::Output;
  async fn execute_with_opt_gas_internal<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + OptionalSync,
  {
    let Self { identity, tx, .. } = self;
    let tx_response = client.execute_transaction(tx, gas_budget).await?;
    let tx_effects_execution_status = tx_response
      .effects_execution_status()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects".to_string()))?;

    if let IotaExecutionStatus::Failure { error } = tx_effects_execution_status {
      Err(Error::TransactionUnexpectedResponse(error.clone()))
    } else {
      *identity = get_identity(client, identity.id())
        .await?
        .expect("identity exists on-chain");

      Proposal::<A>::parse_tx_effects_internal(tx_response.as_ref()).map(move |output| TransactionOutputInternal {
        output,
        response: tx_response,
      })
    }
  }
}

/// A transaction to approve a [`Proposal`].
#[derive(Debug)]
pub struct ApproveProposalTx<'p, 'i, A> {
  proposal: &'p mut Proposal<A>,
  identity: &'i OnChainIdentity,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<A> TransactionInternal for ApproveProposalTx<'_, '_, A>
where
  Proposal<A>: ProposalT<Action = A>,
  A: MoveType + Send,
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
    let Self { proposal, identity, .. } = self;
    let identity_ref = client.get_object_ref_by_id(identity.id()).await?.unwrap();
    let controller_cap = identity.get_controller_cap(client).await?;
    let tx = <IdentityMoveCallsAdapter as IdentityMoveCalls>::approve_proposal::<A>(
      identity_ref.clone(),
      controller_cap,
      proposal.id(),
      client.package_id(),
    )?;

    let response = client.execute_transaction(tx, gas_budget).await?;
    let tx_effects_execution_status = response
      .effects_execution_status()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects".to_string()))?;

    if let IotaExecutionStatus::Failure { error } = tx_effects_execution_status {
      return Err(Error::TransactionUnexpectedResponse(error.clone()));
    }

    let vp = identity
      .controller_voting_power(controller_cap.0)
      .expect("is identity's controller");
    *proposal.votes_mut() = proposal.votes() + vp;

    Ok(TransactionOutputInternal { output: (), response })
  }
}

async fn obj_data_for_id(client: &IdentityClientReadOnly, obj_id: ObjectID) -> anyhow::Result<IotaObjectData> {
  use anyhow::Context;

  client
    .read_api()
    .get_object_with_options(obj_id, IotaObjectDataOptions::default().with_type().with_owner())
    .await?
    .into_object()
    .context("no iota object in response")
}

async fn obj_ref_and_type_for_id(
  client: &IdentityClientReadOnly,
  obj_id: ObjectID,
) -> anyhow::Result<(ObjectRef, TypeTag)> {
  let res = obj_data_for_id(client, obj_id).await?;
  let obj_ref = res.object_ref();
  let obj_type = match res.object_type().expect("object type is requested") {
    ObjectType::Package => anyhow::bail!("a move package cannot be sent"),
    ObjectType::Struct(type_) => type_.into(),
  };

  Ok((obj_ref, obj_type))
}

/// A transaction that requires user input in order to be executed.
pub struct UserDrivenTx<'i, A> {
  identity: &'i mut OnChainIdentity,
  action: A,
  proposal_id: ObjectID,
}
