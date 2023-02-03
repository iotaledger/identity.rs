// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use identity_agent::agent::Agent;
use identity_agent::agent::AgentBuilder;
use identity_agent::agent::AgentId;
use identity_agent::Multiaddr;

use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;
use remote_account::IdentityCreate;
use remote_account::RemoteAccount;

async fn setup() -> (Agent, AgentId, Agent) {
  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = AgentBuilder::new();

  let remote_account = RemoteAccount::new();
  builder.attach::<IdentityCreate, _>(remote_account);

  let mut receiver: Agent = builder.build().await.unwrap();

  let addr = receiver.start_listening(addr).await.unwrap();
  let receiver_agent_id = receiver.agent_id();

  let mut sender: Agent = AgentBuilder::new().build().await.unwrap();

  sender.add_agent_address(receiver_agent_id, addr).await.unwrap();

  (receiver, receiver_agent_id, sender)
}

fn fake_document() -> IotaDocument {
  let rand_bytes: [u8; 32] = rand::random();
  let network_name = NetworkName::try_from("iota").unwrap();
  let did = IotaDID::new(&rand_bytes, &network_name);
  IotaDocument::new_with_id(did)
}

fn bench_remote_account(c: &mut Criterion) {
  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

  let (receiver, receiver_agent_id, sender) = runtime.block_on(setup());

  let mut group = c.benchmark_group("remote_account");

  group.bench_function("IdentityCreate", |bencher| {
    bencher.to_async(&runtime).iter(|| {
      let mut sender_clone: Agent = sender.clone();

      let doc = fake_document();

      async move {
        sender_clone
          .send_request(receiver_agent_id, IdentityCreate(doc))
          .await
          .unwrap()
          .unwrap();
      }
    });
  });

  group.finish();

  runtime.block_on(async move {
    sender.shutdown().await.unwrap();
    receiver.shutdown().await.unwrap();
  });
}

criterion_group!(benches, bench_remote_account);

criterion_main!(benches);

mod remote_account {
  use dashmap::DashMap;
  use identity_agent::agent::Endpoint;
  use identity_agent::agent::Handler;
  use identity_agent::agent::HandlerRequest;
  use identity_agent::agent::RequestContext;
  use identity_iota_core::IotaDID;
  use identity_iota_core::IotaDocument;
  use serde::Deserialize;
  use serde::Serialize;
  use std::sync::Arc;

  /// A proof-of-concept implementation of a remote account
  /// which holds and manages a collection of DID documents.
  #[derive(Debug, Clone)]
  pub(crate) struct RemoteAccount {
    documents: Arc<DashMap<IotaDID, IotaDocument>>,
  }

  impl RemoteAccount {
    pub(crate) fn new() -> Self {
      Self {
        documents: Arc::new(DashMap::new()),
      }
    }
  }

  /// Can be sent to a `RemoteAccount` to instruct it to add a document.
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub(crate) struct IdentityCreate(pub(crate) IotaDocument);

  impl HandlerRequest for IdentityCreate {
    type Response = Result<(), RemoteAccountError>;

    fn endpoint() -> Endpoint {
      "remote_account/create".try_into().unwrap()
    }
  }

  #[async_trait::async_trait]
  impl Handler<IdentityCreate> for RemoteAccount {
    async fn handle(&self, request: RequestContext<IdentityCreate>) -> Result<(), RemoteAccountError> {
      let document = request.input.0;

      if document.id().is_placeholder() {
        return Err(RemoteAccountError::PlaceholderDID);
      }

      self.documents.insert(document.id().to_owned(), document);
      Ok(())
    }
  }

  /// The error type for the [`RemoteAccount`].
  #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
  #[non_exhaustive]
  pub(crate) enum RemoteAccountError {
    #[error("identity not found")]
    IdentityNotFound,
    #[error("placeholder DIDs cannot be managed")]
    PlaceholderDID,
  }
}
