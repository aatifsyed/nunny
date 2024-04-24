//! There should be an `impl` here that corresponds to every [`PartialEq`]
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
//! - Redact unstable items, and implementations we can't do because we're not std.
//! - Switch the types from the standard library to our libraries.
//! - Write implementations _in the same order_ in this file.

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{Array, Slice};
#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, collections::VecDeque};

impl<A, B> PartialEq<Slice<B>> for Slice<A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Slice<B>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

impl<A, B, const N: usize> PartialEq<&Slice<B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &&Slice<B>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

impl<A, B, const N: usize> PartialEq<&mut Slice<B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &&mut Slice<B>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, A>> for &Slice<B>
where
    B: PartialEq<A>,
{
    fn eq(&self, other: &Array<N, A>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, A>> for &mut Slice<B>
where
    B: PartialEq<A>,
{
    fn eq(&self, other: &Array<N, A>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, A>> for Slice<B>
where
    B: PartialEq<A>,
{
    fn eq(&self, other: &Array<N, A>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Array<N, B>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

impl<A, B, const N: usize> PartialEq<Slice<B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Slice<B>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<&Slice<U>> for Cow<'_, Slice<T>>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &&Slice<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<&mut Slice<U>> for Cow<'_, Slice<T>>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &&mut Slice<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<&Slice<U>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Slice<U>) -> bool {
        if self.len() != other.len().get() {
            return false;
        }
        let (sa, sb) = self.as_slices();
        let (oa, ob) = other[..].split_at(sa.len());
        sa == oa && sb == ob
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<&Slice<U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Slice<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<&mut Slice<U>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut Slice<U>) -> bool {
        PartialEq::<&Slice<U>>::eq(self, &&**other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<&mut Slice<U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut Slice<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<Slice<U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Slice<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<Vec<U>> for &Slice<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<Vec<U>> for &mut Slice<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<Vec<U>> for Cow<'_, Slice<T>>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U> PartialEq<Vec<U>> for Slice<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U, const N: usize> PartialEq<&Array<N, U>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Array<N, U>) -> bool {
        PartialEq::<&Slice<U>>::eq(self, &&***other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U, const N: usize> PartialEq<&Array<N, U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Array<N, U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U, const N: usize> PartialEq<&mut Array<N, U>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut Array<N, U>) -> bool {
        PartialEq::<&Slice<U>>::eq(self, &&***other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U, const N: usize> PartialEq<Array<N, U>> for VecDeque<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Array<N, U>) -> bool {
        PartialEq::<&Slice<U>>::eq(self, &&**other)
    }
}

#[cfg(feature = "alloc")]
impl<T, U, const N: usize> PartialEq<Array<N, U>> for Vec<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Array<N, U>) -> bool {
        <[_] as PartialEq<[_]>>::eq(self, other)
    }
}
