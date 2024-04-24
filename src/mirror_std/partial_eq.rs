impl<A, B> PartialEq<Slice<B>> for Slice<A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Slice<B>) -> bool {
        todo!()
    }
}

impl<A, B, const N: usize> PartialEq<&Slice<B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &&Slice<B>) -> bool {
        todo!()
    }
}

impl<A, B, const N: usize> PartialEq<&mut Slice<B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &&mut Slice<B>) -> bool {
        todo!()
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, A>> for &Slice<B>
where
    B: PartialEq<A>,
{
    fn eq(&self, other: &Array<N, A>) -> bool {
        todo!()
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, A>> for &mut Slice<B>
where
    B: PartialEq<A>,
{
    fn eq(&self, other: &Array<N, A>) -> bool {
        todo!()
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, A>> for Slice<B>
where
    B: PartialEq<A>,
{
    fn eq(&self, other: &Array<N, A>) -> bool {
        todo!()
    }
}

impl<A, B, const N: usize> PartialEq<Array<N, B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Array<N, B>) -> bool {
        todo!()
    }
}

impl<A, B, const N: usize> PartialEq<Slice<B>> for Array<N, A>
where
    A: PartialEq<B>,
{
    fn eq(&self, other: &Slice<B>) -> bool {
        todo!()
    }
}

impl<T, U> PartialEq<&Slice<U>> for Cow<'_, Slice<T>>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &&Slice<U>) -> bool {
        todo!()
    }
}

impl<T, U> PartialEq<&mut Slice<U>> for Cow<'_, Slice<T>>
where
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &&mut Slice<U>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<&Slice<U>> for VecDeque<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Slice<U>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<&Slice<U>> for Vec<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Slice<U>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<&mut Slice<U>> for VecDeque<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut Slice<U>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<&mut Slice<U>> for Vec<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut Slice<U>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<Slice<U>> for Vec<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &Slice<U>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<Vec<U, A>> for &Slice<T>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U, A>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<Vec<U, A>> for &mut Slice<T>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U, A>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<Vec<U, A>> for Cow<'_, Slice<T>>
where
    A: Allocator,
    T: PartialEq<U> + Clone,
{
    fn eq(&self, other: &Vec<U, A>) -> bool {
        todo!()
    }
}

impl<T, U, A> PartialEq<Vec<U, A>> for Slice<T>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U, A>) -> bool {
        todo!()
    }
}

impl<T, U, A, const N: usize> PartialEq<&Array<N, U>> for VecDeque<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Array<N, U>) -> bool {
        todo!()
    }
}

impl<T, U, A, const N: usize> PartialEq<&Array<N, U>> for Vec<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &&Array<N, U>) -> bool {
        todo!()
    }
}

impl<T, U, A, const N: usize> PartialEq<&mut Array<N, U>> for VecDeque<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut Array<N, U>) -> bool {
        todo!()
    }
}

impl<T, U, A, const N: usize> PartialEq<Array<N, U>> for VecDeque<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &Array<N, U>) -> bool {
        todo!()
    }
}

impl<T, U, A, const N: usize> PartialEq<Array<N, U>> for Vec<T, A>
where
    A: Allocator,
    T: PartialEq<U>,
{
    fn eq(&self, other: &Array<N, U>) -> bool {
        todo!()
    }
}
