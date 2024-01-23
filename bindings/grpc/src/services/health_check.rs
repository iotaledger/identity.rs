use health_check::health_check_server::HealthCheck;
use health_check::health_check_server::HealthCheckServer;
use health_check::HealthCheckRequest;
use health_check::HealthCheckResponse;
use tonic::Request;
use tonic::Response;
use tonic::Status;

#[allow(clippy::module_inception)]
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
