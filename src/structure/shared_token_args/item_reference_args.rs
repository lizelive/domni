use crate::core::{Choose, Reference};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ItemReferenceArg {
    pub item_type: Reference, // ReferenceTo<ItemToken>
    pub item_subtype: Choose<NoSubtypeEnum, Reference>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NoSubtypeEnum {
    #[serde(alias = "NONE", alias = "NO_SUBTYPE")]
    None,
}
impl Default for NoSubtypeEnum {
    fn default() -> Self {
        Self::None
    }
}
