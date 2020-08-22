//! This module implements python-like `Counter`
use std::cmp::Eq;
use std::cmp::Ord;
use std::collections::btree_map::{BTreeMap, IntoIter, Iter};
use std::fmt::Debug;
use std::iter::FromIterator;
use std::ops::Index;

/// Structure that count occurences of `T` elements
#[derive(Debug)]
pub struct Counter<T> {
    state: BTreeMap<T, u128>,
}

impl<T: Ord + Debug> Counter<T> {
    /// Create a new empty `Counter`.
    ///
    /// # Examples
    /// ```
    /// # use reducto::counter::Counter;
    /// let mut counter: Counter<char> = Counter::new();
    /// ```
    pub fn new() -> Self {
        Counter {
            state: BTreeMap::new(),
        }
    }

    /// Update the `Counter` with an iterator.
    ///
    /// # Arguments
    /// * iter - An iterator used to update the `Counter`
    ///
    /// # Examples
    /// ```
    /// # use reducto::counter::Counter;
    /// # use std::collections::btree_map::BTreeMap;
    /// # use std::iter::FromIterator;
    /// let mut counter: Counter<char> = Counter::new();
    /// counter.update_from_iter("a string".chars());
    /// assert_eq!(counter['a'], 1);
    /// assert_eq!(counter[' '], 1);
    /// assert_eq!(counter['s'], 1);
    /// assert_eq!(counter['t'], 1);
    /// assert_eq!(counter['r'], 1);
    /// assert_eq!(counter['i'], 1);
    /// assert_eq!(counter['n'], 1);
    /// assert_eq!(counter['g'], 1);
    /// ```
    pub fn update_from_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iter {
            let count = self.state.entry(elem).or_insert(0);
            *count += 1;
        }
    }

    /// Update the `Counter` with a value.
    ///
    /// # Arguments
    /// * elem - A value used to update the `Counter`
    ///
    /// # Examples
    /// ```
    /// # use reducto::counter::Counter;
    /// # use std::collections::btree_map::BTreeMap;
    /// # use std::iter::FromIterator;
    /// let mut counter: Counter<char> = Counter::new();
    /// assert_eq!(counter['a'], 0);
    /// counter.update_from_value('a');
    /// assert_eq!(counter['a'], 1);
    /// ```
    pub fn update_from_value(&mut self, elem: T) {
        let count = self.state.entry(elem).or_insert(0);
        *count += 1;
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }
}

impl<T: Ord + Debug> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut counter: Counter<T> = Counter::new();
        counter.update_from_iter(iter.into_iter());
        counter
    }
}

impl<T: Ord + Debug> IntoIterator for Counter<T> {
    type Item = (T, u128);
    type IntoIter = IntoIter<T, u128>;

    fn into_iter(self) -> Self::IntoIter {
        self.state.into_iter()
    }
}

impl<'a, T: Ord + Debug> IntoIterator for &'a Counter<T> {
    type Item = (&'a T, &'a u128);
    type IntoIter = Iter<'a, T, u128>;

    fn into_iter(self) -> Self::IntoIter {
        self.state.iter()
    }
}

impl<T: Ord + Debug> Index<T> for Counter<T> {
    type Output = u128;

    fn index(&self, index: T) -> &Self::Output {
        match self.state.get(&index) {
            Some(value) => value,
            None => &0,
        }
    }
}

impl<T: Ord + Debug> PartialEq for Counter<T> {
    fn eq(&self, other: &Counter<T>) -> bool {
        if self.state.len() == other.state.len() {
            for (key, value) in self.state.iter() {
                let other_value = match other.state.get(key) {
                    Some(val) => val,
                    None => return false,
                };

                if value != other_value {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl<T: Ord + Debug> Eq for Counter<T> {}
