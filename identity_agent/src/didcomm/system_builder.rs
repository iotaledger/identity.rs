// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use futures::AsyncRead;
use futures::AsyncWrite;
use futures::FutureExt;
use libp2p::core::Executor;
use libp2p::dns::TokioDnsConfig;
use libp2p::identity::Keypair;
use libp2p::tcp::TokioTcpConfig;
use libp2p::websocket::WsConfig;
use libp2p::Transport;

use crate::agent::Actor;
use crate::agent::ActorRequest;
use crate::agent::Error;
use crate::agent::Result as AgentResult;
use crate::agent::System;
use crate::agent::SystemBuilder;
use crate::agent::SystemState;
use crate::didcomm::AbstractDidCommActor;
use crate::didcomm::DidCommActor;
use crate::didcomm::DidCommActorMap;
use crate::didcomm::DidCommActorWrapper;
use crate::didcomm::DidCommRequest;
use crate::didcomm::DidCommSystem;
use crate::didcomm::DidCommSystemIdentity;
use crate::didcomm::DidCommSystemState;
use crate::p2p::EventLoop;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;

/// A builder for [`DidCommSystem`]s to customize its configuration and attach actors.
pub struct DidCommSystemBuilder {
  inner: SystemBuilder,
  identity: Option<DidCommSystemIdentity>,
  didcomm_actors: DidCommActorMap,
}

impl DidCommSystemBuilder {
  /// Create a new builder with the default configuration.
  pub fn new() -> DidCommSystemBuilder {
    Self {
      inner: SystemBuilder::new(),
      identity: None,
      didcomm_actors: HashMap::new(),
    }
  }

  /// See [`SystemBuilder::keypair`].
  #[must_use]
  pub fn keypair(mut self, keypair: Keypair) -> Self {
    self.inner.keypair = Some(keypair);
    self
  }

  /// Sets the timeout for [`DidCommSystem::await_message`] and the underlying libp2p
  /// [`RequestResponse`](libp2p::request_response::RequestResponse) protocol.
  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.inner.config.timeout = timeout;
    self
  }

  /// Set the [`DidCommSystemIdentity`] that will be used for DIDComm related tasks, such as en- and decryption.
  #[must_use]
  pub fn identity(mut self, identity: DidCommSystemIdentity) -> Self {
    self.identity = Some(identity);
    self
  }

  /// Attaches a [`DidCommActor`] to this system.
  ///
  /// This means that when the system receives a request of type `REQ`, it will invoke this actor.
  ///
  /// Calling this method with a `REQ` type whose endpoint is already attached to an actor
  /// will overwrite the previous attachment.
  pub fn attach_didcomm<REQ, ACT>(&mut self, actor: ACT)
  where
    ACT: DidCommActor<REQ> + Send + Sync,
    REQ: DidCommRequest + Send + Sync,
  {
    self.didcomm_actors.insert(
      REQ::endpoint(),
      Box::new(DidCommActorWrapper::new(actor)) as Box<dyn AbstractDidCommActor>,
    );
  }

  /// See [`SystemBuilder::attach`].
  pub fn attach<REQ, ACT>(&mut self, actor: ACT)
  where
    ACT: Actor<REQ> + Send + Sync,
    REQ: ActorRequest + Send + Sync,
    REQ::Response: Send,
  {
    self.inner.attach(actor);
  }

  /// See [`SystemBuilder::build`].
  pub async fn build(self) -> AgentResult<DidCommSystem> {
    let transport: _ = {
      let dns_tcp_transport: TokioDnsConfig<_> = TokioDnsConfig::system(TokioTcpConfig::new().nodelay(true))
        .map_err(|err| Error::TransportError("building transport", libp2p::TransportError::Other(err)))?;
      let ws_transport: WsConfig<_> = WsConfig::new(
        TokioDnsConfig::system(TokioTcpConfig::new().nodelay(true))
          .map_err(|err| Error::TransportError("building transport", libp2p::TransportError::Other(err)))?,
      );
      dns_tcp_transport.or_transport(ws_transport)
    };

    self.build_with_transport(transport).await
  }

  /// See [`SystemBuilder::build_with_transport`].
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> AgentResult<DidCommSystem>
  where
    TRA: Transport + Sized + Send + Sync + 'static,
    TRA::Output: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    TRA::Dial: Send + 'static,
    TRA::Listener: Send + 'static,
    TRA::ListenerUpgrade: Send + 'static,
    TRA::Error: Send + Sync,
  {
    let executor = Box::new(|fut| {
      tokio::spawn(fut);
    });

    let (event_loop, actor_state, net_commander): (EventLoop, SystemState, NetCommander) =
      self.inner.build_constituents(transport, executor.clone()).await?;

    let state: DidCommSystemState =
      DidCommSystemState::new(self.didcomm_actors, self.identity.ok_or(Error::IdentityMissing)?);

    let system: System = System::new(net_commander, Arc::new(actor_state));

    let didcomm_system: DidCommSystem = DidCommSystem {
      system,
      state: Arc::new(state),
    };

    let didcomm_system_clone: DidCommSystem = didcomm_system.clone();

    let event_handler = move |event: InboundRequest| {
      didcomm_system_clone.clone().handle_request(event);
    };

    executor.exec(event_loop.run(event_handler).boxed());

    Ok(didcomm_system)
  }
}

impl Default for DidCommSystemBuilder {
  fn default() -> Self {
    Self::new()
  }
}
