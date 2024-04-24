//! For every method or trait impl for a collection in [`core`] or [`std`], there should
//! be a corresponding method or impl in this library.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod array;
mod mirror_std {
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

use core::{fmt, hint::unreachable_unchecked, num::NonZeroUsize};

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

const unsafe fn non_zero_usize(n: usize) -> NonZeroUsize {
    match NonZeroUsize::new(n) {
        Some(it) => it,
        None => match cfg!(debug_assertions) {
            true => unreachable!(),
            false => unsafe { unreachable_unchecked() },
        },
    }
}
