mod health_check;

use tonic::transport::server::{Routes, RoutesBuilder};

pub fn routes() -> Routes {
  let mut routes = RoutesBuilder::default();
  routes.add_service(health_check::service());

  routes.routes()
}
