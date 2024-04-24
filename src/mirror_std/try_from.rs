//! There should be an `impl` here that corresponds to every [`TryFrom`]
//! implementation in [`std`] between:
//! - `[T; N]`
//! - `[T]`
//! - [`Vec`](alloc::vec::Vec)
//!
//! Here's the methodology:
//! - Use the `gen` tool in this repository to scrape rustdoc
//!   (HTML, not JSON - it's much easier)
//! - Output the `test/try_from.rs` file in this repository.
//!   This is effectively a description of the standard library conversions.
//! - Redact unstable items, and conversions we can't implement because we're
//!   not std.
//! - Switch the types from the standard library to our libraries.
//! - Write implementations _in the same order_ in this file.

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{Array, Slice};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
// use alloc::{rc::Rc, sync::Arc};

pub struct TryFromSliceError(());

impl<'a, T, const N: usize> TryFrom<&'a Slice<T>> for &'a Array<N, T> {
    type Error = TryFromSliceError;

    fn try_from(value: &'a Slice<T>) -> Result<Self, Self::Error> {
        value
            .as_slice()
            .try_into()
            .ok()
            .and_then(Array::new_ref)
            .ok_or(TryFromSliceError(()))
    }
}

impl<'a, T, const N: usize> TryFrom<&'a mut Slice<T>> for &'a mut Array<N, T> {
    type Error = TryFromSliceError;

    fn try_from(value: &'a mut Slice<T>) -> Result<Self, Self::Error> {
        value
            .as_mut_slice()
            .try_into()
            .ok()
            .and_then(Array::new_mut)
            .ok_or(TryFromSliceError(()))
    }
}

// impl<T, const N: usize> TryFrom<Arc<Slice<T>>> for Arc<Array<N, T>> {
//     type Error = ();

//     fn try_from(value: Arc<Slice<T>>) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

#[cfg(feature = "alloc")]
impl<T, const N: usize> TryFrom<Vec<T>> for Array<N, T> {
    type Error = Vec<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        match value.into_vec().try_into() {
            Ok(it) => Ok(unsafe { Array::new_unchecked(it) }),
            Err(it) => Err(unsafe { Vec::new_unchecked(it) }),
        }
    }
}

impl<T, const N: usize> TryFrom<&Slice<T>> for Array<N, T>
where
    T: Copy,
{
    type Error = TryFromSliceError;

    fn try_from(value: &Slice<T>) -> Result<Self, Self::Error> {
        match value.as_slice().try_into() {
            Ok(it) => Ok(unsafe { Array::new_unchecked(it) }),
            Err(_) => Err(TryFromSliceError(())),
        }
    }
}
impl<T, const N: usize> TryFrom<&mut Slice<T>> for Array<N, T>
where
    T: Copy,
{
    type Error = TryFromSliceError;

    fn try_from(value: &mut Slice<T>) -> Result<Self, Self::Error> {
        match value.as_mut_slice().try_into() {
            Ok(it) => Ok(unsafe { Array::new_unchecked(it) }),
            Err(_) => Err(TryFromSliceError(())),
        }
    }
}

#[cfg(feature = "alloc")]
impl<T, const N: usize> TryFrom<Box<Slice<T>>> for Box<Array<N, T>> {
    type Error = Box<Slice<T>>;

    fn try_from(value: Box<Slice<T>>) -> Result<Self, Self::Error> {
        match value.len().get() == N {
            true => Ok(unsafe { boxed_slice_as_array_unchecked(value) }),
            false => Err(value),
        }
    }
}

// impl<T, const N: usize> TryFrom<Rc<Slice<T>>> for Rc<Array<N, T>> {
//     type Error = ();

//     fn try_from(value: Rc<Slice<T>>) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

#[cfg(feature = "alloc")]
impl<T, const N: usize> TryFrom<Vec<T>> for Box<Array<N, T>> {
    type Error = Vec<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        match value.len().get() == N {
            true => Ok(unsafe { boxed_slice_as_array_unchecked(value.into_boxed_slice()) }),
            false => Err(value),
        }
    }
}

/// Casts a boxed slice to a boxed array.
///
/// # Safety
///
/// `boxed_slice.len()` must be exactly `N`.
#[cfg(feature = "alloc")]
unsafe fn boxed_slice_as_array_unchecked<T, const N: usize>(
    boxed_slice: Box<Slice<T>>,
) -> Box<Array<N, T>> {
    debug_assert_eq!(boxed_slice.len().get(), N);

    let ptr = Box::into_raw(boxed_slice);
    unsafe { Box::from_raw(ptr as *mut Array<N, T>) }
}
