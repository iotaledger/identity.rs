// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::PeerId;

use crate::didcomm::actor::DidCommMessages;
use crate::Actor;
use crate::RequestContext;

use super::actor::DidCommActor;

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
) -> crate::Result<()> {
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
) -> crate::Result<()> {
  log::debug!("verifier: sending request");
  actor.send_request(peer, PresentationRequest::default()).await?;

  let presentation: Presentation = actor.await_message(peer).await;
  log::debug!("verifier: received presentation: {:?}", presentation);

  log::debug!("verifier: sending presentation result");
  actor.send_request(peer, PresentationResult::default()).await?;
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
