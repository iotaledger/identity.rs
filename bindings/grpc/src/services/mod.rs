pub mod credential;
pub mod document;
pub mod health_check;
pub mod sd_jwt;

use identity_stronghold::StrongholdStorage;
use iota_sdk::client::Client;
use tonic::transport::server::Routes;
use tonic::transport::server::RoutesBuilder;

pub fn routes(client: &Client, stronghold: &StrongholdStorage) -> Routes {
  let mut routes = RoutesBuilder::default();
  routes.add_service(health_check::service());
  credential::init_services(&mut routes, client, stronghold);
  routes.add_service(sd_jwt::service(client));
  routes.add_service(document::service(client, stronghold));

  routes.routes()
}
