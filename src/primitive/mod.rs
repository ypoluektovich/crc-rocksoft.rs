//! Implementations of `CrcSpec` and `CrcHasher` for primitive types `u8`, `u16`, `u32`, `u64`
//! and `usize`. The width of the polynom used for each type corresponds to the type's bit size,
//! e. g. CRC32 variants are backed by `u32`.
//!
//! # Examples
//!
//! This is how you compute a check value for the POSIX CRC-32 variant (found e. g. in
//! [the `cksum` program](http://pubs.opengroup.org/onlinepubs/7990989775/xcu/cksum.html)).
//!
//! ```
//! use crc_rocksoft::*;
//! use crc_rocksoft::primitive::*;
//!
//! let spec = CrcTable::new(0x04C11DB7u32, 0u32, false, false, 0xFFFFFFFFu32);
//! let mut hasher = CrcTableHasher::from(spec);
//! for i in 1..10 {
//!     hasher.update(0x30 + i); // ASCII characters 1, 2, 3, ... 9
//! }
//! assert_eq!(hasher.finish(), 0x765E7680);
//! ```


mod table;
mod spec;
mod hasher;

use std::ops::{Not, Shl, Shr, BitAnd, BitXor};
use bit_reverse::ParallelReverse;
use std::fmt::Debug;


/// A trait that extends all the operation traits necessary for
/// computing a lookup table for optimized CRC computation.
///
/// This module provides implementations of [`CrcSpec`](../trait.CrcSpec.html)
/// and [`CrcHasher`](../trait.CrcHasher.html) for types that implement this trait.
/// This module also includes default implementations of `ValueType` for unsigned integers.
///
/// This was intended for use in abstracting out concrete unsigned integer types,
/// but nobody prevents you from rolling your own implementation.
pub trait ValueType:
    From<u8> +
    Not<Output=Self> +
    Shl<u8, Output=Self> +
    Shr<u8, Output=Self> +
    ParallelReverse<Self> +
    BitXor<Self, Output=Self> +
    BitAnd<Self, Output=Self> +
    Eq +
    Copy +
    Debug
{
    /// `Into<u8>` is apparently not implemented by default for narrowing conversions
    /// of primitive integers, so I decided to make that into a special method. This one.
    fn to_u8(self) -> u8;
}

macro_rules! impl_value_type {
    ($t:ty) => {
        impl ValueType for $t {
            fn to_u8(self) -> u8 {
                self as u8
            }
        }
    };
}
impl_value_type!(u8);
impl_value_type!(u16);
impl_value_type!(u32);
impl_value_type!(u64);
impl_value_type!(usize);


use self::table::*;
pub use self::spec::*;
pub use self::hasher::*;
