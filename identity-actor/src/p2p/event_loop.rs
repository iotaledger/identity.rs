use std::collections::HashMap;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::future::poll_fn;
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
use libp2p::Multiaddr;
use libp2p::PeerId;
use libp2p::Swarm;

use crate::RequestMessage;

use super::behaviour::DidCommCodec;
use super::behaviour::DidCommRequest;
use super::behaviour::DidCommResponse;

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
          let request_message: RequestMessage<Vec<u8>> = match serde_json::from_slice(request.0.as_ref()) {
            Ok(request_message) => request_message,
            Err(_) => {
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
              request_message: request_message,
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
      _ => (),
    }
  }
}

pub struct InboundRequest {
  pub peer_id: PeerId,
  pub request_message: RequestMessage<Vec<u8>>,
  pub response_channel: ResponseChannel<DidCommResponse>,
}

#[derive(Clone)]
pub struct NetCommander {
  command_sender: mpsc::Sender<SwarmCommand>,
}

impl NetCommander {
  pub fn new(command_sender: mpsc::Sender<SwarmCommand>) -> Self {
    NetCommander { command_sender }
  }

  pub async fn send_request(&mut self, peer: PeerId, request: Vec<u8>) -> Result<(), OutboundFailure> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::SendRequest {
      peer,
      request,
      response_channel: sender,
    };
    self.send_command(command).await;
    receiver.await.unwrap()
  }

  pub async fn send_response(&mut self, data: Vec<u8>, channel: ResponseChannel<DidCommResponse>) {
    let (sender, _receiver) = oneshot::channel();
    let command = SwarmCommand::SendResponse {
      response: data,
      cmd_response_channel: sender,
      response_channel: channel,
    };
    self.send_command(command).await;
  }

  pub async fn start_listening(&mut self, address: Multiaddr) -> Result<Multiaddr, OutboundFailure> {
    let (sender, receiver) = oneshot::channel();
    let command = SwarmCommand::StartListening {
      address,
      response_channel: sender,
    };
    self.send_command(command).await;
    receiver.await.unwrap()
  }

  async fn send_command(&mut self, command: SwarmCommand) {
    let _ = poll_fn(|cx| self.command_sender.poll_ready(cx)).await;
    let _ = self.command_sender.start_send(command);
  }
}

pub enum SwarmCommand {
  SendRequest {
    peer: PeerId,
    request: Vec<u8>,
    response_channel: oneshot::Sender<Result<(), OutboundFailure>>,
  },
  SendResponse {
    response: Vec<u8>,
    cmd_response_channel: oneshot::Sender<Result<(), OutboundFailure>>,
    response_channel: ResponseChannel<DidCommResponse>,
  },
  StartListening {
    address: Multiaddr,
    response_channel: oneshot::Sender<Result<Multiaddr, OutboundFailure>>,
  },
}
