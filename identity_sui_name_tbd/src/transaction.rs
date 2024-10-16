use async_trait::async_trait;
use crate::iota_sdk_abstraction::ProgrammableTransactionBcs;
use secret_storage::Signer;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::Error;

#[async_trait]
pub trait Transaction: Sized {
  type Output;

  async fn execute_with_opt_gas<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error>;
  async fn execute<S: Signer<IotaKeySignature> + Sync>(
    self,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error> {
    self.execute_with_opt_gas(None, client).await
  }
  async fn execute_with_gas<S: Signer<IotaKeySignature> + Sync>(
    self,
    gas_budget: u64,
    client: &IdentityClient<S>,
  ) -> Result<Self::Output, Error> {
    self.execute_with_opt_gas(Some(gas_budget), client).await
  }
}

#[derive(Debug)]
pub struct SimpleTransaction(pub ProgrammableTransactionBcs);

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