// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use super::RemoteSendError;
use super::RequestContext;
use super::SyncActorRequest;
use crate::actor::ErrorLocation;

pub type BoxFuture<'me, T> = Pin<Box<dyn Future<Output = T> + Send + 'me>>;

#[async_trait::async_trait]
pub trait SyncActor<REQ: SyncActorRequest>: 'static {
  async fn handle(&self, request: RequestContext<REQ>) -> REQ::Response;
}

pub trait AbstractSyncActor: Send + Sync + 'static {
  fn handle(&self, request: RequestContext<Vec<u8>>) -> BoxFuture<'_, Result<Vec<u8>, RemoteSendError>>;
}

pub struct SyncActorWrapper<ACT, REQ>
where
  REQ: SyncActorRequest + Send + Sync,
  ACT: SyncActor<REQ> + Send + Sync,
{
  actor: ACT,
  _phantom_req: PhantomData<REQ>,
}

impl<ACT, REQ> SyncActorWrapper<ACT, REQ>
where
  REQ: SyncActorRequest + Send + Sync,
  ACT: SyncActor<REQ> + Send + Sync,
{
  pub fn new(actor: ACT) -> Self {
    Self {
      actor,
      _phantom_req: PhantomData,
    }
  }
}

impl<ACT, REQ> AbstractSyncActor for SyncActorWrapper<ACT, REQ>
where
  REQ: SyncActorRequest + Send + Sync,
  REQ::Response: Send,
  ACT: SyncActor<REQ> + Send + Sync,
{
  fn handle(&self, request: RequestContext<Vec<u8>>) -> BoxFuture<'_, Result<Vec<u8>, RemoteSendError>> {
    let future = async move {
      let req: REQ =
        serde_json::from_slice(&request.input).map_err(|error| RemoteSendError::DeserializationFailure {
          location: ErrorLocation::Remote,
          context: format!(
            "deserializing the received bytes into the handler's expected type `{}`",
            std::any::type_name::<REQ>()
          ),
          error_message: error.to_string(),
        })?;

      let req: RequestContext<REQ> = request.convert(req);
      let result: REQ::Response = self.actor.handle(req).await;
      request_handler_serialize_response::<REQ>(&result)
    };

    Box::pin(future)
  }
}

#[inline(always)]
fn request_handler_serialize_response<REQ: SyncActorRequest>(
  input: &REQ::Response,
) -> Result<Vec<u8>, RemoteSendError> {
  log::debug!(
    "Attempt serialization into {:?}",
    std::any::type_name::<REQ::Response>()
  );

  let response: Vec<u8> = serde_json::to_vec(&input).map_err(|error| RemoteSendError::SerializationFailure {
    location: ErrorLocation::Remote,
    context: format!(
      "serializing the handler's response into `{}`",
      std::any::type_name::<REQ::Response>()
    ),
    error_message: error.to_string(),
  })?;

  Ok(response)
}
