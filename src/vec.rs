use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    convert::{AsMut, AsRef},
    iter::{Extend, IntoIterator},
    mem::MaybeUninit,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
};

use alloc::collections::TryReserveError;

use crate::{Error, Slice};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        const new_unchecked(alloc::vec::Vec<T>) -> Self;
        const new_ref_unchecked(&alloc::vec::Vec<T>) -> &Self;
        new_mut_unchecked(&mut alloc::vec::Vec<T>) -> &mut Self;
    }
    pub fn of(item: T) -> Self {
        Self::with_capacity(item, 1)
    }
    pub fn with_capacity(item: T, capacity: usize) -> Self {
        let mut inner = alloc::vec::Vec::with_capacity(capacity);
        inner.push(item);
        debug_assert_eq!(inner.len(), 1);
        // Safety:
        // - len is 1
        unsafe { Self::new_unchecked(inner) }
    }
    fn check(&self) {
        #[cfg(debug_assertions)]
        if self.inner.is_empty() {
            panic!("Vec was empty!")
        }
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
        if cfg!(debug_assertions) {
            NonZeroUsize::new(self.as_vec().capacity()).unwrap()
        } else {
            // Safety:
            // - length != 0 so capacity != zero
            unsafe { NonZeroUsize::new_unchecked(self.as_vec().capacity()) }
        }
    }

    forward_mut! {
        pub fn reserve(&mut self, additional: usize);
        pub fn reserve_exact(&mut self, additional: usize);
        pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
        pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError>;
        pub fn shrink_to_fit(&mut self);
        pub fn shrink_to(&mut self, min_capacity: usize);
    }

    // pub fn into_boxed_slice(self) -> Box<NonEmpty<[T]>> { .. }

    pub fn truncate(&mut self, len: NonZeroUsize) {
        // Safety:
        // - len is not zero, so vector will not be emptied
        unsafe { self.as_mut_vec() }.truncate(len.get())
    }

    /// # Safety
    /// - See [`alloc::vec::Vec::set_len`].
    pub unsafe fn set_len(&mut self, new_len: NonZeroUsize) {
        // Safety:
        // - len is not zero, so vector will not be emptied
        unsafe { self.as_mut_vec() }.set_len(new_len.get())
    }

    /// # Panics
    /// - If the vector would be left empty by this operation.
    /// - If `index` is out of bounds
    pub fn swap_remove(&mut self, index: usize) -> T {
        assert_ne!(
            self.as_vec().len(),
            1,
            "cannot remove from a nonempty vec of length 1"
        );
        // Safety:
        // - removal cannot empty the vec if the above check succeeded
        unsafe { self.as_mut_vec() }.remove(index)
    }

    forward_mut! {
        pub fn insert(&mut self, index: usize, element: T);
    }

    /// # Panics
    /// - If the vector would be left empty by this operation.
    /// - If `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        assert_ne!(
            self.as_vec().len(),
            1,
            "cannot remove from a nonempty vec of length 1"
        );
        // Safety:
        // - removal cannot empty the vec if the above check succeeded
        unsafe { self.as_mut_vec() }.remove(index)
    }

    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        // Safety:
        // - dedup always leaves the first element
        unsafe { self.as_mut_vec() }.dedup_by_key(key)
    }
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        // Safety:
        // - dedup always leaves the first element
        unsafe { self.as_mut_vec() }.dedup_by(same_bucket)
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
        unsafe { self.as_mut_vec() }.resize_with(new_len.get(), f)
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

mod against_primitives {
    use super::*;

    // AsRef/AsMut [T]
    impl<T> AsRef<[T]> for Vec<T> {
        fn as_ref(&self) -> &[T] {
            self
        }
    }
    impl<T> AsMut<[T]> for Vec<T> {
        fn as_mut(&mut self) -> &mut [T] {
            self
        }
    }

    // Borrow/BorrowMut [T]
    impl<T> Borrow<[T]> for Vec<T> {
        fn borrow(&self) -> &[T] {
            self
        }
    }
    impl<T> BorrowMut<[T]> for Vec<T> {
        fn borrow_mut(&mut self) -> &mut [T] {
            self
        }
    }

    // PartialEq<U>/PartialOrd [T]
    impl<T, U> PartialEq<[U]> for Vec<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U]) -> bool {
            self.as_slice().as_slice().eq(other)
        }
    }
    impl<T> PartialOrd<[T]> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            self.as_slice().as_slice().partial_cmp(other)
        }
    }

    // PartialEq<U>/PartialOrd [T; N]
    impl<const N: usize, T, U> PartialEq<[U; N]> for Vec<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U; N]) -> bool {
            self.as_slice().as_slice().eq(other)
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

    // TryFrom &[T]/&mut [T]
    impl<T> TryFrom<&[T]> for Vec<T>
    where
        T: Clone,
    {
        type Error = Error;

        fn try_from(value: &[T]) -> Result<Self, Self::Error> {
            let inner = <&Slice<_>>::try_from(value)?;
            Ok(inner.into())
        }
    }
    impl<T> TryFrom<&mut [T]> for Vec<T>
    where
        T: Clone,
    {
        type Error = Error;

        fn try_from(value: &mut [T]) -> Result<Self, Self::Error> {
            let inner = <&Slice<_>>::try_from(&*value)?;
            Ok(inner.into())
        }
    }
    // TryFrom &[T; N]/&mut [T; N]
    impl<const N: usize, T> TryFrom<&[T; N]> for Vec<T>
    where
        T: Clone,
    {
        type Error = Error;

        fn try_from(value: &[T; N]) -> Result<Self, Self::Error> {
            let inner = <&Slice<_>>::try_from(value)?;
            Ok(inner.into())
        }
    }
    impl<const N: usize, T> TryFrom<&mut [T; N]> for Vec<T>
    where
        T: Clone,
    {
        type Error = Error;

        fn try_from(value: &mut [T; N]) -> Result<Self, Self::Error> {
            let inner = <&Slice<_>>::try_from(&*value)?;
            Ok(inner.into())
        }
    }

    // TryFrom [T; N]
    impl<const N: usize, T> TryFrom<[T; N]> for Vec<T> {
        type Error = Error;

        fn try_from(value: [T; N]) -> Result<Self, Self::Error> {
            let inner = alloc::vec::Vec::from(value);
            Self::try_from(inner)
        }
    }
}

mod against_nonempty {
    use super::*;

    // AsRef/AsMut Slice<T>
    impl<T> AsRef<Slice<T>> for Vec<T> {
        fn as_ref(&self) -> &Slice<T> {
            self
        }
    }
    impl<T> AsMut<Slice<T>> for Vec<T> {
        fn as_mut(&mut self) -> &mut Slice<T> {
            self
        }
    }

    // Borrow/BorrowMut Slice<T>
    impl<T> Borrow<Slice<T>> for Vec<T> {
        fn borrow(&self) -> &Slice<T> {
            self
        }
    }
    impl<T> BorrowMut<Slice<T>> for Vec<T> {
        fn borrow_mut(&mut self) -> &mut Slice<T> {
            self
        }
    }

    // PartialEq<U>/PartialOrd Slice<T>
    impl<T, U> PartialEq<Slice<U>> for Vec<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &Slice<U>) -> bool {
            self.as_slice().eq(other)
        }
    }
    impl<T> PartialOrd<Slice<T>> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Slice<T>) -> Option<Ordering> {
            self.as_slice().partial_cmp(other)
        }
    }

    // From &Slice<T>/&mut Slice<T>
    impl<T> From<&Slice<T>> for Vec<T>
    where
        T: Clone,
    {
        fn from(value: &Slice<T>) -> Self {
            let inner = alloc::vec::Vec::from(value.as_slice());
            // Safety:
            // - inner is nonempty by construction
            unsafe { Self::new_unchecked(inner) }
        }
    }
    impl<T> From<&mut Slice<T>> for Vec<T>
    where
        T: Clone,
    {
        fn from(value: &mut Slice<T>) -> Self {
            let inner = alloc::vec::Vec::from(value.as_slice());
            // Safety:
            // - inner is nonempty by construction
            unsafe { Self::new_unchecked(inner) }
        }
    }
}

mod against_std {
    use super::*;

    impl<T, U> PartialEq<alloc::vec::Vec<U>> for Vec<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &alloc::vec::Vec<U>) -> bool {
            self.as_vec().eq(other)
        }
    }
    impl<T> PartialOrd<alloc::vec::Vec<T>> for Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &alloc::vec::Vec<T>) -> Option<Ordering> {
            self.as_vec().partial_cmp(other)
        }
    }

    impl<T> TryFrom<alloc::vec::Vec<T>> for Vec<T> {
        type Error = Error;

        fn try_from(value: alloc::vec::Vec<T>) -> Result<Self, Self::Error> {
            Self::new(value).ok_or(Error(()))
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
    impl<'a, T> IntoIterator for &'a Vec<T> {
        type Item = &'a T;

        type IntoIter = core::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.as_slice().into_iter()
        }
    }
    impl<'a, T> IntoIterator for &'a mut Vec<T> {
        type Item = &'a mut T;

        type IntoIter = core::slice::IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.as_mut_slice().into_iter()
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
