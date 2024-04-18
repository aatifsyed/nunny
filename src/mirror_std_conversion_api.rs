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
