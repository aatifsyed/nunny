#![cfg(feature = "alloc")]
use core::fmt::Debug;

use crate::{Slice, Vec};

use proptest1::{
    arbitrary::Arbitrary,
    sample::SizeRange,
    strategy::{FilterMap, Map, Strategy as _},
};

impl<T> Arbitrary for Vec<T>
where
    T: Arbitrary + Debug,
{
    type Parameters = (SizeRange, <T as Arbitrary>::Parameters);

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        alloc::vec::Vec::<T>::arbitrary_with(args)
            .prop_filter_map("vec was empty", |it| Vec::new(it).ok())
    }

    type Strategy = FilterMap<
        <alloc::vec::Vec<T> as Arbitrary>::Strategy,
        fn(alloc::vec::Vec<T>) -> Option<Vec<T>>,
    >;
}

impl<T> Arbitrary for Box<Slice<T>>
where
    T: Arbitrary + Debug,
{
    type Parameters = (SizeRange, <T as Arbitrary>::Parameters);

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        Vec::arbitrary_with(args).prop_map(Into::into)
    }

    type Strategy = Map<<Vec<T> as Arbitrary>::Strategy, fn(Vec<T>) -> Box<Slice<T>>>;
}
