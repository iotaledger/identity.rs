// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use crate::traits::ActorRequest;

use serde::{Deserialize, Serialize};

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
