use identity_grpc::server::GRpcServer;
use iota_sdk::client::Client;

const API_ENDPOINT: &str = "http://127.0.0.1:14265";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  let addr = "127.0.0.1:50051".parse()?;
  println!("gRPC server listening on {}", addr);
  GRpcServer::new(client).serve(addr).await?;

  Ok(())
}
