mod config_change;
mod deactive_did;
mod update_did_doc;

use std::ops::Deref;
use std::ops::DerefMut;

pub use config_change::*;
pub use deactive_did::*;
use iota_sdk::error::Error as IotaSdkError;
use iota_sdk::rpc_types::IotaExecutionStatus;
use iota_sdk::rpc_types::IotaTransactionBlockEffects;
use iota_sdk::rpc_types::IotaTransactionBlockEffectsAPI;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::types::object::Owner;
use iota_sdk::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_sdk::types::transaction::Argument;
use iota_sdk::types::transaction::ProgrammableTransaction;
use secret_storage::signer::Signer;
use serde::Deserialize;
pub use update_did_doc::*;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::migration::get_identity;
use crate::migration::OnChainIdentity;
use crate::migration::Proposal;
use crate::sui::move_calls;
use crate::utils::MoveType;
use crate::Error;

pub trait ProposalT {
  type Action;
  type Output;
  fn make_create_tx(
    action: Self::Action,
    expiration: Option<u64>,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    identity_ref: &OnChainIdentity,
    package: ObjectID,
  ) -> Result<(ProgrammableTransactionBuilder, Argument), Error>;
  fn make_chained_execution_tx(
    ptb: ProgrammableTransactionBuilder,
    proposal_arg: Argument,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransaction, Error>;
  fn make_execute_tx(
    &self,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransaction, Error>;
  fn parse_tx_effects(tx_effects: IotaTransactionBlockEffects) -> Result<Self::Output, Error>;
}

impl<A> Proposal<A>
where
  Self: ProposalT<Action = A> + for<'de> Deserialize<'de>,
{
  pub async fn create<S>(
    action: A,
    expiration: Option<u64>,
    gas_budget: u64,
    allow_chained_execution: bool,
    identity: &mut OnChainIdentity,
    client: &IdentityClient,
    signer: &S,
  ) -> Result<ProposalResult<Self>, Error>
  where
    S: Signer<IotaKeySignature> + Send + Sync,
  {
    let identity_ref = identity.obj_ref.as_ref().cloned().unwrap();
    let controller_cap = identity.get_controller_cap(client).await?;

    let (ptb, proposal_arg) = Self::make_create_tx(
      action,
      expiration,
      identity_ref.clone(),
      controller_cap,
      identity,
      client.package_id(),
    )?;
    let is_threshold_reached =
      identity.did_doc.controller_voting_power(controller_cap.0).unwrap() >= identity.did_doc.threshold();
    let chain_execute = allow_chained_execution && is_threshold_reached;
    let tx = if chain_execute {
      Self::make_chained_execution_tx(ptb, proposal_arg, identity_ref, controller_cap, client.package_id())?
    } else {
      ptb.finish()
    };
    let tx_effects = client
      .execute_transaction(tx, gas_budget, signer)
      .await?
      .effects
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects".to_string()))?;
    if let IotaExecutionStatus::Failure { error } = tx_effects.status() {
      return Err(IotaSdkError::DataError(error.clone()).into());
    }

    if chain_execute {
      Self::parse_tx_effects(tx_effects).map(ProposalResult::Executed)
    } else {
      // 2 objects are created, one is the Bag's Field and the other is our Proposal. Proposal is not owned by the bag,
      // but the field is.
      let proposal_id = tx_effects
        .created()
        .iter()
        .find(|obj_ref| obj_ref.owner != Owner::ObjectOwner(identity.did_doc.proposals_bag_id().into()))
        .expect("tx was successful")
        .object_id();

      *identity = get_identity(client, identity.id())
        .await?
        .expect("identity exists on-chain");

      client.get_object_by_id(proposal_id).await.map(ProposalResult::Pending)
    }
  }
  pub async fn execute<S>(
    self,
    gas_budget: u64,
    identity: &mut OnChainIdentity,
    client: &IdentityClient,
    signer: &S,
  ) -> Result<<Self as ProposalT>::Output, Error>
  where
    S: Signer<IotaKeySignature> + Send + Sync,
  {
    let identity_ref = identity.obj_ref.as_ref().unwrap().clone();
    let controller_cap = identity.get_controller_cap(client).await?;

    let tx = Self::make_execute_tx(&self, identity_ref, controller_cap, client.package_id())?;
    let tx_effects = client
      .execute_transaction(tx, gas_budget, signer)
      .await?
      .effects
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects".to_string()))?;
    if let IotaExecutionStatus::Failure { error } = tx_effects.status() {
      Err(IotaSdkError::DataError(error.clone()).into())
    } else {
      Self::parse_tx_effects(tx_effects)
    }
  }
}

impl<A> Proposal<A>
where
  Proposal<A>: ProposalT,
  A: MoveType,
{
  pub async fn approve<S>(
    &mut self,
    gas_budget: u64,
    identity: &mut OnChainIdentity,
    client: &IdentityClient,
    signer: &S,
  ) -> Result<(), Error>
  where
    S: Signer<IotaKeySignature> + Send + Sync,
  {
    let identity_ref = identity.obj_ref.as_ref().unwrap();
    let controller_cap = identity.get_controller_cap(client).await?;
    let tx = move_calls::identity::proposal::approve::<A>(
      identity_ref.clone(),
      controller_cap,
      self.id(),
      client.package_id(),
    )?;

    let response = client.execute_transaction(tx, gas_budget, signer).await?;
    if let IotaExecutionStatus::Failure { error } = response.effects.expect("had effects").into_status() {
      return Err(Error::TransactionUnexpectedResponse(error));
    }

    *identity = get_identity(client, identity.id())
      .await?
      .expect("self exists on-chain");
    let vp = identity
      .did_doc
      .controller_voting_power(controller_cap.0)
      .expect("is identity's controller");
    *self.votes_mut() = self.votes() + vp;

    Ok(())
  }
}

#[derive(Debug)]
pub struct ProposalBuilder<'i, A> {
  identity: &'i mut OnChainIdentity,
  expiration: Option<u64>,
  action: A,
  gas_budget: Option<u64>,
  forbid_chained_execution: bool,
}

impl<'i, A> Deref for ProposalBuilder<'i, A> {
  type Target = A;
  fn deref(&self) -> &Self::Target {
    &self.action
  }
}

impl<'i, A> DerefMut for ProposalBuilder<'i, A> {
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
      gas_budget: None,
      forbid_chained_execution: false,
    }
  }

  pub fn expiration_epoch(mut self, exp: u64) -> Self {
    self.expiration = Some(exp);
    self
  }

  pub fn gas_budget(mut self, amount: u64) -> Self {
    self.gas_budget = Some(amount);
    self
  }

  pub fn forbid_chained_execution(mut self) -> Self {
    self.forbid_chained_execution = true;
    self
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

impl<'i, A> ProposalBuilder<'i, A> {
  /// Creates a [`Proposal`] with the provided arguments. If `forbid_chained_execution` is set to `true`,
  /// the [`Proposal`] won't be executed even if creator alone has enough voting power.
  pub async fn finish<S>(self, client: &IdentityClient, signer: &S) -> Result<ProposalResult<Proposal<A>>, Error>
  where
    Proposal<A>: ProposalT<Action = A> + for<'de> Deserialize<'de>,
    S: Signer<IotaKeySignature> + Send + Sync,
  {
    let ProposalBuilder {
      identity,
      expiration,
      action,
      gas_budget,
      forbid_chained_execution,
    } = self;
    let gas_budget = gas_budget.ok_or_else(|| Error::GasIssue("missing `gas_budget`".to_string()))?;

    Proposal::<A>::create(
      action,
      expiration,
      gas_budget,
      !forbid_chained_execution,
      identity,
      client,
      signer,
    )
    .await
  }
}
