use async_trait::async_trait;
use crate::iota_sdk_abstraction::{IdentityMoveCallsCore, ProgrammableTransactionBcs};
use secret_storage::Signer;

use crate::client::IdentityClient;
use crate::iota_sdk_abstraction::IotaKeySignature;
use crate::iota_sdk_abstraction::IotaClientTraitCore;
use crate::Error;

/// Interface for operations that interact with the ledger through transactions.
#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
pub trait Transaction: Sized {
    /// The result of performing the operation.
  type Output;

  /// Executes this operation using the given `client` and an optional `gas_budget`.
  /// If no value for `gas_budget` is provided, an estimated value will be used.
  async fn execute_with_opt_gas<S, C, M>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S, C, M>,
  ) -> Result<Self::Output, Error>
  where
      S: Signer<IotaKeySignature> + Sync,
      C: IotaClientTraitCore + Sync,
      M: IdentityMoveCallsCore + Sync + Send;

  /// Executes this operation using `client`.
  async fn execute<S, C, M>(
    self,
    client: &IdentityClient<S, C, M>,
  ) -> Result<Self::Output, Error>
  where
      S: Signer<IotaKeySignature> + Sync,
      C: IotaClientTraitCore + Sync,
      M: IdentityMoveCallsCore + Sync + Send,
  {
    self.execute_with_opt_gas(None, client).await
  }

  /// Executes this operation using `client` and a well defined `gas_budget`.
  async fn execute_with_gas<S, C, M>(
    self,
    gas_budget: u64,
    client: &IdentityClient<S, C, M>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
    C: IotaClientTraitCore + Sync,
    M: IdentityMoveCallsCore + Sync + Send,
  {
    self.execute_with_opt_gas(Some(gas_budget), client).await
  }
}

/// A [`Transaction`] that has no output.
#[derive(Debug)]
pub struct SimpleTransaction(pub ProgrammableTransactionBcs);

#[cfg_attr(not(feature = "send-sync-transaction"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-transaction", async_trait)]
impl Transaction for SimpleTransaction {
  type Output = ();
  async fn execute_with_opt_gas<S, C, M>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S, C, M>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
    C: IotaClientTraitCore + Sync,
    M: IdentityMoveCallsCore + Sync + Send,
  {
    client.execute_transaction(self.0, gas_budget).await?;
    Ok(())
  }
}
