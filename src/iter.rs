use core::{
    cmp::Ordering,
    iter::{
        Chain, Cloned, Copied, Cycle, Enumerate, FlatMap, Fuse, Inspect, Map, Peekable, Rev, Take,
    },
    num::NonZeroUsize,
};

use crate::NonEmpty;

/// Return an [`Iterator`] that is guaranteed to yield at least one element.
///
/// Inherent methods on [`NonEmpty<impl Iterator>`] can then rely on that invariant.
pub trait IntoNonEmptyIterator: IntoIterator {
    // type Item;
    // type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter_ne(self) -> NonEmpty<Self::IntoIter>;
}

impl<I> IntoNonEmptyIterator<I> for NonEmpty<I>
where
    I: Iterator,
{
    fn into_iter_ne(self) -> NonEmpty<Self::IntoIter> {
        self
    }
}

macro_rules! unwrap {
    ($expr:expr) => {
        match $expr {
            Some(it) => it,
            // Safety:
            // - NonEmpty<impl Iterator> is only constructed from known NonEmpty items
            // - NonEmpty<impl Iterator> does give out mutable access to the inner iterator
            //   (so it always has one element)
            None => unsafe { crate::unreachable() },
        }
    };
}

/// Methods on [`Iterator`]s with a non-empty invariant.
///
/// See [`Self::relax`] to access the normal iterator inside.
impl<I> NonEmpty<I>
where
    I: Iterator,
{
    /// Consume the [`NonEmpty`] iterator, returning the first element.
    pub fn first(mut self) -> I::Item {
        unwrap!(self.inner.next())
    }
    /// Consume the [`NonEmpty`] iterator, returning the last element.
    pub fn last(self) -> I::Item {
        unwrap!(self.inner.last())
    }
    /// Transform each item in the iterator, maintaining the [`NonEmpty`] invariant.
    /// ```
    /// # use nunny::{slice};
    /// let before = slice![1, 2, 3];
    /// let after = before.iter_ne().map(|it| *it * 2).collect_vec();
    /// assert_eq!(
    ///     after,
    ///     [2, 4, 6]
    /// )
    /// ```
    pub fn map<B, F>(self, f: F) -> NonEmpty<Map<I, F>>
    where
        F: FnMut(I::Item) -> B,
    {
        NonEmpty {
            inner: self.inner.map(f),
        }
    }
    /// Append another iterator to this one, maintaining the [`NonEmpty`] invariant.
    /// ```
    /// # use nunny::{vec};
    /// let iter = vec![1, 2].into_iter_ne().chain([3]);
    /// assert_eq!(
    ///     iter.last(),
    ///       // ^ the invariant is maintained, so we _know_ there's a last element
    ///     3
    /// )
    /// ```
    pub fn chain<U>(self, other: U) -> NonEmpty<Chain<I, <U as IntoIterator>::IntoIter>>
    where
        U: IntoIterator<Item = I::Item>,
    {
        NonEmpty {
            inner: self.inner.chain(other),
        }
    }
    /// Return an `(ix, T)` tuple for each `T`, maintaining the [`NonEmpty`] invariant.
    /// ```
    /// # use nunny::{vec};
    /// let v = vec!['a', 'b'];
    /// assert_eq!(
    ///     v.iter_ne().enumerate().last(),
    ///                          // ^ the invariant is maintained
    ///                          //   so we _know_ there's a last element
    ///     (1, &'b')
    /// )
    /// ```
    pub fn enumerate(self) -> NonEmpty<Enumerate<I>> {
        NonEmpty {
            inner: self.inner.enumerate(),
        }
    }
    /// Return a [`peek`](Self::peek)-able iterator, maintaining the [`NonEmpty`] invariant.
    /// ```
    /// # use nunny::{vec};
    /// let v = vec!['a', 'b'];
    /// let mut peek_me = v.into_iter_ne().peekable();
    /// assert_eq!(
    ///     *peek_me.peek(),
    ///     'a'
    /// );
    /// *peek_me.peek_mut() = 'b';
    /// assert_eq!(
    ///     peek_me.collect_vec(),
    ///     ['b', 'b']
    /// )
    /// ```
    pub fn peekable(self) -> NonEmpty<Peekable<I>> {
        NonEmpty {
            inner: self.inner.peekable(),
        }
    }
    /// Take at most `n` items from this iterator.
    ///
    /// Note that `n` cannot be zero, to maintain the [`NonEmpty`] invariant.
    ///
    /// ```
    /// # use nunny::{vec, nonzero};
    /// assert_eq!(
    ///     vec![1, 2, 3].into_iter_ne().take(nonzero!(1)).collect_vec(),
    ///                                    // ^ compile-time checked
    ///     [1]
    /// )
    /// ```
    pub fn take(self, n: NonZeroUsize) -> NonEmpty<Take<I>> {
        NonEmpty {
            inner: self.inner.take(n.get()),
        }
    }
    // pub fn flat_map
    #[allow(clippy::type_complexity)]
    pub fn flatten<II, T>(self) -> NonEmpty<FlatMap<I, II, fn(I::Item) -> II>>
    where
        I::Item: IntoNonEmptyIterator,
        //                 ^ each item is nonempty
        II: IntoIterator<Item = T>,
        // TODO(aatifsyed): a trait NonEmptyIterator would make this more ergonomic
    {
        todo!()
    }

    pub fn fuse(self) -> NonEmpty<Fuse<I>> {
        NonEmpty {
            inner: self.inner.fuse(),
        }
    }
    pub fn inspect<F>(self, f: F) -> NonEmpty<Inspect<I, F>>
    where
        F: FnMut(&I::Item),
    {
        NonEmpty {
            inner: self.inner.inspect(f),
        }
    }
    pub fn reduce<F>(self, f: F) -> I::Item
    where
        F: FnMut(I::Item, I::Item) -> I::Item,
    {
        unwrap!(self.inner.reduce(f))
    }
    pub fn max(self) -> I::Item
    where
        I::Item: Ord,
    {
        unwrap!(self.inner.max())
    }
    pub fn min(self) -> I::Item
    where
        I::Item: Ord,
    {
        unwrap!(self.inner.min())
    }
    pub fn max_by_key<B, F>(self, f: F) -> I::Item
    where
        B: Ord,
        F: FnMut(&I::Item) -> B,
    {
        unwrap!(self.inner.max_by_key(f))
    }
    pub fn max_by<F>(self, compare: F) -> I::Item
    where
        F: FnMut(&I::Item, &I::Item) -> Ordering,
    {
        unwrap!(self.inner.max_by(compare))
    }
    pub fn min_by_key<B, F>(self, f: F) -> I::Item
    where
        B: Ord,
        F: FnMut(&I::Item) -> B,
    {
        unwrap!(self.inner.min_by_key(f))
    }
    pub fn min_by<F>(self, compare: F) -> I::Item
    where
        F: FnMut(&I::Item, &I::Item) -> Ordering,
    {
        unwrap!(self.inner.min_by(compare))
    }
    pub fn rev(self) -> NonEmpty<Rev<I>>
    where
        I: DoubleEndedIterator,
    {
        NonEmpty {
            inner: self.inner.rev(),
        }
    }
    pub fn unzip_vec<A, B>(self) -> (NonEmpty<Vec<A>>, NonEmpty<Vec<B>>)
    where
        I: Iterator<Item = (A, B)>,
    {
        let (a, b) = self.inner.unzip();
        // Safety:
        // - NonEmpty<impl Iterator> is only constructed from known NonEmpty items
        // - NonEmpty<impl Iterator> does not allow mutable access to the inner iterator
        //   (so it always has one element)
        unsafe { (crate::Vec::new_unchecked(a), crate::Vec::new_unchecked(b)) }
    }
    pub fn copied<'a, T>(self) -> NonEmpty<Copied<I>>
    where
        T: 'a + Copy,
        I: Iterator<Item = &'a T>,
    {
        NonEmpty {
            inner: self.inner.copied(),
        }
    }
    pub fn cloned<'a, T>(self) -> NonEmpty<Cloned<I>>
    where
        T: 'a + Clone,
        I: Iterator<Item = &'a T>,
    {
        NonEmpty {
            inner: self.inner.cloned(),
        }
    }
    pub fn cycle(self) -> NonEmpty<Cycle<I>>
    where
        I: Clone,
    {
        NonEmpty {
            inner: self.inner.cycle(),
        }
    }
    #[doc(alias = "into_iter")]
    #[doc(alias = "into_inner")]
    pub fn relax(self) -> I {
        self.inner
    }
    pub fn collect_vec(self) -> NonEmpty<Vec<I::Item>> {
        // Safety:
        // - NonEmpty<impl Iterator> is only constructed from known NonEmpty items
        // - NonEmpty<impl Iterator> does not allow mutable access to the inner iterator
        //   (so it always has one element)
        unsafe { crate::Vec::new_unchecked(self.inner.collect()) }
    }
    pub fn try_collect_vec<T, E>(self) -> Result<NonEmpty<Vec<T>>, E>
    where
        I: Iterator<Item = Result<T, E>>,
    {
        match self.inner.collect() {
            // Safety:
            // - NonEmpty<impl Iterator> is only constructed from known NonEmpty items
            // - NonEmpty<impl Iterator> does not allow mutable access to the inner iterator
            //   (so it always has one element)
            Ok(it) => Ok(unsafe { crate::Vec::new_unchecked(it) }),
            Err(e) => Err(e),
        }
    }
}

impl<I> NonEmpty<Peekable<I>>
where
    I: Iterator,
{
    /// Peek this [`NonEmpty`] iterator, without advancing it.
    ///
    /// See [`Self::peekable`].
    pub fn peek(&mut self) -> &I::Item {
        unwrap!(self.inner.peek())
    }
    /// Peek and modify this [`NonEmpty`] iterator, without advancing it.
    ///
    /// See [`Self::peekable`].
    pub fn peek_mut(&mut self) -> &mut I::Item {
        unwrap!(self.inner.peek_mut())
    }
}
