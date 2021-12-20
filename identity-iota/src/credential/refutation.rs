// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use num_traits::FromPrimitive;
use serde::Serialize;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, num_derive::FromPrimitive)]
#[non_exhaustive]
pub enum CredentialRefutationCategory {
  #[doc(hidden)] // hidden until we have decided how to act on deactivated documents 
  /// At least one subject document is deactivated
  DeactivatedSubjectDocuments = 0,
  /// The credential has expired
  Expired = 1,
  /// The credential has not yet become active
  // (issuance_date is in the future)
  Dormant = 2,
}

impl CredentialRefutationCategory {

  /// Provides a description of the category
  pub fn description(&self) -> &'static str {
    match self {
      &Self::DeactivatedSubjectDocuments => "contains subjects with deactivated DID documents",
      &Self::Expired => "the expiry date is in the past",
      &Self::Dormant => "the activation date is in the future",
    }
  }
  // The number of refutation categories. We do not use strum for this as we do not want to unnecessarily pollute the public API 
  pub(super) const COUNT: usize = 3;
}

impl Display for CredentialRefutationCategory {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.description())
  }
}

#[derive(Debug,Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CredentialRefutations {
  // true at the i'th slot corresponds to CredentialRefutationCategory::from_usize(i).unwrap() 
  slots: [bool; CredentialRefutationCategory::COUNT],
}

impl CredentialRefutations {
  /// Adds a value to the set.
  /// If the set did not have this value present, `true` is returned.
  /// If the set did have this value present, `false` is returned.
  pub fn insert(&mut self, category: CredentialRefutationCategory) -> bool {
    let ref mut flag = self.slots[category as usize]; 
    let current = *flag;
    *flag = true;
    !current 
  }

  pub fn iter(&self) -> impl Iterator<Item = CredentialRefutationCategory> + '_ { 
    self.slots.iter().enumerate().flat_map(|(index,contained)| if *contained {CredentialRefutationCategory::from_usize(index)} else {None})
  }

  pub fn count(&self) -> usize {
    self.slots.iter().copied().filter(|value| *value).count()
  }

  pub fn empty() -> Self {
    Self {slots : [false; CredentialRefutationCategory::COUNT]}
  }

  pub fn all() -> Self {
    Self {slots: [true; CredentialRefutationCategory::COUNT]}
  }

  pub fn contains(&self, category: &CredentialRefutationCategory) -> bool {
    self.slots[*category as usize]
  }
}

impl Extend<CredentialRefutationCategory> for CredentialRefutations {
  fn extend<T: IntoIterator<Item = CredentialRefutationCategory>>(&mut self, iter: T) {
      for category in iter {
        if self.slots == [true; CredentialRefutationCategory::COUNT] {
          break; 
        }
        self.insert(category);
      }
  }
}

impl FromIterator<CredentialRefutationCategory> for CredentialRefutations {
  fn from_iter<T: IntoIterator<Item = CredentialRefutationCategory>>(iter: T) -> Self {
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
    assert_eq!((0usize..100).map(|i| CredentialRefutationCategory::from_usize(i)).take_while(|value| value.is_some()).filter_map(|value| value).map(|value| value as usize + 1).max().unwrap(),CredentialRefutationCategory::COUNT);
  }

  #[test]
  fn credential_refutations_iterator_roundtrip() {
    let categories = [
      CredentialRefutationCategory::DeactivatedSubjectDocuments,
      CredentialRefutationCategory::Dormant,
    ];
    let refutations: CredentialRefutations = categories.into_iter().collect();
    let expected_set: HashSet<CredentialRefutationCategory> = categories.into_iter().collect(); 
    let round_trip_set: HashSet<CredentialRefutationCategory> = refutations.iter().collect(); 
    assert_eq!(expected_set, round_trip_set);
  }

  #[test]
  fn credential_refutations_all_count() {
    assert_eq!(CredentialRefutations::all().count(), CredentialRefutationCategory::COUNT); 
  }

  #[test]
fn credential_refutations_empty_count() {
  assert_eq!(CredentialRefutations::empty().count(), 0); 
}

#[test]
fn credential_refutations_extend_contains() {
  let dormant = CredentialRefutationCategory::Dormant; 
  let deactivated = CredentialRefutationCategory::DeactivatedSubjectDocuments; 
  let mut refutations = CredentialRefutations::empty();
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
  let mut refutations = CredentialRefutations::empty(); 
  assert!(refutations.insert(CredentialRefutationCategory::DeactivatedSubjectDocuments)); 
  assert!(!refutations.insert(CredentialRefutationCategory::DeactivatedSubjectDocuments)); 
}

#[test]
fn credential_refutations_all_contains() {
  let refutations = CredentialRefutations::all(); 
  for i in 0..CredentialRefutationCategory::COUNT {
    let category = CredentialRefutationCategory::from_usize(i).unwrap(); 
    assert!(refutations.contains(&category));
  }
}
}