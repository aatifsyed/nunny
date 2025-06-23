use core::{
    array,
    ops::{Deref, DerefMut},
};

use crate::{Array, NonEmpty, Slice};

impl<T, const N: usize> Eq for NonEmpty<[T; N]> where T: Eq {}

/// [`Array`] methods
impl<const N: usize, T> Array<T, N> {
    ///////////
    // Creation
    ///////////

    crate::map_non_empty! {
        const
        /// Returns a [`NonEmpty`] array.
        new_ref(&[T; N]) -> &Self: Self::new_ref_unchecked;

        /// Returns a [`NonEmpty`] array.
        new_mut(&mut [T; N]) -> &mut Self: Self::new_mut_unchecked;

        /// Returns a [`NonEmpty`] array.
        new([T; N]) -> Self: Self::new_unchecked;
    }

    crate::transmuting! {
        const
        /// Create a [`NonEmpty`] array.
        new_ref_unchecked(&[T; N]) -> &Self;

        /// Create a [`NonEmpty`] array.
        new_mut_unchecked(&mut [T; N]) -> &mut Self;

        // const new_unchecked([T; N]) -> Self; // compiler can't tell this is OK
    }

    /// Create a [`NonEmpty`] array.
    ///
    /// # Safety
    /// - `src` must not be empty
    pub const unsafe fn new_unchecked(src: [T; N]) -> Self {
        Self { inner: src }
    }

    ////////////
    // Utilities
    ////////////

    /// Borrows each element and returns a [`NonEmpty`] array of references with the same size as self.
    pub fn each_ref(&self) -> Array<&T, N> {
        Array {
            inner: self.as_array().each_ref(),
        }
    }
    /// Borrows each element mutably and returns a [`NonEmpty`] array of mutable references with the same size as self.
    pub fn each_mut(&mut self) -> Array<&mut T, N> {
        Array {
            inner: self.as_mut_array().each_mut(),
        }
    }
    /// Returns a [`NonEmpty`] array of the same size as self, with function f applied to each element in order.
    #[doc(alias = "map")] // [`<[T; N]>::map`](https://doc.rust-lang.org/std/primitive.array.html#method.map)
    pub fn each_map<F, U>(self, f: F) -> Array<U, N>
    where
        F: FnMut(T) -> U,
    {
        Array {
            inner: self.into_array().map(f),
        }
    }

    ///////////////////
    // Inner references
    ///////////////////

    /// Returns a [`NonEmpty`] slice.
    pub const fn as_slice_ne(&self) -> &Slice<T> {
        let src = self.inner.as_slice();
        // Safety
        // - src is not empty by construction
        unsafe { Slice::new_unchecked(src) }
    }
    /// Returns a [`NonEmpty`] slice.
    pub fn as_mut_slice_ne(&mut self) -> &mut Slice<T> {
        let src = self.inner.as_mut_slice();
        // Safety
        // - src is not empty by construction
        unsafe { Slice::new_mut_unchecked(src) }
    }
    /// Returns a [`primitive array`](primitive@array).
    pub const fn as_array(&self) -> &[T; N] {
        &self.inner
    }
    /// Returns a [`primitive array`](primitive@array).
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        &mut self.inner
    }
    /// Returns a [`primitive array`](primitive@array).
    pub fn into_array(self) -> [T; N] {
        let Self { inner } = self;
        inner
    }
}

/// Known non-empty iterator for [`Array`].
impl<T, const N: usize> Array<T, N> {
    pub fn into_iter_ne(self) -> NonEmpty<core::array::IntoIter<T, N>> {
        NonEmpty {
            inner: self.into_iter(),
        }
    }
}

/// Special case for [`Array`]s of length one
impl<T> Array<T, 1> {
    /// Create a [`NonEmpty`] array of a single element
    pub const fn of(item: T) -> Self {
        let src = [item];
        unsafe { Self::new_unchecked(src) }
    }
    /// Create a [`NonEmpty`] array of a single mutable reference
    pub fn of_mut(item: &mut T) -> &mut Self {
        let src = array::from_mut(item);
        unsafe { Self::new_mut_unchecked(src) }
    }
    /// Create a [`NonEmpty`] array of a single reference
    pub const fn of_ref(item: &T) -> &Self {
        let src = array::from_ref(item);
        unsafe { Self::new_ref_unchecked(src) }
    }
}

/// [`Array`] to [`Slice`]
impl<const N: usize, T> Deref for Array<T, N> {
    type Target = Slice<T>;

    fn deref(&self) -> &Self::Target {
        self.as_slice_ne()
    }
}
/// [`Array`] to [`Slice`]
impl<const N: usize, T> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice_ne()
    }
}

crate::as_ref_as_mut! {
    <T, const N: usize> for Array<T, N> as [T];
    <T, const N: usize> for Array<T, N> as Slice<T>;
    <T, const N: usize> for Array<T, N> as Self;
}

crate::borrow_borrow_mut! {
    <T, const N: usize> for Array<T, N> as [T];
    <T, const N: usize> for Array<T, N> as Slice<T>;
}

crate::slice_iter! {
    <T, const N: usize> for Array<T, N>
}

impl<const N: usize, T> IntoIterator for Array<T, N> {
    type Item = T;

    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

mod partial_eq_std {
    use super::*;

    impl<T, U, const N: usize> PartialEq<[U]> for Array<T, N>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<[U; N]> for Array<T, N>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U; N]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<T, U, const N: usize> PartialEq<alloc::vec::Vec<U>> for Array<T, N>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &alloc::vec::Vec<U>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }

    // converse
    //---------

    impl<T, U, const N: usize> PartialEq<Array<T, N>> for [U]
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Array<T, N>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<Array<T, N>> for [U; N]
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Array<T, N>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<T, U, const N: usize> PartialEq<Array<T, N>> for alloc::vec::Vec<U>
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Array<T, N>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
}
mod cmp_std {
    use core::cmp::Ordering;

    use super::*;

    impl<T, const N: usize> PartialOrd<[T]> for Array<T, N>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    impl<T, const N: usize> PartialOrd<[T; N]> for Array<T, N>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<T, const N: usize> PartialOrd<alloc::vec::Vec<T>> for Array<T, N>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &alloc::vec::Vec<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }

    // converse
    //---------

    impl<T, const N: usize> PartialOrd<Array<T, N>> for [T]
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Array<T, N>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    impl<T, const N: usize> PartialOrd<Array<T, N>> for [T; N]
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Array<T, N>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    impl<T, const N: usize> PartialOrd<Array<T, N>> for alloc::vec::Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Array<T, N>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
}

mod convert_std {
    use crate::Error;

    use super::*;

    impl<T, const N: usize> TryFrom<[T; N]> for Array<T, N> {
        type Error = Error;

        fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
            Self::new(value).ok_or(Error(()))
        }
    }
    impl<'a, T, const N: usize> TryFrom<&'a [T; N]> for &'a Array<T, N> {
        type Error = Error;

        fn try_from(value: &'a [T; N]) -> Result<Self, Self::Error> {
            Array::new_ref(value).ok_or(Error(()))
        }
    }
    impl<'a, T, const N: usize> TryFrom<&'a mut [T; N]> for &'a mut Array<T, N> {
        type Error = Error;

        fn try_from(value: &'a mut [T; N]) -> Result<Self, Self::Error> {
            Array::new_mut(value).ok_or(Error(()))
        }
    }

    impl<T, const N: usize> From<Array<T, N>> for [T; N] {
        fn from(value: Array<T, N>) -> Self {
            value.into_array()
        }
    }
    impl<'a, T, const N: usize> From<&'a Array<T, N>> for &'a [T; N] {
        fn from(value: &'a Array<T, N>) -> Self {
            value.as_array()
        }
    }
    impl<'a, T, const N: usize> From<&'a mut Array<T, N>> for &'a mut [T; N] {
        fn from(value: &'a mut Array<T, N>) -> Self {
            value.as_mut_array()
        }
    }
}
