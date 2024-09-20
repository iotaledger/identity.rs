use super::Error;
use super::Resolver;
use super::Result;
use identity_did::CoreDID;
use identity_iota_core::Error as IdentityError;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::IotaIdentityClientExt;
use iota_sdk::client::node_api::error::Error as IotaApiError;
use iota_sdk::client::Client;
use iota_sdk::client::Error as SdkError;

impl Resolver<IotaDID> for Client {
  type Target = IotaDocument;
  async fn resolve(&self, did: &IotaDID) -> Result<Self::Target> {
    self.resolve_did(did).await.map_err(|e| match e {
      IdentityError::DIDResolutionError(SdkError::Node(IotaApiError::NotFound(_))) => Error::NotFound(did.to_string()),
      e => Error::Generic(e.into()),
    })
  }
}

impl Resolver<CoreDID> for Client {
  type Target = IotaDocument;
  async fn resolve(&self, did: &CoreDID) -> Result<Self::Target> {
    let iota_did = IotaDID::try_from(did.clone()).map_err(|e| Error::ParsingFailure(e.into()))?;
    self.resolve(&iota_did).await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Context;

  async fn get_iota_client() -> anyhow::Result<Client> {
    const API_ENDPOINT: &str = "https://api.stardust-mainnet.iotaledger.net";
    Client::builder()
      .with_primary_node(API_ENDPOINT, None)?
      .finish()
      .await
      .context("Failed to create client to iota mainnet")
  }

  #[tokio::test]
  async fn resolution_of_existing_doc_works() -> anyhow::Result<()> {
    let client = get_iota_client().await?;
    let did = "did:iota:0xf4d6f08f5a1b80dd578da7dc1b49c886d580acd4cf7d48119dfeb82b538ad88a".parse::<IotaDID>()?;

    assert!(client.resolve(&did).await.is_ok());

    Ok(())
  }

  #[tokio::test]
  async fn resolution_of_non_existing_doc_fails_with_not_found() -> anyhow::Result<()> {
    let client = get_iota_client().await?;
    let did = "did:iota:0xf4d6f08f5a1b80ee578da7dc1b49c886d580acd4cf7d48119dfeb82b538ad88a".parse::<IotaDID>()?;

    assert!(matches!(client.resolve(&did).await.unwrap_err(), Error::NotFound(_)));

    Ok(())
  }
}
