//! The definitive non-empty slice/array/vec library for Rust.
//!
//! # Features
//! Nonempty-by-construction API
//!   ```
//!   # use nunny::NonEmpty;
//!   let mut my_vec = NonEmpty::<Vec<_>>::of("hello"); // construct once
//!   my_vec.push("world");                             // continue using your normal APIs
//!   let hello: &str = my_vec.first();                 // preserve the guarantee that there is at least one element
//!   ```
//!
//! `#[repr(transparent)]` allows advanced usecases and guarantees optimum performance[^1]:
//!   ```
//!   # use nunny::NonEmpty;
//!   let src = &mut ["hello", "world"];
//!   let ne = NonEmpty::<[_]>::new_mut(src).unwrap();
//!   //  ^ uses the same backing memory
//!   let world: &str = ne.last();
//!   ```
//!
//! Total API coverage.
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
//!
//! `const`-friendly API. Where possible, all methods are `const`.
//!   ```
//!   # use nunny::{NonEmpty, slice};
//!   const TWO: &NonEmpty<[&str]> = slice!["together", "forever"];
//!   const FIRST: &str = TWO.first();
//!   const ONE: &NonEmpty<[&str]> = NonEmpty::<[_]>::of(&"lonely");
//!   ```
//!
//! Extensive feature gating supporting:
//! - `no-std` environments with no allocator.
//! - `alloc`-enabled environments.
//! - full-`std`-enabled environments.
//! - interaction with crates like [`serde`](::serde1) and [`arbitrary`](::arbitrary1).
//!
//! Iterator support:
//!   Specialized [`Iterator`] methods remove branches to handle empty iterators,
//!   _and_ preserve invariants even when chaining combinators.
//!   ```
//!   # use nunny::{vec};
//!   let v = vec![1, 2, 3];
//!   let _: Option<&u8> = v.iter().last();
//!       // ^ normally you have to handle the empty case
//!   let _: &u8 = v.iter_ne().last();
//!       // ^ but we know there is at least one element
//!   let _: u8 = v.iter_ne().copied().last();
//!                        // ^ using this combinator preserves the invariant
//!   ```
//!
//! Thoughtful design:
//! - [`NonZeroUsize`] is inserted [where](Slice::len_ne) [appropriate](Vec::truncate).
//! - Everything [`Deref`](core::ops::Deref)/[`DerefMut`](core::ops::DerefMut)s
//!   down to a [`NonEmpty<Slice<T>>`], which in turn `deref/mut`s down to a `[T]`.
//! - Liberal applications of [`cmp`](core::cmp), [`borrow`](core::borrow), [`convert`](core::convert)
//!   traits.
//!   If there's a missing API that you'd like, please raise an issue!
//!
//! [^1]: Other crates like [`nonempty`](https://docs.rs/nonempty/latest/nonempty/struct.NonEmpty.html)
//!       require an indirection.
//! [^2]: Barring impls on `!#[fundamental]` types like [`Arc`](std::sync::Arc).
//!       Fun fact: our tests were generated from [`std`]'s rustdoc!

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "arbitrary1")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
mod arbitrary1;
#[cfg(feature = "proptest1")]
#[cfg_attr(docsrs, doc(cfg(feature = "proptest")))]
mod proptest1;
#[cfg(feature = "quickcheck1")]
#[cfg_attr(docsrs, doc(cfg(feature = "quickcheck")))]
mod quickcheck1;
#[cfg(feature = "schemars08")]
#[cfg_attr(docsrs, doc(cfg(feature = "schemars")))]
mod schemars08;
#[cfg(feature = "schemars09")]
#[cfg_attr(docsrs, doc(cfg(feature = "schemars")))]
mod schemars09;
#[cfg(feature = "schemars1")]
#[cfg_attr(docsrs, doc(cfg(feature = "schemars")))]
mod schemars1;
#[cfg(feature = "serde1")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde1;

mod array;
mod iter;
mod mirror_std {
    mod cmp;
    mod from;
    mod partial_eq;
    mod try_from;
}
mod slice;
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod vec;

use core::{convert::Infallible, fmt, num::NonZeroUsize};

/// A wrapper struct around non-empty slices/arrays/vectors.
///
/// You may wish to use the following type aliases instead:
/// - [`Slice`].
/// - [`Vec`].
/// - [`Array`].
///
/// See also [crate documentation](crate)
#[derive(Debug, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct NonEmpty<T: ?Sized> {
    inner: T,
}

/// A non-empty [prim@array] of known size.
pub type Array<T, const N: usize> = NonEmpty<[T; N]>;
/// A non-empty, dynamically sized [prim@slice].
pub type Slice<T> = NonEmpty<[T]>;
/// A non-empty, heap allocated [Vec](alloc::vec::Vec).
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub type Vec<T> = NonEmpty<alloc::vec::Vec<T>>;

/// Create a non-empty slice
/// ```
/// # use nunny::{NonEmpty, slice};
/// const SLICE: &NonEmpty<[&str]> = slice!["hello", "world"];
/// ```
///
/// Note that no `slice_mut` is provided[^1] - users are expected to use [`array!`]
/// and [`Array::as_mut_slice`] as required.
///
/// [^1]: See [this blog post](https://blog.m-ou.se/super-let/) for more on why
/// `slice_mut` is impossible to implement in Rust today.
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

/// Create a non-empty array
/// ```
/// # use nunny::array;
/// let mut arr = array!["hello", "world"];
/// *arr.first_mut() = "goodbye";
/// assert_eq!(arr, ["goodbye", "world"])
/// ```
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

/// Create a non-empty heap-allocated vector
/// ```
/// # use nunny::{NonEmpty, vec};
/// let mut v = vec!["hello", "world"];
/// *v.first_mut() = "goodbye";
/// assert_eq!(v, ["goodbye", "world"])
/// ```
///
/// For `vec![T; N]`, `N` must be evaluatable at `const` time, and `T` must be [`Clone`].
/// ```compile_fail
/// # use nunny::vec;
/// let len = 1 + 1; // runtime variable
/// let v = vec!["hello"; len];
/// ```
/// ```
/// # use nunny::vec;
/// let v = vec!["hello"; 1 + 1]; // compile time nonzero
/// ```
/// ```compile_fail
/// # use nunny::vec;
/// let v = vec!["hello"; 0]; // not allowed to be zero!
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[macro_export]
macro_rules! vec {
    ($($el:expr),+ $(,)?) => {
        // Safety:
        // - `+` guarantees that at least one item is given
        $crate::Vec::from(unsafe {
            $crate::Array::new_unchecked([$($el),*])
        })
    };
    ($el:expr; $n:expr) => {
        $crate::Vec::filled($el, $crate::nonzero!($n))
    }
}

/// Checks that an expression is non-zero _at compile time_.
///
/// Provided as a convenience for writing tests.
///
/// ```
/// # use core::num::NonZeroUsize;
/// # use nunny::nonzero;
/// const NONZERO: NonZeroUsize = nonzero!(1);
/// ```
/// ```compile_fail
/// # use core::num::NonZeroUsize;
/// # use nunny::nonzero;
/// const OOPS: NonZeroUsize = nonzero!(0);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! nonzero {
    ($expr:expr) => {{
        const NONZERO: $crate::__private::core::num::NonZeroUsize =
            match $crate::__private::core::num::NonZeroUsize::new($expr) {
                $crate::__private::core::option::Option::Some(it) => it,
                _ => $crate::__private::core::panic!("expression evaluated to zero"),
            };
        NONZERO
    }};
}

/// Implementation detail, semver-exempt.
#[doc(hidden)]
pub mod __private {
    pub extern crate core;
}

/// Error returned in [`TryFrom`] implementations for reference conversions.
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
        ///
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
        ///
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
        ///
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
        ///
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

/// Error returned in [`TryFrom`] implementations for reference conversions.
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
