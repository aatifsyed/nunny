<!-- cargo-rdme start -->

The definitive non-empty slice/array/vec library for Rust.

# Features
Nonempty-by-construction API
  ```rust
  let mut my_vec = NonEmpty::<Vec<_>>::of("hello"); // construct once
  my_vec.push("world");                             // continue using your normal APIs
  let hello: &str = my_vec.first();                 // preserve the guarantee that there is at least one element
  ```

`#[repr(transparent)]` allows advanced usecases and guarantees optimum performance[^1]:
  ```rust
  let src = &mut ["hello", "world"];
  let ne = NonEmpty::<[_]>::new_mut(src).unwrap();
  //  ^ uses the same backing memory
  let world: &str = ne.last();
  ```

Total API coverage.
  For every impl of [`From`], [`TryFrom`], [`PartialEq`] and [`PartialOrd`] in [`std`][^2],
  there is a corresponding impl in this library for [`Slice`], [`Array`] and [`Vec`].
  _This includes more exotic types_:
  ```rust
  let nun: Box<NonEmpty<[_]>> = vec![0xDEAD, 0xBEEF].into();
  let cow: Cow<NonEmpty<[_]>> = (&*nun).into();
  let arc: Arc<NonEmpty<[_]>> = cow.into_owned().into();
  ```

`const`-friendly API. Where possible, all methods are `const`.
  ```rust
  const TWO: &NonEmpty<[&str]> = slice!["together", "forever"];
  const FIRST: &str = TWO.first();
  const ONE: &NonEmpty<[&str]> = NonEmpty::<[_]>::of(&"lonely");
  ```

Extensive feature gating supporting:
- `no-std` environments with no allocator.
- `alloc`-enabled environments.
- full-`std`-enabled environments.
- interaction with crates like `serde` and `arbitrary`.

Thoughtful design:
- [`NonZeroUsize`] is inserted [where](Slice::len) [appropriate](Vec::truncate).
- Everything [`Deref`](core::ops::Deref)/[`DerefMut`](core::ops::DerefMut)s
  down to a [`NonEmpty<Slice<T>>`], which in turn `deref/mut`s down to a `[T]`.
- Liberal applications of [`cmp`](core::cmp), [`borrow`](core::borrow), [`convert`](core::convert)
  traits.
  If there's a missing API that you'd like, please raise an issue!

[^1]: Other crates like [`nonempty`](https://docs.rs/nonempty/latest/nonempty/struct.NonEmpty.html)
      require an indirection.
[^2]: Barring impls on `!#[fundamental]` types like [`Arc`](std::sync::Arc).
      Fun fact: our tests were generated from [`std`]'s rustdoc!

<!-- cargo-rdme end -->
