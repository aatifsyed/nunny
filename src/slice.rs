use core::{
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
    slice,
};

#[derive(Debug, Eq, Hash)]
#[repr(transparent)]
pub struct Slice<T> {
    inner: [T],
}

/// Creation
impl<T> Slice<T> {
    crate::map_non_empty! {
        const new(&[T]) -> &Self: Self::new_unchecked;
        new_mut(&mut [T]) -> &mut Self: Self::new_mut_unchecked;
    }
    crate::transmuting! {
        const new_unchecked(&[T]) -> &Self;
        new_mut_unchecked(&mut [T]) -> &mut Self;
    }
    pub const fn one(item: &T) -> &Self {
        let shared = slice::from_ref(item);
        // Safety:
        // - len is 1
        unsafe { Self::new_unchecked(shared) }
    }
    pub fn one_mut(item: &mut T) -> &mut Self {
        let shared = slice::from_mut(item);
        // Safety:
        // - len is 1
        unsafe { Self::new_mut_unchecked(shared) }
    }
    const fn check(&self) {
        debug_assert!(!self.inner.is_empty());
    }
}

/// Gateway methods to core primitives
impl<T> Slice<T> {
    pub const fn as_slice(&self) -> &[T] {
        self.check();
        &self.inner
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.check();
        &mut self.inner
    }
}

macro_rules! forward {
    ($($(#[$meta:meta])* pub const fn $ident:ident(&self) -> $ty:ty);* $(;)?) => {
        $(
            $(#[$meta])*
            pub const fn $ident(&self) -> $ty {
                match self.as_slice().$ident() {
                    Some(it) => it,
                    // Safety:
                    // - cannot create empty slice without `unsafe`
                    None => unsafe { crate::unreachable() },
                }
            }
        )*
    }
}
macro_rules! forward_mut {
    ($($(#[$meta:meta])* pub fn $ident:ident(&mut self) -> $ty:ty);* $(;)?) => {
        $(
            $(#[$meta])*
            pub fn $ident(&mut self) -> $ty {
                match self.as_mut_slice().$ident() {
                    Some(it) => it,
                    // Safety:
                    // - cannot create empty slice without `unsafe`
                    None => unsafe { crate::unreachable() },
                }
            }
        )*
    }
}

/// Shimmed methods for std::primitive::slice
impl<T> Slice<T> {
    pub const fn len(&self) -> NonZeroUsize {
        unsafe { crate::non_zero_usize(self.inner.len()) }
    }
    forward! {
        pub const fn first(&self) -> &T;
        pub const fn split_first(&self) -> (&T, &[T]);
        pub const fn split_last(&self) -> (&T, &[T]);
        pub const fn last(&self) -> &T;
    }
    forward_mut! {
        pub fn first_mut(&mut self) -> &mut T ;
        pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]);
        pub fn split_last_mut(&mut self) -> (&mut T, &mut [T]);
        pub fn last_mut(&mut self) -> &mut T;
    }
}

impl<T> Deref for Slice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for Slice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

crate::as_ref_as_mut! {
    <T> for Slice<T> as [T];
    <T> for Slice<T> as Self;
}

crate::borrow_borrow_mut! {
    <T> for Slice<T> as [T];
}

crate::slice_iter! {
    <T> for Slice<T>
}

#[cfg(feature = "alloc")]
impl<T> IntoIterator for alloc::boxed::Box<Slice<T>> {
    type Item = T;

    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        crate::Vec::<T>::from(self).into_iter()
    }
}

#[cfg(feature = "alloc")]
impl<T> alloc::borrow::ToOwned for Slice<T>
where
    T: Clone,
{
    type Owned = crate::Vec<T>;

    fn to_owned(&self) -> Self::Owned {
        self.into()
    }
}

mod partial_eq_std {
    use super::*;

    impl<T, U> PartialEq<[U]> for Slice<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<[U; N]> for Slice<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U; N]) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    impl<T, U> PartialEq<alloc::vec::Vec<U>> for Slice<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &alloc::vec::Vec<U>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }

    // converse
    //---------

    impl<T, U> PartialEq<Slice<T>> for [U]
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Slice<T>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    impl<T, U, const N: usize> PartialEq<Slice<T>> for [U; N]
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Slice<T>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    impl<T, U> PartialEq<Slice<T>> for alloc::vec::Vec<U>
    where
        U: PartialEq<T>,
    {
        fn eq(&self, other: &Slice<T>) -> bool {
            <[_] as PartialEq<[_]>>::eq(self, other)
        }
    }
}

mod cmp_std {
    use core::cmp::Ordering;

    use super::*;

    impl<T> PartialOrd<[T]> for Slice<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    impl<T, const N: usize> PartialOrd<[T; N]> for Slice<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    impl<T> PartialOrd<alloc::vec::Vec<T>> for Slice<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &alloc::vec::Vec<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }

    // converse
    //---------

    impl<T> PartialOrd<Slice<T>> for [T]
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Slice<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    impl<T, const N: usize> PartialOrd<Slice<T>> for [T; N]
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Slice<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
    #[cfg(feature = "alloc")]
    impl<T> PartialOrd<Slice<T>> for alloc::vec::Vec<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &Slice<T>) -> Option<Ordering> {
            <[_] as PartialOrd<[_]>>::partial_cmp(self, other)
        }
    }
}

mod convert_std {
    use crate::Error;

    use super::*;

    impl<'a, T> TryFrom<&'a [T]> for &'a Slice<T> {
        type Error = Error;

        fn try_from(value: &'a [T]) -> Result<Self, Self::Error> {
            Slice::new(value).ok_or(Error(()))
        }
    }
    impl<'a, T> TryFrom<&'a mut [T]> for &'a mut Slice<T> {
        type Error = Error;

        fn try_from(value: &'a mut [T]) -> Result<Self, Self::Error> {
            Slice::new_mut(value).ok_or(Error(()))
        }
    }

    impl<'a, T> From<&'a Slice<T>> for &'a [T] {
        fn from(value: &'a Slice<T>) -> Self {
            value.as_slice()
        }
    }
    impl<'a, T> From<&'a mut Slice<T>> for &'a mut [T] {
        fn from(value: &'a mut Slice<T>) -> Self {
            value.as_mut_slice()
        }
    }
}
