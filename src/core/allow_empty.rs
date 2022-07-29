#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// Work same way as `Option<T>` except for `TokenDeserialize` trait
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AllowEmpty<T> {
    None,
    Some(T),
}

impl<T> Default for AllowEmpty<T> {
    fn default() -> Self {
        Self::None
    }
}

/// Work same way as `Option<T>`
impl<T> AllowEmpty<T> {
    pub const fn is_some(&self) -> bool {
        matches!(*self, AllowEmpty::Some(_))
    }
    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }
    pub fn expect(self, msg: &str) -> T {
        match self {
            AllowEmpty::Some(val) => val,
            AllowEmpty::None => panic!("{}", msg),
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            AllowEmpty::Some(val) => val,
            AllowEmpty::None => panic!("called `AllowEmpty::unwrap()` on a `None` value"),
        }
    }
}

impl<T: Copy> AllowEmpty<&T> {
    pub fn copied(self) -> AllowEmpty<T> {
        match self {
            AllowEmpty::Some(&t) => AllowEmpty::Some(t),
            AllowEmpty::None => AllowEmpty::None,
        }
    }
}

impl<T: Clone> AllowEmpty<&T> {
    pub fn cloned(self) -> AllowEmpty<T> {
        match self {
            AllowEmpty::Some(t) => AllowEmpty::Some(t.clone()),
            AllowEmpty::None => AllowEmpty::None,
        }
    }
}
