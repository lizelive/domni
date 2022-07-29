use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
#[serde(from = "ChooseMaybeTagged<T1, T2>")]
pub enum Choose<T1, T2> {
    Choice1(T1),
    Choice2(T2),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChooseTagged<T1, T2> {
    Choice1(T1),
    Choice2(T2),
}

impl<T1, T2> From<ChooseMaybeTagged<T1, T2>> for Choose<T1, T2> {
    fn from(val: ChooseMaybeTagged<T1, T2>) -> Self {
        match val {
            ChooseMaybeTagged::Tagged1 { value } => Choose::Choice1(value),
            ChooseMaybeTagged::Tagged2 { value } => Choose::Choice2(value),
            ChooseMaybeTagged::Untagged1(value) => Choose::Choice1(value),
            ChooseMaybeTagged::Untagged2(value) => Choose::Choice2(value),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ChooseMaybeTagged<T1, T2> {
    Tagged1 {
        #[serde(rename = "Choice1")]
        value: T1,
    },
    Tagged2 {
        #[serde(rename = "Choice2")]
        value: T2,
    },
    Untagged1(T1),
    Untagged2(T2),
}

impl<T1: Default, T2> Default for Choose<T1, T2> {
    fn default() -> Self {
        Self::Choice1(T1::default())
    }
}
