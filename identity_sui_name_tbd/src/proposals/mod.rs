mod config_change;
mod deactive_did;
mod update_did_doc;

use std::ops::Deref;
use std::ops::DerefMut;

use async_trait::async_trait;
pub use config_change::*;
pub use deactive_did::*;
use crate::iota_sdk_abstraction::error::Error as IotaSdkError;
use crate::iota_sdk_abstraction::rpc_types::IotaExecutionStatus;
use crate::iota_sdk_abstraction::IotaTransactionBlockResponseT;
use crate::iota_sdk_abstraction::rpc_types::OwnedObjectRef;
use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::base_types::ObjectRef;
use crate::iota_sdk_abstraction::types::object::Owner;
use crate::iota_sdk_abstraction::TransactionBuilderT;
use crate::iota_sdk_abstraction::types::transaction::Argument;
use crate::iota_sdk_abstraction::ProgrammableTransactionBcs;
use secret_storage::Signer;
use serde::de::DeserializeOwned;
pub use update_did_doc::*;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::migration::OnChainIdentity;
use crate::migration::Proposal;
use crate::sui::iota_sdk_adapter::IdentityMoveCallsAdapter;
use crate::iota_sdk_abstraction::IdentityMoveCalls;
use crate::transaction::Transaction;
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
    identity_ref: OnChainIdentity,
    package: ObjectID,
  ) -> Result<(<IdentityMoveCallsAdapter as IdentityMoveCalls>::TxBuilder, Argument), Error>;
  fn make_chained_execution_tx(
    ptb: <IdentityMoveCallsAdapter as IdentityMoveCalls>::TxBuilder,
    proposal_arg: Argument,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Error>;
  fn make_execute_tx(
    &self,
    identity: OwnedObjectRef,
    controller_cap: ObjectRef,
    package: ObjectID,
  ) -> Result<ProgrammableTransactionBcs, Error>;
  fn parse_tx_effects(_tx_response: &dyn IotaTransactionBlockResponseT<Error = Error>) -> Result<Self::Output, Error>;
}

impl<A> Proposal<A> {
  pub fn approve<'i>(&mut self, identity: &'i OnChainIdentity) -> ApproveProposalTx<'_, 'i, A> {
    ApproveProposalTx {
      proposal: self,
      identity,
    }
  }

  pub fn execute(self, identity: &mut OnChainIdentity) -> ExecuteProposalTx<'_, A> {
    ExecuteProposalTx {
      proposal: self,
      identity,
    }
  }
}

#[derive(Debug)]
pub struct ProposalBuilder<A> {
  identity: OnChainIdentity,
  expiration: Option<u64>,
  action: A,
  forbid_chained_execution: bool,
}

impl<A> Deref for ProposalBuilder<A> {
  type Target = A;
  fn deref(&self) -> &Self::Target {
    &self.action
  }
}

impl<A> DerefMut for ProposalBuilder<A> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.action
  }
}

impl<A> ProposalBuilder<A> {
  pub(crate) fn new(identity: OnChainIdentity, action: A) -> Self {
    Self {
      identity,
      expiration: None,
      action,
      forbid_chained_execution: false,
    }
  }

  pub fn expiration_epoch(mut self, exp: u64) -> Self {
    self.expiration = Some(exp);
    self
  }

  pub fn forbid_chained_execution(mut self) -> Self {
    self.forbid_chained_execution = true;
    self
  }

  /// Creates a [`Proposal`] with the provided arguments. If `forbid_chained_execution` is set to `true`,
  /// the [`Proposal`] won't be executed even if creator alone has enough voting power.
  pub fn finish(self) -> CreateProposalTx<A> {
    CreateProposalTx(self)
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

#[derive(Debug)]
pub struct CreateProposalTx<A>(ProposalBuilder<A>);

#[async_trait]
impl<A> Transaction for CreateProposalTx<A>
where
  Proposal<A>: ProposalT<Action = A> + DeserializeOwned,
  A: Send,
{
  type Output = ProposalResult<Proposal<A>>;

  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<ProposalResult<Proposal<A>>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let ProposalBuilder {
      identity,
      expiration,
      action,
      forbid_chained_execution,
    } = self.0;
    let identity_ref = client.get_object_ref_by_id(identity.id()).await?.unwrap();
    let controller_cap = identity.get_controller_cap(client).await?;

    let is_threshold_reached = identity.controller_voting_power(controller_cap.0).unwrap() >= identity.threshold();
    let multicontroller_proposals_bag_id = Owner::ObjectOwner(identity.multicontroller().proposals_bag_id().into());
    let (ptb, proposal_arg) = Proposal::<A>::make_create_tx(
      action,
      expiration,
      identity_ref.clone(),
      controller_cap,
      identity,
      client.package_id(),
    )?;
    let chain_execute = !forbid_chained_execution && is_threshold_reached;
    let tx = if chain_execute {
      Proposal::<A>::make_chained_execution_tx(ptb, proposal_arg, identity_ref, controller_cap, client.package_id())?
    } else {
      ptb.finish()?
    };
    let tx_response = client
      .execute_transaction(tx, gas_budget)
      .await?;
    let tx_effects_status = tx_response.effects_execution_status()
      .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects".to_string()))?;
    if let IotaExecutionStatus::Failure { error } = tx_effects_status {
      return Err(IotaSdkError::Data(error.clone()).into());
    }

    if chain_execute {
      Proposal::<A>::parse_tx_effects(tx_response.as_ref()).map(ProposalResult::Executed)
    } else {
      // 2 objects are created, one is the Bag's Field and the other is our Proposal. Proposal is not owned by the bag,
      // but the field is.
      let proposal_id = tx_response.effects_created()
        .ok_or_else(|| Error::TransactionUnexpectedResponse("transaction had no effects".to_string()))?
        .iter()
        .find(|obj_ref| obj_ref.owner != multicontroller_proposals_bag_id)
        .expect("tx was successful")
        .object_id();

      // *identity = get_identity(client, identity.id())
      //   .await?
      //   .expect("identity exists on-chain");

      client.get_object_by_id(proposal_id).await.map(ProposalResult::Pending)
    }
  }
}

#[derive(Debug)]
pub struct ExecuteProposalTx<'i, A> {
  proposal: Proposal<A>,
  identity: &'i mut OnChainIdentity,
}

#[async_trait]
impl<'i, A> Transaction for ExecuteProposalTx<'i, A>
where
  Proposal<A>: ProposalT<Action = A>,
  A: Send,
{
  type Output = <Proposal<A> as ProposalT>::Output;
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let Self { proposal, identity } = self;
    let identity_ref = client.get_object_ref_by_id(identity.id()).await?.unwrap();
    let controller_cap = identity.get_controller_cap(client).await?;

    let tx = proposal.make_execute_tx(identity_ref, controller_cap, client.package_id())?;
    let tx_response = client
      .execute_transaction(tx, gas_budget)
      .await?;
    let tx_effects_status = tx_response.effects_execution_status()
        .ok_or_else(|| Error::TransactionUnexpectedResponse("missing effects".to_string()))?;

    if let IotaExecutionStatus::Failure { error } = tx_effects_status {
      Err(IotaSdkError::Data(error.clone()).into())
    } else {
      Proposal::<A>::parse_tx_effects(tx_response.as_ref())
    }
  }
}

#[derive(Debug)]
pub struct ApproveProposalTx<'p, 'i, A> {
  proposal: &'p mut Proposal<A>,
  identity: &'i OnChainIdentity,
}

#[async_trait]
impl<'p, 'i, A> Transaction for ApproveProposalTx<'p, 'i, A>
where
  Proposal<A>: ProposalT<Action = A>,
  A: MoveType + Send,
{
  type Output = ();
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let Self { proposal, identity } = self;
    let identity_ref = client.get_object_ref_by_id(identity.id()).await?.unwrap();
    let controller_cap = identity.get_controller_cap(client).await?;
    let tx = IdentityMoveCallsAdapter::approve_proposal::<A>(
      identity_ref.clone(),
      controller_cap,
      proposal.id(),
      client.package_id(),
    )?;

    let response = client.execute_transaction(tx, gas_budget).await?;
    if let Some(IotaExecutionStatus::Failure { error }) = response.effects_execution_status() {
      return Err(Error::TransactionUnexpectedResponse(error));
    }

    let vp = identity
      .controller_voting_power(controller_cap.0)
      .expect("is identity's controller");
    *proposal.votes_mut() = proposal.votes() + vp;

    Ok(())
  }
}
