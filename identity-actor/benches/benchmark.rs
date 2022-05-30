// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use identity_actor::actor::System;
use identity_actor::actor::SystemBuilder;
use identity_actor::remote_account::IdentityCreate;
use identity_actor::remote_account::IdentityGet;
use identity_actor::remote_account::IdentityList;
use identity_actor::remote_account::RemoteAccount;
use identity_actor::Multiaddr;
use identity_actor::PeerId;

async fn setup() -> (System, PeerId, System) {
  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
  let mut builder = SystemBuilder::new();

  let remote_account = RemoteAccount::new().unwrap();
  builder.attach::<IdentityCreate, _>(remote_account.clone());
  builder.attach::<IdentityList, _>(remote_account.clone());
  builder.attach::<IdentityGet, _>(remote_account);

  let mut receiver: System = builder.build().await.unwrap();

  let addr = receiver.start_listening(addr).await.unwrap();
  let receiver_peer_id = receiver.peer_id();

  let mut sender: System = SystemBuilder::new().build().await.unwrap();

  sender.add_address(receiver_peer_id, addr).await.unwrap();

  (receiver, receiver_peer_id, sender)
}

fn bench_create_remote_account(c: &mut Criterion) {
  static ITERATIONS: &[usize] = &[100, 10_000, 100_000];

  let runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

  let (receiver, receiver_peer_id, sender) = runtime.block_on(setup());

  let mut group = c.benchmark_group("create remote account");

  for size in ITERATIONS.iter() {
    group.bench_function(BenchmarkId::from_parameter(size), |bencher| {
      bencher.to_async(&runtime).iter(|| {
        let mut sender_clone: System = sender.clone();

        async move {
          sender_clone
            .send_request(receiver_peer_id, IdentityCreate::default())
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
