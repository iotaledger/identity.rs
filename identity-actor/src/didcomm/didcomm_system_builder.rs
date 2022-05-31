// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use futures::AsyncRead;
use futures::AsyncWrite;
use futures::FutureExt;
use libp2p::core::Executor;
use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use libp2p::Transport;

use crate::actor::Actor;
use crate::actor::ActorRequest;
use crate::actor::Error;
use crate::actor::ObjectId;
use crate::actor::Result as ActorResult;
use crate::actor::System;
use crate::actor::SystemBuilder;
use crate::actor::SystemState;
use crate::didcomm::AbstractDidCommActor;
use crate::didcomm::ActorIdentity;
use crate::didcomm::AsyncHandlerMap;
use crate::didcomm::DidCommActor;
use crate::didcomm::DidCommActorMap;
use crate::didcomm::DidCommActorWrapper;
use crate::didcomm::DidCommRequest;
use crate::didcomm::DidCommSystem;
use crate::didcomm::DidCommSystemState;
use crate::p2p::EventLoop;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;

/// A builder for [`DidCommSystem`]s that allows for customizing its configuration and attaching actors.
pub struct DidCommSystemBuilder {
  inner: SystemBuilder,
  async_handlers: AsyncHandlerMap,
  identity: Option<ActorIdentity>,
  async_actors: DidCommActorMap,
}

impl DidCommSystemBuilder {
  /// Create a new builder in the default configuration.
  pub fn new() -> DidCommSystemBuilder {
    Self {
      inner: SystemBuilder::new(),
      identity: None,
      async_handlers: HashMap::new(),
      async_actors: HashMap::new(),
    }
  }

  /// See [`SystemBuilder::keypair`].
  #[must_use]
  pub fn keypair(mut self, keypair: Keypair) -> Self {
    self.inner.keypair = Some(keypair);
    self
  }

  /// See [`SystemBuilder::listen_on`].
  #[must_use]
  pub fn listen_on(mut self, address: Multiaddr) -> Self {
    self.inner.listening_addresses.push(address);
    self
  }

  /// Sets the timeout for [`DidCommSystem::await_message`] and the underlying libp2p
  /// [`RequestResponse`](libp2p::request_response::RequestResponse) protocol.
  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.inner.config.timeout = timeout;
    self
  }

  /// Set the [`ActorIdentity`] that will be used for DIDComm related tasks, such as en- and decryption.
  #[must_use]
  pub fn identity(mut self, identity: ActorIdentity) -> Self {
    self.identity = Some(identity);
    self
  }

  /// Attaches a [`DidCommActor`] to this system.
  ///
  /// This means that when the system receives a request of type `REQ`, it will invoke this actor.
  ///
  /// Calling this method multiple times with requests that have the same `Endpoint`
  /// will detach the previous actor.
  pub fn attach_didcomm<REQ, ACT>(&mut self, actor: ACT)
  where
    ACT: DidCommActor<REQ> + Send + Sync,
    REQ: DidCommRequest + Send + Sync,
  {
    self.async_actors.insert(
      REQ::endpoint(),
      Box::new(DidCommActorWrapper::new(actor)) as Box<dyn AbstractDidCommActor>,
    );
  }

  /// Attaches an [`Actor`] to this system.
  ///
  /// This means that when the system receives a request of type `REQ`, it will invoke this actor.
  ///
  /// Calling this method with a `REQ` type whose endpoint is already attached to an actor
  /// will overwrite the previous attachment.
  pub fn attach<REQ, ACT>(&mut self, actor: ACT)
  where
    ACT: Actor<REQ> + Send + Sync,
    REQ: ActorRequest + Send + Sync,
    REQ::Response: Send,
  {
    self.inner.attach(actor);
  }

  // TODO: Slated for removal.
  pub fn add_state<OBJ>(&mut self, state_object: OBJ) -> DidCommHandlerBuilder<OBJ>
  where
    OBJ: Clone + Send + Sync + 'static,
  {
    let object_id: ObjectId = ObjectId::new_v4();
    self.inner.objects.insert(object_id, Box::new(state_object));
    DidCommHandlerBuilder {
      object_id,
      async_handlers: &mut self.async_handlers,
      _marker_obj: PhantomData,
    }
  }

  /// See [`SystemBuilder::build`].
  #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
  pub async fn build(self) -> ActorResult<DidCommSystem> {
    let dns_transport = libp2p::dns::TokioDnsConfig::system(libp2p::tcp::TokioTcpConfig::new())
      .map_err(|err| Error::TransportError("building transport", libp2p::TransportError::Other(err)))?;

    let transport = dns_transport
      .clone()
      .or_transport(libp2p::websocket::WsConfig::new(dns_transport));

    self.build_with_transport(transport).await
  }

  /// See [`SystemBuilder::build_with_transport`].
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> ActorResult<DidCommSystem>
  where
    TRA: Transport + Sized + Clone + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let executor = Box::new(|fut| {
      cfg_if::cfg_if! {
        if #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))] {
          tokio::spawn(fut);
        } else {
          wasm_bindgen_futures::spawn_local(fut);
        }
      }
    });

    let (event_loop, actor_state, net_commander): (EventLoop, SystemState, NetCommander) =
      self.inner.build_actor_constituents(transport, executor.clone()).await?;

    let state: DidCommSystemState = DidCommSystemState::new(
      self.async_handlers,
      self.async_actors,
      self.identity.ok_or(Error::IdentityMissing)?,
    );

    let actor = System::new(net_commander, Arc::new(actor_state));

    let didcomm_actor: DidCommSystem = DidCommSystem {
      actor,
      state: Arc::new(state),
    };

    let didcomm_actor_clone: DidCommSystem = didcomm_actor.clone();

    let event_handler = move |event: InboundRequest| {
      didcomm_actor_clone.clone().handle_request(event);
    };

    executor.exec(event_loop.run(event_handler).boxed());

    Ok(didcomm_actor)
  }
}

impl Default for DidCommSystemBuilder {
  fn default() -> Self {
    Self::new()
  }
}

/// Used to attach handlers and hooks to a [`DidCommSystemBuilder`].
pub struct DidCommHandlerBuilder<'builder, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  pub(crate) object_id: ObjectId,
  pub(crate) async_handlers: &'builder mut AsyncHandlerMap,
  pub(crate) _marker_obj: PhantomData<&'static OBJ>,
}
