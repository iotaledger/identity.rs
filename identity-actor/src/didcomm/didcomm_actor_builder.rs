// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;
use std::time::Duration;

use futures::AsyncRead;
use futures::AsyncWrite;
use futures::Future;
use libp2p::identity::Keypair;
use libp2p::Multiaddr;
use libp2p::Transport;

use crate::actor::ActorBuilder;
use crate::actor::ActorRequest;
use crate::actor::Asynchronous;
use crate::actor::Endpoint;
use crate::actor::Error;
use crate::actor::Handler;
use crate::actor::HandlerBuilder;
use crate::actor::HandlerObject;
use crate::actor::ObjectId;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::actor::SyncMode;

use super::didcomm_actor::ActorIdentity;
use super::didcomm_actor::DidCommActor;
use super::didcomm_actor::DidCommStateExtension;
use super::DidCommPlaintextMessage;

pub struct DidCommActorBuilder {
  inner: ActorBuilder,
  identity: Option<ActorIdentity>,
}

impl DidCommActorBuilder {
  pub fn new() -> DidCommActorBuilder {
    Self {
      inner: ActorBuilder::new(),
      identity: None,
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
  pub fn add_state<MOD, OBJ>(&mut self, state_object: OBJ) -> HandlerBuilder<MOD, OBJ>
  where
    OBJ: Clone + Send + Sync + 'static,
    MOD: SyncMode,
  {
    let object_id: ObjectId = ObjectId::new_v4();
    self.inner.objects.insert(object_id, Box::new(state_object));
    HandlerBuilder {
      object_id,
      handler_map: &mut self.inner.handlers,
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
    let extension: DidCommStateExtension = DidCommStateExtension::new(self.identity.ok_or(Error::IdentityMissing)?);
    self.inner.build_with_transport_and_ext(transport, extension).await
  }
}

impl Default for DidCommActorBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl<'builder, OBJ> HandlerBuilder<'builder, Asynchronous, OBJ>
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
  ) -> ActorResult<Self>
  where
    REQ: ActorRequest<Asynchronous> + Sync,
    FUT: Future<Output = ()> + Send + 'static,
  {
    let handler = Handler::new(handler);
    self.handler_map.insert(
      Endpoint::new(REQ::endpoint())?,
      HandlerObject::new(self.object_id, Box::new(handler)),
    );
    Ok(self)
  }
}
