use std::hint::black_box;

use divan::Bencher;

const LENS: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

#[divan::bench(types = [Ours<String>, Theirs<String>, Ours<u8>, Theirs<u8>], args = LENS)]
fn clone<B: BenchMe>(bencher: Bencher, len: usize)
where
    B::Subject: Clone,
{
    let bench_me = black_box(B::setup(len));
    bencher.bench_local(move || B::run_clone(&bench_me));
}

#[divan::bench(types = [Ours<String>, Theirs<String>, Ours<u8>, Theirs<u8>], args = LENS)]
fn into_iter<B: BenchMe>(bencher: Bencher, len: usize)
where
    B::Subject: Clone + IntoIterator,
{
    let bench_me = black_box(B::setup(len));
    bencher.bench_local(move || B::run_into_iter(&bench_me));
}
#[divan::bench(types = [Ours<String>, Theirs<String>, Ours<u8>, Theirs<u8>], args = LENS)]
fn iter<B: BenchMe>(bencher: Bencher, len: usize)
where
    for<'a> &'a B::Subject: IntoIterator,
{
    let bench_me = black_box(B::setup(len));
    bencher.bench_local(move || B::run_iter(&bench_me));
}
#[divan::bench(types = [Ours<String>, Theirs<String>, Ours<u8>, Theirs<u8>], args = LENS)]
fn partial_eq<B: BenchMe>(bencher: Bencher, len: usize)
where
    B::Subject: PartialEq,
{
    let left = black_box(B::setup(len));
    let right = black_box(B::setup(len));
    bencher.bench_local(move || B::run_partial_eq(&left, &right));
}

struct Ours<T>(std::marker::PhantomData<fn() -> T>);
struct Theirs<T>(std::marker::PhantomData<fn() -> T>);
trait BenchMe {
    type Subject;
    fn setup(len: usize) -> Self::Subject;
    fn run_clone(subject: &Self::Subject)
    where
        Self::Subject: Clone,
    {
        black_box(subject.clone());
    }
    fn run_into_iter(subject: &Self::Subject)
    where
        Self::Subject: IntoIterator + Clone,
    {
        for it in black_box(subject.clone()) {
            black_box(it);
        }
    }
    fn run_iter(subject: &Self::Subject)
    where
        for<'a> &'a Self::Subject: IntoIterator,
    {
        for it in black_box(subject) {
            black_box(it);
        }
    }
    fn run_partial_eq(left: &Self::Subject, right: &Self::Subject)
    where
        Self::Subject: PartialEq,
    {
        black_box(black_box(left) == black_box(right));
    }
}

impl<T> BenchMe for Ours<T>
where
    T: Default,
{
    type Subject = nunny::Vec<T>;

    fn setup(len: usize) -> Self::Subject {
        let mut src = Vec::new();
        src.resize_with(len, Default::default);
        black_box(match nunny::Vec::new(src) {
            Ok(it) => it,
            Err(_) => panic!(),
        })
    }
}
impl<T> BenchMe for Theirs<T>
where
    T: Default,
{
    type Subject = nonempty::NonEmpty<T>;

    fn setup(len: usize) -> Self::Subject {
        let mut src = Vec::new();
        src.resize_with(len, Default::default);
        black_box(nonempty::NonEmpty::from_vec(src).unwrap())
    }
}

fn main() {
    divan::main()
}
