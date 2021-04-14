// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use identity::crypto::*;
use identity::iota::{did::Document, DID};

fn generate_signed_document(n: u64) {
  (0..n).into_iter().for_each(|_| {
    let keypair = KeyPair::new_ed25519().unwrap();
    let mut document: Document = Document::from_keypair(&keypair).unwrap();
    document.sign(keypair.secret()).unwrap();
  });
}

fn generate_did(n: u64) {
  (0..n).into_iter().for_each(|_| {
    let keypair = KeyPair::new_ed25519().unwrap();
    let did = DID::new(keypair.public().as_ref()).unwrap();
  });
}

fn bench_generate_signed_document(c: &mut Criterion) {
  c.bench_function("generate signed document", |b| {
    b.iter(|| generate_signed_document(black_box(10000)))
  });
}

fn bench_generate_did(c: &mut Criterion) {
  c.bench_function("generate did", |b| b.iter(|| generate_did(black_box(10000))));
}

criterion_group!(benches, bench_generate_signed_document, bench_generate_did);
criterion_main!(benches);
