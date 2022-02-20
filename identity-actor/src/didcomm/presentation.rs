// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::didcomm::message::DidCommPlaintextMessage;
use crate::didcomm::thread_id::ThreadId;
use crate::Actor;
use crate::RequestContext;

#[derive(Clone)]
pub struct DidCommHandler;

impl DidCommHandler {
  pub async fn new() -> Self {
    Self
  }

  pub async fn presentation_holder_actor_handler(
    self,
    actor: Actor,
    request: RequestContext<DidCommPlaintextMessage<PresentationRequest>>,
  ) {
    log::debug!("holder: received presentation request");

    let result = presentation_holder_handler(actor, request.peer, Some(request.input)).await;

    if let Err(err) = result {
      log::error!("{:?}", err);
    }
  }

  pub async fn presentation_verifier_actor_handler(
    self,
    actor: Actor,
    request: RequestContext<DidCommPlaintextMessage<PresentationOffer>>,
  ) {
    log::debug!("verifier: received offer from {}", request.peer);

    let result = presentation_verifier_handler(actor, request.peer, Some(request.input)).await;

    if let Err(err) = result {
      log::error!("{:?}", err);
    }
  }
}

pub async fn presentation_holder_handler(
  mut actor: Actor,
  peer: PeerId,
  request: Option<DidCommPlaintextMessage<PresentationRequest>>,
) -> crate::Result<()> {
  let request: DidCommPlaintextMessage<PresentationRequest> = match request {
    Some(request) => request,
    None => {
      log::debug!("holder: sending presentation offer");
      // actor.send_request(peer, PresentationOffer::default()).await?;

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

  // let _result = actor.call_hook("didcomm/presentation/user_consent", request).await?;

  log::debug!("holder: sending presentation");
  actor.send_message(peer, thread_id, Presentation::default()).await?;

  let _result: DidCommPlaintextMessage<PresentationResult> = actor.await_message(thread_id).await?;
  log::debug!("holder: received presentation result");

  // let _result = actor.call_hook("didcomm/presentation/result", result).await?;

  Ok(())
}

pub async fn presentation_verifier_handler(
  mut actor: Actor,
  peer: PeerId,
  offer: Option<DidCommPlaintextMessage<PresentationOffer>>,
) -> crate::Result<()> {
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

use std::borrow::Cow;

use crate::ActorRequest;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct PresentationRequest(u32);

impl ActorRequest for PresentationRequest {
  type Response = ();

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("didcomm/presentation_request")
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct PresentationOffer(u32, u32);

impl ActorRequest for PresentationOffer {
  type Response = ();

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("didcomm/presentation_offer")
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Presentation(u32, u32, u32);

impl ActorRequest for Presentation {
  type Response = ();

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("didcomm/presentation")
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct PresentationResult(u32, u32, u32, u32);

impl ActorRequest for PresentationResult {
  type Response = ();

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("didcomm/presentation_result")
  }
}
