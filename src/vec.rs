use core::{
    cmp::Ordering,
    iter::{Extend, IntoIterator},
    mem::MaybeUninit,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
};

use alloc::{boxed::Box, collections::TryReserveError};

use crate::Slice;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Vec<T> {
    inner: alloc::vec::Vec<T>,
}

/// Constructors
impl<T> Vec<T> {
    crate::map_non_empty! {
        new(alloc::vec::Vec<T>) -> Self: Self::new_unchecked;
        new_ref(&alloc::vec::Vec<T>) -> &Self: Self::new_ref_unchecked;
        new_mut(&mut alloc::vec::Vec<T>) -> &mut Self: Self::new_mut_unchecked;
    }
    crate::transmuting! {
        new_unchecked(alloc::vec::Vec<T>) -> Self;
        new_ref_unchecked(&alloc::vec::Vec<T>) -> &Self;
        new_mut_unchecked(&mut alloc::vec::Vec<T>) -> &mut Self;
    }
    pub fn of(item: T) -> Self {
        Self::with_capacity(item, 1)
    }
    pub fn with_capacity(item: T, capacity: usize) -> Self {
        let mut inner = alloc::vec::Vec::with_capacity(capacity);
        inner.push(item);
        unsafe { Self::new_unchecked(inner) }
    }
    fn check(&self) {
        debug_assert_ne!(self.inner.len(), 0)
    }
}

impl<T> Vec<T> {
    pub fn as_vec(&self) -> &alloc::vec::Vec<T> {
        self.check();
        &self.inner
    }
    /// # Safety
    /// - returned vec must not be emptied through this reference
    pub unsafe fn as_mut_vec(&mut self) -> &mut alloc::vec::Vec<T> {
        self.check();
        &mut self.inner
    }
    pub fn into_vec(self) -> alloc::vec::Vec<T> {
        let Self { inner } = self;
        inner
    }
    pub fn as_slice(&self) -> &Slice<T> {
        unsafe { Slice::new_unchecked(self.as_vec()) }
    }
    pub fn as_mut_slice(&mut self) -> &mut Slice<T> {
        unsafe { Slice::new_mut_unchecked(self.as_mut_vec()) }
    }
}

macro_rules! forward_mut {
    ($( $(#[$meta:meta])* $vis:vis fn $ident:ident(&mut self $(,$arg:ident: $ty:ty)* $(,)?) $(-> $ret:ty)?);* $(;)?) => {
        $(
            $(#[$meta])*
            $vis fn $ident(&mut self $(, $arg: $ty)*) $(-> $ret)? {
                // Safety:
                // - operation does not remove elements
                unsafe { self.as_mut_vec() }.$ident($($arg),*)
            }
        )*
    };
}

/// Forwarded and shimmed methods, in rustdoc order for [`alloc::vec::Vec`].
impl<T> Vec<T> {
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

    pub fn into_boxed_slice(self) -> Box<Slice<T>> {
        match cfg!(debug_assertions) {
            true => {
                let src = self.into_vec().into_boxed_slice();
                let len0 = src.len();
                let ptr = Box::into_raw(src);
                // Safety:
                // - #[repr(transparent)]
                let dst = unsafe { Box::from_raw(ptr as *mut Slice<T>) };
                let len1 = dst.len().get();
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

    pub fn truncate(&mut self, len: NonZeroUsize) {
        // Safety:
        // - len is not zero, so vector will not be emptied
        unsafe { self.as_mut_vec() }.truncate(len.get());
        self.check();
    }

    /// # Safety
    /// - See [`alloc::vec::Vec::set_len`].
    pub unsafe fn set_len(&mut self, new_len: NonZeroUsize) {
        // Safety:
        // - len is not zero, so vector will not be emptied
        unsafe { self.as_mut_vec() }.set_len(new_len.get());
        self.check();
    }

    forward_mut! {
        pub fn insert(&mut self, index: usize, element: T);
    }

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
    pub fn len(&self) -> NonZeroUsize {
        self.as_slice().len()
    }
    // pub fn split_off(&mut self, at: NonZeroUsize)
    pub fn resize_with<F>(&mut self, new_len: NonZeroUsize, f: F)
    where
        F: FnMut() -> T,
    {
        // Safety:
        // - new_len is not zero, so vec cannot be emptied
        unsafe { self.as_mut_vec() }.resize_with(new_len.get(), f);
        self.check();
    }
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

impl<T> Deref for Vec<T> {
    type Target = Slice<T>;

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
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

mod against_primitives {
    use super::*;

    impl<T> PartialOrd<[T]> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            self.as_slice().as_slice().partial_cmp(other)
        }
    }

    impl<const N: usize, T> PartialOrd<[T; N]> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> {
            self.as_slice().as_slice().partial_cmp(other)
        }
    }
}

mod against_std {
    use super::*;

    impl<T> PartialOrd<alloc::vec::Vec<T>> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &alloc::vec::Vec<T>) -> Option<Ordering> {
            self.as_vec().partial_cmp(other)
        }
    }
}

crate::slice_iter! {
    <T> for Vec<T>
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
}
