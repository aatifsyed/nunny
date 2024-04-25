use arbitrary1::Arbitrary;

use crate::{Slice, Vec};

impl<'a> Arbitrary<'a> for &'a Slice<u8> {
    fn arbitrary(u: &mut arbitrary1::Unstructured<'a>) -> arbitrary1::Result<Self> {
        let len = u.arbitrary_len::<u8>()?.saturating_add(1);
        Ok(Slice::new(u.bytes(len)?).expect("`len` was non-zero"))
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let _ = depth;
        (1, None)
    }
}

impl<'a, T> Arbitrary<'a> for Vec<T>
where
    T: Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary1::Unstructured<'a>) -> arbitrary1::Result<Self> {
        let mut it = Vec::one(u.arbitrary::<T>()?);
        for item in u.arbitrary_iter()? {
            it.push(item?)
        }
        Ok(it)
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let _ = depth;
        (1, None)
    }
}

impl<'a, T> Arbitrary<'a> for Box<Slice<T>>
where
    T: Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary1::Unstructured<'a>) -> arbitrary1::Result<Self> {
        Ok(Vec::arbitrary(u)?.into_boxed_slice())
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let _ = depth;
        (1, None)
    }
}
