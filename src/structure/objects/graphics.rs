use crate::core::ReferenceTo;
use crate::structure::CreatureToken;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum GraphicsToken {
    #[serde(alias = "TILE_PAGE")]
    TilePage(TilePageToken),
    #[serde(alias = "CREATURE_GRAPHICS")]
    CreatureGraphics(CreatureGraphicsToken),
}
impl Default for GraphicsToken {
    fn default() -> Self {
        Self::TilePage(TilePageToken::default())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TilePageToken {
    /// Argument 1 of `[TILE_PAGE:...]`
    #[serde(alias = "TILE_PAGE")]
    pub reference: Option<ReferenceTo<Self>>,
    /// The relative path to the image.
    /// This file is should be relative to the current file and should include the extension.
    /// Allowed extensions are: `png`, `bmp`, ...
    // TODO Filepath, unix encoded (`/`), relative path from current file and with extension.
    #[serde(alias = "FILE")]
    pub file: Option<String>,
    /// The dimensions or size of a tile (1 character).
    /// For a 32x32 tileset this is `32:32`.
    ///
    /// Arguments: `[TILE_DIM:height:width]`
    #[serde(alias = "TILE_DIM")]
    pub tile_dimensions: Option<(u32, u32)>,
    /// The dimensions or size of the page.
    /// For a 32x32 tileset with 10 rows and 12 columns this is `10:12`.
    /// So in this case the actual image should be: 32*12 and 32*10 = `384x320px`
    ///
    /// Arguments: `[PAGE_DIM:width:height]` (NOTE: flipped compared to `TILE_DIM`)
    #[serde(alias = "PAGE_DIM")]
    pub page_dimensions: Option<(u32, u32)>,
}

type CreatureGraphicsTokenArg = (
    ReferenceTo<TilePageToken>,
    u32,
    u32,
    ColorTypeEnum,
    TextureTypeEnum,
);
type TextureGraphicsTokenArg = (
    ReferenceTo<TilePageToken>,
    u32,
    u32,
    ColorTypeEnum,
    Option<TextureTypeEnum>,
);

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TextureTypeEnum {
    #[serde(alias = "DEFAULT")]
    Default,
    #[serde(alias = "ADVENTURER")]
    Adventurer,
    /// Deprecated: Please user`Default`
    // TODO: issue #83
    #[serde(alias = "GUARD")]
    Guard,
    /// Deprecated: Please use `Default`
    // TODO: issue #83
    #[serde(alias = "ROYALGUARD")]
    RoyalGuard,
    /// Deprecated: Please use `Default`
    // TODO: issue #83
    #[serde(alias = "ANIMATED")]
    Animated,
    /// Deprecated: Please use `Default`
    // TODO: issue #83
    #[serde(alias = "GHOST")]
    Ghost,
}
impl Default for TextureTypeEnum {
    fn default() -> Self {
        Self::Default
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ColorTypeEnum {
    #[serde(alias = "ADD_COLOR")]
    AddColor,
    #[serde(alias = "AS_IS")]
    AsIs,
}
impl Default for ColorTypeEnum {
    fn default() -> Self {
        Self::AddColor
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CreatureGraphicsToken {
    /// Argument 1 of `[CREATURE_GRAPHICS:...]`
    // #[serde(alias = "CREATURE_GRAPHICS")]
    pub reference: Option<ReferenceTo<CreatureToken>>,

    // All tokens "DEFAULT", "ADVENTURER", "GUARD", "ROYALGUARD", "ANIMATED", "GHOST".
    pub main_texture_tokens: IndexMap<String, Vec<TextureGraphicsTokenArg>>,

    // All others, including professions
    pub other_graphics_tokens: IndexMap<String, Vec<CreatureGraphicsTokenArg>>,
}
