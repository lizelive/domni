use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub struct DFChar(pub char);

impl From<char> for DFChar {
    fn from(item: char) -> DFChar {
        DFChar(item)
    }
}

impl From<DFChar> for char {
    fn from(item: DFChar) -> char {
        item.0
    }
}
