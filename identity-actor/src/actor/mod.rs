// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[allow(clippy::module_inception)]
mod actor;
mod actor_builder;
mod actor_request;
mod config;
mod endpoint;
mod errors;
mod handler;
mod invocation;
mod request_context;
pub(crate) mod traits;

pub use actor::Actor;
pub(crate) use actor::HandlerObject;
pub use actor_builder::ActorBuilder;
pub use actor_builder::HandlerBuilder;
pub(crate) use actor_request::private::SyncMode;
pub use actor_request::ActorRequest;
pub use actor_request::Asynchronous;
pub use actor_request::RequestMode;
pub use actor_request::Synchronous;
pub(crate) use config::ActorConfig;
pub use endpoint::Endpoint;
pub use errors::Error;
pub use errors::RemoteSendError;
pub use errors::Result;
pub use handler::Handler;
pub(crate) use invocation::AsynchronousInvocationStrategy;
pub(crate) use invocation::InvocationStrategy;
pub(crate) use invocation::SynchronousInvocationStrategy;
pub use request_context::RequestContext;
pub(crate) use traits::RequestHandler;
