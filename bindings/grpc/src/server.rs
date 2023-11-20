use tonic::transport::server::{Router, Server};

use crate::services;

pub fn make_server() -> Router {
  Server::builder().add_routes(services::routes())
}
