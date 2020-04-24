//! A sized queue that keeps its size and drops any excess items

use std::collections::vec_deque::{self, VecDeque};
use std::iter::FusedIterator;

/// A sized queue which drops the old values if full
#[derive(Clone, Debug)]
pub struct SizedQueue<T> {
    queue: VecDeque<T>,
    size: usize,
}

impl<T> SizedQueue<T> {
    /// Create a new instance with the given size
    pub fn new(size: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(size),
            size,
        }
    }

    /// Create a new instance with the given size
    ///
    /// Alias method for [`new`]
    ///
    /// [`new`]: #method.new
    pub fn from_size(size: usize) -> Self {
        Self::new(size)
    }

    /// Get the oldest inserted item
    pub fn oldest(&self) -> Option<&T> {
        self.queue.front()
    }

    /// Get the last inserted item
    pub fn newest(&self) -> Option<&T> {
        self.queue.back()
    }

    /// Push a new value into the queue and get the old one if the queue was full
    pub fn push(&mut self, item: T) -> Option<T> {
        let dropped = if self.queue.len() == self.size {
            self.queue.pop_front()
        } else {
            None
        };
        self.queue.push_back(item);
        dropped
    }

    /// Get the size of this queue
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get how many items are actually stored in the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if the queue is empty or not
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get an iterator of this queue
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.queue.iter(),
        }
    }
}

/// Iterator of [`SizedQueue<T>`] returned by [`iter`] method
///
/// [`SizedQueue<T>`]: ./struct.SizedQueue.html
/// [`iter`]: ./struct.SizedQueue.html#method.iter
#[derive(Clone, Debug)]
pub struct Iter<'a, T> {
    iter: vec_deque::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn last(self) -> Option<Self::Item> {
        self.iter.last()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n)
    }

    fn fold<Acc, F>(self, accumulator: Acc, f: F) -> Acc
    where
        F: FnMut(Acc, Self::Item) -> Acc,
    {
        self.iter.fold(accumulator, f)
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn rfold<B, F>(self, accumulator: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(accumulator, f)
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

impl<'a, T> FusedIterator for Iter<'a, T> {}
