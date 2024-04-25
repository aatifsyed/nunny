use core::{
    array,
    ops::{Deref, DerefMut},
};

use crate::{Array, Slice};

impl<T, const N: usize> Eq for Array<T, N> where T: Eq {}

/// Creation
impl<const N: usize, T> Array<T, N> {
    crate::map_non_empty! {
        /// Create a new reference to an array.
        /// If you have special knowledge of the array
        new_ref(&[T; N]) -> &Self: Self::new_ref_unchecked;
        new_mut(&mut [T; N]) -> &mut Self: Self::new_mut_unchecked;
    }

    pub const fn new(src: [T; N]) -> Result<Self, [T; N]> {
        match N != 0 {
            true => Ok(unsafe { Self::new_unchecked(src) }),
            false => Err(src),
        }
    }

    crate::transmuting! {
        const new_ref_unchecked(&[T; N]) -> &Self;
        new_mut_unchecked(&mut [T; N]) -> &mut Self;
        // const new_unchecked([T; N]) -> Self; // compiler can't tell this is OK
    }

    /// # Safety
    /// - `src` must not be empty
    pub const unsafe fn new_unchecked(src: [T; N]) -> Self {
        Self { inner: src }
    }
}

impl<const N: usize, T> Array<T, N> {
    pub fn each_ref(&self) -> Array<&T, N> {
        Array {
            inner: self.as_array().each_ref(),
        }
    }
    pub fn each_mut(&mut self) -> Array<&mut T, N> {
        Array {
            inner: self.as_mut_array().each_mut(),
        }
    }
    pub fn map<F, U>(self, f: F) -> Array<U, N>
    where
        F: FnMut(T) -> U,
    {
        Array {
            inner: self.into_array().map(f),
        }
    }
}

impl<const N: usize, T> Array<T, N> {
    pub const fn as_slice(&self) -> &Slice<T> {
        let src = self.inner.as_slice();
        // Safety
        // - src is not empty by construction
        unsafe { Slice::new_unchecked(src) }
    }
    pub fn as_mut_slice(&mut self) -> &mut Slice<T> {
        let src = self.inner.as_mut_slice();
        // Safety
        // - src is not empty by construction
        unsafe { Slice::new_mut_unchecked(src) }
    }
    pub const fn as_array(&self) -> &[T; N] {
        &self.inner
    }
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        &mut self.inner
    }
    pub fn into_array(self) -> [T; N] {
        let Self { inner } = self;
        inner
    }
}

impl<T> Array<T, 1> {
    pub const fn of(item: T) -> Self {
        let src = [item];
        unsafe { Self::new_unchecked(src) }
    }
    pub fn of_mut(item: &mut T) -> &mut Self {
        let src = array::from_mut(item);
        unsafe { Self::new_mut_unchecked(src) }
    }
    pub const fn of_ref(item: &T) -> &Self {
        let src = array::from_ref(item);
        unsafe { Self::new_ref_unchecked(src) }
    }
}

/// These methods are provided as a convenience for fixed arrays where length is:
/// - any number `1..=256`
/// - powers of two `..=65536`
impl Array<(), 0> {
    /// Create an array for a known non-zero array
    pub fn exact<A: Exact>(item: A) -> A::Out {
        item.exact()
    }
    pub fn exact_ref<A: Exact>(item: &A) -> &A::Out {
        item.exact_ref()
    }
    pub fn exact_mut<A: Exact>(item: &mut A) -> &mut A::Out {
        item.exact_mut()
    }
}

/// See impl block on [`Array::exact`]
pub trait Exact {
    type Out;
    fn exact(self) -> Self::Out;
    fn exact_ref(&self) -> &Self::Out;
    fn exact_mut(&mut self) -> &mut Self::Out;
}

macro_rules! impl_exact {
    ($n:literal) => {
        #[allow(clippy::zero_prefixed_literal)]
        impl<T> Exact for [T; $n] {
            type Out = Array<T, $n>;
            fn exact(self) -> Self::Out {
                unsafe { Array::new_unchecked(self) }
            }
            fn exact_ref(&self) -> &Self::Out {
                unsafe { Array::new_ref_unchecked(self) }
            }
            fn exact_mut(&mut self) -> &mut Self::Out {
                unsafe { Array::new_mut_unchecked(self) }
            }
        }
    };
}

crate::for_nonzeroes!(impl_exact);

impl<const N: usize, T> Deref for Array<T, N> {
    type Target = Slice<T>;

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<const N: usize, T> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
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
        type Error = [T; N];

        fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
            Self::new(value)
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
