// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::Throughput;
use identity_core::convert::FromJson;
// This is a benchmark measuring the time it takes to deserialize a DID document from JSON.
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use identity_document::document::CoreDocument;

const JSON_DOC_SHORT: &str = include_str!("./json_documents/small.json");

const JSON_DOC_DID_KEY: &str = include_str!("./json_documents/did_key.json");

// This is not a realistic document in any way.
const JSON_DOCUMENT_LARGE: &str = include_str!("./json_documents/large.json");

fn deserialize_json_document(c: &mut Criterion) {
  let mut group = c.benchmark_group("deserialize_json_document");
  for (json, name) in [
    (JSON_DOC_SHORT, "short document"),
    (JSON_DOC_DID_KEY, "did:key document"),
    (JSON_DOCUMENT_LARGE, "large document"),
  ] {
    group.throughput(Throughput::Bytes(json.as_bytes().len() as u64));
    group.bench_with_input(
      BenchmarkId::from_parameter(format!("{name}, document size: {} bytes", json.as_bytes().len())),
      json,
      |b, json| {
        b.iter(|| {
          let doc: Result<CoreDocument, _> = CoreDocument::from_json(json);
          assert!(doc.is_ok(), "bench {name} failed: {doc:#?}");
        })
      },
    );
  }
  group.finish();
}

criterion_group!(benches, deserialize_json_document);
criterion_main!(benches);
