//! For every method or trait impl for a collection in [`core`] or [`std`], there should
//! be a corresponding method or impl in this library.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "serde1")]
mod serde1;

mod array;
mod mirror_std {
    mod cmp;
    mod from;
    mod partial_eq;
    mod try_from;
}
mod slice;
#[cfg(feature = "alloc")]
mod vec;

use core::{convert::Infallible, fmt, num::NonZeroUsize};

#[derive(Debug, Clone, Copy, Hash)]
#[repr(transparent)]
pub struct NonEmpty<T: ?Sized> {
    inner: T,
}

pub type Array<T, const N: usize> = NonEmpty<[T; N]>;
pub type Slice<T> = NonEmpty<[T]>;
#[cfg(feature = "alloc")]
pub type Vec<T> = NonEmpty<alloc::vec::Vec<T>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(());

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("collection was empty")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

macro_rules! transmuting {
    () => {}; // base case
    (const $(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty; $($rest:tt)*) => {
        $(#[$meta])*
        /// # Safety
        /// - `src` must not be empty
        pub const unsafe fn $ident(src: $in) -> $out {
            debug_assert!(!src.is_empty());
            // Safety
            // - #[repr(transparent)]
            unsafe { core::mem::transmute(src) }
        }
        $crate::transmuting!($($rest)*);
    };
    ($(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty; $($rest:tt)*) => {
        $(#[$meta])*
        /// # Safety
        /// - `src` must not be empty
        pub unsafe fn $ident(src: $in) -> $out {
            debug_assert!(!src.is_empty());
            // Safety
            // - #[repr(transparent)]
            unsafe { core::mem::transmute(src) }
        }
        $crate::transmuting!($($rest)*);
    };
}
pub(crate) use transmuting;

macro_rules! map_non_empty {
    () => {}; // base case
    (const $(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty: $mapper:path; $($rest:tt)*) => {
        $(#[$meta])*
        /// Returns [`None`] if `src` is empty.
        pub const fn $ident(src: $in) -> Option<$out> {
            match src.is_empty() {
                true => None,
                // Safety
                // - checked non empty
                false => Some(unsafe { $mapper(src) })
            }
        }
        $crate::map_non_empty!($($rest)*);
    };
    ($(#[$meta:meta])* $ident:ident($in:ty) -> $out:ty: $mapper:path; $($rest:tt)*) => {
        $(#[$meta])*
        /// Returns [`None`] if `src` is empty.
        pub fn $ident(src: $in) -> Option<$out> {
            match src.is_empty() {
                true => None,
                // Safety
                // - checked non empty
                false => Some(unsafe { $mapper(src) })
            }
        }
        $crate::map_non_empty!($($rest)*);
    };
}
pub(crate) use map_non_empty;

macro_rules! as_ref_as_mut {
    ($($(<$ty_param:ident $(, const $const_param:ident: usize)?>)? for $self:ty as $ty:ty);* $(;)?) => {
        $(
            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::convert::AsRef<$ty> for $self {
                fn as_ref(&self) -> &$ty { self }
            }

            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::convert::AsMut<$ty> for $self {
                fn as_mut(&mut self) -> &mut $ty { self }
            }

        )*
    };
}
pub(crate) use as_ref_as_mut;

macro_rules! borrow_borrow_mut {
    ($($(<$ty_param:ident $(, const $const_param:ident: usize)?>)? for $self:ty as $ty:ty);* $(;)?) => {
        $(
            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::borrow::Borrow<$ty> for $self {
                fn borrow(&self) -> &$ty { self }
            }

            impl$(<$ty_param $(, const $const_param: usize)?>)? ::core::borrow::BorrowMut<$ty> for $self {
                fn borrow_mut(&mut self) -> &mut $ty { self }
            }

        )*
    };
}
pub(crate) use borrow_borrow_mut;

macro_rules! slice_iter {
    (<$ty_param:ident $(, const $const_param:ident: usize)?> for $self:ty) => {
        impl<'a, $ty_param $(, const $const_param: usize)?> ::core::iter::IntoIterator for &'a $self {
            type Item = &'a $ty_param;
            type IntoIter = ::core::slice::Iter<'a, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a, $ty_param $(, const $const_param: usize)?> ::core::iter::IntoIterator for &'a mut $self {
            type Item = &'a mut $ty_param;
            type IntoIter = ::core::slice::IterMut<'a, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }
    };
}
pub(crate) use slice_iter;

macro_rules! define_for_nonzeroes {
    ($dollar:tt $($n:literal)*) => {
        $(
            #[allow(clippy::zero_prefixed_literal)]
            const _: () = assert!($n != 0usize);
        )*
        /// Usage: special case `one` for documentation
        macro_rules! for_nonzeroes {
            ($dollar callback:ident) => {
                $dollar callback!(one);
                $(
                    $dollar callback!($n);
                )*
            }
        }
        pub(crate) use for_nonzeroes;
    };
}

define_for_nonzeroes!($
    // 0..256
/* 1 */	002	003	004	005	006	007	008	009	010
    011	012	013	014	015	016	017	018	019	020
    021	022	023	024	025	026	027	028	029	030
    031	032	033	034	035	036	037	038	039	040
    041	042	043	044	045	046	047	048	049	050
    051	052	053	054	055	056	057	058	059	060
    061	062	063	064	065	066	067	068	069	070
    071	072	073	074	075	076	077	078	079	080
    081	082	083	084	085	086	087	088	089	090
    091	092	093	094	095	096	097	098	099	100
    101	102	103	104	105	106	107	108	109	110
    111	112	113	114	115	116	117	118	119	120
    121	122	123	124	125	126	127	128	129	130
    131	132	133	134	135	136	137	138	139	140
    141	142	143	144	145	146	147	148	149	150
    151	152	153	154	155	156	157	158	159	160
    161	162	163	164	165	166	167	168	169	170
    171	172	173	174	175	176	177	178	179	180
    181	182	183	184	185	186	187	188	189	190
    191	192	193	194	195	196	197	198	199	200
    201	202	203	204	205	206	207	208	209	210
    211	212	213	214	215	216	217	218	219	220
    221	222	223	224	225	226	227	228	229	230
    231	232	233	234	235	236	237	238	239	240
    241	242	243	244	245	246	247	248	249	250
    251	252	253	254	255	256

    // Powers of two
    512 1024 2048 4096 8192 16384 32768 65536
);

#[track_caller]
const unsafe fn non_zero_usize(n: usize) -> NonZeroUsize {
    match NonZeroUsize::new(n) {
        Some(it) => it,
        None => unreachable(),
    }
}

#[track_caller]
const unsafe fn unreachable() -> ! {
    match cfg!(debug_assertions) {
        true => unreachable(),
        false => unsafe { core::hint::unreachable_unchecked() },
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TryFromSliceError(());

impl From<Infallible> for TryFromSliceError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl fmt::Display for TryFromSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("could not convert slice to array")
    }
}
