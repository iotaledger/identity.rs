use core::{
    borrow::Borrow,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};
use std::collections::BTreeMap;

use crate::tangle::{MessageId, TangleRef};

type __Index<T> = BTreeMap<MessageId, Vec<T>>;

#[derive(Clone, Debug)]
pub struct MessageIndex<T> {
    inner: __Index<T>,
}

impl<T> MessageIndex<T> {
    /// Creates a new `MessageIndex`.
    pub fn new() -> Self {
        Self { inner: BTreeMap::new() }
    }

    /// Returns the total size of the index.
    pub fn size(&self) -> usize {
        self.inner.values().map(Vec::len).sum()
    }

    pub fn remove_where<U>(&mut self, key: &U, f: impl Fn(&T) -> bool) -> Option<T>
    where
        MessageId: Borrow<U>,
        U: Ord + ?Sized,
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
        fn new<T, U>(message_id: T, previous_message_id: U, state: bool) -> Self
        where
            T: Into<String>,
            U: Into<String>,
        {
            Self {
                message_id: MessageId::new(message_id.into()),
                previous_message_id: MessageId::new(previous_message_id.into()),
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
            Case::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999A", "", true),
            Case::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999B", "99999999999999999999999999999999999999999999999999999999999999999999999999999999A", false),
            Case::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999C", "99999999999999999999999999999999999999999999999999999999999999999999999999999999A", true),
            Case::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999D", "99999999999999999999999999999999999999999999999999999999999999999999999999999999C", false),
            Case::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999E", "99999999999999999999999999999999999999999999999999999999999999999999999999999999B", false),
            Case::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999F", "99999999999999999999999999999999999999999999999999999999999999999999999999999999B", true),
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
        assert_eq!(index[&MessageId::new("")].len(), 1);
        assert_eq!(index[&MessageId::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999A")].len(), 2);
        assert_eq!(index[&MessageId::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999B")].len(), 2);
        assert_eq!(index[&MessageId::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999C")].len(), 1);
    }

    #[test]
    #[rustfmt::skip]
    fn test_remove_where() {
        let mut index: MessageIndex<Case> = setup();

        let removed: Case = index.remove_where(&MessageId::new(""), |_| true).unwrap();
        assert_eq!(removed.message_id, "99999999999999999999999999999999999999999999999999999999999999999999999999999999A");
        assert_eq!(removed.previous_message_id, MessageId::NONE);
        assert!(index.remove_where(&MessageId::new(""), |_| true).is_none());

        let first: Case = index
            .remove_where(&MessageId::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999B"), |case| !case.state)
            .unwrap();

        assert_eq!(first.message_id, "99999999999999999999999999999999999999999999999999999999999999999999999999999999E");
        assert_eq!(first.previous_message_id, "99999999999999999999999999999999999999999999999999999999999999999999999999999999B");

        let second: Case = index
            .remove_where(&MessageId::new("99999999999999999999999999999999999999999999999999999999999999999999999999999999B"), |_| true)
            .unwrap();

        assert_eq!(second.message_id, "99999999999999999999999999999999999999999999999999999999999999999999999999999999F");
        assert_eq!(second.previous_message_id, "99999999999999999999999999999999999999999999999999999999999999999999999999999999B");
    }
}
