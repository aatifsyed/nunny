//! Make sure that std conversions are covered in this library

extern crate alloc;

use alloc::{borrow::Cow, rc::Rc, sync::Arc};
use nunny::{Array, Slice, Vec};

const N: usize = 1;
type CopyT = u8;
type CloneT = String;
struct AnyT {}

macro_rules! assert {
    ($($tt:tt)*) => {
        const _: () = {
            const fn test() where
            for <'a> $($tt)* {}
            test();
        };
    };
}
assert!(Arc<[AnyT]>: From<[AnyT; N]>);
assert!(Arc<Slice<AnyT>>: From<Array<N, AnyT>>);

assert!(Box<[AnyT]>: From<[AnyT; N]>);
assert!(Box<Slice<AnyT>>: From<Array<N, AnyT>>);

assert!(Rc<[AnyT]>: From<[AnyT; N]>);
assert!(Rc<Slice<AnyT>>: From<Array<N, AnyT>>);

assert!(alloc::vec::Vec<AnyT>: From<[AnyT; N]>);
assert!(Vec<AnyT>: From<Array<N, AnyT>>);

assert!(alloc::collections::vec_deque::VecDeque<AnyT>: From<[AnyT; N]>);
// assert!(VecDeque<T>: From<[T;N]>);

assert!(Cow<'a, [CloneT]>: From<&'a [CloneT; N]>);
assert!(Cow<'a, Slice<CloneT>>: From<&'a Array<N, CloneT>>);

assert!(&'a [AnyT; N]: TryFrom<&'a [AnyT]>);
assert!(&'a Array<N, AnyT>: TryFrom<&'a Slice<AnyT>>);

assert!(Cow<'a, [CloneT]>: From<&'a [CloneT]>);
assert!(Cow<'a, Slice<CloneT>>: From<&'a Slice<CloneT>>);

assert!(&'a mut [AnyT; N]: TryFrom<&'a mut [AnyT]>);
assert!(&'a mut Array<N, AnyT>: TryFrom<&'a mut Slice<AnyT>>);

assert!(Cow<'a, [CloneT]>: From<&'a alloc::vec::Vec<CloneT>>);
assert!(Cow<'a, Slice<CloneT>>: From<&'a Vec<CloneT>>);

assert!(alloc::vec::Vec<CloneT>: From<&'a [CloneT; N]>);
assert!(Vec<CloneT>: From<&'a Array<N, CloneT>>);

assert!([CopyT; N]: TryFrom<&'a [CopyT]>);
assert!(Array<N, CopyT>: TryFrom<&'a Slice<CopyT>>);

assert!(Arc<[CloneT]>: From<&'a [CloneT]>);
assert!(Arc<Slice<CloneT>>: From<&'a Slice<CloneT>>);

assert!(Box<[CloneT]>: From<&'a [CloneT]>);
assert!(Box<Slice<CloneT>>: From<&'a Slice<CloneT>>);

assert!(Rc<[CloneT]>: From<&'a [CloneT]>);
assert!(Rc<Slice<CloneT>>: From<&'a Slice<CloneT>>);

assert!(alloc::vec::Vec<CloneT>: From<&'a [CloneT]>);
assert!(Vec<CloneT>: From<&'a Slice<CloneT>>);

// assert!(Cow<'a, [T]>: From<&'a [T]>);
// assert!(Cow<'a, Slice<T>>: From<&'a Slice<T>>);
