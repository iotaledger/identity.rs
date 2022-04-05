// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::AsyncRead;
use futures::AsyncWrite;
use futures::AsyncWriteExt;
use libp2p::core::upgrade;
use libp2p::core::ProtocolName;
use libp2p::request_response::RequestResponseCodec;

use std::io::{self};

use super::message::RequestMessage;
use super::message::ResponseMessage;

#[derive(Debug, Clone)]
pub struct ActorProtocol();
#[derive(Clone)]
pub struct ActorRequestResponseCodec();

impl ProtocolName for ActorProtocol {
  fn protocol_name(&self) -> &[u8] {
    "/actor/0.5.0".as_bytes()
  }
}

#[async_trait::async_trait]
impl RequestResponseCodec for ActorRequestResponseCodec {
  type Protocol = ActorProtocol;
  type Request = RequestMessage;
  type Response = ResponseMessage;

  async fn read_request<T>(&mut self, _protocol: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
  where
    T: AsyncRead + Unpin + Send,
  {
    let vec = upgrade::read_length_prefixed(io, 1_000_000).await?;

    let request: RequestMessage = RequestMessage::from_bytes(vec.as_ref())?;

    Ok(request)
  }

  async fn read_response<T>(&mut self, _protocol: &Self::Protocol, io: &mut T) -> io::Result<Self::Response>
  where
    T: AsyncRead + Unpin + Send,
  {
    let vec = upgrade::read_length_prefixed(io, 1_000_000).await?;

    Ok(ResponseMessage(vec))
  }

  async fn write_request<T>(&mut self, _protocol: &Self::Protocol, io: &mut T, request: Self::Request) -> io::Result<()>
  where
    T: AsyncWrite + Unpin + Send,
  {
    let bytes: Vec<u8> = request.to_bytes()?;

    upgrade::write_length_prefixed(io, bytes).await?;
    io.close().await
  }

  async fn write_response<T>(
    &mut self,
    _protocol: &Self::Protocol,
    io: &mut T,
    ResponseMessage(data): Self::Response,
  ) -> io::Result<()>
  where
    T: AsyncWrite + Unpin + Send,
  {
    upgrade::write_length_prefixed(io, data).await?;
    io.close().await
  }
}
