//! There aren't that many implementations in [`std`]...

use crate::{Array, Slice};
use core::cmp::Ordering;

#[cfg(feature = "alloc")]
use crate::Vec;

impl<T> PartialOrd for Slice<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <[_] as PartialOrd>::partial_cmp(self, other)
    }
}

impl<T> Ord for Slice<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        <[_] as Ord>::cmp(self, other)
    }
}

impl<const N: usize, T> PartialOrd for Array<T, N>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <[_] as PartialOrd>::partial_cmp(self, other)
    }
}
impl<const N: usize, T> Ord for Array<T, N>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        <[_] as Ord>::cmp(self, other)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(do_doc_cfg, doc(cfg(feature = "alloc")))]
impl<T> PartialOrd<Vec<T>> for Vec<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {
        <[_] as PartialOrd>::partial_cmp(self, other)
    }
}

#[cfg(feature = "alloc")]
impl<T> Ord for Vec<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        <[_] as Ord>::cmp(self, other)
    }
}
