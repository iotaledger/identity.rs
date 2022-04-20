// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

use futures::AsyncRead;
use futures::AsyncWrite;
use futures::Future;
use futures::FutureExt;
use libp2p::core::Executor;
use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use libp2p::Transport;

use crate::actor::Actor;
use crate::actor::ActorBuilder;
use crate::actor::ActorRequest;
use crate::actor::ActorState;
use crate::actor::Asynchronous;
use crate::actor::Error;
use crate::actor::Handler;
use crate::actor::ObjectId;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::SyncHandlerMap;
use crate::actor::SyncHandlerObject;
use crate::actor::SyncMode;
use crate::actor::Synchronous;
use crate::didcomm::AsyncHandlerMap;
use crate::didcomm::AsyncHandlerObject;
use crate::p2p::EventLoop;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;

use super::didcomm_actor::DidActorCommState;
use super::didcomm_actor::DidCommActor;
use super::ActorIdentity;
use super::DidCommPlaintextMessage;

pub struct DidCommActorBuilder {
  inner: ActorBuilder,
  async_handlers: AsyncHandlerMap,
  identity: Option<ActorIdentity>,
}

impl DidCommActorBuilder {
  pub fn new() -> DidCommActorBuilder {
    Self {
      inner: ActorBuilder::new(),
      identity: None,
      async_handlers: HashMap::new(),
    }
  }

  /// See [`ActorBuilder::keypair`].
  #[must_use]
  pub fn keypair(mut self, keypair: Keypair) -> Self {
    self.inner.keypair = Some(keypair);
    self
  }

  /// See [`ActorBuilder::listen_on`].
  #[must_use]
  pub fn listen_on(mut self, address: Multiaddr) -> Self {
    self.inner.listening_addresses.push(address);
    self
  }

  /// Sets the timeout for [`DidCommActor::await_message`] and the underlying libp2p
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

  /// See [`ActorBuilder::add_state`].
  pub fn add_state<MOD, OBJ>(&mut self, state_object: OBJ) -> DidCommHandlerBuilder<MOD, OBJ>
  where
    OBJ: Clone + Send + Sync + 'static,
    MOD: SyncMode,
  {
    let object_id: ObjectId = ObjectId::new_v4();
    self.inner.objects.insert(object_id, Box::new(state_object));
    DidCommHandlerBuilder {
      object_id,
      sync_handlers: &mut self.inner.handlers,
      async_handlers: &mut self.async_handlers,
      _marker_obj: PhantomData,
      _marker_mod: PhantomData,
    }
  }

  /// See [`ActorBuilder::build`].
  #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
  pub async fn build(self) -> ActorResult<DidCommActor> {
    let dns_transport =
      libp2p::dns::TokioDnsConfig::system(libp2p::tcp::TokioTcpConfig::new()).map_err(|err| Error::TransportError {
        context: "unable to build transport",
        source: libp2p::TransportError::Other(err),
      })?;

    let transport = dns_transport
      .clone()
      .or_transport(libp2p::websocket::WsConfig::new(dns_transport));

    self.build_with_transport(transport).await
  }

  /// See [`ActorBuilder::build_with_transport`].
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> ActorResult<DidCommActor>
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

    let (event_loop, actor_state, net_commander): (EventLoop, ActorState, NetCommander) =
      self.inner.build_actor_constituents(transport, executor.clone()).await?;

    let state: DidActorCommState = DidActorCommState::new(
      actor_state,
      self.async_handlers,
      self.identity.ok_or(Error::IdentityMissing)?,
    );

    let didcomm_actor: DidCommActor = DidCommActor {
      net_commander,
      state: Arc::new(state),
    };

    let didcomm_actor_clone: DidCommActor = didcomm_actor.clone();

    let event_handler = move |event: InboundRequest| {
      didcomm_actor_clone.clone().handle_request(event);
    };

    executor.exec(event_loop.run(event_handler).boxed());

    Ok(didcomm_actor)
  }
}

impl Default for DidCommActorBuilder {
  fn default() -> Self {
    Self::new()
  }
}

/// Used to attach handlers and hooks to an [`DidCommActorBuilder`].
pub struct DidCommHandlerBuilder<'builder, MOD, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
  MOD: SyncMode + 'static,
{
  pub(crate) object_id: ObjectId,
  pub(crate) sync_handlers: &'builder mut SyncHandlerMap,
  pub(crate) async_handlers: &'builder mut AsyncHandlerMap,
  pub(crate) _marker_obj: PhantomData<&'static OBJ>,
  pub(crate) _marker_mod: PhantomData<&'static MOD>,
}

impl<'builder, OBJ> DidCommHandlerBuilder<'builder, Asynchronous, OBJ>
where
  OBJ: Clone + Send + Sync + 'static,
{
  /// Add an asynchronous handler function that operates on a shared state object and some
  /// [`ActorRequest`]. The function will be called if the actor receives a request
  /// on the given `endpoint` and can deserialize it into `DidCommPlaintextMessage<REQ>`.
  /// The handler is not expected to return anything.
  pub fn add_async_handler<REQ, FUT>(
    self,
    handler: fn(OBJ, DidCommActor, RequestContext<DidCommPlaintextMessage<REQ>>) -> FUT,
  ) -> Self
  where
    REQ: ActorRequest<Asynchronous> + Sync,
    FUT: Future<Output = ()> + Send + 'static,
  {
    let handler: Handler<_, DidCommActor, _, _, _> = Handler::new(handler);
    self.async_handlers.insert(
      REQ::endpoint(),
      AsyncHandlerObject::new(self.object_id, Box::new(handler)),
    );
    self
  }

  /// Add a synchronous handler function that operates on a shared state object and some
  /// [`ActorRequest`]. The function will be called if the actor receives a request
  /// on the given `endpoint` and can deserialize it into `REQ`. The handler is expected
  /// to return an instance of `REQ::Response`.
  pub fn add_sync_handler<REQ, FUT>(self, handler: fn(OBJ, Actor, RequestContext<REQ>) -> FUT) -> Self
  where
    REQ: ActorRequest<Synchronous> + Sync,
    REQ::Response: Send,
    FUT: Future<Output = REQ::Response> + Send + 'static,
  {
    let handler = Handler::new(handler);
    self.sync_handlers.insert(
      REQ::endpoint(),
      SyncHandlerObject::new(self.object_id, Box::new(handler)),
    );
    self
  }
}
