//! # A fast byte comparison library
//!
//! The library is intended to provide a faster byte slice comparison than the standard library.
//! Also raw string literals `b"like this"` are compareable this way.
//!
//! ## Example usage
//!
//! ```rust
//! use fastcmp::Compare;
//!
//! let vec = vec![1, 2, 3, 4, 5];
//! assert!(vec.feq(&[1, 2, 3, 4, 5]));
//! ```
include!(concat!(env!("OUT_DIR"), "/compare.rs"));

#[cfg(feature = "simd_support")]
extern crate simd;

#[cfg(feature = "simd_support")]
use simd::u8x16;

// The pointer compare macro with offset support
macro_rules! cmp (
    ($left:expr, $right: expr, $var:ident, $offset:expr) => {
        unsafe {*($left.offset($offset) as *const $var) == *($right.offset($offset) as *const $var)}
    }
);

#[cfg(feature = "simd_support")]
macro_rules! cmp_simd (
    ($var:ident, $left:expr, $right:expr, $offset:expr) => { $var::load($left, $offset).eq($var::load($right, $offset)).all() }
);

#[cfg(feature = "simd_support")]
macro_rules! cmp_u128 (
    ($left:expr, $right:expr, $len:expr, $offset:expr) => { cmp_simd!(u8x16, $left, $right, $offset) }
);

/// Memory compare trait
pub trait Compare {
    /// Compares an `&[u8]` to another one
    fn feq(self: &Self, to: &Self) -> bool;
}

impl Compare for [u8] {
    #[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
    #[inline(always)]
    fn feq(&self, to: &[u8]) -> bool {

        // Fallback if the slices are too large
        extern "C" {
            fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
        }

        // Get the comparison pointers
        let a = to.as_ptr() as *const i8;
        let b = self.as_ptr() as *const i8;
        let len = to.len();

        // Do the comparison
        self.len() == len && slice_comare!(a, b, to, self, len)
    }
}
