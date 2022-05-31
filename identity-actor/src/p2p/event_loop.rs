// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::ops::ControlFlow;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::FutureExt;
use futures::StreamExt;
use libp2p::core::connection::ListenerId;
use libp2p::request_response::InboundFailure;
use libp2p::request_response::OutboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::RequestResponse;
use libp2p::request_response::RequestResponseEvent;
use libp2p::request_response::RequestResponseMessage;
use libp2p::request_response::ResponseChannel;
use libp2p::swarm::SwarmEvent;
use libp2p::Multiaddr;
use libp2p::PeerId;
use libp2p::Swarm;
use libp2p::TransportError;

use crate::actor::Endpoint;
use crate::actor::RequestMode;

use super::behaviour::ActorRequestResponseCodec;
use super::message::RequestMessage;
use super::message::ResponseMessage;
use super::net_commander::SwarmCommand;

/// The background loop that handles libp2p swarm events and `NetCommander` commands simultaneously.
pub struct EventLoop {
  swarm: Swarm<RequestResponse<ActorRequestResponseCodec>>,
  command_channel: mpsc::Receiver<SwarmCommand>,
  await_response: HashMap<RequestId, oneshot::Sender<Result<ResponseMessage, OutboundFailure>>>,
  await_response_sent: HashMap<RequestId, oneshot::Sender<Result<(), InboundFailure>>>,
  await_listen: HashMap<ListenerId, oneshot::Sender<Result<Multiaddr, TransportError<std::io::Error>>>>,
}

impl EventLoop {
  pub fn new(
    swarm: Swarm<RequestResponse<ActorRequestResponseCodec>>,
    command_channel: mpsc::Receiver<SwarmCommand>,
  ) -> Self {
    EventLoop {
      swarm,
      command_channel,
      await_response: HashMap::new(),
      await_response_sent: HashMap::new(),
      await_listen: HashMap::new(),
    }
  }

  pub async fn run<F>(mut self, event_handler: F)
  where
    F: Fn(InboundRequest),
  {
    loop {
      futures::select_biased! {
          event = self.swarm.select_next_some() => self.handle_swarm_event(event, &event_handler).await,
          command = self.command_channel.next().fuse() => {
              if let Some(c) = command {
                  if let ControlFlow::Break(_) = self.handle_command(c) {
                    break;
                  }
              } else {
                  break;
              }
          },
      }
    }
  }

  // This is where events coming from all peers are handled.
  // This is the intended place for didcomm authentication to take place, setup the sender-authenticated
  // encryption and from that point forward, transparently encrypt and decrypt messages.
  // Once encryption is taken care of, this handler then distributes messages based on ThreadIds, so
  // higher layers can easily await_message(thread_id).
  async fn handle_swarm_event<F, THandleErr>(
    &mut self,
    event: SwarmEvent<RequestResponseEvent<RequestMessage, ResponseMessage>, THandleErr>,
    event_handler: &F,
  ) where
    F: Fn(InboundRequest),
  {
    match event {
      SwarmEvent::Behaviour(RequestResponseEvent::Message {
        message: RequestResponseMessage::Request {
          channel,
          request,
          request_id,
        },
        peer,
      }) => {
        event_handler(InboundRequest {
          peer_id: peer,
          endpoint: request.endpoint,
          request_mode: request.request_mode,
          input: request.data,
          response_channel: channel,
          request_id,
        });
      }
      SwarmEvent::Behaviour(RequestResponseEvent::Message {
        message: RequestResponseMessage::Response { request_id, response },
        ..
      }) => {
        if let Some(response_channel) = self.await_response.remove(&request_id) {
          let _ = response_channel.send(Ok(response));
        }
      }
      SwarmEvent::Behaviour(RequestResponseEvent::OutboundFailure { request_id, error, .. }) => {
        if let Some(response_channel) = self.await_response.remove(&request_id) {
          let _ = response_channel.send(Err(error));
        }
      }
      SwarmEvent::Behaviour(RequestResponseEvent::InboundFailure { error, request_id, .. }) => {
        if let Some(response_channel) = self.await_response_sent.remove(&request_id) {
          let _ = response_channel.send(Err(error));
        }
      }
      SwarmEvent::Behaviour(RequestResponseEvent::ResponseSent { request_id, .. }) => {
        if let Some(response_channel) = self.await_response_sent.remove(&request_id) {
          let _ = response_channel.send(Ok(()));
        }
      }
      SwarmEvent::NewListenAddr { listener_id, address } => {
        if let Some(response_channel) = self.await_listen.remove(&listener_id) {
          let _ = response_channel.send(Ok(address));
        }
      }
      _ => (),
    }
  }

  fn handle_command(&mut self, command: SwarmCommand) -> ControlFlow<()> {
    match command {
      SwarmCommand::SendRequest {
        peer,
        request,
        response_channel,
      } => {
        let request_id = self.swarm.behaviour_mut().send_request(&peer, request);
        self.await_response.insert(request_id, response_channel);
      }
      SwarmCommand::SendResponse {
        response,
        response_channel,
        cmd_response_channel,
        request_id,
      } => {
        if self
          .swarm
          .behaviour_mut()
          .send_response(response_channel, ResponseMessage(response))
          .is_err()
        {
          // TODO: Shoudl we change this to `let _ = channel.send(...)` to make it best-effort?
          // Panicking is not desirable, as this event loop might handle multiple systems.
          // Alternatively, we can log a warning instead of ignoring it completely.
          cmd_response_channel
            .send(Err(InboundFailure::ConnectionClosed))
            .expect("receiver should not have been dropped")
        } else {
          self.await_response_sent.insert(request_id, cmd_response_channel);
        }
      }
      SwarmCommand::StartListening {
        address,
        response_channel,
      } => match self.swarm.listen_on(address) {
        Ok(listener_id) => {
          self.await_listen.insert(listener_id, response_channel);
        }
        Err(err) => {
          response_channel
            .send(Err(err))
            .expect("receiver should not have been dropped");
        }
      },
      SwarmCommand::AddAddresses { peer, addresses } => {
        for addr in addresses {
          self.swarm.behaviour_mut().add_address(&peer, addr);
        }
      }
      SwarmCommand::GetAddresses { response_channel } => {
        response_channel
          .send(self.swarm.listeners().map(ToOwned::to_owned).collect())
          .expect("receiver should not have been dropped");
      }
      SwarmCommand::Shutdown { response_channel } => {
        for (listener, channel) in std::mem::take(&mut self.await_listen).into_iter() {
          let _ = self.swarm.remove_listener(listener);
          let err = TransportError::Other(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "actor was shut down",
          ));

          let _ = channel.send(Err(err));
        }

        for (_, channel) in std::mem::take(&mut self.await_response) {
          log::warn!("draining channel");
          let _ = channel.send(Err(OutboundFailure::ConnectionClosed));
        }

        for (_, channel) in std::mem::take(&mut self.await_response_sent) {
          let _ = channel.send(Err(InboundFailure::ConnectionClosed));
        }

        response_channel
          .send(())
          .expect("receiver should not have been dropped");

        return ControlFlow::Break(());
      }
    }
    ControlFlow::Continue(())
  }
}

#[derive(Debug)]
pub struct InboundRequest {
  pub peer_id: PeerId,
  pub endpoint: Endpoint,
  pub request_mode: RequestMode,
  pub input: Vec<u8>,
  pub response_channel: ResponseChannel<ResponseMessage>,
  pub request_id: RequestId,
}

#[derive(Debug)]
pub struct ThreadRequest {
  pub peer_id: PeerId,
  pub endpoint: Endpoint,
  pub input: Vec<u8>,
}
