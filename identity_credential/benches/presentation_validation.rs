use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use identity_core::convert::FromJson;
use identity_credential::presentation::Presentation;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::PresentationValidator;
use identity_credential::validator::SubjectHolderRelationship;
use identity_document::verifiable::VerifierOptions;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::OwnedRwLockReadGuard;
use tokio::sync::RwLock;

use identity_document::document::CoreDocument;

// Emulates validate_presentation in the Wasm bindings with enum CoreDocumentView {Core(Arc<RwLock<CoreDocument>>),
// Iota(Arc<RwLock<IotaDocument>>)}
fn validate_presentation_lock_document(
  presentation: &Presentation,
  holder: &dyn Fn() -> LockDocumentTypes,
  issuers: Box<dyn Iterator<Item = Box<dyn Fn() -> LockDocumentTypes>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
  let issuers: Vec<ReadLockedDocumentTypes> = issuers
    .into_iter()
    .map(|fn_ptr| fn_ptr().into_read_locked_document_types())
    .collect::<Result<Vec<ReadLockedDocumentTypes>, _>>()?;

  let holder = holder().into_read_locked_document_types()?;

  PresentationValidator::validate(
    presentation,
    &holder,
    &issuers,
    &PresentationValidationOptions::new()
      .presentation_verifier_options(
        VerifierOptions::new().challenge(
          presentation
            .proof()
            .map(|proof| proof.challenge.clone().unwrap())
            .unwrap(),
        ),
      )
      .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
    identity_credential::validator::FailFast::FirstError,
  )
  .map_err(Into::into)
}

// Emulates validate_presentation in the Wasm bindings with enum CoreDocumentView {Core(Rc<RefCell<CoreDocument>>),
// Iota(Rc<RefCell<IotaDocument>>)}
fn validate_presentation_refcelll_document(
  presentation: &Presentation,
  holder: &dyn Fn() -> RefCellDocumentTypes,
  issuers: Box<dyn Iterator<Item = Box<dyn Fn() -> RefCellDocumentTypes>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
  // Cannot go straight to Vec<ReadRefCellLockedDocumentTypes> because of lifetimes.
  let issuer_docs: Vec<RefCellDocumentTypes> = issuers.into_iter().map(|fn_ptr| fn_ptr()).collect();

  let issuers: Vec<ReadRefcellLockedDocumentTypes<'_>> = issuer_docs
    .iter()
    .map(|doc| doc.to_read_refcell_locked_document_types())
    .collect();

  let holder = holder();
  let holder = holder.to_read_refcell_locked_document_types();

  PresentationValidator::validate(
    presentation,
    &holder,
    &issuers,
    &PresentationValidationOptions::new()
      .presentation_verifier_options(
        VerifierOptions::new().challenge(
          presentation
            .proof()
            .map(|proof| proof.challenge.clone().unwrap())
            .unwrap(),
        ),
      )
      .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
    identity_credential::validator::FailFast::FirstError,
  )
  .map_err(Into::into)
}

// Emulates validate_presentation in the Wasm bindings without any CoreDocumentView Interface by directly cloning the
// documents instead.
fn validate_presentation_clone_documents(
  presentation: &Presentation,
  holder: &dyn Fn() -> CoreDocument,
  issuers: Box<dyn Iterator<Item = Box<dyn Fn() -> CoreDocument>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
  let issuers: Vec<CoreDocument> = issuers.into_iter().map(|fn_ptr| fn_ptr()).collect();
  let holder = holder();

  PresentationValidator::validate(
    presentation,
    &holder,
    &issuers,
    &PresentationValidationOptions::new()
      .presentation_verifier_options(
        VerifierOptions::new().challenge(
          presentation
            .proof()
            .map(|proof| proof.challenge.clone().unwrap())
            .unwrap(),
        ),
      )
      .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
    identity_credential::validator::FailFast::FirstError,
  )
  .map_err(Into::into)
}

#[derive(Clone)]
enum LockDocumentTypes {
  Core(Arc<RwLock<CoreDocument>>),
  // Emulate some other type since we can't import IotaDocument here
  Iota(Arc<RwLock<(CoreDocument, usize)>>),
}

enum ReadLockedDocumentTypes {
  Core(OwnedRwLockReadGuard<CoreDocument>),
  Iota(OwnedRwLockReadGuard<(CoreDocument, usize)>),
}

impl LockDocumentTypes {
  fn into_read_locked_document_types(
    self,
  ) -> Result<ReadLockedDocumentTypes, Box<dyn std::error::Error + Send + Sync + 'static>> {
    match self {
      Self::Core(doc) => Ok(ReadLockedDocumentTypes::Core(
        doc.try_read_owned().map_err(|_| "contention")?,
      )),
      Self::Iota(doc) => Ok(ReadLockedDocumentTypes::Iota(
        doc.try_read_owned().map_err(|_| "contention")?,
      )),
    }
  }
}
impl<'a> AsRef<CoreDocument> for ReadLockedDocumentTypes {
  fn as_ref(&self) -> &CoreDocument {
    match self {
      Self::Core(doc) => &doc,
      Self::Iota(doc) => &doc.0,
    }
  }
}

#[derive(Clone)]
enum RefCellDocumentTypes {
  Core(Rc<RefCell<CoreDocument>>),
  Iota(Rc<RefCell<(CoreDocument, usize)>>),
}

impl RefCellDocumentTypes {
  fn to_read_refcell_locked_document_types(&self) -> ReadRefcellLockedDocumentTypes<'_> {
    match self {
      Self::Core(doc) => ReadRefcellLockedDocumentTypes::Core(doc.borrow()),
      Self::Iota(doc) => ReadRefcellLockedDocumentTypes::Iota(doc.borrow()),
    }
  }
}

enum ReadRefcellLockedDocumentTypes<'a> {
  Core(Ref<'a, CoreDocument>),
  Iota(Ref<'a, (CoreDocument, usize)>),
}

impl<'a> AsRef<CoreDocument> for ReadRefcellLockedDocumentTypes<'a> {
  fn as_ref(&self) -> &CoreDocument {
    match self {
      Self::Core(doc_ref) => &doc_ref,
      Self::Iota(doc_ref) => &doc_ref.0,
    }
  }
}

// Similar fixtures to those used when testing `Resolver::verify_presentation` in the `identity_resolver` crate.
const PRESENTATION_JSON: &str =
  include_str!("../../identity_credential/tests/fixtures/signed_presentation/presentation.json");

fn holder_foo_doc() -> CoreDocument {
  let json: &str = r#"
  {
    "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
    "verificationMethod": [
      {
        "id": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5#root",
        "controller": "did:foo:586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "z586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5"
      },
      {
        "id": "did:example:123#key2",
        "controller": "did:example:1234",
        "type": "Ed25519VerificationKey2018",
        "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
    },
    {
        "id": "did:example:123#key3",
        "controller": "did:example:1234",
        "type": "Ed25519VerificationKey2018",
        "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
    },
    {
        "id": "did:example:123#key4",
        "controller": "did:example:1234",
        "type": "Ed25519VerificationKey2018",
        "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
    },
    {
        "id": "did:example:123#key5",
        "controller": "did:example:1234",
        "type": "Ed25519VerificationKey2018",
        "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
    },
    {
        "id": "did:example:123#key6",
        "controller": "did:example:1234",
        "type": "Ed25519VerificationKey2018",
        "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
    }
  ],
"authentication": [
  {
    "id": "did:example:123#z6MkecaLyHuYWkayBDLw5ihndj3T1m6zKTGqau3A51G7RBf3EMB1",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf"
  },
  "did:example:123#key1", 
  {
    "id": "did:example:123#z6MkecaLyHuYWkayBDLw5ihndj3T1m6zKTGqau3A51G7RBf3EMB2",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf"
  },
  "did:example:123#key4",
  "did:example:123#key2", 
  {
    "id": "did:example:123#z6MkecaLyHuYWkayBDLw5ihndj3T1m6zKTGqau3A51G7RBf3EMB3",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf"
  },
  {
    "id": "did:example:123#z6MkecaLyHuYWkayBDLw5ihndj3T1m6zKTGqau3A51G7RBf3EMB4",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf"
  }
],
"capabilityInvocation": [
  {
    "id": "did:example:123#z6MkhdmzFu659ZJ4XKj31vtEDmjvsi5yDZG5L7Caz63oP39k",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z4BWwfeqdp1obQptLLMvPNgBw48p7og1ie6Hf9p5nTpNN"
  },
  "did:example:123#key1",
  {
    "id": "did:example:123#z6MkhdmzFu659ZJ4XKj31vtEDmjvsi5yDZG5L7Caz63oP39kEMB2",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z4BWwfeqdp1obQptLLMvPNgBw48p7og1ie6Hf9p5nTpNN"
  },
  {
    "id": "did:example:123#z6MkhdmzFu659ZJ4XKj31vtEDmjvsi5yDZG5L7Caz63oP39kEMB3",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z4BWwfeqdp1obQptLLMvPNgBw48p7og1ie6Hf9p5nTpNN"
  },
  "did:example:123#key5" 
],
"capabilityDelegation": [
  {
    "id": "did:example:123#z6Mkw94ByR26zMSkNdCUi6FNRsWnc2DFEeDXyBGJ5KTzSWyi",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "zHgo9PAmfeoxHG8Mn2XHXamxnnSwPpkyBHAMNF3VyXJCL"
  },
  "did:example:123#key6"
],
"assertionMethod": [
  {
    "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomY",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
  },
  {
    "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomYEMB2",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
  },
  {
    "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomYEMB3",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
  },
  {
    "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomYEMB4",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
  },
  {
    "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomYEMB5",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
  },
  {
    "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomYEMB6",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
  },
  "did:example:123#key4",
  {
    "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomYEMB7",
    "type": "Ed25519VerificationKey2018",
    "controller": "did:example:123",
    "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
  }
],
"service": [{
    "id":"did:example:123#linked-domain",
    "type": "LinkedDomains", 
    "serviceEndpoint": "https://bar.example.com"
  }]
  }"#;

  CoreDocument::from_json(json).unwrap()
}

fn issuer_bar_doc() -> CoreDocument {
  let json: &str = r#"
  {
    "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
    "verificationMethod": [
      {
        "id": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr#root",
        "controller": "did:bar:Hyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "zHyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr"
      }
    ],
    "authentication": [
      "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
    ],
    "assertionMethod": [
      "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
    ],
    "capabilityDelegation": [
      "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
    ],
    "capabilityInvocation": [
      "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
    ],
    "keyAgreement": [{
      "id": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6LSj72tK8brWgZja8NLRwPigth2T9QRiG1uH9oKZuKjdh9p",
      "type": "X25519KeyAgreementKey2019",
      "controller": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
      "publicKeyMultibase": "z6LSj72tK8brWgZja8NLRwPigth2T9QRiG1uH9oKZuKjdh9p"
    }]
  }
  "#;
  CoreDocument::from_json(json).unwrap()
}

fn issuer_iota_doc() -> (CoreDocument, usize) {
  let json: &str = r#"{
    "id": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "verificationMethod": [
      {
        "id": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa#issuerKey",
        "controller": "did:iota:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "zFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z"
      }
    ],
    "service": [{
      "id":"did:example:123#linked-domain",
      "type": "LinkedDomains", 
      "serviceEndpoint": "https://bar.example.com"
    }]
  }"#;

  (CoreDocument::from_json(json).unwrap(), 42)
}

fn bench_insert(c: &mut Criterion) {
  let mut group = c.benchmark_group("presentation validation bindings emulation");
  let holder_doc = holder_foo_doc();
  let issuer_bar_doc = issuer_bar_doc();
  let presentation: Presentation = Presentation::from_json(PRESENTATION_JSON).unwrap();

  group.bench_function("validate_presentation_rust_single_doc_type", |b| {
    b.iter_batched(
      || {
        (
          presentation.clone(),
          holder_doc.clone(),
          vec![issuer_bar_doc.clone(), issuer_iota_doc().0],
        )
      },
      |(presentation, holder, issuers)| {
        assert!(PresentationValidator::validate(
          &presentation,
          &holder,
          &issuers,
          &PresentationValidationOptions::new()
            .presentation_verifier_options(
              VerifierOptions::new().challenge(
                presentation
                  .proof()
                  .map(|proof| proof.challenge.clone().unwrap())
                  .unwrap(),
              ),
            )
            .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
          identity_credential::validator::FailFast::FirstError,
        )
        .map_err(|error| Box::new(error) as Box<dyn std::error::Error + Send + Sync + 'static>)
        .is_ok());
      },
      criterion::BatchSize::SmallInput,
    );
  });

  group.bench_function("validate_presentation_cloning", |b| {
    b.iter_batched(
      || {
        let holder = holder_doc.clone();
        let issuer_bar_doc = issuer_bar_doc.clone();
        let issuer_iota_doc = issuer_iota_doc();
        let holder_fn = Box::new(move || holder.clone()) as Box<dyn Fn() -> CoreDocument>;
        let issuer_bar_fn = Box::new(move || issuer_bar_doc.clone()) as Box<dyn Fn() -> CoreDocument>;
        let issuer_iota_fn = Box::new(move || issuer_iota_doc.0.clone()) as Box<dyn Fn() -> CoreDocument>;
        let issuers = [issuer_bar_fn, issuer_iota_fn];
        (
          presentation.clone(),
          holder_fn,
          Box::new(issuers.into_iter()) as Box<dyn Iterator<Item = Box<dyn Fn() -> CoreDocument>>>,
        )
      },
      |(presentation, holder, issuers)| {
        assert!(validate_presentation_clone_documents(&presentation, &holder, issuers).is_ok());
      },
      criterion::BatchSize::SmallInput,
    );
  });

  group.bench_function("validate_presentation_rwlock", |b| {
    b.iter_batched(
      || {
        let holder = LockDocumentTypes::Core(Arc::new(RwLock::new(holder_doc.clone())));
        let issuer_bar_doc = LockDocumentTypes::Core(Arc::new(RwLock::new(issuer_bar_doc.clone())));
        let issuer_iota_doc = LockDocumentTypes::Iota(Arc::new(RwLock::new(issuer_iota_doc())));
        let holder_fn = Box::new(move || holder.clone()) as Box<dyn Fn() -> LockDocumentTypes>;
        let issuer_bar_fn = Box::new(move || issuer_bar_doc.clone()) as Box<dyn Fn() -> LockDocumentTypes>;
        let issuer_iota_fn = Box::new(move || issuer_iota_doc.clone()) as Box<dyn Fn() -> LockDocumentTypes>;
        let issuers = [issuer_bar_fn, issuer_iota_fn];
        (
          presentation.clone(),
          holder_fn,
          Box::new(issuers.into_iter()) as Box<dyn Iterator<Item = Box<dyn Fn() -> LockDocumentTypes>>>,
        )
      },
      |(presentation, holder, issuers)| {
        assert!(validate_presentation_lock_document(&presentation, &holder, issuers).is_ok());
      },
      criterion::BatchSize::SmallInput,
    );
  });

  group.bench_function("validate_presentation_refcell", |b| {
    b.iter_batched(
      || {
        let holder = RefCellDocumentTypes::Core(Rc::new(RefCell::new(holder_doc.clone())));
        let issuer_bar_doc = RefCellDocumentTypes::Core(Rc::new(RefCell::new(issuer_bar_doc.clone())));
        let issuer_iota_doc = RefCellDocumentTypes::Iota(Rc::new(RefCell::new(issuer_iota_doc())));
        let holder_fn = Box::new(move || holder.clone()) as Box<dyn Fn() -> RefCellDocumentTypes>;
        let issuer_bar_fn = Box::new(move || issuer_bar_doc.clone()) as Box<dyn Fn() -> RefCellDocumentTypes>;
        let issuer_iota_fn = Box::new(move || issuer_iota_doc.clone()) as Box<dyn Fn() -> RefCellDocumentTypes>;
        let issuers = [issuer_bar_fn, issuer_iota_fn];
        (
          presentation.clone(),
          holder_fn,
          Box::new(issuers.into_iter()) as Box<dyn Iterator<Item = Box<dyn Fn() -> RefCellDocumentTypes>>>,
        )
      },
      |(presentation, holder, issuers)| {
        assert!(validate_presentation_refcelll_document(&presentation, &holder, issuers).is_ok());
      },
      criterion::BatchSize::SmallInput,
    );
  });

  group.finish();
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
