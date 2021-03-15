// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::hash::Hash;
use core::{
    borrow::Borrow,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};
use iota::MessageId;
use std::collections::HashMap;

use crate::tangle::TangleRef;

type __Index<T> = HashMap<MessageId, Vec<T>>;

#[derive(Clone, Debug)]
pub struct MessageIndex<T> {
    inner: __Index<T>,
}

impl<T> MessageIndex<T> {
    /// Creates a new `MessageIndex`.
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    /// Returns the total size of the index.
    pub fn size(&self) -> usize {
        self.inner.values().map(Vec::len).sum()
    }

    pub fn remove_where<U>(&mut self, key: &U, f: impl Fn(&T) -> bool) -> Option<T>
    where
        MessageId: Borrow<U>,
        U: Hash + Eq + ?Sized,
    {
        if let Some(list) = self.inner.get_mut(key) {
            list.iter().position(f).map(|index| list.remove(index))
        } else {
            None
        }
    }
}

impl<T> MessageIndex<T>
where
    T: TangleRef,
{
    pub fn insert(&mut self, element: T) {
        let key: &MessageId = element.previous_message_id();

        if let Some(scope) = self.inner.get_mut(key) {
            scope.insert(0, element);
        } else {
            self.inner.insert(key.clone(), vec![element]);
        }
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for element in iter.into_iter() {
            self.insert(element);
        }
    }
}

impl<T> Default for MessageIndex<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for MessageIndex<T> {
    type Target = __Index<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for MessageIndex<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> FromIterator<T> for MessageIndex<T>
where
    T: TangleRef,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut this: Self = Self::new();
        this.extend(iter);
        this
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Case {
        message_id: MessageId,
        previous_message_id: MessageId,
        state: bool,
    }

    impl Case {
        fn new(message_id: [u8; 32], previous_message_id: [u8; 32], state: bool) -> Self
where {
            Self {
                message_id: MessageId::new(message_id),
                previous_message_id: MessageId::new(previous_message_id),
                state,
            }
        }
    }

    impl TangleRef for Case {
        fn message_id(&self) -> &MessageId {
            &self.message_id
        }

        fn set_message_id(&mut self, message_id: MessageId) {
            self.message_id = message_id;
        }

        fn previous_message_id(&self) -> &MessageId {
            &self.previous_message_id
        }

        fn set_previous_message_id(&mut self, message_id: MessageId) {
            self.previous_message_id = message_id;
        }
    }

    #[rustfmt::skip]
    fn setup() -> MessageIndex<Case> {
        let cases: Vec<Case> = vec![
            Case::new(*b"9999999999999999999999999999999A", *b"99999999999999999999999999999990", true),
            Case::new(*b"9999999999999999999999999999999B", *b"9999999999999999999999999999999A", false),
            Case::new(*b"9999999999999999999999999999999C", *b"9999999999999999999999999999999A", true),
            Case::new(*b"9999999999999999999999999999999D", *b"9999999999999999999999999999999C", false),
            Case::new(*b"9999999999999999999999999999999E", *b"9999999999999999999999999999999B", false),
            Case::new(*b"9999999999999999999999999999999F", *b"9999999999999999999999999999999B", true),
        ];

        let mut index: MessageIndex<Case> = MessageIndex::new();
        index.extend(cases);
        index
    }

    #[test]
    #[rustfmt::skip]
    fn test_works() {
        let index: MessageIndex<Case> = setup();

        assert_eq!(index.size(), 6);
        assert_eq!(index[&MessageId::new(*b"99999999999999999999999999999990")].len(), 1);
        assert_eq!(index[&MessageId::new(*b"9999999999999999999999999999999A")].len(), 2);
        assert_eq!(index[&MessageId::new(*b"9999999999999999999999999999999B")].len(), 2);
        assert_eq!(index[&MessageId::new(*b"9999999999999999999999999999999C")].len(), 1);
    }

    #[test]
    #[rustfmt::skip]
    fn test_remove_where() {
        let mut index: MessageIndex<Case> = setup();

        let removed: Case = index.remove_where(&MessageId::new(*b"99999999999999999999999999999990"), |_| true).unwrap();
        assert_eq!(removed.message_id, MessageId::new(*b"9999999999999999999999999999999A"));
        assert_eq!(removed.previous_message_id, MessageId::new(*b"99999999999999999999999999999990") );
        assert!(index.remove_where(&MessageId::new(*b"99999999999999999999999999999990"), |_| true).is_none());

        let first: Case = index
            .remove_where(&MessageId::new(*b"9999999999999999999999999999999B"), |case| !case.state)
            .unwrap();

        assert_eq!(first.message_id, MessageId::new(*b"9999999999999999999999999999999E"));
        assert_eq!(first.previous_message_id, MessageId::new(*b"9999999999999999999999999999999B"));

        let second: Case = index
            .remove_where(&MessageId::new(*b"9999999999999999999999999999999B"), |_| true)
            .unwrap();

        assert_eq!(second.message_id, MessageId::new(*b"9999999999999999999999999999999F"));
        assert_eq!(second.previous_message_id, MessageId::new(*b"9999999999999999999999999999999B"));
    }
}
