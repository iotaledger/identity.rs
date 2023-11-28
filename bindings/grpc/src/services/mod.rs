pub mod credential;
pub mod health_check;

use iota_sdk::client::Client;
use tonic::transport::server::{Routes, RoutesBuilder};

pub fn routes(client: Client) -> Routes {
  let mut routes = RoutesBuilder::default();
  routes.add_service(health_check::service());
  routes.add_service(credential::service(&client));

  routes.routes()
}
