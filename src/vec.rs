use core::{
    mem::MaybeUninit,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
};

use alloc::{boxed::Box, collections::TryReserveError};

use crate::{NonEmpty, Slice, Vec};

impl<T> Eq for NonEmpty<alloc::vec::Vec<T>> where T: Eq {}
impl<T, U> PartialEq<Vec<U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        <[_] as PartialEq<[U]>>::eq(self, other)
    }
}

macro_rules! forward_mut {
    ($( $(#[$meta:meta])* $vis:vis fn $ident:ident(&mut self $(,$arg:ident: $ty:ty)* $(,)?) $(-> $ret:ty)?);* $(;)?) => {
        $(
            $(#[$meta])*
            ///
            #[doc = concat!("See [`", stringify!($ident), "`](alloc::vec::Vec::", stringify!($ident), ").")]
            $vis fn $ident(&mut self $(, $arg: $ty)*) $(-> $ret)? {
                // Safety:
                // - operation does not remove elements
                unsafe { self.as_mut_vec() }.$ident($($arg),*)
            }
        )*
    };
}

/// [`Vec`] methods
impl<T> Vec<T> {
    ///////////
    // Creation
    ///////////

    crate::map_non_empty! {
        /// Create a new [`NonEmpty`] heap-allocated vec
        new_ref(&alloc::vec::Vec<T>) -> &Self: Self::new_ref_unchecked;
        /// Create a new [`NonEmpty`] heap-allocated vec
        new_mut(&mut alloc::vec::Vec<T>) -> &mut Self: Self::new_mut_unchecked;
    }
    crate::transmuting! {
        /// Create a new [`NonEmpty`] heap-allocated vec
        new_unchecked(alloc::vec::Vec<T>) -> Self;
        /// Create a new [`NonEmpty`] heap-allocated vec
        new_ref_unchecked(&alloc::vec::Vec<T>) -> &Self;
        /// Create a new [`NonEmpty`] heap-allocated vec
        new_mut_unchecked(&mut alloc::vec::Vec<T>) -> &mut Self;
    }
    /// Create a new [`NonEmpty`] heap-allocated vec, returning the original
    /// allocation if it was empty.
    pub fn new(src: alloc::vec::Vec<T>) -> Result<Self, alloc::vec::Vec<T>> {
        match src.is_empty() {
            false => Ok(unsafe { Self::new_unchecked(src) }),
            true => Err(src),
        }
    }

    ////////////
    // Utilities
    ////////////

    /// Create a [`NonEmpty`] heap-allocated vec, of a single element.
    pub fn of(item: T) -> Self {
        Self::of_with_capacity(item, 1)
    }
    /// Create a [`NonEmpty`] heap-allocated vec, of a single element, with
    /// capacity for `capacity` elements without (re)-allocating.
    pub fn of_with_capacity(item: T, capacity: usize) -> Self {
        let mut inner = alloc::vec::Vec::with_capacity(capacity);
        inner.push(item);
        // Safety:
        // - pushing the element succeeded
        unsafe { Self::new_unchecked(inner) }
    }
    /// Creating a [`NonEmpty`] heap-allocated vec where the first element is known.
    ///
    /// This is an convenience method, equivalent to
    /// ```
    /// let mut it = nunny::vec!["first"];
    /// it.extend(["this", "is", "the", "rest"]);
    /// ```
    pub fn of_extending<A>(first: T, rest: impl IntoIterator<Item = A>) -> Self
    where
        Self: Extend<A>,
    {
        let rest = rest.into_iter();
        let mut this = Self::of_with_capacity(first, rest.size_hint().0);
        this.extend(rest);
        this
    }

    /// Create a [`NonEmpty`] heap-allocated vec with `len` items, filled with
    /// [`Clone`]s of the given `value`.
    ///
    /// See also [`Self::filled_with`].
    pub fn filled(value: T, len: NonZeroUsize) -> Self
    where
        T: Clone,
    {
        let mut inner = alloc::vec::Vec::new();
        inner.resize(len.get(), value);
        // Safety:
        // - len is nonzero
        unsafe { Self::new_unchecked(inner) }
    }

    /// Create a [`NonEmpty`] heap-allocated vec with `len` items, filled with
    /// values returned from repeating the closure `f`.
    ///
    /// See also [`Self::filled`].
    pub fn filled_with<F>(f: F, len: NonZeroUsize) -> Self
    where
        F: FnMut() -> T,
    {
        let mut inner = alloc::vec::Vec::new();
        inner.resize_with(len.get(), f);
        // Safety:
        // - len is nonzero
        unsafe { Self::new_unchecked(inner) }
    }
    fn check(&self) {
        debug_assert_ne!(self.inner.len(), 0)
    }

    /// Returns a [`std::vec::Vec`].
    pub fn as_vec(&self) -> &alloc::vec::Vec<T> {
        self.check();
        &self.inner
    }
    /// Returns a [`std::vec::Vec`].
    ///
    /// # Safety
    /// - returned vec must not be emptied through this reference
    pub unsafe fn as_mut_vec(&mut self) -> &mut alloc::vec::Vec<T> {
        self.check();
        &mut self.inner
    }
    /// Returns a [`std::vec::Vec`].
    pub fn into_vec(self) -> alloc::vec::Vec<T> {
        let Self { inner } = self;
        inner
    }
    /// Returns a [`NonEmpty`] slice.
    pub fn as_slice_ne(&self) -> &Slice<T> {
        unsafe { Slice::new_unchecked(self.as_vec()) }
    }
    /// Returns a [`NonEmpty`] slice.
    pub fn as_mut_slice_ne(&mut self) -> &mut Slice<T> {
        unsafe { Slice::new_mut_unchecked(self.as_mut_vec()) }
    }

    //////////////////
    // Shimmed methods (rustdoc order)
    //////////////////

    /// Returns the known non-zero length.
    pub fn capacity(&self) -> NonZeroUsize {
        self.check();
        unsafe { crate::non_zero_usize(self.as_vec().capacity()) }
    }

    forward_mut! {
        pub fn reserve(&mut self, additional: usize);
        pub fn reserve_exact(&mut self, additional: usize);
        pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
        pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>;
        pub fn shrink_to_fit(&mut self);
        pub fn shrink_to(&mut self, min_capacity: usize);
    }

    /// Return a [`NonEmpty`] boxed slice.
    pub fn into_boxed_slice(self) -> Box<Slice<T>> {
        match cfg!(debug_assertions) {
            true => {
                let src = self.into_vec().into_boxed_slice();
                let len0 = src.len();
                let ptr = Box::into_raw(src);
                // Safety:
                // - #[repr(transparent)]
                let dst = unsafe { Box::from_raw(ptr as *mut Slice<T>) };
                let len1 = dst.len_ne().get();
                assert_eq!(len0, len1);
                dst
            }
            false => {
                let ptr = Box::into_raw(self.into_vec().into_boxed_slice());
                // Safety:
                // - #[repr(transparent)]
                unsafe { Box::from_raw(ptr as *mut Slice<T>) }
            }
        }
    }

    /// Shortens the vector to a guaranteed-nonzero length
    ///
    /// See [`truncate`](alloc::vec::Vec::truncate).
    pub fn truncate(&mut self, len: NonZeroUsize) {
        // Safety:
        // - len is not zero, so vector will not be emptied
        unsafe { self.as_mut_vec() }.truncate(len.get());
        self.check();
    }

    /// # Safety
    /// - See [`set_len`](alloc::vec::Vec::set_len).
    pub unsafe fn set_len(&mut self, new_len: NonZeroUsize) {
        // Safety:
        // - len is not zero, so vector will not be emptied
        unsafe { self.as_mut_vec() }.set_len(new_len.get());
        self.check();
    }

    forward_mut! {
        pub fn insert(&mut self, index: usize, element: T);
    }

    /// See [`dedup_by_key`](alloc::vec::Vec::dedup_by_key).
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        // Safety:
        // - dedup always leaves the first element
        unsafe { self.as_mut_vec() }.dedup_by_key(key);
        self.check();
    }
    /// See [`dedup_by`](alloc::vec::Vec::dedup_by).
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        // Safety:
        // - dedup always leaves the first element
        unsafe { self.as_mut_vec() }.dedup_by(same_bucket);
        self.check();
    }
    forward_mut! {
        pub fn push(&mut self, value: T);
        pub fn append(&mut self, other: &mut alloc::vec::Vec<T>);
    }

    // pub fn split_off(&mut self, at: NonZeroUsize)

    /// See [`resize_with`](alloc::vec::Vec::resize_with).
    pub fn resize_with<F>(&mut self, new_len: NonZeroUsize, f: F)
    where
        F: FnMut() -> T,
    {
        // Safety:
        // - new_len is not zero, so vec cannot be emptied
        unsafe { self.as_mut_vec() }.resize_with(new_len.get(), f);
        self.check();
    }
    /// Returns a [`NonEmpty`] slice.
    ///
    /// See [`leak`](alloc::vec::Vec::leak).
    pub fn leak<'a>(self) -> &'a mut Slice<T> {
        let inner = self.into_vec().leak();
        // Safety:
        // - originating slice is non-empty by construction
        unsafe { Slice::new_mut_unchecked(inner) }
    }
    forward_mut! {
        pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>];
    }
}

/// [`Vec`] to [`Slice`]
impl<T> Deref for Vec<T> {
    type Target = Slice<T>;

    fn deref(&self) -> &Self::Target {
        self.as_slice_ne()
    }
}

/// [`Vec`] to [`Slice`]
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice_ne()
    }
}

crate::as_ref_as_mut! {
    <T> for Vec<T> as [T];
    <T> for Vec<T> as Slice<T>;
    <T> for Vec<T> as Self;
}

crate::borrow_borrow_mut! {
    <T> for Vec<T> as [T];
    <T> for Vec<T> as Slice<T>;
}

crate::slice_iter! {
    <T> for Vec<T>
}

impl<T> crate::iter::IntoNonEmptyIterator for Vec<T> {
    fn into_iter_ne(self) -> NonEmpty<Self::IntoIter> {
        NonEmpty {
            inner: self.into_iter(),
        }
    }
}

mod iter {
    use super::*;
    impl<T> IntoIterator for Vec<T> {
        type Item = T;

        type IntoIter = alloc::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.into_vec().into_iter()
        }
    }
    impl<'a, T> Extend<&'a T> for Vec<T>
    where
        T: Copy + 'a,
    {
        fn extend<II: IntoIterator<Item = &'a T>>(&mut self, iter: II) {
            // Safety:
            // - append-only operation
            unsafe { self.as_mut_vec() }.extend(iter)
        }
    }
    impl<T> Extend<T> for Vec<T> {
        fn extend<II: IntoIterator<Item = T>>(&mut self, iter: II) {
            // Safety:
            // - append-only operation
            unsafe { self.as_mut_vec() }.extend(iter)
        }
    }
}

mod partial_eq_std {
    use super::*;

    impl<T, U> PartialEq<[U]> for Vec<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<[U; N]> for Vec<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U; N]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U> PartialEq<alloc::vec::Vec<U>> for Vec<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &alloc::vec::Vec<U>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }

    // converse
    //---------

    impl<T, U> PartialEq<Vec<T>> for [U]
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Vec<T>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<Vec<T>> for [U; N]
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Vec<T>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U> PartialEq<Vec<T>> for alloc::vec::Vec<U>
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Vec<T>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
}

mod cmp_std {
    use core::cmp::Ordering;

    use super::*;

    impl<T> PartialOrd<[T]> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    impl<T, const N: usize> PartialOrd<[T; N]> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }

    impl<T> PartialOrd<alloc::vec::Vec<T>> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &alloc::vec::Vec<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }

    // converse
    //---------

    impl<T> PartialOrd<Vec<T>> for [T]
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    impl<T, const N: usize> PartialOrd<Vec<T>> for [T; N]
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    impl<T> PartialOrd<Vec<T>> for alloc::vec::Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
}

mod convert_std {
    use crate::Error;

    use super::*;

    impl<T> TryFrom<alloc::vec::Vec<T>> for Vec<T> {
        type Error = alloc::vec::Vec<T>;

        fn try_from(value: alloc::vec::Vec<T>) -> Result<Self, Self::Error> {
            Vec::new(value)
        }
    }
    impl<'a, T> TryFrom<&'a alloc::vec::Vec<T>> for &'a Vec<T> {
        type Error = Error;

        fn try_from(value: &'a alloc::vec::Vec<T>) -> Result<Self, Self::Error> {
            Vec::new_ref(value).ok_or(Error(()))
        }
    }
    impl<'a, T> TryFrom<&'a mut alloc::vec::Vec<T>> for &'a mut Vec<T> {
        type Error = Error;

        fn try_from(value: &'a mut alloc::vec::Vec<T>) -> Result<Self, Self::Error> {
            Vec::new_mut(value).ok_or(Error(()))
        }
    }

    impl<T> From<Vec<T>> for alloc::vec::Vec<T> {
        fn from(value: Vec<T>) -> Self {
            value.into_vec()
        }
    }
    impl<'a, T> From<&'a Vec<T>> for &'a alloc::vec::Vec<T> {
        fn from(value: &'a Vec<T>) -> Self {
            value.as_vec()
        }
    }
}
