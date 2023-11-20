use std::net::SocketAddr;
use tokio::{net::TcpListener, task::JoinHandle};
use tonic::transport::Uri;

#[derive(Debug)]
pub struct TestServer {
  addr: SocketAddr,
  _handle: JoinHandle<Result<(), tonic::transport::Error>>,
}

impl TestServer {
  pub async fn new() -> Self {
    let listener = TcpListener::bind("127.0.0.1:0")
      .await
      .expect("Failed to bind to random OS's port");
    let addr = listener.local_addr().unwrap();

    let server = identity_grpc::server::make_server()
      .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener));
    TestServer {
      _handle: tokio::spawn(server),
      addr,
    }
  }

  pub fn endpoint(&self) -> Uri {
    format!("https://{}", self.addr)
      .parse()
      .expect("Failed to parse server's URI")
  }
}
