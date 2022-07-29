use crate::core::{Choose, Reference, ReferenceTo};
use crate::structure::{
    CreatureToken, InorganicToken, NoMatGlossEnum, NoneEnum, PlantToken, ReactionToken,
    ReagentToken,
};

use serde::{Deserialize, Serialize};

/// Wiki page: https://dwarffortresswiki.org/index.php/Material_token
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MaterialTokenArg {
    // |---0------|---------1--------|----2----|---3-----|
    // [TOKEN_NAME:LOCAL_CREATURE_MAT          :MUSCLE   ]
    // [TOKEN_NAME:PLANT_MAT         :APPLE    :FRUIT    ]
    // [TOKEN_NAME:CREATURE_MAT      :ANIMAL   :WOOL     ]
    // [TOKEN_NAME:INORGANIC                   :QUICKLIME]
    // [TOKEN_NAME:LYE                                   ]
    /// Argument group 1: with Enum arguments
    pub material: MaterialTypeEnum,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// The shape of the tissue
pub enum MaterialTypeEnum {
    /// Specifies a standalone inorganic material defined in the raws, generally a stone or metal.
    /// For example, `INORGANIC:IRON` refers to iron, and `INORGANIC:CERAMIC_PORCELAIN`
    /// refers to porcelain. The material name can be substituted with `USE_LAVA_STONE`
    /// to automatically select the local lava stone, which is normally obsidian.
    // #[serde(alias = "INORGANIC", alias = "STONE", alias = "METAL")]
    Inorganic(ReferenceTo<InorganicToken>),
    /// Specifies a material associated with a specific creature.
    /// Examples: `CREATURE_MAT:DWARF:SKIN` refers to dwarf skin.
    // #[serde(alias = "CREATURE_MAT")]
    CreatureMat((ReferenceTo<CreatureToken>, Reference)),
    /// Alias for `CREATURE_MAT:CREATURE_ID:MATERIAL_NAME`,
    /// where `CREATURE_ID` is the creature currently being defined;
    /// as such, it can only be used in creature definitions.
    // #[serde(alias = "LOCAL_CREATURE_MAT")]
    LocalCreatureMat(Reference),
    /// Specifies a material associated with a specific plant.
    /// Example: `PLANT_MAT:BUSH_QUARRY:LEAF` refers to quarry bush leaves.
    // #[serde(alias = "PLANT_MAT")]
    PlantMat((ReferenceTo<PlantToken>, Reference)),
    /// Alias for `PLANT_MAT:PLANT_ID:MATERIAL_NAME`,
    /// where `PLANT_ID` is the plant currently being defined;
    /// as such, it can only be used in plant definitions.
    // #[serde(alias = "LOCAL_PLANT_MAT")]
    LocalPlantMat(Reference),
    /// Specifies a material related to a reagent's material within a reaction.
    /// `REAGENT_ID` must match a `[REAGENT]`, and `REACTION_PRODUCT_ID` must either match a
    /// `[MATERIAL_REACTION_PRODUCT]` belonging to the reagent's material
    /// or be equal to `NONE` to use the reagent's material itself.
    // #[serde(alias = "GET_MATERIAL_FROM_REAGENT")]
    GetMaterialFromReagent(
        (
            ReferenceTo<ReactionToken>,
            Choose<NoneEnum, ReferenceTo<ReagentToken>>,
        ),
    ),
    //-----------Hardcoded materials--------
    /// Specifies one of the hardcoded materials.
    /// Amber is a type of material made from fossilized tree resin.
    // #[serde(alias = "AMBER")]
    Amber(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Coral is a type of material composed of the dead remains of corals,
    /// creatures that have not yet been implemented into the game.
    // #[serde(alias = "CORAL")]
    Coral(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Glass is produced at the glass furnace using fuel with either sand (green glass),
    /// sand and pearlash (clear glass), or rock crystal and pearlash (crystal glass).
    // #[serde(alias = "GLASS_GREEN")]
    GlassGreen(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Glass is produced at the glass furnace using fuel with either sand (green glass),
    /// sand and pearlash (clear glass), or rock crystal and pearlash (crystal glass).
    // #[serde(alias = "GLASS_CLEAR")]
    GlassClear(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Glass is produced at the glass furnace using fuel with either sand (green glass),
    /// sand and pearlash (clear glass), or rock crystal and pearlash (crystal glass).
    // #[serde(alias = "GLASS_CRYSTAL")]
    GlassCrystal(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Water, when placed in buckets or when mining out ice.
    // #[serde(alias = "WATER")]
    Water(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Specifies a material that can be used as fuel - charcoal or coke.
    /// Specifying `NO_MATGLOSS` (not `NONE`) will make it accept "refined coal" in general,
    /// which matches charcoal, coke, and generic refined coal.
    // #[serde(alias = "COAL")]
    Coal(CoalMaterialEnum),
    /// Specifies one of the hardcoded materials.
    /// Potash is a wood-based product which has applications in farming,
    /// as well as production of mid- and high-end glass products.
    // #[serde(alias = "POTASH")]
    Potash(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Ash is an intermediate good used to make potash, lye, or to glaze ceramics.
    // #[serde(alias = "ASH")]
    Ash(Option<Choose<NoneEnum, NoMatGlossEnum>>),
    /// Specifies one of the hardcoded materials.
    /// Pearlash is a wood-based product which is used primarily
    /// in the manufacture of clear and crystal glass.
    // #[serde(alias = "PEARLASH")]
    Pearlash(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Lye is a material used to make soap, and can also be used to make potash.
    // #[serde(alias = "LYE")]
    Lye(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Mud is a contaminant produced when an area is covered with water, and colors tiles brown.
    // #[serde(alias = "MUD")]
    Mud(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Under certain conditions, creatures (such as your dwarves) will vomit,
    /// creating a puddle of vomit.
    // #[serde(alias = "VOMIT")]
    Vomit(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Salt is a contaminant that makes oceanic water unsuitable for drinking.
    // #[serde(alias = "SALT")]
    Salt(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Filth comes in two varieties: solid brown (B) and liquid yellow (Y) filth.
    // #[serde(alias = "FILTH_B")]
    FilthB(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Filth comes in two varieties: solid brown (B) and liquid yellow (Y) filth.
    // #[serde(alias = "FILTH_Y")]
    FilthY(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Unknown substance is a hardcoded material that takes the form of a light gray liquid.
    /// No longer used in recent versions of game.
    // #[serde(alias = "UNKNOWN_SUBSTANCE")]
    UnknownSubstance(Option<NoneEnum>),
    /// Specifies one of the hardcoded materials.
    /// Grime is a brown-colored contaminant that makes water from murky pools disgusting to drink.
    // #[serde(alias = "GRIME")]
    Grime(Option<NoneEnum>),
}
impl Default for MaterialTypeEnum {
    fn default() -> Self {
        Self::Inorganic(ReferenceTo::new(String::default()))
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum CoalMaterialEnum {
    #[serde(alias = "CHARCOAL")]
    Charcoal,
    #[serde(alias = "COKE")]
    Coke,
    /// Make it accept "refined coal" in general,
    /// which matches charcoal, coke, and generic refined coal.
    #[serde(alias = "NO_MATGLOSS")]
    NoMatgloss,
}
impl Default for CoalMaterialEnum {
    fn default() -> Self {
        Self::Charcoal
    }
}
