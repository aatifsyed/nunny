use crate::Error;
use core::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    hint::unreachable_unchecked,
    iter::IntoIterator,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
    slice::{self, Iter, IterMut},
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
        #[cfg(debug_assertions)]
        if self.inner.is_empty() {
            panic!("Slice was empty!")
        }
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
                    // - cannot create an empty slice without unsafe
                    None => unsafe { unreachable_unchecked() },
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
                    // - cannot create an empty slice without unsafe
                    None => unsafe { unreachable_unchecked() },
                }
            }
        )*
    }
}

/// Shimmed methods for std::primitive::slice
impl<T> Slice<T> {
    pub const fn len(&self) -> NonZeroUsize {
        if cfg!(debug_assertions) {
            match NonZeroUsize::new(self.as_slice().len()) {
                Some(it) => it,
                None => panic!("Slice was empty!"),
            }
        } else {
            unsafe { NonZeroUsize::new_unchecked(self.as_slice().len()) }
        }
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

mod against_primitives {
    use super::*;

    // AsRef/AsMut [T]
    impl<T> AsRef<[T]> for Slice<T> {
        fn as_ref(&self) -> &[T] {
            self.as_slice()
        }
    }
    impl<T> AsMut<[T]> for Slice<T> {
        fn as_mut(&mut self) -> &mut [T] {
            self.as_mut_slice()
        }
    }

    // Borrow/BorrowMut [T]
    impl<T> Borrow<[T]> for Slice<T> {
        fn borrow(&self) -> &[T] {
            self.as_slice()
        }
    }
    impl<T> BorrowMut<[T]> for Slice<T> {
        fn borrow_mut(&mut self) -> &mut [T] {
            self.as_mut_slice()
        }
    }

    // PartialEq<U>/PartialOrd [T]
    impl<T, U> PartialEq<[U]> for Slice<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U]) -> bool {
            self.as_slice().eq(other)
        }
    }
    impl<T> PartialOrd<[T]> for Slice<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
            self.as_slice().partial_cmp(other)
        }
    }

    // PartialEq<U>/PartialOrd [T; N]
    impl<const N: usize, T, U> PartialEq<[U; N]> for Slice<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &[U; N]) -> bool {
            self.as_slice().eq(other)
        }
    }
    impl<const N: usize, T> PartialOrd<[T; N]> for Slice<T>
    where
        T: PartialOrd,
    {
        fn partial_cmp(&self, other: &[T; N]) -> Option<Ordering> {
            self.as_slice().partial_cmp(other)
        }
    }

    // TryFrom [T]
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

    // TryFrom [T; N]
    impl<'a, const N: usize, T> TryFrom<&'a [T; N]> for &'a Slice<T> {
        type Error = Error;

        fn try_from(value: &'a [T; N]) -> Result<Self, Self::Error> {
            Slice::new(value).ok_or(Error(()))
        }
    }
    impl<'a, const N: usize, T> TryFrom<&'a mut [T; N]> for &'a mut Slice<T> {
        type Error = Error;

        fn try_from(value: &'a mut [T; N]) -> Result<Self, Self::Error> {
            Slice::new_mut(value).ok_or(Error(()))
        }
    }
}

mod against_self {
    use super::*;
    // AsRef/AsMut Slice<T>
    impl<T> AsRef<Self> for Slice<T> {
        fn as_ref(&self) -> &Self {
            self
        }
    }
    impl<T> AsMut<Self> for Slice<T> {
        fn as_mut(&mut self) -> &mut Self {
            self
        }
    }

    // PartialEq<U> Slice<T>
    impl<T, U> PartialEq<Slice<U>> for Slice<T>
    where
        T: PartialEq<U>,
    {
        fn eq(&self, other: &Slice<U>) -> bool {
            self.as_slice().eq(other.as_slice())
        }
    }
}

mod iter {
    use super::*;

    impl<'a, T> IntoIterator for &'a Slice<T> {
        type Item = &'a T;

        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.as_slice().iter()
        }
    }
    impl<'a, T> IntoIterator for &'a mut Slice<T> {
        type Item = &'a mut T;

        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.as_mut_slice().iter_mut()
        }
    }
}
