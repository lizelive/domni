use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ReferenceTo<T>(pub String, PhantomData<T>);

fn get_ref_type<T>() -> &'static str {
    "ReferenceTo"
}

impl<T> fmt::Debug for ReferenceTo<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ReferenceTo")
            .field(&self.0)
            .field(&format_args!("T: {}", get_ref_type::<T>()))
            .finish()
    }
}

impl<T> PartialEq for ReferenceTo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for ReferenceTo<T> {}

impl<T> ReferenceTo<T> {
    pub fn new(reference: String) -> Self {
        ReferenceTo::<T>(reference, PhantomData)
    }

    pub fn get_type() -> &'static str {
        get_ref_type::<T>()
    }
}

impl<T> Hash for ReferenceTo<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> From<String> for ReferenceTo<T> {
    fn from(item: String) -> ReferenceTo<T> {
        ReferenceTo::<T>(item, PhantomData)
    }
}

impl<T> From<ReferenceTo<T>> for String {
    fn from(item: ReferenceTo<T>) -> String {
        item.0
    }
}
