use core::{
    cmp::Ordering,
    iter::{Chain, Cloned, Copied, Cycle, Enumerate, Fuse, Inspect, Map, Peekable, Rev, Take},
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
    pub fn first(mut self) -> I::Item {
        unwrap!(self.inner.next())
    }
    pub fn last(self) -> I::Item {
        unwrap!(self.inner.last())
    }
    pub fn map<B, F>(self, f: F) -> NonEmpty<Map<I, F>>
    where
        F: FnMut(I::Item) -> B,
    {
        NonEmpty {
            inner: self.inner.map(f),
        }
    }
    pub fn chain<U>(self, other: U) -> NonEmpty<Chain<I, <U as IntoIterator>::IntoIter>>
    where
        U: IntoIterator<Item = I::Item>,
    {
        NonEmpty {
            inner: self.inner.chain(other),
        }
    }
    pub fn enumerate(self) -> NonEmpty<Enumerate<I>> {
        NonEmpty {
            inner: self.inner.enumerate(),
        }
    }
    pub fn peekable(self) -> NonEmpty<Peekable<I>> {
        NonEmpty {
            inner: self.inner.peekable(),
        }
    }
    pub fn take(self, n: NonZeroUsize) -> NonEmpty<Take<I>> {
        NonEmpty {
            inner: self.inner.take(n.get()),
        }
    }
    // pub fn flat_map
    // pub fn flatten
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
    pub fn peek(&mut self) -> &I::Item {
        unwrap!(self.inner.peek())
    }
    pub fn peek_mut(&mut self) -> &mut I::Item {
        unwrap!(self.inner.peek_mut())
    }
}
