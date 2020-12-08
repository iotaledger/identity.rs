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

    pub fn remove_where<U>(&mut self, key: &U, f: impl Fn(&T) -> bool) -> Option<T>
    where
        MessageId: Borrow<U>,
        U: Ord + ?Sized,
    {
        if let Some(mut list) = self.inner.remove(key) {
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
