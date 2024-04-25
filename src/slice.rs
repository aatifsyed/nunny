use core::{
    hint::unreachable_unchecked,
    iter::IntoIterator,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
    slice,
};

#[derive(Debug, Eq, PartialOrd, Ord, Hash)]
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
    pub const fn of(item: &T) -> &Self {
        let shared = slice::from_ref(item);
        // Safety:
        // - len is 1
        unsafe { Self::new_unchecked(shared) }
    }
    pub fn of_mut(item: &mut T) -> &mut Self {
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
                    None => match cfg!(debug_assertions) {
                        true => unreachable!(),
                        // Safety:
                        // - cannot create an empty slice without unsafe
                        false => unsafe { unreachable_unchecked() },
                    },
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
                    None => match cfg!(debug_assertions) {
                        true => unreachable!(),
                        // Safety:
                        // - cannot create an empty slice without unsafe
                        false => unsafe { unreachable_unchecked() },
                    },
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

mod iter {
    use super::*;
    use core::slice::{Iter, IterMut};

    impl<'a, T> IntoIterator for &'a Slice<T> {
        type Item = &'a T;

        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    impl<'a, T> IntoIterator for &'a mut Slice<T> {
        type Item = &'a mut T;

        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }

    #[cfg(feature = "alloc")]
    impl<T> IntoIterator for alloc::boxed::Box<Slice<T>> {
        type Item = T;

        type IntoIter = alloc::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            crate::Vec::<T>::from(self).into_iter()
        }
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
