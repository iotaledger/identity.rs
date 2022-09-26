// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::io::{self};

use futures::AsyncRead;
use futures::AsyncWrite;
use futures::AsyncWriteExt;
use libp2p::core::upgrade;
use libp2p::core::ProtocolName;
use libp2p::request_response::RequestResponseCodec;

use crate::p2p::RequestMessage;
use crate::p2p::ResponseMessage;

/// The protocol of the agent.
#[derive(Debug, Clone)]
pub(crate) struct AgentProtocol();

/// Defines the request and response types for the libp2p RequestResponse layer.
#[derive(Clone)]
pub(crate) struct AgentRequestResponseCodec();

impl ProtocolName for AgentProtocol {
  fn protocol_name(&self) -> &[u8] {
    "/agent/0.1.0".as_bytes()
  }
}

#[async_trait::async_trait]
impl RequestResponseCodec for AgentRequestResponseCodec {
  type Protocol = AgentProtocol;
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
