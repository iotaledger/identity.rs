use std::ops::Deref;

use async_trait::async_trait;
use iota_sdk::rpc_types::IotaTransactionBlockResponse;
use iota_sdk::types::transaction::ProgrammableTransaction;
use secret_storage::Signer;

use crate::rebased::client::IdentityClient;
use crate::rebased::client::IotaKeySignature;
use crate::rebased::Error;

/// The output type of a [`Transaction`].
#[derive(Debug, Clone)]
pub struct TransactionOutput<T> {
  /// The parsed Transaction output. See [`Transaction::Output`].
  pub output: T,
  /// The "raw" transaction execution response received.
  pub response: IotaTransactionBlockResponse,
}

impl<T> Deref for TransactionOutput<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.output
  }
}

/// Interface for operations that interact with the ledger through transactions.
#[async_trait]
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

#[async_trait]
impl Transaction for ProgrammableTransaction {
  type Output = ();
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<TransactionOutput<Self::Output>, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    let response = client.execute_transaction(self, gas_budget).await?;
    Ok(TransactionOutput { output: (), response })
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
  T: Transaction,
{
  type Input = ();
  type Tx = Self;

  fn with(self, _: Self::Input) -> Self::Tx {
    self
  }
}
