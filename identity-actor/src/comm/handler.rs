// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
  borrow::Cow,
  collections::{hash_map::Entry, HashMap, VecDeque},
  sync::Arc,
  time::Duration,
};

use libp2p::{Multiaddr, PeerId};
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

use crate::{endpoint::Endpoint, errors::RemoteSendError, traits::ActorRequest, types::RequestContext, Actor};

use super::requests::{Presentation, PresentationOffer, PresentationRequest, PresentationResult};

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

  pub async fn await_message<T: DeserializeOwned>(&self, peer: PeerId) -> T {
    loop {
      if let Some(messages) = self.messages.messages.write().await.get_mut(&peer) {
        log::debug!(
          "number of {} messages from peer {}: {}",
          std::any::type_name::<T>(),
          peer,
          messages.len()
        );

        if let Some(msg) = messages.pop_front() {
          return serde_json::from_value(msg).unwrap();
        }
      }

      tokio::time::sleep(Duration::from_millis(300)).await;
    }
  }

  pub async fn send_request<REQ: ActorRequest>(
    &mut self,
    peer: PeerId,
    command: REQ,
  ) -> crate::errors::Result<REQ::Response> {
    let hook_result: Result<Result<REQ, DidCommTermination>, RemoteSendError> = self
      .actor
      .call_hook(Endpoint::new_hook(command.request_name())?, peer, command)
      .await;

    // TODO: Since the hook `RemoteSendError` is somewhat different from `send_request`
    // we should wrap it in a HookInvocationError or something, to make it clearer for
    // the caller that the hook caused the error.
    let hook_result = hook_result?;

    match hook_result {
      Ok(request) => self.actor.send_request(peer, request).await,
      Err(_) => {
        panic!("did comm termination")
      }
    }
  }

  pub async fn add_peer(&mut self, peer: PeerId, addr: Multiaddr) {
    self.actor.add_peer(peer, addr).await
  }
}

impl ActorRequest for serde_json::Value {
  type Response = ();

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("n/a")
  }
}

#[derive(Clone)]
pub struct DidCommHandler;

impl DidCommHandler {
  pub async fn new() -> Self {
    Self
  }

  pub async fn presentation_holder_actor_handler(self, mut actor: Actor, request: RequestContext<PresentationRequest>) {
    log::debug!("holder: received presentation request");

    let did_comm_actor = DidCommActor::new(actor.clone());

    actor
      .add_state(did_comm_actor.messages.clone())
      .add_handler("didcomm/*", DidCommMessages::catch_all_handler)
      .unwrap();

    presentation_holder_handler(DidCommActor::new(actor), request.peer, Some(request.input))
      .await
      .unwrap();
  }

  pub async fn presentation_verifier_actor_handler(self, mut actor: Actor, request: RequestContext<PresentationOffer>) {
    log::debug!("verifier: received offer from {}", request.peer);

    let did_comm_actor = DidCommActor::new(actor.clone());

    actor
      .add_state(did_comm_actor.messages.clone())
      .add_handler("didcomm/*", DidCommMessages::catch_all_handler)
      .unwrap();

    presentation_verifier_handler(did_comm_actor, request.peer, Some(request.input))
      .await
      .unwrap();
  }
}

pub async fn presentation_holder_handler(
  mut actor: DidCommActor,
  peer: PeerId,
  request: Option<PresentationRequest>,
) -> crate::errors::Result<()> {
  let _request: PresentationRequest = match request {
    Some(request) => request,
    None => {
      log::debug!("holder: sending presentation offer");
      actor.send_request(peer, PresentationOffer::default()).await?;

      let req = actor.await_message(peer).await;
      log::debug!("holder: received presentation request");

      req
    }
  };

  // let _result = actor.call_hook("didcomm/presentation/user_consent", request).await?;

  log::debug!("holder: sending presentation");
  actor.send_request(peer, Presentation::default()).await?;

  let _result: PresentationResult = actor.await_message(peer).await;
  log::debug!("holder: received presentation result");

  // let _result = actor.call_hook("didcomm/presentation/result", result).await?;

  Ok(())
}

pub async fn presentation_verifier_handler(
  mut actor: DidCommActor,
  peer: PeerId,
  _offer: Option<PresentationOffer>,
) -> crate::errors::Result<()> {
  log::debug!("verifier: sending request");
  actor.send_request(peer, PresentationRequest::default()).await?;

  let presentation: Presentation = actor.await_message(peer).await;
  log::debug!("verifier: received presentation: {:?}", presentation);

  log::debug!("verifier: sending presentation result");
  actor.send_request(peer, PresentationResult::default()).await?;
  Ok(())
}
