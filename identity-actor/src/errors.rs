use communication_refactored::TransportErr;
use libp2p::Multiaddr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the identity actor crate.
pub enum Error {
  #[error("Lock In Use")]
  LockInUse,
  #[error("{0}")]
  OutboundFailure(#[from] communication_refactored::OutboundFailure),
  #[error("Unkown Request {0}")]
  UnknownRequest(String),
  #[error("IoError: {0}")]
  IoError(std::io::Error),
  #[error("Multiaddr {0} is not supported")]
  MultiaddrNotSupported(Multiaddr),
  #[error("could not respond to a {0} request, due to the handler taking too long to produce a response, the connection timing out or a transport error.")]
  CouldNotRespond(String),
}

impl From<TransportErr> for Error {
  fn from(err: TransportErr) -> Self {
    match err {
      TransportErr::Io(io_err) => Self::IoError(io_err),
      TransportErr::MultiaddrNotSupported(addr) => Self::MultiaddrNotSupported(addr),
    }
  }
}
