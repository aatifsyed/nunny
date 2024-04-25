//! For every method or trait impl for a collection in [`core`] or [`std`], there should
//! be a corresponding method or impl in this library.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

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

pub use array::Array;
pub use slice::Slice;
#[cfg(feature = "alloc")]
pub use vec::Vec;

use core::{convert::Infallible, fmt, num::NonZeroUsize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error<T = ()>(T);

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

const unsafe fn non_zero_usize(n: usize) -> NonZeroUsize {
    match NonZeroUsize::new(n) {
        Some(it) => it,
        None => unreachable(),
    }
}

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
