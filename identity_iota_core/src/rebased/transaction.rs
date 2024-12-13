// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;

#[cfg(not(target_arch = "wasm32"))]
use identity_iota_interaction::rpc_types::IotaTransactionBlockResponse;
#[cfg(not(target_arch = "wasm32"))]
use identity_iota_interaction::types::transaction::{ProgrammableTransaction};
#[cfg(target_arch = "wasm32")]
use iota_interaction_ts::ProgrammableTransaction;

use async_trait::async_trait;
use identity_iota_interaction::{IotaKeySignature, ProgrammableTransactionBcs};
use crate::iota_interaction_adapter::IotaTransactionBlockResponseAdaptedTraitObj;
use secret_storage::Signer;
use crate::rebased::client::IdentityClient;
use crate::rebased::Error;

/// The output type of a [`Transaction`].
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct TransactionOutput<T> {
  /// The parsed Transaction output. See [`Transaction::Output`].
  pub output: T,
  /// The "raw" transaction execution response received.
  pub response: IotaTransactionBlockResponse,
}

#[cfg(not(target_arch = "wasm32"))]
impl<T> Deref for TransactionOutput<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.output
  }
}

/// Interface for operations that interact with the ledger through transactions.
#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Transaction: Sized {
  /// The result of performing the operation.
  type Output;

  /// Executes this operation using the given `client` and an optional `gas_budget`.
  /// If no value for `gas_budget` is provided, an estimated value will be used.
  async fn execute_with_opt_gas<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutput<Self::Output>, Error>;

  /// Executes this operation using `client`.
  async fn execute<S: Signer<IotaKeySignature> + Sync>(
    self,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutput<Self::Output>, Error> {
    self.execute_with_opt_gas(None, client).await
  }

  /// Executes this operation using `client` and a well defined `gas_budget`.
  async fn execute_with_gas<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: u64,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutput<Self::Output>, Error> {
    self.execute_with_opt_gas(Some(gas_budget), client).await
  }
}

pub(crate) struct TransactionOutputInternal<T> {
  pub output: T,
  pub response: IotaTransactionBlockResponseAdaptedTraitObj,
}

impl<T> Deref for TransactionOutputInternal<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.output
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T> From<TransactionOutputInternal<T>> for TransactionOutput<T> {
  fn from(value: TransactionOutputInternal<T>) -> Self {
    let response_bcs = value
        .response
        .to_bcs()
        .expect("TransactionOutputInternal bcs serialization failed");
    let response =
        bcs::from_bytes::<IotaTransactionBlockResponse>(&response_bcs)
            .expect("IotaTransactionBlockResponse bcs deserialization failed");
    TransactionOutput {
      output: value.output,
      response,
    }
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub(crate) trait TransactionInternal: Sized {
  type Output;

  async fn execute_with_opt_gas_internal<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error>;

  #[cfg(target_arch = "wasm32")]
  async fn execute<S: Signer<IotaKeySignature> + Sync>(
    self,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error> {
    self.execute_with_opt_gas_internal(None, client).await
  }

  #[cfg(target_arch = "wasm32")]
  async fn execute_with_gas<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: u64,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error> {
    self.execute_with_opt_gas_internal(Some(gas_budget), client).await
  }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T: TransactionInternal<Output=O> + Send, O> Transaction for T {
  type Output = O;

  async fn execute_with_opt_gas<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutput<O>, Error> {
    let tx_output = self.execute_with_opt_gas_internal(gas_budget, client).await?;
    Ok(tx_output.into())
  }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TransactionInternal for ProgrammableTransaction {
  type Output = ();
  async fn execute_with_opt_gas_internal<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let tx_bcs = bcs::to_bytes(&self)?;
    let response = client.execute_transaction(tx_bcs, gas_budget).await?;
    Ok(TransactionOutputInternal { output: (), response })
  }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TransactionInternal for ProgrammableTransaction {
  type Output = ();
  async fn execute_with_opt_gas_internal<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutputInternal<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    unimplemented!("TransactionInternal::execute_with_opt_gas_internal for ProgrammableTransaction");
  }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TransactionInternal for ProgrammableTransactionBcs {
  type Output = ();

  async fn execute_with_opt_gas_internal<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>
  ) -> Result<TransactionOutputInternal<Self::Output>, Error> {
    // For wasm32 targets, the following line will result in a compiler error[E0412]
    // TODO: Implement wasm-bindings for the ProgrammableTransaction TS equivalent
    //       and use them to do the BCS serialization
    let self_tx = bcs::from_bytes::<ProgrammableTransaction>(&self)?;
    self_tx
        .execute_with_opt_gas_internal(gas_budget, client)
        .await
  }
}

/// Interface to describe an operation that can eventually
/// be turned into a [`Transaction`], given the right input.
pub trait ProtoTransaction {
  /// The input required by this operation.
  type Input;
  /// This operation's next state. Can either be another [`ProtoTransaction`]
  /// or a whole [`Transaction`] ready to be executed.
  type Tx: ProtoTransaction;

  /// Feed this operation with its required input, advancing its
  /// state to another [`ProtoTransaction`] that may or may not
  /// be ready for execution.
  fn with(self, input: Self::Input) -> Self::Tx;
}

// Every Transaction is a QuasiTransaction that requires no input
// and that has itself as its next state.
impl<T> ProtoTransaction for T
where
  T: TransactionInternal,
{
  type Input = ();
  type Tx = Self;

  fn with(self, _: Self::Input) -> Self::Tx {
    self
  }
}
