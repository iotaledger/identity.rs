// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::actor::AsyncActorRequest;
use crate::actor::Endpoint;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::didcomm::message::DidCommPlaintextMessage;
use crate::didcomm::thread_id::ThreadId;

#[derive(Clone)]
pub struct DidCommState;

impl DidCommState {
  pub async fn new() -> Self {
    Self
  }

  pub async fn presentation_holder_actor_handler(
    self,
    actor: DidCommActor,
    request: RequestContext<DidCommPlaintextMessage<PresentationRequest>>,
  ) {
    log::debug!("holder: received presentation request");

    let result = presentation_holder_handler(actor, request.peer, Some(request.input)).await;

    if let Err(err) = result {
      log::error!("presentation_holder_actor_handler errored: {:?}", err);
    }
  }

  pub async fn presentation_verifier_actor_handler(
    self,
    actor: DidCommActor,
    request: RequestContext<DidCommPlaintextMessage<PresentationOffer>>,
  ) {
    log::debug!("verifier: received offer from {}", request.peer);

    let result = presentation_verifier_handler(actor, request.peer, Some(request.input)).await;

    if let Err(err) = result {
      log::error!("presentation_verifier_actor_handler errored: {:?}", err);
    }
  }
}

pub async fn presentation_holder_handler(
  mut actor: DidCommActor,
  peer: PeerId,
  request: Option<DidCommPlaintextMessage<PresentationRequest>>,
) -> ActorResult<()> {
  let request: DidCommPlaintextMessage<PresentationRequest> = match request {
    Some(request) => request,
    None => {
      log::debug!("holder: sending presentation offer");
      let thread_id = ThreadId::new();
      actor
        .send_message(peer, &thread_id, PresentationOffer::default())
        .await?;

      let req = actor.await_message(&thread_id).await;
      log::debug!("holder: received presentation request");

      req?
    }
  };

  let thread_id = request.thread_id();

  log::debug!("holder: sending presentation");
  actor.send_message(peer, thread_id, Presentation::default()).await?;

  let _result: DidCommPlaintextMessage<PresentationResult> = actor.await_message(thread_id).await?;
  log::debug!("holder: received presentation result");

  Ok(())
}

pub async fn presentation_verifier_handler(
  mut actor: DidCommActor,
  peer: PeerId,
  offer: Option<DidCommPlaintextMessage<PresentationOffer>>,
) -> ActorResult<()> {
  let thread_id: ThreadId = if let Some(offer) = offer {
    offer.thread_id().to_owned()
  } else {
    ThreadId::new()
  };

  log::debug!("verifier: sending request");
  actor
    .send_message(peer, &thread_id, PresentationRequest::default())
    .await?;

  log::debug!("verifier: awaiting presentation");
  let presentation: DidCommPlaintextMessage<Presentation> = actor.await_message(&thread_id).await?;
  log::debug!("verifier: received presentation: {:?}", presentation);

  log::debug!("verifier: sending presentation result");
  actor
    .send_message(peer, &thread_id, PresentationResult::default())
    .await?;
  Ok(())
}

use serde::Deserialize;
use serde::Serialize;

use super::didcomm_actor::DidCommActor;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct PresentationRequest([u8; 2]);

impl AsyncActorRequest for PresentationRequest {
  fn endpoint() -> Endpoint {
    "didcomm/presentation_request".parse().unwrap()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct PresentationOffer([u8; 3]);

impl AsyncActorRequest for PresentationOffer {
  fn endpoint() -> Endpoint {
    "didcomm/presentation_offer".parse().unwrap()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Presentation([u8; 4]);

impl AsyncActorRequest for Presentation {
  fn endpoint() -> Endpoint {
    "didcomm/presentation".parse().unwrap()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct PresentationResult([u8; 5]);

impl AsyncActorRequest for PresentationResult {
  fn endpoint() -> Endpoint {
    "didcomm/presentation_result".parse().unwrap()
  }
}
