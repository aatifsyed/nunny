use core::{
    array,
    ops::{Deref, DerefMut},
};

use crate::Slice;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Array<const N: usize, T> {
    inner: [T; N],
}

/// Creation
impl<const N: usize, T> Array<N, T> {
    crate::map_non_empty! {
        new([T; N]) -> Self: Self::new_unchecked;
        new_ref(&[T; N]) -> &Self: Self::new_ref_unchecked;
        new_mut(&mut [T; N]) -> &mut Self: Self::new_mut_unchecked;
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

impl<T> Array<1, T> {
    pub fn of(item: T) -> Self {
        let src = [item];
        unsafe { Self::new_unchecked(src) }
    }
    pub fn of_mut(item: &mut T) -> &mut Self {
        let src = array::from_mut(item);
        unsafe { Self::new_mut_unchecked(src) }
    }
    pub fn of_ref(item: &T) -> &Self {
        let src = array::from_ref(item);
        unsafe { Self::new_ref_unchecked(src) }
    }
}

impl<const N: usize, T> Array<N, T> {
    pub fn each_ref(&self) -> Array<N, &T> {
        Array {
            inner: self.as_array().each_ref(),
        }
    }
    pub fn each_mut(&mut self) -> Array<N, &mut T> {
        Array {
            inner: self.as_mut_array().each_mut(),
        }
    }
    pub fn map<F, U>(self, f: F) -> Array<N, U>
    where
        F: FnMut(T) -> U,
    {
        Array {
            inner: self.into_array().map(f),
        }
    }
}

impl<const N: usize, T> Array<N, T> {
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

impl<const N: usize, T> Deref for Array<N, T> {
    type Target = Slice<T>;

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<const N: usize, T> DerefMut for Array<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

crate::as_ref_as_mut! {
    <T, const N: usize> for Array<N, T> as [T];
    <T, const N: usize> for Array<N, T> as Slice<T>;
    <T, const N: usize> for Array<N, T> as Self;
}

crate::borrow_borrow_mut! {
    <T, const N: usize> for Array<N, T> as [T];
    <T, const N: usize> for Array<N, T> as Slice<T>;
}

crate::slice_iter! {
    <T, const N: usize> for Array<N, T>
}

impl<const N: usize, T> IntoIterator for Array<N, T> {
    type Item = T;

    type IntoIter = core::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

mod partial_eq_std {
    use super::*;

    impl<T, U, const N: usize> PartialEq<[U]> for Array<N, T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<[U; N]> for Array<N, T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U; N]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<alloc::vec::Vec<U>> for Array<N, T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &alloc::vec::Vec<U>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
}
