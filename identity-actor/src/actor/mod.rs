// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[allow(clippy::module_inception)]
mod actor;
mod actor_builder;
pub(crate) mod actor_request;
mod config;
mod endpoint;
mod errors;
mod generic_actor;
mod handler;
mod invocation;
mod request_context;
pub(crate) mod traits;

pub use actor::Actor;
pub use actor::ActorStateExtension;
pub(crate) use actor::HandlerMap;
pub(crate) use actor::HandlerObject;
pub(crate) use actor::ObjectId;
pub(crate) use actor::ObjectMap;
pub use actor_builder::ActorBuilder;
pub use actor_builder::HandlerBuilder;
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
pub use generic_actor::GenericActor;
pub(crate) use handler::Handler;
pub(crate) use invocation::send_response;
pub(crate) use invocation::SynchronousInvocationStrategy;
pub use request_context::RequestContext;
pub(crate) use traits::RequestHandler;
