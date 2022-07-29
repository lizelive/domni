use serde::{Deserialize, Serialize};

use super::Reference;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Any {
    /// Limited to `i32::MIN` and `u32::MAX`
    Integer(i64),
    Character(char),
    String(String),
    Reference(Reference),
}

impl Default for Any {
    fn default() -> Self {
        Self::String(String::default())
    }
}

macro_rules! from_integer {
    ( $x:ty ) => {
        impl From<$x> for Any {
            fn from(item: $x) -> Any {
                Any::Integer(item as i64)
            }
        }
    };
}

// Big numbers like i128 should not be used.
// from_integer!(i128);
from_integer!(i64);
from_integer!(i32);
from_integer!(i16);
from_integer!(i8);

// Big numbers like u128 should not be used.
// from_integer!(u128);
// Can not cast i64 to u64 without loss
// from_integer!(u64);
from_integer!(u32);
from_integer!(u16);
from_integer!(u8);

impl From<char> for Any {
    fn from(item: char) -> Any {
        Any::Character(item)
    }
}

impl From<String> for Any {
    fn from(item: String) -> Any {
        Any::String(item)
    }
}

impl From<Reference> for Any {
    fn from(item: Reference) -> Any {
        Any::Reference(item)
    }
}
