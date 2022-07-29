use crate::core::{Choose, Clamp, DFChar, Reference, ReferenceTo, Flag};
use crate::structure::{ItemReferenceArg, KeyBindEnum, LaborEnum, MaterialTokenArg, NoneEnum};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BuildingToken {
    #[serde(alias = "BUILDING_WORKSHOP")]
    Workshop(BuildingGeneralToken),
    #[serde(alias = "BUILDING_FURNACE")]
    Furnace(BuildingGeneralToken),
}
impl Default for BuildingToken {
    fn default() -> Self {
        Self::Workshop(BuildingGeneralToken::default())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BuildingGeneralToken {
    /// Argument 1 of `[BUILDING_WORKSHOP:...]` or `[BUILDING_FURNACE:...]`
    #[serde(
        alias = "BUILDING_WORKSHOP",
        alias = "BUILDING_FURNACE", // TODO should not give alias warning
    )]
    pub reference: Option<ReferenceTo<Self>>,
    /// The name of the custom building.
    #[serde(alias = "NAME")]
    pub name: Option<String>,
    /// The color of the building's name when querying it.
    /// Seemingly ignored for furnaces, which are hardcoded to 4:0:1.
    ///
    /// Arguments: `[NAME_COLOR:fg:bg:bright]`
    #[serde(alias = "NAME_COLOR")]
    pub name_color: Option<(u8, u8, u8)>,
    /// The size of the custom building, in number of tiles.
    ///
    /// Arguments: `[DIM:width:height]`
    /// Defaults to 3x3.
    /// Maximum possible size is 31x31.
    #[serde(alias = "DIM")]
    pub dim: Option<(Clamp<u8, 0, 31>, Clamp<u8, 0, 31>)>,
    /// The tile (1:1 for upper-left) in which dwarves will stand when they are performing tasks.
    ///
    /// Arguments: `[WORK_LOCATION:x:y]`
    /// Defaults to 3:3 (bottom-right).
    #[serde(alias = "WORK_LOCATION")]
    pub work_location: Option<(Clamp<u8, 1, 31>, Clamp<u8, 1, 31>)>,
    /// The labor required to construct the custom building.
    /// If multiple `BUILD_LABOR` tokens are specified, then any of the indicated labors can
    /// be used to construct the building; if none are specified, then no labors are required.
    /// For furnaces, this labor does not come into play until after the
    /// workshop has been designed by an architect.
    #[serde(alias = "BUILD_LABOR")]
    pub build_labor: Vec<LaborEnum>,
    /// The shortcut key used in the Build menu for selecting the custom building.
    #[serde(alias = "BUILD_KEY")]
    pub build_key: Option<KeyBindEnum>,
    /// Specifies whether or not each workshop tile blocks movement.
    /// The first parameter is the row (1 = top), and each subsequent parameter
    /// is a 0 (nonblocking) or 1 (blocking) for each column, left to right.
    ///
    /// Arguments: `[BLOCK:row_nr:blocking_args]`
    #[serde(alias = "BLOCK")]
    pub block: Vec<(u8, bool, Vec<bool>)>,
    /// Specifies the characters used to represent the custom building.
    /// The first parameter is the building stage, varying from 0 (awaiting construction)
    /// to N (completed) where N is between 1 and 3, the 2nd parameter is the row number,
    /// and each subsequent parameter is a character number
    /// (or literal character enclosed in 'quotes').
    ///
    /// Arguments: `[TILE:building_stage:row_nr:df_chars]`
    #[serde(alias = "TILE")]
    pub tile: Vec<(
        Clamp<u8, 0, 3>, // Building stage
        u8,              // Row number
        DFChar,          // Building characters (at least 1 required)
        Vec<DFChar>,     // Building characters
    )>,
    /// Specifies the colors in which the custom building's tiles will be displayed.
    /// The first parameter is the building stage, the 2nd parameter is the row number,
    /// and subsequent parameters are either sets of 3 numbers (`foreground:background:brightness`)
    /// or the token `MAT` to use the color of the primary building material.
    /// `MAT` may not be available on `BUILDING_FURNACE`s.
    ///
    /// Arguments: `[COLOR:building_stage:row_nr:colors]`
    #[serde(alias = "COLOR")]
    pub color: Vec<(
        Clamp<u8, 0, 3>,                    // Building stage
        u8,                                 // Row number
        Choose<MatEnum, (u8, u8, u8)>,      // Building color (at least 1 required)
        Vec<Choose<MatEnum, (u8, u8, u8)>>, // Building color
    )>,
    /// Specifies one of the objects necessary to construct the custom building.
    /// Each `BUILD_ITEM` can be followed by zero or more modifiers.
    ///
    /// Arguments: `[BUILD_ITEM:quantity:item_token:material_token]`
    #[serde(alias = "BUILD_ITEM")]
    pub build_item: Vec<BuildItemToken>,
    /// Specifies that one of the building's tiles (other than the `WORK_LOCATION`)
    /// must be hanging over magma in order for the building to function.
    /// Buildings with this token also ignore the `[FUEL]` token in their reactions.
    #[serde(alias = "NEEDS_MAGMA")]
    pub needs_magma: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BuildItemToken {
    /// Argument 1 of `[BUILD_ITEM:...]`
    #[serde(alias = "BUILD_ITEM")]
    pub build_item: Option<(
        u32,                                            // quantity
        ItemReferenceArg,                               // Item token
        Choose<MaterialTokenArg, (NoneEnum, NoneEnum)>, // Material token
    )>,
    //-------------------------------------------------------------
    // All tokens below are similar or the same as `ReagentToken`.
    //-------------------------------------------------------------
    /// Item material must have the `[BONE]` token.
    #[serde(alias = "ANY_BONE_MATERIAL")]
    pub any_bone_material: Flag,
    /// Item material must have the `[HORN]` token.
    #[serde(alias = "ANY_HORN_MATERIAL")]
    pub any_horn_material: Flag,
    /// Item material must have the `[LEATHER]` token.
    #[serde(alias = "ANY_LEATHER_MATERIAL")]
    pub any_leather_material: Flag,
    /// Item material must have the `[PEARL]` token.  
    #[serde(alias = "ANY_PEARL_MATERIAL")]
    pub any_pearl_material: Flag,
    /// Item material must be subordinate to a `PLANT` object.
    #[serde(alias = "ANY_PLANT_MATERIAL")]
    pub any_plant_material: Flag,
    /// Item material must have the `[SHELL]` token.
    #[serde(alias = "ANY_SHELL_MATERIAL")]
    pub any_shell_material: Flag,
    /// Item material must have the `[SILK]` token.
    #[serde(alias = "ANY_SILK_MATERIAL")]
    pub any_silk_material: Flag,
    /// Item material must have the `[SOAP]` token.
    #[serde(alias = "ANY_SOAP_MATERIAL")]
    pub any_soap_material: Flag,
    /// Item is made of a tissue having `[TISSUE_SHAPE:STRANDS]`,
    /// intended for matching hair and wool.
    /// Must be used with `[USE_BODY_COMPONENT]`.
    #[serde(alias = "ANY_STRAND_TISSUE")]
    pub any_strand_tissue: Flag,
    /// Item material must have the `[TOOTH]` token.
    #[serde(alias = "ANY_TOOTH_MATERIAL")]
    pub any_tooth_material: Flag,
    /// Item material must have the `[YARN]` token.
    #[serde(alias = "ANY_YARN_MATERIAL")]
    pub any_yarn_material: Flag,
    /// Item has to be a bag. Intended to be used with an item type of `BOX`,
    /// to prevent chests, coffers, and other containers from being used instead.
    #[serde(alias = "BAG")]
    pub bag: Flag,
    /// Item is able to be used to build structures (Stone, Wood, Blocks, Bars?).
    #[serde(alias = "BUILDMAT")]
    pub build_material: Flag,
    /// Item can be an Artifact.
    #[serde(alias = "CAN_USE_ARTIFACT")]
    pub can_use_artifact: Flag,

    // Not used in Reaction
    /// Item must be a `BARREL` or `TOOL` which contains at least one item of
    /// type `LIQUID_MISC` made of `LYE`.
    #[serde(alias = "CONTAINS_LYE", alias = "POTASHABLE")]
    pub contains: Option<Reference>,

    /// If the item is a container, it must be empty.
    #[serde(alias = "EMPTY")]
    pub empty: Flag,
    /// Item material must be considered fire-safe (stable temperature below 11000 °U ).
    /// Only works with items of type `BAR`, `BLOCKS`, `BOULDER`, `WOOD`, and `ANVIL` -
    /// all others are considered unsafe.
    #[serde(alias = "FIRE_BUILD_SAFE")]
    pub fire_build_safe: Flag,
    /// Item material has `[IS_GLASS]`. All 3 types of glass have this token hardcoded.
    #[serde(alias = "GLASS_MATERIAL")]
    pub glass_material: Flag,
    /// Similar to `REACTION_CLASS`, but requires the reagents material to have a matching
    /// `MATERIAL_REACTION_PRODUCT` entry. Intended for reactions which transform one class of
    /// material into another, such as skin->leather and fat->tallow.
    #[serde(alias = "HAS_MATERIAL_REACTION_PRODUCT")]
    pub has_material_reaction_product: Option<Reference>, // TODO See REACTION_CLASS in MaterialDefinitionToken
    /// Item must be a tool with the specific `TOOL_USE` value.
    /// The item type must be `TOOL:NONE` for this to make any sense.
    pub has_tool_use: Option<Reference>, // TODO reference to Item ToolToken (must have `TOOL_USE`)
    /// Item material must be considered fire-safe (stable temperature below 12000 °U ).
    /// Only works with items of type `BAR`, `BLOCKS`, `BOULDER`, `WOOD`, and `ANVIL` -
    /// all others are considered unsafe.
    #[serde(alias = "MAGMA_BUILD_SAFE")]
    pub magma_build_safe: Flag,
    /// Item material must be an ore of the specified metal.
    #[serde(alias = "METAL_ORE")]
    pub metal_ore: Option<Reference>, // TODO reference to Inorganic material
    /// Item's item dimension must be at least this large. The reagent's item type must be
    /// `BAR`, `POWDER_MISC`, `LIQUID_MISC`, `DRINK`, `THREAD`, `CLOTH`, or `GLOB` for this to work.
    #[serde(alias = "MIN_DIMENSION")]
    pub min_dimension: Option<u32>,
    /// Item must not have an edge, so must be blunt.
    /// Sharp stones (produced using knapping) and most types of weapon/ammo
    /// can not be used with this token.
    #[serde(alias = "NO_EDGE_ALLOWED")]
    pub no_edge_allowed: Flag,
    /// If the item is a container, it must not contain lye or milk.
    /// Not necessary if specifying `[EMPTY]`.
    #[serde(alias = "NOT_CONTAIN_BARREL_ITEM")]
    pub not_contain_barrel_item: Flag,
    /// Item can not be engraved. For example, a memorial slab can not be engraved.
    #[serde(alias = "NOT_ENGRAVED")]
    pub not_engraved: Flag,
    /// Item must be "collected" - used with `THREAD:NONE` to exclude webs.
    #[serde(alias = "NOT_WEB")]
    pub not_web: Flag,
    /// Requires the reagents material to have a matching `REACTION_CLASS` entry.
    /// Intended for reactions which accept a variety of materials but where the input material
    /// does not determine the output material, such as `FLUX` (for making pig iron and steel)
    /// and `GYPSUM` (for producing plaster powder).
    #[serde(alias = "REACTION_CLASS")]
    pub reaction_class: Option<Reference>, // TODO reference to Reaction token
    /// Item must not be rotten, mainly for organic materials.
    #[serde(alias = "UNROTTEN")]
    pub unrotten: Flag,
    /// Item material must come off a creature's body (`CORPSE` or `CORPSEPIECE`).
    #[serde(alias = "USE_BODY_COMPONENT")]
    pub use_body_component: Flag,
    /// Item must be "undisturbed" - used with `THREAD:NONE` to gather webs.
    #[serde(alias = "WEB_ONLY")]
    pub web_only: Flag,
    /// Item is made of an non-economic stone.
    #[serde(alias = "WORTHLESS_STONE_ONLY")]
    pub worthless_stone_only: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MatEnum {
    #[serde(alias = "MAT")]
    Mat,
}

impl Default for MatEnum {
    fn default() -> Self {
        Self::Mat
    }
}
