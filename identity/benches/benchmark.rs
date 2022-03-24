// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(deprecated)]

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

use identity::crypto::KeyPair;
use identity::iota::DocumentChain;
use identity::iota::IntegrationChain;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaDocument;
use identity_core::crypto::KeyType;

use self::diff_chain::setup_diff_chain_bench;
use self::diff_chain::update_diff_chain;
use self::diff_chain::update_integration_chain;

mod diff_chain;

fn generate_signed_document(keypair: &KeyPair) {
  let mut document: IotaDocument = IotaDocument::new(keypair).unwrap();

  document
    .sign_self(
      keypair.private(),
      document.default_signing_method().unwrap().id().clone(),
    )
    .unwrap();
}

fn generate_did(keypair: &KeyPair) {
  let _ = IotaDID::new(keypair.public().as_ref()).unwrap();
}

fn bench_generate_signed_document(c: &mut Criterion) {
  let keypair = KeyPair::new(KeyType::Ed25519).unwrap();

  c.bench_function("generate signed document", |b| {
    b.iter(|| generate_signed_document(&keypair))
  });
}

fn bench_generate_did(c: &mut Criterion) {
  let keypair = KeyPair::new(KeyType::Ed25519).unwrap();
  c.bench_function("generate did", |b| b.iter(|| generate_did(&keypair)));
}

fn bench_diff_chain_updates(c: &mut Criterion) {
  static ITERATIONS: &[usize] = &[1, 10, 100, 1000];

  let (doc, keys) = setup_diff_chain_bench();

  let mut group = c.benchmark_group("update diff chain");
  for size in ITERATIONS.iter() {
    let mut chain: DocumentChain = DocumentChain::new(IntegrationChain::new(doc.clone()).unwrap());

    update_diff_chain(*size, &mut chain, &keys);

    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_| {
      b.iter(|| update_diff_chain(1, &mut chain.clone(), &keys));
    });
  }
  group.finish();
}

fn bench_integration_chain_updates(c: &mut Criterion) {
  static ITERATIONS: &[usize] = &[1, 10, 100, 1000];

  let (doc, keys) = setup_diff_chain_bench();

  let mut group = c.benchmark_group("update integration chain");

  for size in ITERATIONS.iter() {
    let mut chain: DocumentChain = DocumentChain::new(IntegrationChain::new(doc.clone()).unwrap());

    update_integration_chain(*size, &mut chain, &keys);

    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_| {
      b.iter(|| update_integration_chain(1, &mut chain.clone(), &keys));
    });
  }
  group.finish();
}

criterion_group!(
  benches,
  bench_generate_signed_document,
  bench_generate_did,
  bench_diff_chain_updates,
  bench_integration_chain_updates,
);
criterion_main!(benches);
