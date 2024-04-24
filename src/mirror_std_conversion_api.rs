use core::{convert::Infallible, fmt};

use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    sync::Arc,
};

use crate::{Array, Slice, Vec};

impl<const N: usize, T> From<Array<N, T>> for Arc<Slice<T>> {
    fn from(value: Array<N, T>) -> Self {
        let src = Arc::<[T]>::from(value.into_array());
        unsafe { Arc::<Slice<T>>::from_raw(Arc::into_raw(src) as *const Slice<T>) }
    }
}

impl<const N: usize, T> From<Array<N, T>> for Box<Slice<T>> {
    fn from(value: Array<N, T>) -> Self {
        let src = Box::<[T]>::from(value.into_array());
        unsafe { Box::<Slice<T>>::from_raw(Box::into_raw(src) as *mut Slice<T>) }
    }
}

impl<const N: usize, T> From<Array<N, T>> for Rc<Slice<T>> {
    fn from(value: Array<N, T>) -> Self {
        let src = Rc::<[T]>::from(value.into_array());
        unsafe { Rc::<Slice<T>>::from_raw(Rc::into_raw(src) as *const Slice<T>) }
    }
}

impl<const N: usize, T> From<Array<N, T>> for Vec<T> {
    fn from(value: Array<N, T>) -> Self {
        let src = alloc::vec::Vec::from(value.into_array());
        unsafe { Vec::new_unchecked(src) }
    }
}

impl<'a, const N: usize, T> From<&'a Array<N, T>> for Cow<'a, Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Array<N, T>) -> Self {
        Cow::Borrowed(value.as_slice())
    }
}

impl<T> ToOwned for Slice<T>
where
    T: Clone,
{
    type Owned = Vec<T>;

    fn to_owned(&self) -> Self::Owned {
        self.into()
    }
}

impl<'a, T> From<&'a Slice<T>> for Cow<'a, Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Slice<T>) -> Self {
        Cow::Borrowed(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TryFromSliceError(());
impl fmt::Display for TryFromSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("could not convert slice to array")
    }
}
#[cfg(feature = "std")]
impl std::error::Error for TryFromSliceError {}

impl From<Infallible> for TryFromSliceError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl<'a, const N: usize, T> TryFrom<&'a Slice<T>> for &'a Array<N, T> {
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

impl<'a, const N: usize, T> TryFrom<&'a mut Slice<T>> for &'a mut Array<N, T> {
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

impl<'a, T> From<&'a Vec<T>> for Cow<'a, Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Vec<T>) -> Self {
        Cow::Borrowed(value)
    }
}

impl<'a, const N: usize, T> From<&'a Array<N, T>> for Vec<T>
where
    T: Clone,
{
    fn from(value: &'a Array<N, T>) -> Self {
        value.as_slice().into()
    }
}

impl<'a, const N: usize, T> TryFrom<&'a Slice<T>> for Array<N, T>
where
    T: Copy,
{
    type Error = TryFromSliceError;

    fn try_from(value: &'a Slice<T>) -> Result<Self, Self::Error> {
        value
            .as_slice()
            .try_into()
            .ok()
            .and_then(Array::new)
            .ok_or(TryFromSliceError(()))
    }
}

impl<'a, T> From<&'a Slice<T>> for Arc<Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Slice<T>) -> Self {
        let src = Arc::<[T]>::from(value.as_slice());
        unsafe { Arc::<Slice<T>>::from_raw(Arc::into_raw(src) as *const Slice<T>) }
    }
}
impl<'a, T> From<&'a Slice<T>> for Box<Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Slice<T>) -> Self {
        let src = Box::<[T]>::from(value.as_slice());
        unsafe { Box::<Slice<T>>::from_raw(Box::into_raw(src) as *mut Slice<T>) }
    }
}

impl<'a, T> From<&'a Slice<T>> for Rc<Slice<T>>
where
    T: Clone,
{
    fn from(value: &'a Slice<T>) -> Self {
        let src = Rc::<[T]>::from(value.as_slice());
        unsafe { Rc::<Slice<T>>::from_raw(Rc::into_raw(src) as *mut Slice<T>) }
    }
}
