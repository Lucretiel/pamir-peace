use std::{cmp::Ordering, iter, mem, ops::AddAssign};

use enum_map::{enum_map, Enum, EnumMap};

/// A set of keys, efficiently implemented with EnumMap
pub struct EnumSet<K: Enum<bool>> {
    set: EnumMap<K, bool>,
}

impl<K: Enum<bool>> EnumSet<K> {
    pub fn new() -> Self {
        Self {
            set: enum_map! { _ => false },
        }
    }

    pub fn len(&self) -> usize {
        self.set.values().filter(|&&b| b).count()
    }

    pub fn is_empty(&self) -> bool {
        self.set.values().all(|&b| !b)
    }

    pub fn set(&mut self, key: K, setting: bool) -> bool {
        mem::replace(&mut self.set[key], setting)
    }

    /// Add a key to the set, and return true if the key was already present
    pub fn insert(&mut self, key: K) -> bool {
        self.set(key, true)
    }

    /// Remove a key from the set, and return true if it was already present
    pub fn remove(&mut self, key: K) -> bool {
        self.set(key, false)
    }

    /// Check if a key is in this set
    pub fn contains(&self, key: K) -> bool {
        self.set[key]
    }

    /// Iterate over all the keys in this set
    pub fn iter(&self) -> impl Iterator<Item = K> + '_ {
        self.set.iter().filter_map(|(k, &b)| b.then(|| k))
    }

    /// Iterate over all the keys in this set.
    // TODO: impl IntoIterator
    pub fn into_iter(self) -> impl Iterator<Item = K> {
        self.set.into_iter().filter_map(|(k, b)| b.then(|| k))
    }

    /// Remove all the keys in this set
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<K: Enum<bool>> Default for EnumSet<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Enum<bool>> Clone for EnumSet<K> {
    fn clone(&self) -> Self {
        Self {
            set: enum_map! { key => self.set[key] },
        }
    }
}

impl<K: Enum<bool>> Copy for EnumSet<K> where K::Array: Copy {}

impl<K: Enum<bool>> FromIterator<K> for EnumSet<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut this = Self::new();
        this.extend(iter);
        this
    }
}

impl<K: Enum<bool>> Extend<K> for EnumSet<K> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        iter.into_iter().for_each(|key| {
            self.insert(key);
        })
    }
}

impl<K: Enum<bool>> PartialEq for EnumSet<K> {
    fn eq(&self, other: &Self) -> bool {
        self.set.as_slice() == other.set.as_slice()
    }
}

impl<K: Enum<bool>> Eq for EnumSet<K> {}

/// Adapter type that implements `FromIterator` by computing the sum
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Sum<T> {
    pub sum: T,
}

impl<T: iter::Sum<U>, U> FromIterator<U> for Sum<T> {
    fn from_iter<I: IntoIterator<Item = U>>(iter: I) -> Self {
        Self {
            sum: iter.into_iter().sum(),
        }
    }
}

impl<T: iter::Sum<U>, U> Extend<U> for Sum<T>
where
    T: AddAssign<T>,
{
    fn extend<I: IntoIterator<Item = U>>(&mut self, iter: I) {
        self.sum += iter.into_iter().sum()
    }
}

/// Check if there is a plurality winner in a list. A plurality winner is the
/// key which uniquely has a higher count than any other key in the list
pub fn unique_max_by_key<T, K>(input: impl Iterator<Item = T>, key: impl Fn(&T) -> K) -> Option<T>
where
    K: Ord,
{
    let mut iter = input.into_iter();
    let first = iter.next()?;

    let (best, unique) = iter.fold((first, true), |(best, unique), item| {
        match Ord::cmp(&key(&item), &key(&best)) {
            Ordering::Less => (best, unique),
            Ordering::Equal => (best, false),
            Ordering::Greater => (item, true),
        }
    });

    unique.then(|| best)
}
