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

use crate::agent::Agent;
use crate::agent::AgentBuilder;
use crate::agent::AgentState;
use crate::agent::Error;
use crate::agent::Handler;
use crate::agent::HandlerRequest;
use crate::agent::Result as AgentResult;
use crate::didcomm::AbstractDidCommHandler;
use crate::didcomm::DidCommAgent;
use crate::didcomm::DidCommAgentIdentity;
use crate::didcomm::DidCommAgentState;
use crate::didcomm::DidCommHandler;
use crate::didcomm::DidCommHandlerMap;
use crate::didcomm::DidCommHandlerWrapper;
use crate::didcomm::DidCommRequest;
use crate::p2p::EventLoop;
use crate::p2p::InboundRequest;
use crate::p2p::NetCommander;

/// A builder for [`DidCommAgent`]s to customize its configuration and attach handlers.
pub struct DidCommAgentBuilder {
  inner: AgentBuilder,
  identity: Option<DidCommAgentIdentity>,
  didcomm_handlers: DidCommHandlerMap,
}

impl DidCommAgentBuilder {
  /// Create a new builder with the default configuration.
  pub fn new() -> DidCommAgentBuilder {
    Self {
      inner: AgentBuilder::new(),
      identity: None,
      didcomm_handlers: HashMap::new(),
    }
  }

  /// See [`AgentBuilder::keypair`].
  #[must_use]
  pub fn keypair(mut self, keypair: Keypair) -> Self {
    self.inner.keypair = Some(keypair);
    self
  }

  /// Sets the timeout for [`DidCommAgent::await_message`] and the underlying libp2p
  /// [`RequestResponse`](libp2p::request_response::RequestResponse) protocol.
  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.inner.config.timeout = timeout;
    self
  }

  /// Set the [`DidCommAgentIdentity`] that will be used for DIDComm related tasks, such as en- and decryption.
  #[must_use]
  pub fn identity(mut self, identity: DidCommAgentIdentity) -> Self {
    self.identity = Some(identity);
    self
  }

  /// Attaches a [`DidCommHandler`] to this agent.
  ///
  /// This means that when the agent receives a request of type `REQ`, it will invoke this handler.
  ///
  /// Calling this method with a `REQ` type whose endpoint is already attached to a handler
  /// will overwrite the previous attachment.
  pub fn attach_didcomm<REQ, HND>(&mut self, handler: HND)
  where
    HND: DidCommHandler<REQ> + Send + Sync,
    REQ: DidCommRequest + Send + Sync,
  {
    self.didcomm_handlers.insert(
      REQ::endpoint(),
      Box::new(DidCommHandlerWrapper::new(handler)) as Box<dyn AbstractDidCommHandler>,
    );
  }

  /// See [`AgentBuilder::attach`].
  pub fn attach<REQ, HND>(&mut self, handler: HND)
  where
    HND: Handler<REQ> + Send + Sync,
    REQ: HandlerRequest + Send + Sync,
    REQ::Response: Send,
  {
    self.inner.attach(handler);
  }

  /// See [`AgentBuilder::build`].
  pub async fn build(self) -> AgentResult<DidCommAgent> {
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

  /// See [`AgentBuilder::build_with_transport`].
  pub async fn build_with_transport<TRA>(self, transport: TRA) -> AgentResult<DidCommAgent>
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

    let (event_loop, handler_state, net_commander): (EventLoop, AgentState, NetCommander) =
      self.inner.build_constituents(transport, executor.clone()).await?;

    let state: DidCommAgentState =
      DidCommAgentState::new(self.didcomm_handlers, self.identity.ok_or(Error::IdentityMissing)?);

    let agent: Agent = Agent::new(net_commander, Arc::new(handler_state));

    let didcomm_agent: DidCommAgent = DidCommAgent {
      agent,
      state: Arc::new(state),
    };

    let didcomm_agent_clone: DidCommAgent = didcomm_agent.clone();

    let event_handler = move |event: InboundRequest| {
      didcomm_agent_clone.clone().handle_request(event);
    };

    executor.exec(event_loop.run(event_handler).boxed());

    Ok(didcomm_agent)
  }
}

impl Default for DidCommAgentBuilder {
  fn default() -> Self {
    Self::new()
  }
}
