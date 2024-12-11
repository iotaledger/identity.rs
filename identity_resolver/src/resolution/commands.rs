// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::future::Future;
use identity_did::DID;

use crate::Error;
use crate::ErrorCause;
use crate::Result;
use std::pin::Pin;

/// Internal trait used by the resolver to apply the command pattern.
///
/// The resolver is generic over the type of command which enables  
/// support for both multi-threaded and single threaded use cases.
pub trait Command<'a, T>: std::fmt::Debug + private::Sealed {
  type Output: Future<Output = T> + 'a;

  fn apply(&self, input: &'a str) -> Self::Output;
}

mod private {
  use super::SendSyncCommand;
  use super::SingleThreadedCommand;
  pub trait Sealed {}
  impl<DOC: 'static> Sealed for SendSyncCommand<DOC> {}
  impl<DOC: 'static> Sealed for SingleThreadedCommand<DOC> {}
}

impl<DOC: 'static> std::fmt::Debug for SendSyncCommand<DOC> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("<resolution_handler>")
  }
}

impl<DOC: 'static> std::fmt::Debug for SingleThreadedCommand<DOC> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("<resolution_handler>")
  }
}

/// Internal representation of a thread safe handler.
type SendSyncCallback<DOC> =
  Box<dyn for<'r> Fn(&'r str) -> Pin<Box<dyn Future<Output = Result<DOC>> + 'r + Send>> + Send + Sync>;

/// Wrapper around a thread safe callback.
pub struct SendSyncCommand<DOC: 'static> {
  fun: SendSyncCallback<DOC>,
}

impl<'a, DOC: 'static> Command<'a, Result<DOC>> for SendSyncCommand<DOC> {
  type Output = Pin<Box<dyn Future<Output = Result<DOC>> + 'a + Send>>;
  fn apply(&self, input: &'a str) -> Self::Output {
    (self.fun)(input)
  }
}

impl<DOC: 'static> SendSyncCommand<DOC> {
  /// Converts a handler represented as a closure to a command.
  ///
  /// This is achieved by first producing a callback represented as a dynamic asynchronous function pointer
  /// which is invoked by the [Resolver](crate::Resolver) at a later point.
  /// When the callback is invoked the `Resolver` will then pass a DID represented as a string slice which is then
  /// converted to the DID type required by the handler and then the handler is called.  
  pub(super) fn new<D, F, Fut, DOCUMENT, E, DIDERR>(handler: F) -> Self
  where
    D: DID + Send + for<'r> TryFrom<&'r str, Error = DIDERR> + 'static,
    DOCUMENT: 'static + Into<DOC>,
    F: Fn(D) -> Fut + 'static + Clone + Send + Sync,
    Fut: Future<Output = std::result::Result<DOCUMENT, E>> + Send,
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    DIDERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    let fun: SendSyncCallback<DOC> = Box::new(move |input: &str| {
      let handler_clone: F = handler.clone();
      let did_parse_attempt = D::try_from(input)
        .map_err(|error| ErrorCause::DIDParsingError { source: error.into() })
        .map_err(Error::new);

      Box::pin(async move {
        let did: D = did_parse_attempt?;
        handler_clone(did)
          .await
          .map(Into::into)
          .map_err(|error| ErrorCause::HandlerError { source: error.into() })
          .map_err(Error::new)
      })
    });

    Self { fun }
  }
}

// ===========================================================================
// Single threaded commands
// ===========================================================================

/// Internal representation of a single threaded handler.
pub(super) type SingleThreadedCallback<DOC> =
  Box<dyn for<'r> Fn(&'r str) -> Pin<Box<dyn Future<Output = Result<DOC>> + 'r>>>;

/// Wrapper around a single threaded callback.
pub struct SingleThreadedCommand<DOC> {
  fun: SingleThreadedCallback<DOC>,
}
impl<'a, DOC: 'static> Command<'a, Result<DOC>> for SingleThreadedCommand<DOC> {
  type Output = Pin<Box<dyn Future<Output = Result<DOC>> + 'a>>;
  fn apply(&self, input: &'a str) -> Self::Output {
    (self.fun)(input)
  }
}

impl<DOC: 'static> SingleThreadedCommand<DOC> {
  /// Equivalent to [`SendSyncCommand::new`](SendSyncCommand::new()), but with less `Send` + `Sync` bounds.
  pub(super) fn new<D, F, Fut, DOCUMENT, E, DIDERR>(handler: F) -> Self
  where
    D: DID + for<'r> TryFrom<&'r str, Error = DIDERR> + 'static,
    DOCUMENT: 'static + Into<DOC>,
    F: Fn(D) -> Fut + 'static + Clone,
    Fut: Future<Output = std::result::Result<DOCUMENT, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    DIDERR: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  {
    let fun: SingleThreadedCallback<DOC> = Box::new(move |input: &str| {
      let handler_clone: F = handler.clone();
      let did_parse_attempt = D::try_from(input)
        .map_err(|error| ErrorCause::DIDParsingError { source: error.into() })
        .map_err(Error::new);

      Box::pin(async move {
        let did: D = did_parse_attempt?;
        handler_clone(did)
          .await
          .map(Into::into)
          .map_err(|error| ErrorCause::HandlerError { source: error.into() })
          .map_err(Error::new)
      })
    });

    Self { fun }
  }
}
