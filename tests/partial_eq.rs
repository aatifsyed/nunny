const _: () = {
    fn _test<A, B>()
    where
        A: PartialEq<B>,
    {
        <Slice<A> as PartialEq<Slice<B>>>::eq;
    }
};

const _: () = {
    fn _test<A, B, const N: usize>()
    where
        A: PartialEq<B>,
    {
        <Array<N, A> as PartialEq<&Slice<B>>>::eq;
    }
};

const _: () = {
    fn _test<A, B, const N: usize>()
    where
        A: PartialEq<B>,
    {
        <Array<N, A> as PartialEq<&mut Slice<B>>>::eq;
    }
};

const _: () = {
    fn _test<A, B, const N: usize>()
    where
        B: PartialEq<A>,
    {
        <&Slice<B> as PartialEq<Array<N, A>>>::eq;
    }
};

const _: () = {
    fn _test<A, B, const N: usize>()
    where
        B: PartialEq<A>,
    {
        <&mut Slice<B> as PartialEq<Array<N, A>>>::eq;
    }
};

const _: () = {
    fn _test<A, B, const N: usize>()
    where
        B: PartialEq<A>,
    {
        <Slice<B> as PartialEq<Array<N, A>>>::eq;
    }
};

const _: () = {
    fn _test<A, B, const N: usize>()
    where
        A: PartialEq<B>,
    {
        <Array<N, A> as PartialEq<Array<N, B>>>::eq;
    }
};

const _: () = {
    fn _test<A, B, const N: usize>()
    where
        A: PartialEq<B>,
    {
        <Array<N, A> as PartialEq<Slice<B>>>::eq;
    }
};

const _: () = {
    fn _test<T, U>()
    where
        T: PartialEq<U> + Clone,
    {
        <Cow<'_, Slice<T>> as PartialEq<&Slice<U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U>()
    where
        T: PartialEq<U> + Clone,
    {
        <Cow<'_, Slice<T>> as PartialEq<&mut Slice<U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <VecDeque<T, A> as PartialEq<&Slice<U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <Vec<T, A> as PartialEq<&Slice<U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <VecDeque<T, A> as PartialEq<&mut Slice<U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <Vec<T, A> as PartialEq<&mut Slice<U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <Vec<T, A> as PartialEq<Slice<U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <&Slice<T> as PartialEq<Vec<U, A>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <&mut Slice<T> as PartialEq<Vec<U, A>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U> + Clone,
    {
        <Cow<'_, Slice<T>> as PartialEq<Vec<U, A>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <Slice<T> as PartialEq<Vec<U, A>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A, const N: usize>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <VecDeque<T, A> as PartialEq<&Array<N, U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A, const N: usize>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <Vec<T, A> as PartialEq<&Array<N, U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A, const N: usize>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <VecDeque<T, A> as PartialEq<&mut Array<N, U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A, const N: usize>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <VecDeque<T, A> as PartialEq<Array<N, U>>>::eq;
    }
};

const _: () = {
    fn _test<T, U, A, const N: usize>()
    where
        A: Allocator,
        T: PartialEq<U>,
    {
        <Vec<T, A> as PartialEq<Array<N, U>>>::eq;
    }
};

