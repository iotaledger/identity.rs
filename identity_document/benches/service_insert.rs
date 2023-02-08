// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use criterion::Criterion;
// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use criterion::criterion_group;
use criterion::criterion_main;
use identity_core::convert::FromJson;
use identity_document::document::CoreDocument;
use identity_document::service::Service;

use tokio::sync::RwLock;

struct LockDocument(Rc<RwLock<CoreDocument>>);

struct ArcLockDocument(Arc<RwLock<CoreDocument>>);
struct RefCellDocument(Rc<RefCell<CoreDocument>>);

const JSON_DOC_SHORT: &str = include_str!("./json_documents/small.json");

const JSON_DOC_DID_KEY: &str = include_str!("./json_documents/did_key.json");

// This is not a realistic document in any way.
const JSON_DOCUMENT_LARGE: &str = include_str!("./json_documents/large.json");

fn small_doc() -> CoreDocument {
  CoreDocument::from_json(JSON_DOC_SHORT).unwrap()
}

fn key_doc() -> CoreDocument {
  CoreDocument::from_json(JSON_DOC_DID_KEY).unwrap()
}

fn large_doc() -> CoreDocument {
  CoreDocument::from_json(JSON_DOCUMENT_LARGE).unwrap()
}

fn service() -> Service {
  Service::from_json(
    r#"{
        "id":"did:example:123#benchservice",
        "type": "BenchService", 
        "serviceEndpoint": "https://foo.example.com"
      }"#,
  )
  .unwrap()
}

fn bench_insert(c: &mut Criterion) {
  let mut group = c.benchmark_group("insert_service");
  let documents = [small_doc(), key_doc(), large_doc()];
  let ids = ["small", "key", "large"];

  for (document, id) in documents.into_iter().zip(ids.into_iter()) {
    group.bench_with_input(format!("{id} without any locks"), &document, |b, doc| {
      b.iter_batched(
        || (service(), doc.clone()),
        |(service, mut doc)| {
          doc.insert_service(service).unwrap();
        },
        criterion::BatchSize::SmallInput,
      );
    });
    group.bench_with_input(format!("{id} with rc tokio RwLock"), &document, |b, doc| {
      b.iter_batched(
        || (service(), LockDocument(Rc::new(tokio::sync::RwLock::new(doc.clone())))),
        |(service, lock_doc)| {
          lock_doc.0.blocking_write().insert_service(service).unwrap();
        },
        criterion::BatchSize::SmallInput,
      );
    });

    group.bench_with_input(format!("{id} with arc tokio RwLock"), &document, |b, doc| {
      b.iter_batched(
        || {
          (
            service(),
            ArcLockDocument(Arc::new(tokio::sync::RwLock::new(doc.clone()))),
          )
        },
        |(service, lock_doc)| {
          lock_doc.0.blocking_write().insert_service(service).unwrap();
        },
        criterion::BatchSize::SmallInput,
      );
    });

    group.bench_with_input(format!("{id} with RefCell"), &document, |b, doc| {
      b.iter_batched(
        || (service(), RefCellDocument(Rc::new(RefCell::new(doc.clone())))),
        |(service, refcell_doc)| {
          refcell_doc.0.borrow_mut().insert_service(service).unwrap();
        },
        criterion::BatchSize::SmallInput,
      );
    });
  }
  group.finish();
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
