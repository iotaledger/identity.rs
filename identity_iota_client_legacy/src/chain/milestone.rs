// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::cmp::Ordering;

use futures::stream::FuturesUnordered;
use futures::TryStreamExt;
use itertools::Itertools;

use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::TangleRef;

/// Fetches the milestones of messages and sorts them in ascending order of the milestone
/// index that references them. Messages not referenced by a milestone are filtered out.
/// If multiple messages are referenced by the same milestone, they will be sorted
/// by [`MessageId`](crate::tangle::MessageId).
///
/// NOTE: this will NOT fetch milestones if only one message is present.
///
/// # Errors
///
/// [`ClientError`](crate::error::Error::ClientError) if fetching a milestone fails.
pub(crate) async fn sort_by_milestone<T: TangleRef>(messages: Vec<T>, client: &Client) -> Result<Vec<T>> {
  if messages.len() == 1 || messages.is_empty() {
    return Ok(messages);
  }

  // Fetch metadata from the Tangle.
  let milestones: Vec<(Option<u32>, T)> = messages
    .into_iter()
    .map(|message| async {
      client
        .client
        .get_message()
        .metadata(message.message_id())
        .await
        .map(|metadata| (metadata.referenced_by_milestone_index, message))
    })
    .collect::<FuturesUnordered<_>>()
    .try_collect()
    .await?;

  let sorted: Vec<T> = sort_by_milestone_index(milestones);
  Ok(sorted)
}

/// Sort by milestone index in ascending order, breaking ties by `message_id`.
fn sort_by_milestone_index<T: TangleRef>(messages_milestones: Vec<(Option<u32>, T)>) -> Vec<T> {
  messages_milestones
    .into_iter()
    .filter_map(|(milestone_index, message)|
      // Ignore any messages not referenced by a milestone.
      milestone_index.map(|index| (index, message)))
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
  use identity_iota_core_legacy::did::IotaDID;
  use identity_iota_core_legacy::tangle::MessageId;

  use super::*;

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
    let m0 = FakeTangleRef(MessageId::new([0_u8; 32]));
    let m1 = FakeTangleRef(MessageId::new([1_u8; 32]));
    let m2 = FakeTangleRef(MessageId::new([2_u8; 32]));
    let m3 = FakeTangleRef(MessageId::new([3_u8; 32]));
    let m4 = FakeTangleRef(MessageId::new([4_u8; 32]));

    let unsorted = vec![(Some(1), m4), (Some(1), m0), (Some(0), m1), (None, m2), (Some(0), m3)];
    let sorted = sort_by_milestone_index(unsorted);
    assert_eq!(sorted, vec![m1, m3, m0, m4]);
  }
}
