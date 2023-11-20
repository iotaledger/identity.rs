use anyhow::Result;
use tonic::transport::Server;

use identity_grpc::services;

#[tokio::main]
async fn main() -> Result<()> {
  let addr = "[::1]:50051".parse()?;

  println!("gRPC server listening on {}", addr);

  Server::builder().add_routes(services::routes()).serve(addr).await?;

  Ok(())
}
