use async_trait::async_trait;
use crate::iota_sdk_abstraction::{IotaClientTrait, ProgrammableTransactionBcs};
use secret_storage::Signer;

use crate::client::IdentityClient;
use crate::client::IotaKeySignature;
use crate::Error;

#[async_trait]
pub trait Transaction: Sized {
  type Output;

  async fn execute_with_opt_gas<S, C>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S, C>,
  ) -> Result<Self::Output, Error>
  where
      S: Signer<IotaKeySignature> + Sync,
      C: IotaClientTrait<Error=Error> + Sync;
  
  async fn execute<S, C>(
    self,
    client: &IdentityClient<S, C>,
  ) -> Result<Self::Output, Error> 
  where
      S: Signer<IotaKeySignature> + Sync,
      C: IotaClientTrait<Error=Error> + Sync,
  {
    self.execute_with_opt_gas(None, client).await
  }
  
  async fn execute_with_gas<S, C>(
    self,
    gas_budget: u64,
    client: &IdentityClient<S, C>,
  ) -> Result<Self::Output, Error> 
  where
    S: Signer<IotaKeySignature> + Sync,  
    C: IotaClientTrait<Error=Error> + Sync,  
  {
    self.execute_with_opt_gas(Some(gas_budget), client).await
  }
}

#[derive(Debug)]
pub struct SimpleTransaction(pub ProgrammableTransactionBcs);

#[async_trait]
impl Transaction for SimpleTransaction {
  type Output = ();
  async fn execute_with_opt_gas<S, C>(
    self,
    gas_budget: Option<u64>,
    client: &IdentityClient<S, C>,
  ) -> Result<Self::Output, Error>
  where
    S: Signer<IotaKeySignature> + Sync,
    C: IotaClientTrait<Error=Error> + Sync,
  {
    client.execute_transaction(self.0, gas_budget).await?;
    Ok(())
  }
}
