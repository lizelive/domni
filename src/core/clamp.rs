#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Clamps the value to the range between `L` and `H` (inclusive)
/// The min a max value are also effected by the type and `L` and `H` should never
/// go out of that range.
///
/// Can be used with any integer value that can be safely converted to a `i64`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Clamp<T, const L: isize, const H: isize> {
    pub value: T,
}

impl<T, const L: isize, const H: isize> Clamp<T, L, H> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Default, const L: isize, const H: isize> Default for Clamp<T, L, H> {
    fn default() -> Self {
        Self {
            value: T::default(),
        }
    }
}

macro_rules! clamp_from_integer {
    ( $x:ty ) => {
        impl<const L: isize, const H: isize> From<$x> for Clamp<$x, L, H> {
            fn from(item: $x) -> Self {
                Self { value: item }
            }
        }
    };
}

// Big numbers like i128 should not be used.
// clamp_from_integer!(i128);
clamp_from_integer!(i64);
clamp_from_integer!(i32);
clamp_from_integer!(i16);
clamp_from_integer!(i8);

// Big numbers like u128 should not be used.
// clamp_from_integer!(u128);
// Can not cast i64 to u64 without loss
// clamp_from_integer!(u64);
clamp_from_integer!(u32);
clamp_from_integer!(u16);
clamp_from_integer!(u8);
