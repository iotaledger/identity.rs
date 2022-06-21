// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use identity_agent::agent::AgentId;
use identity_agent::agent::System;
use identity_agent::agent::SystemBuilder;
use identity_agent::Multiaddr;

use remote_account::IdentityCreate;
use remote_account::RemoteAccount;

async fn setup() -> (System, AgentId, System) {
  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = SystemBuilder::new();

  let remote_account = RemoteAccount::new().unwrap();
  builder.attach::<IdentityCreate, _>(remote_account);

  let mut receiver: System = builder.build().await.unwrap();

  let addr = receiver.start_listening(addr).await.unwrap();
  let receiver_agent_id = receiver.agent_id();

  let mut sender: System = SystemBuilder::new().build().await.unwrap();

  sender.add_agent_address(receiver_agent_id, addr).await.unwrap();

  (receiver, receiver_agent_id, sender)
}

fn bench_create_remote_account(c: &mut Criterion) {
  static ITERATIONS: &[usize] = &[100, 10_000, 100_000];

  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

  let (receiver, receiver_agent_id, sender) = runtime.block_on(setup());

  let mut group = c.benchmark_group("create remote account");

  for size in ITERATIONS.iter() {
    group.bench_function(BenchmarkId::from_parameter(size), |bencher| {
      bencher.to_async(&runtime).iter(|| {
        let mut sender_clone: System = sender.clone();

        async move {
          sender_clone
            .send_request(receiver_agent_id, IdentityCreate::default())
            .await
            .unwrap()
            .unwrap();
        }
      });
    });
  }

  group.finish();

  runtime.block_on(async move {
    sender.shutdown().await.unwrap();
    receiver.shutdown().await.unwrap();
  });
}

criterion_group!(benches, bench_create_remote_account);

criterion_main!(benches);

mod remote_account {
  use dashmap::DashMap;
  use identity_account::account::Account;
  use identity_account::account::AccountBuilder;
  use identity_account::types::IdentitySetup;
  use identity_agent::agent::Actor;
  use identity_agent::agent::ActorRequest;
  use identity_agent::agent::Endpoint;
  use identity_agent::agent::RequestContext;
  use identity_iota_core::did::IotaDID;
  use identity_iota_core::document::IotaDocument;
  use serde::Deserialize;
  use serde::Serialize;
  use std::sync::Arc;
  use tokio::sync::Mutex;

  #[derive(Debug, Clone)]
  pub struct RemoteAccount {
    builder: Arc<Mutex<AccountBuilder>>,
    accounts: Arc<DashMap<IotaDID, Account>>,
  }

  impl RemoteAccount {
    pub fn new() -> identity_account::Result<Self> {
      let builder: AccountBuilder = Account::builder().autopublish(false);

      Ok(Self {
        builder: Arc::new(Mutex::new(builder)),
        accounts: Arc::new(DashMap::new()),
      })
    }
  }

  /// Can be sent to a `RemoteAccount` to instruct it to create an identity.
  #[derive(Debug, Default, Clone, Serialize, Deserialize)]
  pub struct IdentityCreate;

  impl From<IdentityCreate> for IdentitySetup {
    fn from(_: IdentityCreate) -> Self {
      IdentitySetup::default()
    }
  }

  impl ActorRequest for IdentityCreate {
    type Response = Result<IotaDocument, RemoteAccountError>;

    fn endpoint() -> Endpoint {
      "remote_account/create".try_into().unwrap()
    }
  }

  #[async_trait::async_trait]
  impl Actor<IdentityCreate> for RemoteAccount {
    async fn handle(&self, request: RequestContext<IdentityCreate>) -> Result<IotaDocument, RemoteAccountError> {
      let account: Account = self.builder.lock().await.create_identity(request.input.into()).await?;
      let doc = account.document().to_owned();
      self.accounts.insert(account.did().to_owned(), account);
      Ok(doc)
    }
  }

  /// The error type for the [`RemoteAccount`].
  #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
  #[non_exhaustive]
  pub enum RemoteAccountError {
    #[error("identity not found")]
    IdentityNotFound,
    #[error("{0}")]
    AccountError(String),
  }

  impl From<identity_account::Error> for RemoteAccountError {
    fn from(err: identity_account::Error) -> Self {
      Self::AccountError(err.to_string())
    }
  }
}
