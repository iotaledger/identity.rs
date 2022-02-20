// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::FutureExt;
use futures::StreamExt;
use libp2p::core::connection::ListenerId;
use libp2p::request_response::OutboundFailure;
use libp2p::request_response::RequestId;
use libp2p::request_response::RequestResponse;
use libp2p::request_response::RequestResponseEvent;
use libp2p::request_response::RequestResponseMessage;
use libp2p::request_response::ResponseChannel;
use libp2p::swarm::SwarmEvent;
use libp2p::PeerId;
use libp2p::Swarm;

use crate::Endpoint;

use super::behaviour::DidCommCodec;
use super::messages::RequestMessage;
use super::messages::ResponseMessage;
use super::net_commander::SwarmCommand;

pub struct EventLoop {
  swarm: Swarm<RequestResponse<DidCommCodec>>,
  command_channel: mpsc::Receiver<SwarmCommand>,
  listener_ids: Vec<ListenerId>,
  await_response: HashMap<RequestId, oneshot::Sender<Result<ResponseMessage, OutboundFailure>>>,
}

impl EventLoop {
  pub fn new(swarm: Swarm<RequestResponse<DidCommCodec>>, command_channel: mpsc::Receiver<SwarmCommand>) -> Self {
    EventLoop {
      swarm,
      command_channel,
      listener_ids: Vec::new(),
      await_response: HashMap::new(),
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
                  self.handle_command(c)
              } else {
                  break;
              }
          },
      }
    }

    // self.shutdown();
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
        message: RequestResponseMessage::Request { channel, request, .. },
        peer,
      }) => {
        event_handler(InboundRequest {
          peer_id: peer,
          endpoint: request.endpoint,
          input: request.data,
          response_channel: channel,
        });
      }
      SwarmEvent::Behaviour(RequestResponseEvent::Message {
        message: RequestResponseMessage::Response { request_id, response },
        ..
      }) => {
        // TODO: Decrypt/Deserialize response and return potential error or OutboundFailure?
        if let Some(response_channel) = self.await_response.remove(&request_id) {
          let _ = response_channel.send(Ok(response));
        }
      }
      SwarmEvent::Behaviour(RequestResponseEvent::OutboundFailure { request_id, error, .. }) => {
        if let Some(response_channel) = self.await_response.remove(&request_id) {
          let _ = response_channel.send(Err(error));
        }
      }
      _ => (),
    }
  }

  fn handle_command(&mut self, command: SwarmCommand) {
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
        cmd_response_channel: _,
      } => {
        // TODO: Handle by listening for InboundFailure and returning that via a channel?
        let _ = self
          .swarm
          .behaviour_mut()
          .send_response(response_channel, ResponseMessage(response));
      }
      SwarmCommand::StartListening {
        address,
        response_channel,
      } => {
        let result: Result<(), _> = self.swarm.listen_on(address).map(|listener_id| {
          self.listener_ids.push(listener_id);
        });

        response_channel.send(result).expect("sender was dropped");
      }
      SwarmCommand::AddAddress { peer, address } => {
        self.swarm.behaviour_mut().add_address(&peer, address);
      }
      SwarmCommand::GetAddresses { response_channel } => {
        response_channel
          .send(self.swarm.listeners().map(|addr| addr.to_owned()).collect())
          .expect("sender was dropped");
      }
      SwarmCommand::StopListening { response_channel } => {
        for listener in std::mem::take(&mut self.listener_ids).into_iter() {
          let _ = self.swarm.remove_listener(listener);
        }
        response_channel.send(()).expect("sender was dropped");
      }
    }
  }
}

#[derive(Debug)]
pub struct InboundRequest {
  pub peer_id: PeerId,
  pub endpoint: Endpoint,
  pub input: Vec<u8>,
  pub response_channel: ResponseChannel<ResponseMessage>,
}

#[derive(Debug)]
pub struct ThreadRequest {
  pub peer_id: PeerId,
  pub endpoint: Endpoint,
  pub input: Vec<u8>,
}
