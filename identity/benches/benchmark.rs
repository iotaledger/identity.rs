// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use diff_chain::create_diff_chain;
use diff_chain::setup_diff_chain_bench;
use diff_chain::update_diff_chain;
use identity::iota::did::Document;
use identity::iota::did::DID;
use identity::{
  crypto::*,
  iota::{AuthChain, DocumentChain},
};
mod diff_chain;

fn generate_signed_document(keypair: &KeyPair) {
  let mut document: Document = Document::from_keypair(&keypair).unwrap();
  document.sign(keypair.secret()).unwrap();
}

fn generate_did(keypair: &KeyPair) {
  let _ = DID::new(keypair.public().as_ref()).unwrap();
}

fn bench_generate_signed_document(c: &mut Criterion) {
  let keypair = KeyPair::new_ed25519().unwrap();
  c.bench_function("generate signed document", |b| {
    b.iter(|| generate_signed_document(&keypair))
  });
}

fn bench_generate_did(c: &mut Criterion) {
  let keypair = KeyPair::new_ed25519().unwrap();
  c.bench_function("generate did", |b| b.iter(|| generate_did(&keypair)));
}

fn bench_generate_doc_chain(c: &mut Criterion) {
  let (doc, _) = setup_diff_chain_bench();
  c.bench_function("generate document chain", |b| b.iter(|| create_diff_chain(doc.clone())));
}

fn bench_diff_chain_updates(c: &mut Criterion) {
  static ITERATIONS: &[usize] = &[1, 10, 100, 1000];
  let (doc, keys) = setup_diff_chain_bench();

  let mut group = c.benchmark_group("update diff chain");
  for size in ITERATIONS.iter() {
    let mut chain: DocumentChain;
    chain = DocumentChain::new(AuthChain::new(doc.clone()).unwrap());
    update_diff_chain(*size, &mut chain, &keys);
    // group.throughput(Throughput::Elements(*size as u64));
    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_| {
      b.iter(|| update_diff_chain(1, &mut chain.clone(), &keys));
    });
  }
  group.finish();
}

criterion_group!(
  benches,
  bench_generate_signed_document,
  bench_generate_did,
  bench_generate_doc_chain,
  bench_diff_chain_updates
);
criterion_main!(benches);
