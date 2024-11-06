use async_trait::async_trait;
use iota_sdk::types::transaction::ProgrammableTransaction;
use secret_storage::Signer;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::Error;

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
  ) -> Result<Self::Output, Error>;

  /// Executes this operation using `client`.
  async fn execute<S: Signer<IotaKeySignature> + Sync>(
    self,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error> {
    self.execute_with_opt_gas(None, client).await
  }

  /// Executes this operation using `client` and a well defined `gas_budget`.
  async fn execute_with_gas<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: u64,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error> {
    self.execute_with_opt_gas(Some(gas_budget), client).await
  }
}

/// A [`Transaction`] that has no output.
#[derive(Debug)]
pub struct SimpleTransaction(pub ProgrammableTransaction);

#[async_trait]
impl Transaction for SimpleTransaction {
  type Output = ();
  async fn execute_with_opt_gas<S>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
  {
    client.execute_transaction(self.0, gas_budget).await?;
    Ok(())
  }
}
