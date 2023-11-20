use health_check::{
  health_check_server::{HealthCheck, HealthCheckServer},
  HealthCheckRequest, HealthCheckResponse,
};
use tonic::{Request, Response, Status};

mod health_check {
  tonic::include_proto!("health_check");
}

#[derive(Debug, Default)]
pub struct HealthChecker {}

#[tonic::async_trait]
impl HealthCheck for HealthChecker {
  async fn check(&self, _req: Request<HealthCheckRequest>) -> Result<Response<HealthCheckResponse>, Status> {
    Ok(Response::new(HealthCheckResponse { status: "OK".into() }))
  }
}

pub fn service() -> HealthCheckServer<HealthChecker> {
  HealthCheckServer::new(HealthChecker::default())
}
