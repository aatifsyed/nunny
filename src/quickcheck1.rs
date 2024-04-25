use crate::{Slice, Vec};
use quickcheck1::Arbitrary;

impl<T> Arbitrary for Vec<T>
where
    T: Arbitrary,
{
    fn arbitrary(g: &mut quickcheck1::Gen) -> Self {
        let mut it = Vec::one(T::arbitrary(g));
        it.extend(alloc::vec::Vec::arbitrary(g));
        it
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let raw = self.clone().into_vec();
        Box::new(Arbitrary::shrink(&raw).flat_map(Vec::new))
    }
}

impl<T> Arbitrary for Box<Slice<T>>
where
    T: Arbitrary,
{
    fn arbitrary(g: &mut quickcheck1::Gen) -> Self {
        let mut it = Vec::one(T::arbitrary(g));
        it.extend(alloc::vec::Vec::arbitrary(g));
        it.into_boxed_slice()
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let raw = Vec::from(self.clone());
        Box::new(Arbitrary::shrink(&raw).map(Self::from))
    }
}
