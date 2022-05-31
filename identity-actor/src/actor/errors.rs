// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::request_response::OutboundFailure;

use crate::didcomm::ThreadId;

/// The `Result` type for the actor.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during actor execution.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[non_exhaustive]
  #[error("transport error during {0}")]
  TransportError(&'static str, #[source] libp2p::TransportError<std::io::Error>),
  #[error("invalid endpoint")]
  InvalidEndpoint,
  #[non_exhaustive]
  #[error("failure during sending an outbound request and receiving the response")]
  OutboundFailure(#[source] OutboundFailure),
  #[error("unexpected request `{0}`")]
  UnexpectedRequest(String),
  #[error("handler invocation error: {0}")]
  HandlerInvocationError(String),
  #[error("hook invocation error: {0}")]
  HookInvocationError(String),
  #[non_exhaustive]
  #[error("{location} serialization failed during {context} due to: {error_message}")]
  SerializationFailure {
    location: ErrorLocation,
    context: String,
    error_message: String,
  },
  #[error("{location} deserialization failed during {context} due to: {error_message}")]
  DeserializationFailure {
    location: ErrorLocation,
    context: String,
    error_message: String,
  },
  #[error("thread with id `{0}` not found")]
  ThreadNotFound(ThreadId),
  #[error("awaiting message timed out on thread `{0}`")]
  AwaitTimeout(ThreadId),
  #[error("actor was shutdown")]
  Shutdown,
  #[error("actor identity missing")]
  IdentityMissing,
}

/// Errors that can occur on the remote actor.
#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum RemoteSendError {
  #[error("unexpected request: {0}")]
  UnexpectedRequest(String),
  #[error("handler invocation error: {0}")]
  HandlerInvocationError(String),
  #[error("hook invocation error: {0}")]
  HookInvocationError(String),
  #[error("{location} serialization failed during {context} due to: {error_message}")]
  SerializationFailure {
    location: ErrorLocation,
    context: String,
    error_message: String,
  },
  #[error("{location} deserialization failed during {context} due to: {error_message}")]
  DeserializationFailure {
    location: ErrorLocation,
    context: String,
    error_message: String,
  },
}

impl From<RemoteSendError> for Error {
  fn from(err: RemoteSendError) -> Self {
    match err {
      RemoteSendError::UnexpectedRequest(req) => Error::UnexpectedRequest(req),
      RemoteSendError::HandlerInvocationError(err) => Error::HandlerInvocationError(err),
      RemoteSendError::HookInvocationError(err) => Error::HookInvocationError(err),
      RemoteSendError::DeserializationFailure {
        location,
        context,
        error_message,
      } => Error::DeserializationFailure {
        location,
        context,
        error_message,
      },
      RemoteSendError::SerializationFailure {
        location,
        context,
        error_message,
      } => Error::SerializationFailure {
        location,
        context,
        error_message,
      },
    }
  }
}

/// A type indicating whether an error occured locally or remotely.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorLocation {
  /// The error occured locally.
  Local,
  /// The error occured remotely.
  Remote,
}

impl std::fmt::Display for ErrorLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let display = match self {
      ErrorLocation::Local => "local",
      ErrorLocation::Remote => "remote",
    };

    write!(f, "{}", display)
  }
}
