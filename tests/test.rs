const _: () = {
    fn _test() {
        <Rc<Slice<u8>> as From<Rc<str>>>::from;
    }
};

const _: () = {
    fn _test() {
        <Arc<Slice<u8>> as From<Arc<str>>>::from;
    }
};

const _: () = {
    fn _test() {
        <IpAddr as From<Array<4, u8>>>::from;
    }
};

const _: () = {
    fn _test() {
        <Ipv4Addr as From<Array<4, u8>>>::from;
    }
};

const _: () = {
    fn _test() {
        <IpAddr as From<Array<16, u8>>>::from;
    }
};

const _: () = {
    fn _test() {
        <Ipv6Addr as From<Array<16, u8>>>::from;
    }
};

const _: () = {
    fn _test() {
        <IpAddr as From<Array<8, u16>>>::from;
    }
};

const _: () = {
    fn _test() {
        <Ipv6Addr as From<Array<8, u16>>>::from;
    }
};

const _: () = {
    fn _test<'a, T>()
    where
        T: Clone,
    {
        <Cow<'a, Slice<T>> as From<&'a Slice<T>>>::from;
    }
};

const _: () = {
    fn _test<'a, T>()
    where
        T: Clone,
    {
        <Cow<'a, Slice<T>> as From<&'a Vec<T>>>::from;
    }
};

const _: () = {
    fn _test<'a, T>()
    where
        Slice<T>: ToOwned<Owned = Vec<T>>,
    {
        <Vec<T> as From<Cow<'a, Slice<T>>>>::from;
    }
};

const _: () = {
    fn _test<'a, T>()
    where
        T: Clone,
    {
        <Cow<'a, Slice<T>> as From<Vec<T>>>::from;
    }
};

const _: () = {
    fn _test<'a, T, const N: usize>()
    where
        T: Clone,
    {
        <Cow<'a, Slice<T>> as From<&'a Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<'data>() {
        <BorrowedBuf<'data> as From<&'data mut Slice<u8>>>::from;
    }
};

const _: () = {
    fn _test<'data>() {
        <BorrowedBuf<'data> as From<&'data mut Slice<MaybeUninit<u8>>>>::from;
    }
};

const _: () = {
    fn _test<A>()
    where
        A: Allocator,
    {
        <Box<Slice<u8>, A> as From<Box<str, A>>>::from;
    }
};

const _: () = {
    fn _test<K, V, const N: usize>()
    where
        K: Eq + Hash,
    {
        <HashMap<K, V, RandomState> as From<Array<N, (K, V)>>>::from;
    }
};

const _: () = {
    fn _test<K, V, const N: usize>()
    where
        K: Ord,
    {
        <BTreeMap<K, V> as From<Array<N, (K, V)>>>::from;
    }
};

const _: () = {
    fn _test<T>()
    where
        T: Clone,
    {
        <Box<Slice<T>> as From<&Slice<T>>>::from;
    }
};

const _: () = {
    fn _test<T>()
    where
        T: Clone,
    {
        <Rc<Slice<T>> as From<&Slice<T>>>::from;
    }
};

const _: () = {
    fn _test<T>()
    where
        T: Clone,
    {
        <Arc<Slice<T>> as From<&Slice<T>>>::from;
    }
};

const _: () = {
    fn _test<T>()
    where
        T: Clone,
    {
        <Vec<T> as From<&Slice<T>>>::from;
    }
};

const _: () = {
    fn _test<T>()
    where
        T: Clone,
    {
        <Vec<T> as From<&mut Slice<T>>>::from;
    }
};

const _: () = {
    fn _test<T>()
    where
        T: Clone,
    {
        <Box<Slice<T>> as From<Cow<'_, Slice<T>>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T,) as From<Array<1, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T) as From<Array<2, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T) as From<Array<3, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T) as From<Array<4, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T) as From<Array<5, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T, T) as From<Array<6, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T, T, T) as From<Array<7, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T, T, T, T) as From<Array<8, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T, T, T, T, T) as From<Array<9, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T, T, T, T, T, T) as From<Array<10, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T, T, T, T, T, T, T) as From<Array<11, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <(T, T, T, T, T, T, T, T, T, T, T, T) as From<Array<12, T>>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<2, T> as From<(T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<3, T> as From<(T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<4, T> as From<(T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<5, T> as From<(T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<6, T> as From<(T, T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<7, T> as From<(T, T, T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<8, T> as From<(T, T, T, T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<9, T> as From<(T, T, T, T, T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<10, T> as From<(T, T, T, T, T, T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<11, T> as From<(T, T, T, T, T, T, T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<12, T> as From<(T, T, T, T, T, T, T, T, T, T, T, T)>>::from;
    }
};

const _: () = {
    fn _test<T>() {
        <Array<1, T> as From<(T,)>>::from;
    }
};

const _: () = {
    fn _test<T, A>()
    where
        A: Allocator,
    {
        <Vec<T, A> as From<Box<Slice<T>, A>>>::from;
    }
};

const _: () = {
    fn _test<T, A>()
    where
        A: Allocator,
    {
        <Box<Slice<T>, A> as From<Vec<T, A>>>::from;
    }
};

const _: () = {
    fn _test<T, A>()
    where
        A: Allocator,
    {
        <Rc<Slice<T>, A> as From<Vec<T, A>>>::from;
    }
};

const _: () = {
    fn _test<T, A>()
    where
        A: Allocator + Clone,
    {
        <Arc<Slice<T>, A> as From<Vec<T, A>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Clone,
    {
        <Vec<T> as From<&Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Clone,
    {
        <Vec<T> as From<&mut Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <Box<Slice<T>> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Eq + Hash,
    {
        <HashSet<T, RandomState> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Ord,
    {
        <BTreeSet<T> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Ord,
    {
        <BinaryHeap<T> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <LinkedList<T> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <VecDeque<T> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <Rc<Slice<T>> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        LaneCount<N>: SupportedLaneCount,
        T: SimdElement,
    {
        <Simd<T, N> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <Arc<Slice<T>> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>() {
        <Vec<T> as From<Array<N, T>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: MaskElement,
        LaneCount<N>: SupportedLaneCount,
    {
        <Array<N, bool> as From<Mask<T, N>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        LaneCount<N>: SupportedLaneCount,
        T: SimdElement,
    {
        <Array<N, T> as From<Simd<T, N>>>::from;
    }
};

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: MaskElement,
        LaneCount<N>: SupportedLaneCount,
    {
        <Mask<T, N> as From<Array<N, bool>>>::from;
    }
};

