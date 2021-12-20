// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use num_traits::FromPrimitive;
use serde::Serialize;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, num_derive::FromPrimitive)]
#[non_exhaustive]
/// A possibly acceptable deficiency a credential might have.
pub enum CredentialDeficiency {
  #[doc(hidden)] // hidden until we have decided how to act on deactivated documents
  /// At least one subject document is deactivated
  DeactivatedSubjectDocuments = 0,
  /// The credential has expired
  Expired = 1,
  /// The credential has not yet become active
  // (issuance_date is in the future)
  Dormant = 2,
}

impl CredentialDeficiency {
  /// Provides a description of the category
  pub fn description(&self) -> &'static str {
    match *self {
      Self::DeactivatedSubjectDocuments => "contains subjects with deactivated DID documents",
      Self::Expired => "the expiry date is in the past",
      Self::Dormant => "the activation date is in the future",
    }
  }
  // The number of refutation categories. We do not use strum for this as we do not want to unnecessarily pollute the
  // public API
  pub(super) const COUNT: usize = 3;
}

impl Display for CredentialDeficiency {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.description())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
/// A set containing zero or more variants of [`CredentialDeficiency`]
pub struct CredentialDeficiencySet {
  // true at the i'th slot corresponds to CredentialRefutationCategory::from_usize(i).unwrap()
  slots: [bool; CredentialDeficiency::COUNT],
}

impl CredentialDeficiencySet {
  /// Adds a value to the set.
  /// If the set did not have this value present, `true` is returned.
  /// If the set did have this value present, `false` is returned.
  pub fn insert(&mut self, value: CredentialDeficiency) -> bool {
    let flag = &mut self.slots[value as usize];
    let current = *flag;
    *flag = true;
    !current
  }

  pub fn iter(&self) -> impl Iterator<Item = CredentialDeficiency> + '_ {
    self.slots.iter().enumerate().flat_map(|(index, contained)| {
      if *contained {
        CredentialDeficiency::from_usize(index)
      } else {
        None
      }
    })
  }

  /// The number of elements in this set
  pub fn count(&self) -> usize {
    self.slots.iter().copied().filter(|value| *value).count()
  }

  /// Constructs an empty [`CredentialDeficiencySet`]
  pub fn empty() -> Self {
    Self {
      slots: [false; CredentialDeficiency::COUNT],
    }
  }

  /// Constructs a [`CredentialDeficiencySet`] containing every possible value of [`CredentialDeficiency`]
  pub fn all() -> Self {
    Self {
      slots: [true; CredentialDeficiency::COUNT],
    }
  }
  /// Checks whether the value is contained in the set
  pub fn contains(&self, deficiency: &CredentialDeficiency) -> bool {
    self.slots[*deficiency as usize]
  }
}

impl Extend<CredentialDeficiency> for CredentialDeficiencySet {
  fn extend<T: IntoIterator<Item = CredentialDeficiency>>(&mut self, iter: T) {
    for deficiency in iter {
      if self.slots == [true; CredentialDeficiency::COUNT] {
        break;
      }
      self.insert(deficiency);
    }
  }
}

impl FromIterator<CredentialDeficiency> for CredentialDeficiencySet {
  fn from_iter<T: IntoIterator<Item = CredentialDeficiency>>(iter: T) -> Self {
    let mut refutations = Self::empty();
    refutations.extend(iter);
    refutations
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::*;

  #[test]
  fn credential_refutation_category_count() {
    assert_eq!(
      (0usize..100)
        .map(CredentialDeficiency::from_usize)
        .take_while(|value| value.is_some())
        .flatten()
        .map(|value| value as usize + 1)
        .max()
        .unwrap(),
      CredentialDeficiency::COUNT
    );
  }

  #[test]
  fn credential_refutations_iterator_roundtrip() {
    let categories = [
      CredentialDeficiency::DeactivatedSubjectDocuments,
      CredentialDeficiency::Dormant,
    ];
    let refutations: CredentialDeficiencySet = categories.into_iter().collect();
    let expected_set: HashSet<CredentialDeficiency> = categories.into_iter().collect();
    let round_trip_set: HashSet<CredentialDeficiency> = refutations.iter().collect();
    assert_eq!(expected_set, round_trip_set);
  }

  #[test]
  fn credential_refutations_all_count() {
    assert_eq!(CredentialDeficiencySet::all().count(), CredentialDeficiency::COUNT);
  }

  #[test]
  fn credential_refutations_empty_count() {
    assert_eq!(CredentialDeficiencySet::empty().count(), 0);
  }

  #[test]
  fn credential_refutations_extend_contains() {
    let dormant = CredentialDeficiency::Dormant;
    let deactivated = CredentialDeficiency::DeactivatedSubjectDocuments;
    let mut refutations = CredentialDeficiencySet::empty();
    // check that refutations does not contain dormant at this point
    assert!(!refutations.contains(&dormant));
    refutations.extend([dormant]);
    // check again now after extending
    assert!(refutations.contains(&dormant));
    // now extend with dormant (again), but also deactivated
    refutations.extend([dormant, deactivated]);
    // check that they are deactivated is there
    assert!(refutations.contains(&deactivated));
    // check that dormant is still there
    assert!(refutations.contains(&dormant));
  }

  #[test]
  fn credential_refutations_insert() {
    let mut refutations = CredentialDeficiencySet::empty();
    assert!(refutations.insert(CredentialDeficiency::DeactivatedSubjectDocuments));
    assert!(!refutations.insert(CredentialDeficiency::DeactivatedSubjectDocuments));
  }

  #[test]
  fn credential_refutations_all_contains() {
    let refutations = CredentialDeficiencySet::all();
    for i in 0..CredentialDeficiency::COUNT {
      let category = CredentialDeficiency::from_usize(i).unwrap();
      assert!(refutations.contains(&category));
    }
  }
}
