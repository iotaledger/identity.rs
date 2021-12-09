// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use identity_core::diff::Diff;
use identity_core::diff::Error;
use identity_core::diff::Result;
use identity_did::diff::DiffDocument;
use identity_did::document::CoreDocument;

use crate::document::diff_iota_document_metadata::DiffIotaDocumentMetadata;
use crate::document::IotaDocument;
use crate::document::IotaDocumentMetadata;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffIotaDocument {
  #[serde(skip_serializing_if = "Option::is_none")]
  document: Option<DiffDocument>,
  #[serde(skip_serializing_if = "Option::is_none")]
  metadata: Option<DiffIotaDocumentMetadata>,
}

impl Diff for IotaDocument {
  type Type = DiffIotaDocument;

  fn diff(&self, other: &Self) -> Result<Self::Type> {
    Ok(DiffIotaDocument {
      document: if self.core_document() == other.core_document() {
        None
      } else {
        Some(Diff::diff(self.core_document(), other.core_document())?)
      },
      metadata: if self.metadata == other.metadata {
        None
      } else {
        Some(Diff::diff(&self.metadata, &other.metadata)?)
      },
    })
  }

  fn merge(&self, diff: Self::Type) -> Result<Self> {
    let document: CoreDocument = diff
      .document
      .map(|value| self.core_document().merge(value))
      .transpose()?
      .unwrap_or_else(|| self.core_document().clone());

    let metadata: IotaDocumentMetadata = diff
      .metadata
      .map(|value| self.metadata.merge(value))
      .transpose()?
      .unwrap_or_else(|| self.metadata.clone());

    IotaDocument::try_from_core(document, metadata).map_err(identity_core::diff::Error::merge)
  }

  fn from_diff(diff: Self::Type) -> Result<Self> {
    let document: CoreDocument = diff
      .document
      .map(CoreDocument::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `document`"))?;

    let metadata: IotaDocumentMetadata = diff
      .metadata
      .map(IotaDocumentMetadata::from_diff)
      .transpose()?
      .ok_or_else(|| Error::convert("Missing field `metadata`"))?;

    IotaDocument::try_from_core(document, metadata).map_err(identity_core::diff::Error::convert)
  }

  fn into_diff(self) -> Result<Self::Type> {
    Ok(DiffIotaDocument {
      document: Some(self.document.into_diff()?),
      metadata: Some(self.metadata.into_diff()?),
    })
  }
}
