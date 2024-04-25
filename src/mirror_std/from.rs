//! There should be an `impl` here that corresponds to every [`From`]
//! implementation in [`std`] between:
//! - `[T; N]`
//! - `[T]`
//! - [`Vec`](alloc::vec::Vec)
//!
//! Here's the methodology:
//! - Use the `gen` tool in this repository to scrape rustdoc
//!   (HTML, not JSON - it's much easier)
//! - Output the `test/from.rs` file in this repository.
//!   This is effectively a description of the standard library conversions.
//! - Redact unstable items, and the [`std::net`] conversions.
//! - Switch the types from the standard library to our libraries.
//! - Write implementations _in the same order_ in this file.

#[cfg(feature = "std")]
use core::hash::Hash;
#[cfg(feature = "std")]
use std::{
    collections::{HashMap, HashSet},
    hash::RandomState,
};

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "alloc")]
use crate::{Array, Slice, Vec};

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a Slice<T>> for Cow<'a, Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Slice<T>) -> Self {
        Cow::Borrowed(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<&'a Vec<T>> for Cow<'a, Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Vec<T>) -> Self {
        Cow::Borrowed(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<Cow<'a, Slice<T>>> for Vec<T>
where
    Slice<T>: ToOwned<Owned = Vec<T>>,
{
    fn from(value: Cow<'a, Slice<T>>) -> Self {
        value.into_owned()
    }
}

#[cfg(feature = "alloc")]
impl<'a, T> From<Vec<T>> for Cow<'a, Slice<T>>
where
    T: Clone,
{
    fn from(value: Vec<T>) -> Self {
        Cow::Owned(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize, T> From<&'a Array<N, T>> for Cow<'a, Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Array<N, T>) -> Self {
        Cow::Borrowed(value.as_slice())
    }
}

#[cfg(feature = "std")]
impl<const N: usize, K, V> From<Array<N, (K, V)>> for HashMap<K, V, RandomState>
where
    K: Eq + Hash,
{
    fn from(value: Array<N, (K, V)>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, K, V> From<Array<N, (K, V)>> for BTreeMap<K, V>
where
    K: Ord,
{
    fn from(value: Array<N, (K, V)>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<T> From<&Slice<T>> for Box<Slice<T>>
where
    T: Clone,
{
    fn from(value: &Slice<T>) -> Self {
        let value = Box::<[T]>::from(value.as_slice());
        // Safety:
        // - transmuting is safe because #[repr(transparent)]
        // - already non-empty by construction
        unsafe { Box::<Slice<T>>::from_raw(Box::into_raw(value) as *mut Slice<T>) }
    }
}

#[cfg(feature = "alloc")]
impl<T> From<&Slice<T>> for Rc<Slice<T>>
where
    T: Clone,
{
    fn from(value: &Slice<T>) -> Self {
        let src = Rc::<[T]>::from(value.as_slice());
        // Safety:
        // - transmuting is safe because #[repr(transparent)]
        // - already non-empty by construction
        unsafe { Rc::<Slice<T>>::from_raw(Rc::into_raw(src) as *mut Slice<T>) }
    }
}

#[cfg(feature = "alloc")]
impl<T> From<&Slice<T>> for Arc<Slice<T>>
where
    T: Clone,
{
    fn from(value: &Slice<T>) -> Self {
        let value = Arc::<[T]>::from(value.as_slice());
        // Safety:
        // - transmuting is safe because #[repr(transparent)]
        // - already non-empty by construction
        unsafe { Arc::<Slice<T>>::from_raw(Arc::into_raw(value) as *const Slice<T>) }
    }
}

#[cfg(feature = "alloc")]
impl<T> From<&Slice<T>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &Slice<T>) -> Self {
        let value = alloc::vec::Vec::from(value.as_slice());
        // Safety:
        // - already non-empty by construction
        unsafe { Self::new_unchecked(value) }
    }
}

#[cfg(feature = "alloc")]
impl<T> From<&mut Slice<T>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &mut Slice<T>) -> Self {
        let value = alloc::vec::Vec::from(value.as_slice());
        // Safety:
        // - already non-empty by construction
        unsafe { Self::new_unchecked(value) }
    }
}

#[cfg(feature = "alloc")]
impl<T> From<Cow<'_, Slice<T>>> for Box<Slice<T>>
where
    T: Clone,
{
    fn from(value: Cow<'_, Slice<T>>) -> Self {
        value.into_owned().into_boxed_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T> From<Box<Slice<T>>> for Vec<T> {
    fn from(value: Box<Slice<T>>) -> Self {
        let value = Box::into_raw(value);
        // Safety:
        // - transmuting is safe because #[repr(transparent)]
        let value = unsafe { Box::from_raw(value as *mut [T]) };
        let value = alloc::vec::Vec::from(value);
        // Safety:
        // - already non-empty by construction
        unsafe { Self::new_unchecked(value) }
    }
}

#[cfg(feature = "alloc")]
impl<T> From<Vec<T>> for Box<Slice<T>> {
    fn from(value: Vec<T>) -> Self {
        value.into_boxed_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T> From<Vec<T>> for Rc<Slice<T>> {
    fn from(value: Vec<T>) -> Self {
        value.into_boxed_slice().into()
    }
}

#[cfg(feature = "alloc")]
impl<T> From<Vec<T>> for Arc<Slice<T>> {
    fn from(value: Vec<T>) -> Self {
        value.into_boxed_slice().into()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<&Array<N, T>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &Array<N, T>) -> Self {
        value.as_slice().into()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<&mut Array<N, T>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &mut Array<N, T>) -> Self {
        value.as_slice().into()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for Box<Slice<T>> {
    fn from(value: Array<N, T>) -> Self {
        let value = Box::<[T]>::from(value.into_array());
        // Safety:
        // - transmuting is safe because #[repr(transparent)]
        unsafe { Box::<Slice<T>>::from_raw(Box::into_raw(value) as *mut Slice<T>) }
    }
}

#[cfg(feature = "std")]
impl<const N: usize, T> From<Array<N, T>> for HashSet<T, RandomState>
where
    T: Eq + Hash,
{
    fn from(value: Array<N, T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for BTreeSet<T>
where
    T: Ord,
{
    fn from(value: Array<N, T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for BinaryHeap<T>
where
    T: Ord,
{
    fn from(value: Array<N, T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for LinkedList<T> {
    fn from(value: Array<N, T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for VecDeque<T> {
    fn from(value: Array<N, T>) -> Self {
        value.into_iter().collect()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for Rc<Slice<T>> {
    fn from(value: Array<N, T>) -> Self {
        let value = Rc::<[T]>::from(value.into_array());
        let value = Rc::into_raw(value);
        // Safety:
        // - transmuting is safe because #[repr(transparent)]
        unsafe { Rc::from_raw(value as *const Slice<T>) }
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for Arc<Slice<T>> {
    fn from(value: Array<N, T>) -> Self {
        let value = Arc::<[T]>::from(value.into_array());
        // Safety:
        // - transmuting is safe because #[repr(transparent)]
        unsafe { Arc::<Slice<T>>::from_raw(Arc::into_raw(value) as *const Slice<T>) }
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize, T> From<Array<N, T>> for Vec<T> {
    fn from(value: Array<N, T>) -> Self {
        let value = alloc::vec::Vec::from(value.into_array());
        // Safety:
        // - already non-empty by construction
        unsafe { Vec::new_unchecked(value) }
    }
}
