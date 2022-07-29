use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Choose<T1, T2> {
    Choice1(T1),
    Choice2(T2),
}

impl<T1: Default, T2> Default for Choose<T1, T2> {
    fn default() -> Self {
        Self::Choice1(T1::default())
    }
}
