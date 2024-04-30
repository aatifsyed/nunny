use std::hint::black_box;

use divan::Bencher;

type Ours = nunny::Vec<u8>;
type Theirs = nonempty::NonEmpty<u8>;

const LENS: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

#[divan::bench(types = [Ours, Theirs], args = LENS)]
fn clone<T: BenchMe>(bencher: Bencher, len: usize) {
    let bench_me = black_box(T::new(vec![0; len]));
    bencher.bench_local(move || black_box(bench_me.clone()));
}

#[divan::bench(types = [Ours, Theirs], args = LENS)]
fn into_iter<T: BenchMe>(bencher: Bencher, len: usize) {
    let bench_me = black_box(T::new(vec![0; len]));
    bencher.bench_local(move || {
        for item in bench_me.clone() {
            black_box(item);
        }
    });
}

#[divan::bench(types = [Ours, Theirs], args = LENS)]
fn iter<T: BenchMe>(bencher: Bencher, len: usize) {
    let bench_me = black_box(T::new(vec![0; len]));
    bencher.bench_local(|| {
        for item in bench_me.iter() {
            black_box(item);
        }
    })
}

#[divan::bench(types = [Ours, Theirs], args = LENS)]
fn partial_eq<T: BenchMe>(bencher: Bencher, len: usize) {
    let left = black_box(T::new(vec![0; len]));
    let right = black_box(T::new(vec![0; len]));
    bencher.bench_local(|| black_box(left == right))
}

trait BenchMe: IntoIterator<Item = u8> + Clone + PartialEq {
    fn new(raw: Vec<u8>) -> Self;
    fn iter(&self) -> impl Iterator<Item = &u8>;
}

impl BenchMe for Ours {
    fn new(raw: Vec<u8>) -> Self {
        Self::new(raw).unwrap()
    }

    fn iter(&self) -> impl Iterator<Item = &u8> {
        self.as_slice().iter()
    }
}

impl BenchMe for Theirs {
    fn new(mut raw: Vec<u8>) -> Self {
        Self {
            head: raw.remove(0),
            tail: raw,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &u8> {
        Theirs::iter(self)
    }
}

fn main() {
    divan::main()
}
