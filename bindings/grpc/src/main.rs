use anyhow::Context;
use identity_grpc::server::GRpcServer;
use iota_sdk::client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let api_endpoint = std::env::var("API_ENDPOINT").context("Missing environmental variable API_ENDPOINT")?;

  let client: Client = Client::builder()
    .with_primary_node(&api_endpoint, None)?
    .finish()
    .await?;

  let addr = "127.0.0.1:50051".parse()?;
  println!("gRPC server listening on {}", addr);
  GRpcServer::new(client).serve(addr).await?;

  Ok(())
}
