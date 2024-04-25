//! The definitive non-empty slice/array/vec library for Rust.
//!
//! # Features
//! - Nonempty-by-construction API
//!   ```
//!   # use nunny::NonEmpty;
//!   let mut my_vec = NonEmpty::<Vec<_>>::of("hello"); // construct once
//!   my_vec.push("world");                             // continue using your normal APIs
//!   let hello: &str = my_vec.first();                 // preserve the guarantee that there is at least one element
//!   ```
//! - `#[repr(transparent)]` allows advanced usecases and guarantees optimum performance[^1]:
//!   ```
//!   # use nunny::NonEmpty;
//!   let src = &mut ["hello", "world"];
//!   let ne = NonEmpty::<[_]>::new_mut(src).unwrap();
//!   //  ^ uses the same backing memory
//!   let world: &str = ne.last();
//!   ```
//! - Total API coverage.
//!   For every impl of [`From`], [`TryFrom`], [`PartialEq`] and [`PartialOrd`] in [`std`][^2],
//!   there is a corresponding impl in this library for [`Slice`], [`Array`] and [`Vec`].
//!   _This includes more exotic types_:
//!   ```
//!   # use nunny::{vec, NonEmpty};
//!   # use std::{borrow::Cow, sync::Arc};
//!   let nun: Box<NonEmpty<[_]>> = vec![0xDEAD, 0xBEEF].into();
//!   let cow: Cow<NonEmpty<[_]>> = (&*nun).into();
//!   let arc: Arc<NonEmpty<[_]>> = cow.into_owned().into();
//!   ```
//! - `const`-friendly API. Where possible, all methods are `const`.
//!   ```
//!   # use nunny::{NonEmpty, slice};
//!   const TWO: &NonEmpty<[&str]> = slice!["together", "forever"];
//!   const FIRST: &str = TWO.first();
//!   const ONE: &NonEmpty<[&str]> = NonEmpty::<[_]>::of(&"lonely");
//!   ```
//! - Extensive feature gating supporting:
//!   - `no-std` environments with no allocator.
//!   - `alloc`-enabled environments.
//!   - full-`std`-enabled environments.
//!   - interaction with crates like [`serde`](serde1) and [`arbitrary`](arbitrary1).
//! - Thoughtful design:
//!   - [`NonZeroUsize`] is inserted [where](Slice::len) [appropriate](Vec::truncate).
//!   - Everything [`Deref`](core::ops::Deref)/[`DerefMut`](core::ops::DerefMut)s
//!     down to a [`NonEmpty<Slice<T>>`], which in turn `deref/mut`s down to a `[T]`.
//!   - Liberal applications of [`cmp`](core::cmp), [`borrow`](core::borrow), [`convert`](core::convert)
//!     traits.
//!     If there's a missing API that you'd like, please raise an issue!
//!
//! [^1]: Other crates like [`nonempty`](https://docs.rs/nonempty/latest/nonempty/struct.NonEmpty.html)
//!       require an indirection.
//! [^2]: Barring impls on `!#[fundamental]` types like [`Arc`](std::sync::Arc).
//!       Fun fact: our tests were generated from [`std`]'s rustdoc!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "arbitrary1")]
mod arbitrary1;
#[cfg(feature = "proptest1")]
mod proptest1;
#[cfg(feature = "quickcheck1")]
mod quickcheck1;
#[cfg(feature = "serde1")]
mod serde1;

mod array;
mod mirror_std {
    mod cmp;
    mod from;
    mod partial_eq;
    mod try_from;
}
mod slice;
#[cfg(feature = "alloc")]
mod vec;

use core::{convert::Infallible, fmt, num::NonZeroUsize};

#[derive(Debug, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct NonEmpty<T: ?Sized> {
    inner: T,
}

/// Type alias to save keystrokes
pub type Array<T, const N: usize> = NonEmpty<[T; N]>;
/// Type alias to save keystrokes
pub type Slice<T> = NonEmpty<[T]>;
/// Type alias to save keystrokes
#[cfg(feature = "alloc")]
pub type Vec<T> = NonEmpty<alloc::vec::Vec<T>>;

#[macro_export]
macro_rules! slice {
    ($($el:expr),+ $(,)?) => {
        // Safety:
        // - `+` guarantees that at least one item is given
        unsafe {
            $crate::Slice::new_unchecked(&[$($el),*])
        }
    };
}

#[macro_export]
macro_rules! slice_mut {
    ($($el:expr),+ $(,)?) => {
        // Safety:
        // - `+` guarantees that at least one item is given
        unsafe {
            $crate::Slice::new_mut_unchecked(&mut [$($el),*])
        }
    };
}

#[macro_export]
macro_rules! array {
    ($($el:expr),+ $(,)?) => {
        // Safety:
        // - `+` guarantees that at least one item is given
        unsafe {
            $crate::Array::new_unchecked([$($el),*])
        }
    };
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! vec {
    ($($el:expr),+ $(,)?) => {
        // Safety:
        // - `+` guarantees that at least one item is given
        $crate::Vec::from(unsafe {
            $crate::Array::new_unchecked([$($el),*])
        })
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(());

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("collection was empty")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

macro_rules! transmuting {
    () => {}; // base case
    (const $(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty; $($rest:tt)*) => {
        $(#[$meta])*
        /// # Safety
        /// - `src` must not be empty
        pub const unsafe fn $ident(src: $in) -> $out {
            debug_assert!(!src.is_empty());
            // Safety
            // - #[repr(transparent)]
            unsafe { core::mem::transmute(src) }
        }
        $crate::transmuting!($($rest)*);
    };
    ($(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty; $($rest:tt)*) => {
        $(#[$meta])*
        /// # Safety
        /// - `src` must not be empty
        pub unsafe fn $ident(src: $in) -> $out {
            debug_assert!(!src.is_empty());
            // Safety
            // - #[repr(transparent)]
            unsafe { core::mem::transmute(src) }
        }
        $crate::transmuting!($($rest)*);
    };
}
pub(crate) use transmuting;

macro_rules! map_non_empty {
    () => {}; // base case
    (const $(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty: $mapper:path; $($rest:tt)*) => {
        $(#[$meta])*
        /// Returns [`None`] if `src` is empty.
        pub const fn $ident(src: $in) -> Option<$out> {
            match src.is_empty() {
                true => None,
                // Safety
                // - checked non empty
                false => Some(unsafe { $mapper(src) })
            }
        }
        $crate::map_non_empty!($($rest)*);
    };
    ($(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty: $mapper:path; $($rest:tt)*) => {
        $(#[$meta])*
        /// Returns [`None`] if `src` is empty.
        pub fn $ident(src: $in) -> Option<$out> {
            match src.is_empty() {
                true => None,
                // Safety
                // - checked non empty
                false => Some(unsafe { $mapper(src) })
            }
        }
        $crate::map_non_empty!($($rest)*);
    };
}
pub(crate) use map_non_empty;

macro_rules! as_ref_as_mut {
    ($($(<$ty_param:ident $(, const $const_param:ident: usize)?>)? for $self:ty as $ty:ty);* $(;)?) => {
        $(
            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::convert::AsRef<$ty> for $self {
                fn as_ref(&self) -> &$ty { self }
            }

            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::convert::AsMut<$ty> for $self {
                fn as_mut(&mut self) -> &mut $ty { self }
            }

        )*
    };
}
pub(crate) use as_ref_as_mut;

macro_rules! borrow_borrow_mut {
    ($($(<$ty_param:ident $(, const $const_param:ident: usize)?>)? for $self:ty as $ty:ty);* $(;)?) => {
        $(
            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::borrow::Borrow<$ty> for $self {
                fn borrow(&self) -> &$ty { self }
            }

            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::borrow::BorrowMut<$ty> for $self {
                fn borrow_mut(&mut self) -> &mut $ty { self }
            }

        )*
    };
}
pub(crate) use borrow_borrow_mut;

macro_rules! slice_iter {
    (<$ty_param:ident $(, const $const_param:ident: usize)?> for $self:ty) => {
        impl<'a, $ty_param $(, const $const_param: usize)?> ::core::iter::IntoIterator for &'a $self {
            type Item = &'a $ty_param;
            type IntoIter = ::core::slice::Iter<'a, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a, $ty_param $(, const $const_param: usize)?> ::core::iter::IntoIterator for &'a mut $self {
            type Item = &'a mut $ty_param;
            type IntoIter = ::core::slice::IterMut<'a, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }
    };
}
pub(crate) use slice_iter;

#[track_caller]
const unsafe fn non_zero_usize(n: usize) -> NonZeroUsize {
    match NonZeroUsize::new(n) {
        Some(it) => it,
        None => unreachable(),
    }
}

#[track_caller]
const unsafe fn unreachable() -> ! {
    match cfg!(debug_assertions) {
        true => unreachable(),
        false => unsafe { core::hint::unreachable_unchecked() },
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TryFromSliceError(());

impl From<Infallible> for TryFromSliceError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl fmt::Display for TryFromSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("could not convert slice to array")
    }
}
