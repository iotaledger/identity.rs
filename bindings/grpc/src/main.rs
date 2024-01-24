use anyhow::Context;
use identity_grpc::server::GRpcServer;
use iota_sdk::client::Client;

#[tokio::main]
#[tracing::instrument(err)]
async fn main() -> anyhow::Result<()> {
  tracing::subscriber::set_global_default(tracing_subscriber::fmt().compact().finish())
    .expect("Failed to setup global tracing subscriber.");

  let api_endpoint = std::env::var("API_ENDPOINT").context("Missing environmental variable API_ENDPOINT")?;

  let client: Client = Client::builder()
    .with_primary_node(&api_endpoint, None)?
    .finish()
    .await?;

  let addr = "0.0.0.0:50051".parse()?;
  tracing::info!("gRPC server listening on {}", addr);
  GRpcServer::new(client).serve(addr).await?;

  Ok(())
}
