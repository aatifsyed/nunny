const _: () = {
    fn _test<'a, T, const N: usize>() {
        <&'a Array<N, T> as TryFrom<&'a Slice<T>>>::try_from;
    }
};

const _: () = {
    fn _test<'a, T, const N: usize>() {
        <&'a mut Array<N, T> as TryFrom<&'a mut Slice<T>>>::try_from;
    }
};

const _: () = {
    fn _test<T, A, const N: usize>()
    where
        A: Allocator,
    {
        <Arc<Array<N, T>, A> as TryFrom<Arc<Slice<T>, A>>>::try_from;
    }
};

const _: () = {
    fn _test<T, A, const N: usize>()
    where
        A: Allocator,
    {
        <Array<N, T> as TryFrom<Vec<T, A>>>::try_from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Copy,
    {
        <Array<N, T> as TryFrom<&Slice<T>>>::try_from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        LaneCount<N>: SupportedLaneCount,
        T: SimdElement,
    {
        <Simd<T, N> as TryFrom<&Slice<T>>>::try_from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Copy,
    {
        <Array<N, T> as TryFrom<&mut Slice<T>>>::try_from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        LaneCount<N>: SupportedLaneCount,
        T: SimdElement,
    {
        <Simd<T, N> as TryFrom<&mut Slice<T>>>::try_from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <Box<Array<N, T>> as TryFrom<Box<Slice<T>>>>::try_from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <Rc<Array<N, T>> as TryFrom<Rc<Slice<T>>>>::try_from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <Box<Array<N, T>> as TryFrom<Vec<T>>>::try_from;
    }
};

