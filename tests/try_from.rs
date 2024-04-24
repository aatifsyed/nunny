//! See the documentation in `mirror_std/try_from.rs` for an explanation.

#![allow(path_statements, clippy::no_effect)]

// use std::{rc::Rc, sync::Arc};

use nunny::{Array, Slice, Vec};

const _: () = {
    fn _test<'a, T, const N: usize>()
    where
        T: 'a,
    {
        <&'a Array<N, T> as TryFrom<&'a Slice<T>>>::try_from;
    }
};

const _: () = {
    fn _test<'a, T, const N: usize>()
    where
        T: 'a,
    {
        <&'a mut Array<N, T> as TryFrom<&'a mut Slice<T>>>::try_from;
    }
};

// const _: () = {
//     fn _test<T, const N: usize>() {
//         <Arc<Array<N, T>> as TryFrom<Arc<Slice<T>>>>::try_from;
//     }
// };

const _: () = {
    fn _test<T, const N: usize>() {
        <Array<N, T> as TryFrom<Vec<T>>>::try_from;
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

// const _: () = {
//     fn _test<T, const N: usize>()
//     where
//         LaneCount<N>: SupportedLaneCount,
//         T: SimdElement,
//     {
//         <Simd<T, N> as TryFrom<&Slice<T>>>::try_from;
//     }
// };

const _: () = {
    fn _test<T, const N: usize>()
    where
        T: Copy,
    {
        <Array<N, T> as TryFrom<&mut Slice<T>>>::try_from;
    }
};

// const _: () = {
//     fn _test<T, const N: usize>()
//     where
//         LaneCount<N>: SupportedLaneCount,
//         T: SimdElement,
//     {
//         <Simd<T, N> as TryFrom<&mut Slice<T>>>::try_from;
//     }
// };

const _: () = {
    fn _test<T, const N: usize>() {
        <Box<Array<N, T>> as TryFrom<Box<Slice<T>>>>::try_from;
    }
};

// const _: () = {
//     fn _test<T, const N: usize>() {
//         <Rc<Array<N, T>> as TryFrom<Rc<Slice<T>>>>::try_from;
//     }
// };

const _: () = {
    fn _test<T, const N: usize>() {
        <Box<Array<N, T>> as TryFrom<Vec<T>>>::try_from;
    }
};
