use crate::core::{DFChar, Reference, ReferenceTo, Flag};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShapeToken {
    /// Argument 1 of `[SHAPE:...]`
    #[serde(alias = "SHAPE")]
    pub reference: Option<ReferenceTo<Self>>,
    /// The name of the shape. Is not always used all by itself, see `GEMS_USE_ADJ`.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// The tile the shape uses onscreen, as an engraving or as an item (gem, die).
    #[serde(alias = "TILE")]
    pub tile: Option<DFChar>,
    /// Makes gems in this shape use the syntax '`ADJ` + material' e.g. "conglomerate gizzard stone".
    /// This, `GEMS_USE_NOUN` or `GEMS_USE_ADJ_NOUN` must be used for the name of a gem in this shape to show up.
    #[serde(alias = "GEMS_USE_ADJ")]
    pub gems_use_adj: Flag,
    /// Makes gems in this shape use the syntax '`ADJ` + material + `NAME`' e.g. "smooth conglomerate cabochon".
    /// This, `GEMS_USE_ADJ` or `GEMS_USE_ADJ_NOUN` must be used for the name of a gem in this shape to show up.
    #[serde(alias = "GEMS_USE_ADJ_NOUN")]
    pub gems_use_adj_noun: Flag,
    /// Makes gems in this shape use the syntax 'material + `NAME`' e.g. "point cut conglomerate".
    /// This, `GEMS_USE_ADJ` or`GEMS_USE_ADJ_NOUN` must be used for the name of a gem in this shape to show up.
    #[serde(alias = "GEMS_USE_NOUN")]
    pub gems_use_noun: Flag,
    /// The amount of sides on the dice.
    #[serde(alias = "FACES")]
    pub faces: Option<u32>,
    /// Effect unknown.
    #[serde(alias = "WORD")]
    pub word: Option<ReferenceTo<crate::WordToken>>,
    /// An adjective to be paired with the name. Can be used multiple times,
    /// allowing for variants of the same shape e.g. "thin cross", "tall cross".
    #[serde(alias = "ADJ")]
    pub adj: Vec<String>,
    /// A category the shape belongs to, which can be used by the `TOOL` token `SHAPE_CATEGORY`.
    /// Vanilla categories are `SIMPLE`, `PLATONIC`, and `DICE`, but any arbitrary category name is allowed.
    #[serde(alias = "CATEGORY")]
    pub category: Vec<Reference>,
}
