use std::net::SocketAddr;

use iota_sdk::client::Client;
use tonic::transport::server::{Router, Server};

use crate::services;

#[derive(Debug)]
pub struct GRpcServer {
  router: Router,
}

impl GRpcServer {
  pub fn new(client: Client) -> Self {
    let router = Server::builder().add_routes(services::routes(client));
    Self { router }
  }
  pub async fn serve(self, addr: SocketAddr) -> Result<(), tonic::transport::Error> {
    self.router.serve(addr).await
  }
  pub fn into_router(self) -> Router {
    self.router
  }
}
