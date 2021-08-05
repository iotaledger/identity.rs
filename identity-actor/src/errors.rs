use libp2p::Multiaddr;
use p2p::{ListenErr, TransportErr};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the identity actor crate.
pub enum Error {
  #[error("Lock In Use")]
  LockInUse,
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

/// Errors that can occur during [Actor::send_request] calls.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum SendError {
  #[error("{0}")]
  OutboundFailure(#[from] p2p::OutboundFailure),
  /// No handler was set on the receiver and thus we cannot process this request.
  #[error("unkown request: `{0}`")]
  UnknownRequest(String),
  #[error("failed to deserialize the response: {0}")]
  ResponseDeserializationFailure(String),
}

/// Errors that can occur on the remote actor during [Actor::send_request] calls.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RemoteSendError {
  /// No handler was set on the receiver and thus this request is not processable.
  UnknownRequest(String),
}

impl From<RemoteSendError> for SendError {
  fn from(err: RemoteSendError) -> Self {
    match err {
      RemoteSendError::UnknownRequest(req) => SendError::UnknownRequest(req),
    }
  }
}
