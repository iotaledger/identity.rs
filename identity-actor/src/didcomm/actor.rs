// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

use libp2p::Multiaddr;
use libp2p::PeerId;
use tokio::sync::RwLock;

use crate::Actor;
use crate::ActorRequest;
use crate::Endpoint;
use crate::RemoteSendError;
use crate::RequestContext;

/// Can be returned from a hook to indicate that the protocol should immediately terminate.
/// This doesn't include any way to set a cause for the termination, as it is expected that
/// a hook sends a problem report to the peer before returning this type.
pub struct DidCommTermination;

// Putting this field in the DidCommActor directly would leak memory,
// since the DidCommActor contains an Actor, but the DidCommActor would
// have to be added as state to the actor -> circular reference -> Arcs aren't deallocated.
#[derive(Clone)]
pub struct DidCommMessages {
  pub(crate) messages: Arc<RwLock<HashMap<PeerId, VecDeque<serde_json::Value>>>>,
}

impl DidCommMessages {
  fn new() -> Self {
    Self {
      messages: Arc::new(RwLock::new(HashMap::new())),
    }
  }

  pub async fn catch_all_handler(self, _actor: Actor, request: RequestContext<serde_json::Value>) {
    log::debug!("Received {}, from {}", request.endpoint, request.peer,);

    match self.messages.write().await.entry(request.peer) {
      Entry::Occupied(mut messages) => {
        messages.get_mut().push_back(request.input);
      }
      Entry::Vacant(entry) => {
        let mut messages = VecDeque::new();
        messages.push_back(request.input);

        entry.insert(messages);
      }
    }
  }
}

#[derive(Clone)]
pub struct DidCommActor {
  actor: Actor,
  pub(crate) messages: DidCommMessages,
}

impl DidCommActor {
  pub fn new(actor: Actor) -> Self {
    Self {
      actor,
      messages: DidCommMessages::new(),
    }
  }

  pub fn actor(&mut self) -> &mut Actor {
    &mut self.actor
  }

  pub async fn await_message<REQ: ActorRequest>(&self, peer: PeerId) -> crate::Result<REQ> {
    loop {
      if let Some(messages) = self.messages.messages.write().await.get_mut(&peer) {
        log::debug!(
          "number of {} messages from peer {}: {}",
          std::any::type_name::<REQ>(),
          peer,
          messages.len()
        );

        if let Some(msg) = messages.pop_front() {
          let message: REQ =
            serde_json::from_value(msg).map_err(|err| crate::Error::DeserializationFailure(err.to_string()))?;

          let hook_endpoint: Endpoint = Endpoint::new_hook(message.request_name())?;

          if self.actor.handlers().contains_key(&hook_endpoint) {
            log::debug!("Calling hook: {}", hook_endpoint);

            let hook_result: Result<Result<REQ, DidCommTermination>, RemoteSendError> =
              self.actor.call_hook(hook_endpoint, peer, message).await;

            match hook_result {
              Ok(Ok(request)) => return Ok(request),
              Ok(Err(_)) => {
                unimplemented!("didcomm termination");
              }
              Err(err) => return Err(err.into()),
            }
          } else {
            return Ok(message);
          }
        }
      }

      tokio::time::sleep(Duration::from_millis(300)).await;
    }
  }

  pub async fn send_request<REQ: ActorRequest>(&mut self, peer: PeerId, input: REQ) -> crate::Result<REQ::Response> {
    let endpoint = Endpoint::new_hook(input.request_name())?;

    if self.actor.handlers().contains_key(&endpoint) {
      log::debug!("Calling hook: {}", endpoint);

      let hook_result: Result<Result<REQ, DidCommTermination>, RemoteSendError> =
        self.actor.call_hook(endpoint, peer, input).await;

      match hook_result {
        Ok(Ok(request)) => self.actor.send_request(peer, request).await,
        Ok(Err(_)) => {
          unimplemented!("didcomm termination");
        }
        Err(err) => Err(err.into()),
      }
    } else {
      self.actor.send_request(peer, input).await
    }
  }

  pub async fn add_peer(&mut self, peer: PeerId, addr: Multiaddr) {
    self.actor.add_address(peer, addr).await
  }
}

impl ActorRequest for serde_json::Value {
  type Response = ();

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("n/a")
  }
}
