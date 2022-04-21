// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[allow(clippy::module_inception)]
mod actor;
mod actor_builder;
pub(crate) mod actor_request;
mod config;
mod endpoint;
mod errors;
mod handler;
mod invocation;
mod request_context;
pub(crate) mod traits;

pub use actor::Actor;
pub(crate) use actor::ActorState;
pub(crate) use actor::ObjectId;
pub(crate) use actor::SyncHandlerMap;
pub(crate) use actor::SyncHandlerObject;
pub use actor_builder::ActorBuilder;
pub use actor_builder::ActorHandlerBuilder;
pub use actor_request::ActorRequest;
pub use actor_request::Asynchronous;
pub use actor_request::RequestMode;
pub(crate) use actor_request::SyncMode;
pub use actor_request::Synchronous;
pub(crate) use config::ActorConfig;
pub use endpoint::Endpoint;
pub use errors::Error;
pub use errors::ErrorLocation;
pub use errors::RemoteSendError;
pub use errors::Result;
pub(crate) use handler::Handler;
pub(crate) use invocation::send_response;
pub(crate) use invocation::SynchronousInvocationStrategy;
pub use request_context::RequestContext;
pub(crate) use traits::request_handler_clone_object;
pub(crate) use traits::request_handler_deserialize_request;
pub(crate) use traits::request_handler_serialize_response;
pub(crate) use traits::AnyFuture;
pub(crate) use traits::SyncRequestHandler;
