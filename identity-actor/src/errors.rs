use communication_refactored::{ListenErr, TransportErr};
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
  IoError(#[from] std::io::Error),
  #[error("Multiaddr {0} is not supported")]
  MultiaddrNotSupported(Multiaddr),
  #[error("could not respond to a {0} request, due to the handler taking too long to produce a response, the connection timing out or a transport error.")]
  CouldNotRespond(String),
  #[error("the actor was shut down")]
  Shutdown,
}

impl From<ListenErr> for Error {
  fn from(err: ListenErr) -> Self {
    match err {
      ListenErr::Shutdown => Error::Shutdown,
      ListenErr::Transport(TransportErr::Io(io_err)) => Error::IoError(io_err),
      ListenErr::Transport(TransportErr::MultiaddrNotSupported(addr)) => Error::MultiaddrNotSupported(addr),
    }
  }
}
