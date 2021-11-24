// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::tangle::TangleRef;
use crate::Error::MilestoneError;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use iota_client::Client as IotaClient;
use itertools::Itertools;
use std::cmp::Ordering;

/// Fetches the milestones of messages and sorts them in ascending order of the milestone index
/// that references them. If multiple messages are referenced by the same milestone, they will
/// be sorted by [`MessageId`](crate::tangle::MessageId).
///
/// NOTE: this will NOT fetch milestones if only one message is present.
///
/// # Errors
///
/// [`MilestoneError`] if fetching a milestone fails.
pub(crate) async fn sort_by_milestone<T: TangleRef>(client: &IotaClient, messages: Vec<T>) -> Result<Vec<T>> {
  if messages.len() == 1 || messages.is_empty() {
    return Ok(messages);
  }

  // Fetch metadata from the Tangle.
  let milestones: Vec<(Option<u32>, T)> = messages
    .into_iter()
    .map(|message| async {
      client
        .get_message()
        .metadata(message.message_id())
        .await
        .map(|metadata| (metadata.referenced_by_milestone_index, message))
    })
    .collect::<FuturesUnordered<_>>()
    .try_collect()
    .await
    .map_err(|_| MilestoneError)?; // TODO: return actual error / error message too?

  let sorted = sort_by_milestone_index(milestones);
  Ok(sorted)
}

/// Sort by milestone index in ascending order, breaking ties by `message_id`.
fn sort_by_milestone_index<T: TangleRef>(messages_milestones: Vec<(Option<u32>, T)>) -> Vec<T> {
  messages_milestones
    .into_iter()
    .filter_map(|(milestone_index, message)| {
      if let Some(milestone_index) = milestone_index {
        Some((milestone_index, message))
      } else {
        // Ignore any messages not referenced by a milestone.
        None
      }
    })
    .sorted_unstable_by(|(a_milestone, a_message), (b_milestone, b_message)| {
      let milestone_cmp: Ordering = a_milestone.cmp(b_milestone);
      if milestone_cmp == Ordering::Equal {
        // Sort by message_id when both are referenced by the same milestone.
        a_message.message_id().cmp(b_message.message_id())
      } else {
        milestone_cmp
      }
    })
    .map(|(_, message)| message)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::did::IotaDID;
  use crate::tangle::MessageId;

  #[derive(Clone, Copy, Debug, PartialEq, Eq)]
  struct FakeTangleRef(MessageId);
  impl TangleRef for FakeTangleRef {
    fn did(&self) -> &IotaDID {
      unimplemented!()
    }

    fn message_id(&self) -> &MessageId {
      &self.0
    }

    fn set_message_id(&mut self, _message_id: MessageId) {
      unimplemented!()
    }

    fn previous_message_id(&self) -> &MessageId {
      unimplemented!()
    }

    fn set_previous_message_id(&mut self, _message_id: MessageId) {
      unimplemented!()
    }
  }

  #[test]
  fn test_sort_by_milestone_index() {
    let a = FakeTangleRef(MessageId::new([0_u8; 32]));
    let b = FakeTangleRef(MessageId::new([1_u8; 32]));
    let c = FakeTangleRef(MessageId::new([2_u8; 32]));
    let d = FakeTangleRef(MessageId::new([3_u8; 32]));
    let e = FakeTangleRef(MessageId::new([4_u8; 32]));

    let unsorted = vec![(Some(1), e), (Some(1), a), (Some(0), b), (None, c), (Some(0), d)];
    let sorted = sort_by_milestone_index(unsorted);
    assert_eq!(sorted, vec![b, d, a, e]);
  }
}
