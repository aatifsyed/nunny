use core::{
    cmp::Ordering,
    iter::{
        Chain, Cloned, Copied, Cycle, Enumerate, FlatMap, Fuse, Inspect, Map, Peekable, Rev, Take,
    },
    num::NonZeroUsize,
};

use crate::NonEmpty;

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
    /// [`NonEmpty`] version of [`Iterator::next`].
    /// ```
    /// # use nunny::{vec};
    /// let v = vec![1, 2, 3];
    /// let _: Option<&u8> = v.iter().next();
    ///     // ^ normally you have to handle the empty case
    /// let _: &u8 = v.iter_ne().first();
    ///     // ^ but we know there is at least one element
    /// ```
    pub fn first(mut self) -> I::Item {
        unwrap!(self.inner.next())
    }
    /// [`NonEmpty`] version of [`Iterator::last`].
    /// ```
    /// # use nunny::{vec};
    /// let v = vec![1, 2, 3];
    /// let _: Option<&u8> = v.iter().last();
    ///     // ^ normally you have to handle the empty case
    /// let _: &u8 = v.iter_ne().last();
    ///     // ^ but we know there is at least one element
    /// ```
    pub fn last(self) -> I::Item {
        unwrap!(self.inner.last())
    }
    /// [`NonEmpty`] version of [`Iterator::map`].
    /// ```
    /// # use nunny::{slice};
    /// let iter = slice![1, 2, 3].iter_ne();
    /// assert_eq!(
    ///     iter.map(|it| *it * 2).last(),
    ///                         // ^ the invariant is maintained
    ///                         //   so we _know_ there's a last element
    ///     6
    /// );
    /// ```
    pub fn map<B, F>(self, f: F) -> NonEmpty<Map<I, F>>
    where
        F: FnMut(I::Item) -> B,
    {
        NonEmpty {
            inner: self.inner.map(f),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::chain`].
    /// ```
    /// # use nunny::{slice};
    /// let iter = slice![1, 2].iter_ne();
    /// assert_eq!(
    ///     iter.chain(&[3]).last(),
    ///                   // ^ the invariant is maintained
    ///                   //   so we _know_ there's a last element
    ///     &3
    /// );
    /// ```
    pub fn chain<U>(self, other: U) -> NonEmpty<Chain<I, <U as IntoIterator>::IntoIter>>
    where
        U: IntoIterator<Item = I::Item>,
    {
        NonEmpty {
            inner: self.inner.chain(other),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::enumerate`].
    /// ```
    /// # use nunny::{slice};
    /// let iter = slice!['a', 'b'].iter_ne();
    /// assert_eq!(
    ///     iter.enumerate().last(),
    ///                  // ^ the invariant is maintained
    ///                  //   so we _know_ there's a last element
    ///     (1, &'b')
    /// );
    /// ```
    pub fn enumerate(self) -> NonEmpty<Enumerate<I>> {
        NonEmpty {
            inner: self.inner.enumerate(),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::peekable`], allowing you to use
    /// [`Self::peek`] and [`Self::peek_mut`]
    /// ```
    /// # use nunny::{vec, NonEmpty};
    /// let mut peek_me = vec!['a', 'b'].into_iter_ne().peekable();
    /// assert_eq!(
    ///     *peek_me.peek(),
    ///     'a'
    /// );
    /// *peek_me.peek_mut() = 'b';
    /// assert_eq!(
    ///     peek_me.collect_vec(),
    ///     ['b', 'b']
    /// );
    /// ```
    pub fn peekable(self) -> NonEmpty<Peekable<I>> {
        NonEmpty {
            inner: self.inner.peekable(),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::take`].
    ///
    /// Note that `n` cannot be zero, to maintain the [`NonEmpty`] invariant.
    ///
    /// ```
    /// # use nunny::{slice, nonzero};
    /// let iter = slice!['a', 'b'].iter_ne();
    /// assert_eq!(
    ///     iter.take(nonzero!(1)).last(),
    ///            // ^ compile time checked
    ///     &'a'
    /// )
    /// ```
    pub fn take(self, n: NonZeroUsize) -> NonEmpty<Take<I>> {
        NonEmpty {
            inner: self.inner.take(n.get()),
        }
    }
    // pub fn flat_map
    /// [`NonEmpty`] version of [`Iterator::flatten`].
    ///
    /// Note that the inner items must also be [`NonEmpty`], to maintain the invariant.
    /// ```
    /// use nunny::{vec};
    /// let nested = vec![vec![1], vec![2, 3]];
    /// assert_eq!(
    ///     nested.into_iter_ne().flatten().collect_vec(),
    ///     [1, 2, 3],
    /// );
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn flatten<II, T>(self) -> NonEmpty<FlatMap<I, II, fn(I::Item) -> II>>
    where
        I: Iterator<Item = NonEmpty<II>>,
        //                 ^ each item is nonempty
        II: IntoIterator<Item = T>,
        // TODO(aatifsyed): a trait NonEmptyIterator would make this more ergonomic
        //                  See commit history for an attempt
    {
        NonEmpty {
            inner: self.inner.flat_map(|it| it.inner),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::fuse`].
    pub fn fuse(self) -> NonEmpty<Fuse<I>> {
        NonEmpty {
            inner: self.inner.fuse(),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::inspect`].
    pub fn inspect<F>(self, f: F) -> NonEmpty<Inspect<I, F>>
    where
        F: FnMut(&I::Item),
    {
        NonEmpty {
            inner: self.inner.inspect(f),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::reduce`].
    /// ```
    /// # use nunny::{vec};
    /// # use core::cmp::min;
    /// let v = vec![1, 2, 3];
    /// let _: Option<&u8> = v.iter().reduce(min);
    ///     // ^ normally you have to handle the empty case
    /// let _: &u8 = v.iter_ne().reduce(min);
    ///     // ^ but we know there is at least one element
    /// ```
    pub fn reduce<F>(self, f: F) -> I::Item
    where
        F: FnMut(I::Item, I::Item) -> I::Item,
    {
        unwrap!(self.inner.reduce(f))
    }
    /// [`NonEmpty`] version of [`Iterator::max`].
    /// ```
    /// # use nunny::{vec};
    /// let v = vec![1, 2, 3];
    /// let _: Option<&u8> = v.iter().max();
    ///     // ^ normally you have to handle the empty case
    /// let _: &u8 = v.iter_ne().max();
    ///     // ^ but we know there is at least one element
    /// ```
    pub fn max(self) -> I::Item
    where
        I::Item: Ord,
    {
        unwrap!(self.inner.max())
    }
    /// [`NonEmpty`] version of [`Iterator::min`].
    /// ```
    /// # use nunny::{vec};
    /// let v = vec![1, 2, 3];
    /// let _: Option<&u8> = v.iter().min();
    ///     // ^ normally you have to handle the empty case
    /// let _: &u8 = v.iter_ne().min();
    ///     // ^ but we know there is at least one element
    /// ```
    pub fn min(self) -> I::Item
    where
        I::Item: Ord,
    {
        unwrap!(self.inner.min())
    }
    /// [`NonEmpty`] version of [`Iterator::max_by_key`].
    pub fn max_by_key<B, F>(self, f: F) -> I::Item
    where
        B: Ord,
        F: FnMut(&I::Item) -> B,
    {
        unwrap!(self.inner.max_by_key(f))
    }
    /// [`NonEmpty`] version of [`Iterator::max_by`].
    pub fn max_by<F>(self, compare: F) -> I::Item
    where
        F: FnMut(&I::Item, &I::Item) -> Ordering,
    {
        unwrap!(self.inner.max_by(compare))
    }
    /// [`NonEmpty`] version of [`Iterator::min_by_key`].
    pub fn min_by_key<B, F>(self, f: F) -> I::Item
    where
        B: Ord,
        F: FnMut(&I::Item) -> B,
    {
        unwrap!(self.inner.min_by_key(f))
    }
    /// [`NonEmpty`] version of [`Iterator::min_by`].
    pub fn min_by<F>(self, compare: F) -> I::Item
    where
        F: FnMut(&I::Item, &I::Item) -> Ordering,
    {
        unwrap!(self.inner.min_by(compare))
    }
    /// [`NonEmpty`] version of [`Iterator::rev`].
    pub fn rev(self) -> NonEmpty<Rev<I>>
    where
        I: DoubleEndedIterator,
    {
        NonEmpty {
            inner: self.inner.rev(),
        }
    }

    /// [`NonEmpty`] version of [`Iterator::unzip`].
    #[cfg(feature = "alloc")]
    #[cfg_attr(do_doc_cfg, doc(cfg(feature = "alloc")))]
    pub fn unzip_vec<A, B>(self) -> (crate::Vec<A>, crate::Vec<B>)
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
    /// [`NonEmpty`] version of [`Iterator::copied`].
    pub fn copied<'a, T>(self) -> NonEmpty<Copied<I>>
    where
        T: 'a + Copy,
        I: Iterator<Item = &'a T>,
    {
        NonEmpty {
            inner: self.inner.copied(),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::cloned`].
    pub fn cloned<'a, T>(self) -> NonEmpty<Cloned<I>>
    where
        T: 'a + Clone,
        I: Iterator<Item = &'a T>,
    {
        NonEmpty {
            inner: self.inner.cloned(),
        }
    }
    /// [`NonEmpty`] version of [`Iterator::cycle`].
    pub fn cycle(self) -> NonEmpty<Cycle<I>>
    where
        I: Clone,
    {
        NonEmpty {
            inner: self.inner.cycle(),
        }
    }
    /// Remove the [`NonEmpty`] wrapper, allowing you to access normal iterator
    /// methods like [`Iterator::filter`].
    #[doc(alias = "into_iter")]
    #[doc(alias = "into_inner")]
    pub fn relax(self) -> I {
        self.inner
    }
    /// Collect this iterator into a [`NonEmpty<Vec>`].
    #[cfg(feature = "alloc")]
    #[cfg_attr(do_doc_cfg, doc(cfg(feature = "alloc")))]
    pub fn collect_vec(self) -> crate::Vec<I::Item> {
        // Safety:
        // - NonEmpty<impl Iterator> is only constructed from known NonEmpty items
        // - NonEmpty<impl Iterator> does not allow mutable access to the inner iterator
        //   (so it always has one element)
        unsafe { crate::Vec::new_unchecked(self.inner.collect()) }
    }
    /// Collect [`Ok`] items into a [`NonEmpty<Vec>`], short-circuiting on [`Err`].
    #[cfg(feature = "alloc")]
    #[cfg_attr(do_doc_cfg, doc(cfg(feature = "alloc")))]
    pub fn try_collect_vec<T, E>(self) -> Result<crate::Vec<T>, E>
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
