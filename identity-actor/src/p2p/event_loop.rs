// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::FutureExt;
use futures::SinkExt;
use futures::StreamExt;
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
use crate::RequestMessage;

use super::behaviour::DidCommCodec;
use super::behaviour::DidCommRequest;
use super::behaviour::DidCommResponse;
use super::net_commander::SwarmCommand;

pub struct EventLoop {
  swarm: Swarm<RequestResponse<DidCommCodec>>,
  command_channel: mpsc::Receiver<SwarmCommand>,
  event_channel: mpsc::Sender<InboundRequest>,
  await_response: HashMap<RequestId, oneshot::Sender<Result<(), OutboundFailure>>>,
}

impl EventLoop {
  pub fn new(
    swarm: Swarm<RequestResponse<DidCommCodec>>,
    command_channel: mpsc::Receiver<SwarmCommand>,
    event_channel: mpsc::Sender<InboundRequest>,
  ) -> Self {
    EventLoop {
      swarm,
      command_channel,
      event_channel,
      await_response: HashMap::new(),
    }
  }

  pub async fn run(mut self) {
    loop {
      futures::select_biased! {
          event = self.swarm.select_next_some() => self.handle_swarm_event(event).await,
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
  async fn handle_swarm_event<THandleErr>(
    &mut self,
    event: SwarmEvent<RequestResponseEvent<DidCommRequest, DidCommResponse>, THandleErr>,
  ) {
    match event {
      SwarmEvent::Behaviour(RequestResponseEvent::Message { message, peer }) => match message {
        RequestResponseMessage::Request { channel, request, .. } => {
          // In the general case, we would decrypt the message and deserialize to RequestMessage.
          let request_message: RequestMessage = match serde_json::from_slice(request.0.as_ref()) {
            Ok(request_message) => request_message,
            Err(err) => {
              log::error!("could not deserialize to `RequestMessage`: {err:?}");
              // TODO: Handle _somehow_?
              let _ = self
                .swarm
                .behaviour_mut()
                .send_response(channel, DidCommResponse(b"nope".to_vec()));
              return;
            }
          };

          self
            .event_channel
            .send(InboundRequest {
              peer_id: peer,
              endpoint: request_message.endpoint,
              input: request_message.data,
              response_channel: channel,
            })
            .await
            .expect("event receiver was dropped")
        }
        RequestResponseMessage::Response {
          request_id,
          response: _,
        } => {
          // TODO: Decrypt/Deserialize response and return potential error or OutboundFailure?
          if let Some(result_tx) = self.await_response.remove(&request_id) {
            let _ = result_tx.send(Ok(()));
          }
          return;
        }
      },
      SwarmEvent::Behaviour(RequestResponseEvent::OutboundFailure { request_id, error, .. }) => {
        if let Some(result_tx) = self.await_response.remove(&request_id) {
          let _ = result_tx.send(Err(error));
        }
        return;
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
        let request_id = self.swarm.behaviour_mut().send_request(&peer, DidCommRequest(request));
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
          .send_response(response_channel, DidCommResponse(response));
      }
      SwarmCommand::StartListening {
        address,
        response_channel,
      } => {
        let result: Result<(), _> = self.swarm.listen_on(address).map(|_| ());
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
      SwarmCommand::GetPeerId { response_channel } => {
        response_channel
          .send(self.swarm.local_peer_id().clone())
          .expect("sender was dropped");
      }
    }
  }
}

pub struct InboundRequest {
  pub peer_id: PeerId,
  pub endpoint: Endpoint,
  pub input: Vec<u8>,
  pub response_channel: ResponseChannel<DidCommResponse>,
}
