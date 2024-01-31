pub mod jwt;
pub mod revocation;

use identity_stronghold::StrongholdStorage;
use iota_sdk::client::Client;
use tonic::transport::server::RoutesBuilder;

pub fn init_services(routes: &mut RoutesBuilder, client: &Client, stronghold: &StrongholdStorage) {
  routes.add_service(revocation::service(client));
  routes.add_service(jwt::service(client, stronghold));
}
