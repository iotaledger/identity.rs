mod health_check;
mod credential_verification;

use tonic::transport::server::{Routes, RoutesBuilder};

pub fn routes() -> Routes {
  let mut routes = RoutesBuilder::default();
  routes.add_service(health_check::service());
  routes.add_service(credential_verification::service());

  routes.routes()
}
