//! Make sure that std conversions are covered in this library

extern crate alloc;

use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use nunny::{Array, Slice, Vec};

const N: usize = 1;
type CopyAble = u8;
type T = String;

macro_rules! assert {
    ($($tt:tt)*) => {
        const _: () = {
            const fn test() where
            for <'a> $($tt)* {}
            test();
        };
    };
}
assert!(Arc<[T]>: From<[T; N]>);
assert!(Arc<Slice<T>>: From<Array<N, T>>);

assert!(Box<[T]>: From<[T; N]>);
assert!(Box<Slice<T>>: From<Array<N, T>>);

assert!(Rc<[T]>: From<[T; N]>);
assert!(Rc<Slice<T>>: From<Array<N, T>>);

assert!(alloc::vec::Vec<T>: From<[T; N]>);
assert!(Vec<T>: From<Array<N, T>>);

assert!(alloc::collections::vec_deque::VecDeque<T>: From<[T;N]>);
// assert!(VecDeque<T>: From<[T;N]>);

assert!(Cow<'a, [T]>: From<&'a [T; N]>);
assert!(Cow<'a, Slice<T>>: From<&'a Array<N, T>>);

// assert!(Cow<'a, [T]>: From<&'a [T]>);
// assert!(Cow<'a, Slice<T>>: From<&'a Slice<T>>);
