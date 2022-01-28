// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::request_response::OutboundFailure;
use libp2p::Multiaddr;

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
  #[error("invalid endpoint")]
  InvalidEndpoint,
  #[error("{0}")]
  OutboundFailure(#[from] OutboundFailure),
  /// No handler was set on the receiver and thus we cannot process this request.
  #[error("unkown request: `{0}`")]
  UnknownRequest(String),
  #[error("could not invoke the handler: {0}")]
  HandlerInvocationError(String),
  #[error("hook invocation error: {0}")]
  HookInvocationError(String),
  #[error("failed to deserialize: {0}")]
  DeserializationFailure(String),
}

// impl From<ListenErr> for Error {
//   fn from(err: ListenErr) -> Self {
//     match err {
//       ListenErr::Shutdown => Error::Shutdown,
//       ListenErr::Transport(TransportErr::Io(io_err)) => Error::IoError(io_err),
//       ListenErr::Transport(TransportErr::MultiaddrNotSupported(addr)) => Error::MultiaddrNotSupported(addr),
//     }
//   }
// }

/// Errors that can occur on the remote actor during [Actor::send_request] calls.
#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum RemoteSendError {
  /// No handler was set on the receiver and thus this request is not processable.
  #[error("unkown request: `{0}`")]
  UnknownRequest(String),
  #[error("could not invoke the handler: {0}")]
  HandlerInvocationError(String),
  #[error("hook invocation error: {0}")]
  HookInvocationError(String),
  #[error("failed to deserialize: {0}")]
  DeserializationFailure(String),
}

impl From<RemoteSendError> for Error {
  fn from(err: RemoteSendError) -> Self {
    match err {
      RemoteSendError::UnknownRequest(req) => Error::UnknownRequest(req),
      RemoteSendError::HandlerInvocationError(err) => Error::HandlerInvocationError(err),
      RemoteSendError::HookInvocationError(err) => Error::HookInvocationError(err),
      RemoteSendError::DeserializationFailure(err) => Error::DeserializationFailure(err),
    }
  }
}

impl From<serde_json::Error> for RemoteSendError {
  fn from(err: serde_json::Error) -> Self {
    Self::DeserializationFailure(err.to_string())
  }
}

/// Categories that errors can be classified in, to learn about where the
/// error originated from.
pub enum Category {
  /// An error that the client is responsible for.
  Client,
  /// An error that the peer is responsible for.
  Remote,
}
