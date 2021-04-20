// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use identity::iota::MessageId;
use identity_core::common::Timestamp;
use identity_core::crypto::KeyPair;
use identity_iota::chain::AuthChain;
use identity_iota::chain::DocumentChain;
use identity_iota::did::Document;
use identity_iota::did::DocumentDiff;
use identity_iota::tangle::TangleRef;

fn setup() -> (Document, KeyPair) {
  let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
  let mut document: Document = Document::from_keypair(&keypair).unwrap();
  document.sign(keypair.secret()).unwrap();
  document.set_message_id(MessageId::new([8; 32]));

  (document, keypair)
}

fn update_diff_chain(n: usize, document: Document, keypair: KeyPair) {
  let mut chain: DocumentChain;
  chain = DocumentChain::new(AuthChain::new(document).unwrap());

  for i in 0..n {
    let new: Document = {
      let mut this: Document = chain.current().clone();
      this.properties_mut().insert(i.to_string(), 123.into());
      this.set_updated(Timestamp::now());
      this
    };

    let message_id = *chain.diff_message_id();
    let mut diff: DocumentDiff = chain.current().diff(&new, message_id, keypair.secret()).unwrap();
    diff.set_message_id(message_id);
    assert!(chain.try_push_diff(diff).is_ok());
  }
}

fn bench_diff_chain_updates(c: &mut Criterion) {
  static ITERATIONS: &[usize] = &[10, 100, 200];
  let (doc, keys) = setup();

  let mut group = c.benchmark_group("update_diff_chain");
  for size in ITERATIONS.iter() {
    group.throughput(Throughput::Elements(*size as u64));
    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
      b.iter(|| update_diff_chain(size, doc.clone(), keys.clone()));
    });
  }
  group.finish();
}

criterion_group!(benches, bench_diff_chain_updates);
criterion_main!(benches);
