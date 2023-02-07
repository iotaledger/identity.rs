use std::cell::Ref;
use std::sync::Arc;
use std::rc::Rc; 
use tokio::sync::RwLock; 
use tokio::sync::RwLockReadGuard; 
use std::cell::RefCell; 

use identity_document::document::CoreDocument;


const PRESENTATION_JSON: &str =
include_str!("../../identity_credential/tests/fixtures/signed_presentation/presentation.json");
const HOLDER_FOO_DOC_JSON: &str =
include_str!("../../identity_credential/tests/fixtures/signed_presentation/subject_foo_doc.json");
const ISSUER_IOTA_DOC_JSON: &str =
include_str!("../../identity_credential/tests/fixtures/signed_presentation/issuer_iota_doc.json");
const ISSUER_BAR_DOC_JSON: &str =
include_str!("../../identity_credential/tests/fixtures/signed_presentation/issuer_bar_doc.json");


enum ArcLockDocumentTypes {
    Core(Arc<RwLock<CoreDocument>>), 
    // Emulate some other type since we can't import IotaDocument here
    Iota(Arc<RwLock<(CoreDocument,usize)>>),
  }

enum ReadLockedDocumentTypes<'a> {
    Core(RwLockReadGuard<'a, CoreDocument>),
    Iota(RwLockReadGuard<'a, (CoreDocument, usize)>)
}

impl ArcLockDocumentTypes{
  fn to_read_locked_document_types(&self) -> ReadLockedDocumentTypes<'_> {
    match self {
      Self::Core(doc) => ReadLockedDocumentTypes::Core(doc.blocking_read()), 
      Self::Iota(doc) => ReadLockedDocumentTypes::Iota(doc.blocking_read())
    }
  }
}
impl<'a> AsRef<CoreDocument> for ReadLockedDocumentTypes<'a> {
  fn as_ref(&self) -> &CoreDocument {
      match self {
        Self::Core(doc) => &doc, 
        Self::Iota(doc) => &doc.0
      }
  }
}

enum RefCellDocumentTypes {
  Core(Rc<RefCell<CoreDocument>>),
  Iota(Rc<RefCell<(CoreDocument, usize)>>)
}

impl RefCellDocumentTypes {
  fn to_read_refcell_locked_document_types(&self) -> ReadRefcellLockedDocumentTypes<'_> {
    match self {
      Self::Core(doc) => ReadRefcellLockedDocumentTypes::Core(doc.borrow()), 
      Self::Iota(doc) => ReadRefcellLockedDocumentTypes::Iota(doc.borrow())
    }
  }
}


enum ReadRefcellLockedDocumentTypes<'a> {
  Core(Ref<'a, CoreDocument>),
  Iota(Ref<'a, (CoreDocument, usize)>)
}

impl<'a> AsRef<CoreDocument> for ReadRefcellLockedDocumentTypes<'a> {
  fn as_ref(&self) -> &CoreDocument {
      match self {
        Self::Core(doc_ref) => &doc_ref, 
        Self::Iota(doc_ref) => &doc_ref.0
      }
  }
}


