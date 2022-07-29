use crate::structure::*;

use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DFRaw {
    pub header: String,
    pub object_tokens: Vec<ObjectToken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ObjectToken {
    #[serde(alias = "BODY")]
    pub body_tokens: Vec<BodyObjectToken>,
    #[serde(alias = "BODY_DETAIL_PLAN")]
    pub body_detail_plan_tokens: Vec<BodyDetailPlanToken>,
    #[serde(alias = "BUILDING")]
    pub building_tokens: Vec<BuildingToken>,
    #[serde(alias = "CREATURE")]
    pub creature_tokens: Vec<CreatureToken>,
    #[serde(alias = "CREATURE_VARIATION")]
    pub creature_variation_tokens: Vec<CreatureVariationToken>,
    #[serde(alias = "DESCRIPTOR_COLOR")]
    pub color_tokens: Vec<ColorToken>,
    #[serde(alias = "DESCRIPTOR_PATTERN")]
    pub pattern_tokens: Vec<PatternToken>,
    #[serde(alias = "DESCRIPTOR_SHAPE")]
    pub shape_tokens: Vec<ShapeToken>,
    #[serde(alias = "ENTITY")]
    pub entity_tokens: Vec<EntityToken>,
    #[serde(alias = "GRAPHICS")]
    pub graphics_tokens: Vec<GraphicsToken>,
    #[serde(alias = "INTERACTION")]
    pub interaction_tokens: Vec<InteractionToken>,
    #[serde(alias = "INORGANIC")]
    pub inorganic_tokens: Vec<InorganicToken>,
    #[serde(alias = "ITEM")]
    pub item_tokens: Vec<ItemToken>,
    #[serde(alias = "LANGUAGE")]
    pub language_tokens: Vec<LanguageToken>,
    #[serde(alias = "MATERIAL_TEMPLATE")]
    pub material_tokens: Vec<MaterialToken>,
    #[serde(alias = "PLANT")]
    pub plant_tokens: Vec<PlantToken>,
    #[serde(alias = "REACTION")]
    pub reaction_tokens: Vec<ReactionToken>,
    #[serde(alias = "TISSUE_TEMPLATE")]
    pub tissue_template_tokens: Vec<TissueToken>,
}
