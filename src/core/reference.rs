use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct Reference(pub String);

impl From<String> for Reference {
    fn from(item: String) -> Reference {
        Reference(item)
    }
}

impl From<Reference> for String {
    fn from(item: Reference) -> String {
        item.0
    }
}
