use crate::core::ReferenceTo;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ColorToken {
    /// Argument 1 of `COLOR`
    /// The reference for the Color in other RAW files and tokens
    pub reference: Option<ReferenceTo<Self>>,
    /// The name of the color.
    pub name: Option<String>,
    /// The word for the color, see the `WORD` token in `languages_X.txt`.
    #[serde(alias = "WORD")]
    pub word: Vec<ReferenceTo<crate::WordToken>>,
    /// The RGB (Red,Green,Blue) color value of the color.
    #[serde(alias = "RGB")]
    pub rgb: Option<(u8, u8, u8)>,
}
