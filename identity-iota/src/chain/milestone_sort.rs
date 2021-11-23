// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::tangle::TangleRef;
use crate::Error::MilestoneError;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use iota_client::bee_rest_api::types::responses::MessageMetadataResponse;
use iota_client::Client as IotaClient;
use std::collections::HashMap;

/// Fetches the milestones of documents and sorts them in ascending order of the milestone index
/// that references them.
/// If multiple documents are referenced by the same milestone, they will be sorted by message_id.
///
///
/// #Errors
///
/// [`MilestoneError`] if fetching a milestone fails.
pub(crate) async fn sort_by_milestone<T: TangleRef>(client: &IotaClient, documents: Vec<T>) -> Result<Vec<T>> {
  if documents.len() == 1 || documents.is_empty() {
    return Ok(documents);
  }

  let mut milestone_index = fetch_milestones(client, documents).await?;
  let mut milestones: Vec<u32> = milestone_index.get_milestones();
  milestones.sort_unstable();

  let mut sorted_documents = Vec::with_capacity(milestone_index.total_docs);
  for milestone in milestones {
    if let Some(docs_with_same_milestone) = milestone_index.remove(milestone) {
      for doc in docs_with_same_milestone {
        sorted_documents.push(doc);
      }
    }
  }
  Ok(sorted_documents)
}

async fn fetch_milestones<T: TangleRef>(client: &IotaClient, documents: Vec<T>) -> Result<MilestoneIndex<T>> {
  let mut milestone_index = MilestoneIndex::new();

  let all_metadata: Vec<MessageMetadataResponse> = documents
    .iter()
    .map(|doc| client.get_message().metadata(doc.message_id()))
    .collect::<FuturesUnordered<_>>()
    .try_collect()
    .await
    .map_err(|_| MilestoneError)?;

  for doc in documents {
    let metadata_doc = all_metadata
      .iter()
      .find(|ms| ms.message_id == doc.message_id().to_string())
      .ok_or(MilestoneError)?;

    milestone_index.insert(metadata_doc.referenced_by_milestone_index.ok_or(MilestoneError)?, doc);
  }
  Ok(milestone_index)
}

struct MilestoneIndex<T: TangleRef> {
  index: HashMap<u32, Vec<T>>,
  total_docs: usize,
}

impl<T: TangleRef> MilestoneIndex<T> {
  fn new() -> MilestoneIndex<T> {
    Self {
      index: HashMap::new(),
      total_docs: 0,
    }
  }

  fn insert(&mut self, milestone: u32, doc: T) {
    if let Some(v) = self.index.get_mut(&milestone) {
      v.push(doc);
      v.sort_by(|a, b| a.message_id().cmp(b.message_id()))
    } else {
      let v = vec![doc];
      self.index.insert(milestone, v);
    }
    self.total_docs += 1;
  }

  fn remove(&mut self, milestone: u32) -> Option<Vec<T>> {
    self.index.remove(&milestone)
  }

  fn get_milestones(&self) -> Vec<u32> {
    self.index.keys().copied().collect()
  }
}
