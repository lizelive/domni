use serde::{Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Clone, Default)]
pub struct ReferenceTo<T>(pub String, PhantomData<T>);

fn get_ref_type<T>() -> &'static str {
    "ReferenceTo"
}
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor, IgnoredAny};

impl<T> Serialize for ReferenceTo<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de, T> Deserialize<'de> for ReferenceTo<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReferenceVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for ReferenceVisitor<T> {
            type Value = ReferenceTo<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`str` or `[str]`")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let value = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                // Skip over any remaining elements in the sequence 
                while let Some(IgnoredAny) = seq.next_element()? {
                    // ignore
                }
                Ok(value)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ReferenceTo::from(value.to_string()))
            }
        }
        deserializer.deserialize_any(ReferenceVisitor(PhantomData::<T>))
    }
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
impl<T> Eq for ReferenceTo<T> where T: Clone {}

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
