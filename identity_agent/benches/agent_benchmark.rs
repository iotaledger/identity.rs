// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

use identity_agent::agent::AgentId;
use identity_agent::didcomm::DidCommAgent;
use identity_agent::didcomm::DidCommAgentBuilder;
use identity_agent::didcomm::DidCommAgentIdentity;
use identity_agent::didcomm::DidCommPlaintextMessage;
use identity_agent::didcomm::ThreadId;
use identity_agent::Multiaddr;

use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_iota_core::document::IotaDocument;
use test_handler::PresentationRequest;
use test_handler::TestHandler;

use crate::test_handler::PresentationOffer;

async fn setup() -> (DidCommAgent, AgentId, DidCommAgent) {
  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = DidCommAgentBuilder::new().identity(DidCommAgentIdentity {
    document: IotaDocument::new(&KeyPair::new(KeyType::Ed25519).unwrap()).unwrap(),
  });

  builder.attach_didcomm(TestHandler);

  let mut receiver: DidCommAgent = builder.build().await.unwrap();

  let addr = receiver.start_listening(addr).await.unwrap();
  let receiver_agent_id = receiver.agent_id();

  let mut sender: DidCommAgent = DidCommAgentBuilder::new()
    .identity(DidCommAgentIdentity {
      document: IotaDocument::new(&KeyPair::new(KeyType::Ed25519).unwrap()).unwrap(),
    })
    .build()
    .await
    .unwrap();

  sender.add_agent_address(receiver_agent_id, addr).await.unwrap();

  (receiver, receiver_agent_id, sender)
}

fn bench_send_didcomm_messages(c: &mut Criterion) {
  static ITERATIONS: &[usize] = &[100, 10_000, 100_000];

  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

  let (receiver, receiver_agent_id, sender) = runtime.block_on(setup());

  let mut group = c.benchmark_group("send didcomm messages");

  for size in ITERATIONS.iter() {
    group.bench_function(BenchmarkId::from_parameter(size), |bencher| {
      bencher.to_async(&runtime).iter(|| {
        let mut sender_clone: DidCommAgent = sender.clone();

        let thread_id: ThreadId = ThreadId::new();

        async move {
          sender_clone
            .send_didcomm_request(receiver_agent_id, &thread_id, PresentationRequest::default())
            .await
            .unwrap();

          let _: DidCommPlaintextMessage<PresentationOffer> =
            sender_clone.await_didcomm_request(&thread_id).await.unwrap();
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

criterion_group!(benches, bench_send_didcomm_messages);

criterion_main!(benches);

mod test_handler {
  use identity_agent::agent::Endpoint;
  use identity_agent::agent::RequestContext;
  use identity_agent::didcomm::DidCommAgent;
  use identity_agent::didcomm::DidCommHandler;
  use identity_agent::didcomm::DidCommPlaintextMessage;
  use identity_agent::didcomm::DidCommRequest;
  use serde::Deserialize;
  use serde::Serialize;

  #[derive(Debug, Clone)]
  pub struct TestHandler;

  #[derive(Clone, Debug, Deserialize, Serialize, Default)]
  pub(crate) struct PresentationRequest(u8);

  impl DidCommRequest for PresentationRequest {
    fn endpoint() -> Endpoint {
      "didcomm/presentation_request".try_into().unwrap()
    }
  }

  #[derive(Clone, Debug, Deserialize, Serialize, Default)]
  pub(crate) struct PresentationOffer(u16);

  impl DidCommRequest for PresentationOffer {
    fn endpoint() -> Endpoint {
      "didcomm/presentation_offer".try_into().unwrap()
    }
  }

  #[async_trait::async_trait]
  impl DidCommHandler<DidCommPlaintextMessage<PresentationRequest>> for TestHandler {
    async fn handle(
      &self,
      mut agent: DidCommAgent,
      request: RequestContext<DidCommPlaintextMessage<PresentationRequest>>,
    ) {
      agent
        .send_didcomm_request(
          request.agent_id,
          request.input.thread_id(),
          PresentationOffer(request.input.body().0 as u16),
        )
        .await
        .unwrap();
    }
  }
}
