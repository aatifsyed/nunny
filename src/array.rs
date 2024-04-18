use core::ops::{Deref, DerefMut};

use crate::Slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    }

    /// # Safety
    /// - `src` must not be empty
    pub const unsafe fn new_unchecked(src: [T; N]) -> Self {
        Self { inner: src }
    }
}

impl<const N: usize, T> Array<N, T> {
    pub fn each_ref(&self) -> Array<N, &T> {
        Array {
            inner: self.inner.each_ref(),
        }
    }
    pub fn each_mut(&mut self) -> Array<N, &mut T> {
        Array {
            inner: self.inner.each_mut(),
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
