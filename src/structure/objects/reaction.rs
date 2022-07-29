use crate::core::{AllowEmpty, Choose, Clamp, Reference, ReferenceTo};
use crate::structure::{
    BuildingToken, ItemReferenceArg, KeyBindEnum, MaterialTokenArg, NoneEnum, SkillEnum,
};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReactionToken {
    /// Defines a new reaction
    #[serde(alias = "REACTION")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Defines the name used by the reaction in-game.
    #[serde(alias = "NAME")]
    pub name: Option<String>,
    /// Description of the reaction
    #[serde(alias = "DESCRIPTION")]
    pub description: Vec<AllowEmpty<String>>,
    /// This version of the reaction is not used by dwarves at home in a fortress, but rather
    /// the wanderers of Adventure Mode. When using this token, it will be allowed for adventurers
    /// of any race, without editing Entity files.
    #[serde(alias = "ADVENTURE_MODE_ENABLED")]
    pub adventure_mode_enabled: Option<()>,
    /// Amount of attributes given per skill improvement.
    ///
    /// Default is 10.
    ///
    /// Adding since: v0.47.01
    // TODO always positive?
    #[serde(alias = "ATTRIBUTE_IP")]
    pub attribute_ip: Option<i32>,
    /// The reaction will be queued automatically if the reaction reagents are all present.
    #[serde(alias = "AUTOMATIC")]
    pub automatic: Option<()>,
    /// Sets the building that the reaction will be performed in, and the button used to queue
    /// the reaction once that building's menu is accessed in-game
    #[serde(alias = "BUILDING")]
    pub building: Vec<(ReferenceTo<BuildingToken>, Choose<KeyBindEnum, NoneEnum>)>, // 2nd param is KeyBind
    /// Puts the reaction in a category. Categories are custom submenus for reaction menus.
    /// The category ID is a unique identifier for the category. It is only used in the raws, and
    /// will not appear in the game.
    ///
    /// If you're defining multiple categories within the same reaction - for example, if you
    /// intend the reaction to be nested two deep, and haven't yet defined the super-category -
    /// the last CATEGORY token within the reaction definition is the one that the reaction will
    /// appear in.
    #[serde(alias = "CATEGORY")]
    pub category: Option<ReactionCategoryToken>,
    /// Requires that the reaction either use up a unit of coal or charcoal or
    /// else be performed at a magma workshop
    #[serde(alias = "FUEL")]
    pub fuel: Option<()>,
    /// Sets the maximum number of times a reaction is allowed to run when using stacked reagents.
    /// This can be used to ensure that the reaction doesn't repeat until
    /// the entire stack is depleted.
    #[serde(alias = "MAX_MULTIPLIER")]
    pub max_multiplier: Option<u32>,
    /// Skill used by the reaction
    #[serde(alias = "SKILL")]
    pub skill: Option<SkillEnum>,
    /// Amount of skill given per product made.
    ///
    /// Default is 30.
    ///
    /// Adding since: v0.47.01
    // TODO always positive?
    #[serde(alias = "SKILL_IP")]
    pub skill_ip: Option<i32>,
    /// Proportion of how much the skill level effects the outcome (example: quality) of the result.
    ///
    /// The skill roll is `random(basic range) + random((skill level * multiplier)/2 + 1) +
    /// random((skill level * multiplier)/2 + 1)`.
    /// random(x) returns a number between 0 and x-1, so basic range is always 1 or more.
    /// Higher skill rolls give better results.
    ///
    /// Arguments:
    /// - Basic Range: The maximum value for the basic range. Need to be equal or bigger then `1`.
    /// - Skill level multiplier: Used to increase effectiveness of the skill level.
    ///
    /// Default is `[SKILL_ROLL_RANGE:11:5]`
    ///
    /// Adding since: v0.47.01
    #[serde(alias = "SKILL_ROLL_RANGE")]
    pub skill_roll_range: Option<(Clamp<u32, 1, { u32::MAX as isize }>, u32)>,
    /// Reagent must contain writing.
    /// Arguments are: `<chance>:<reagent/product target>:<type of improvement>:<mat tokens>`đđ
    /// `<mat tokens>` might consists of 3 arguments,
    /// for example: `GET_MATERIAL_FROM_REAGENT:reagent:REACTION_PRODUCT_ID`
    /// Other times it has 2 extra arguments like: `INORGANIC:BRONZE_COATING`
    // |-----0-----|-1-|----2-----|----------3----------|---------------------4--------------------|
    // [IMPROVEMENT:100:instrument:INSTRUMENT_PIECE:BODY:GET_MATERIAL_FROM_REAGENT:drum  :NONE     ]
    // [IMPROVEMENT:100:jug       :GLAZED               :GET_MATERIAL_FROM_REAGENT:glaze :GLAZE_MAT]
    // [IMPROVEMENT:100:a         :INSTRUMENT_PIECE:BODY:METAL                    :FRAME           ]
    // [IMPROVEMENT:100:target    :SPIKES               :GET_MATERIAL_FROM_REAGENT:gem   :NONE     ]
    #[serde(alias = "IMPROVEMENT")]
    pub improvement: Vec<(
        u8,                                 // chance
        Reference,                          // ReferenceTo<ReagentToken> // reagent
        Choose<ImprovementType, Reference>, // improvement_type
        MaterialTokenArg,                   // Material arguments
    )>,
    /// Requires a given reagent as an input for a reaction
    #[serde(alias = "REAGENT")]
    pub reagents: Vec<ReagentToken>,
    /// See description on `ProductToken`
    #[serde(alias = "PRODUCT")]
    pub products: Vec<ProductToken>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReagentToken {
    /// Requires a given reagent as an input for a reaction
    /// Arguments are: `<name/id>:<quantity>:<item token>:<mat tokens>`
    /// `<mat tokens>` and `<item token>` might consists of 2 arguments,
    /// for example: `BOULDER:NO_SUBTYPE` or `INORGANIC:COAL_BITUMINOUS`
    #[serde(alias = "REAGENT")]
    // [REAGENT:A            :1  :NONE       :NONE      :NONE     :NONE           ]
    // [REAGENT:lye          :150:LIQUID_MISC:NONE      :LYE                      ]
    // [REAGENT:lye container:1  :NONE       :NONE      :NONE     :NONE           ]
    // [REAGENT:A            :1  :BOULDER    :NO_SUBTYPE:INORGANIC:COAL_BITUMINOUS]
    pub reference: Option<(
        ReferenceTo<Self>,                              // Name/id
        u32,                                            // Quantity
        Choose<(NoneEnum, NoneEnum), ItemReferenceArg>, // Item token
        Choose<MaterialTokenArg, (NoneEnum, NoneEnum)>, // Material token
    )>,
    /// Reagent material must have the `[BONE]` token.
    #[serde(alias = "ANY_BONE_MATERIAL")]
    pub any_bone_material: Option<()>,
    /// Reagent material must have the `[HORN]` token.
    #[serde(alias = "ANY_HORN_MATERIAL")]
    pub any_horn_material: Option<()>,
    /// Reagent material must have the `[LEATHER]` token.
    #[serde(alias = "ANY_LEATHER_MATERIAL")]
    pub any_leather_material: Option<()>,
    /// Reagent material must have the `[PEARL]` token.  
    #[serde(alias = "ANY_PEARL_MATERIAL")]
    pub any_pearl_material: Option<()>,
    /// Reagent material must be subordinate to a `PLANT` object.
    #[serde(alias = "ANY_PLANT_MATERIAL")]
    pub any_plant_material: Option<()>,
    /// Reagent material must have the `[SHELL]` token.
    #[serde(alias = "ANY_SHELL_MATERIAL")]
    pub any_shell_material: Option<()>,
    /// Reagent material must have the `[SILK]` token.
    #[serde(alias = "ANY_SILK_MATERIAL")]
    pub any_silk_material: Option<()>,
    /// Reagent material must have the `[SOAP]` token.
    #[serde(alias = "ANY_SOAP_MATERIAL")]
    pub any_soap_material: Option<()>,
    /// Reagent is made of a tissue having `[TISSUE_SHAPE:STRANDS]`, intended for matching hair and
    /// wool. Must be used with `[USE_BODY_COMPONENT]`.
    #[serde(alias = "ANY_STRAND_TISSUE")]
    pub any_strand_tissue: Option<()>,
    /// Reagent material must have the `[TOOTH]` token.
    #[serde(alias = "ANY_TOOTH_MATERIAL")]
    pub any_tooth_material: Option<()>,
    /// Reagent material must have the `[YARN]` token.
    #[serde(alias = "ANY_YARN_MATERIAL")]
    pub any_yarn_material: Option<()>,
    /// Reagent has to be a bag. Intended to be used with an item type of `BOX`, to prevent chests,
    /// coffers, and other containers from being used instead.
    #[serde(alias = "BAG")]
    pub bag: Option<()>,
    /// Reagent is able to be used to build structures (Stone, Wood, Blocks, Bars?).
    #[serde(alias = "BUILDMAT")]
    pub build_material: Option<()>,
    /// Reagent can be an Artifact. Using `[PRESERVE_REAGENT]` with this is strongly advised.
    #[serde(alias = "CAN_USE_ARTIFACT")]
    pub can_use_artifact: Option<()>,
    /// Allows the reagent to be an item that is otherwise reserved for use by a hospital.
    #[serde(alias = "CAN_USE_HOSPITAL_RESERVED")]
    pub can_use_hospital_reserved: Option<()>,
    /// Allows the reagent to be an item that is otherwise reserved for use by a location.
    #[serde(alias = "CAN_USE_LOCATION_RESERVED")]
    pub can_use_location_reserved: Option<()>,
    /// Reagent is a container that holds the specified reagent.
    #[serde(alias = "CONTAINS")]
    pub contains: Option<Reference>,
    /// Reagent material must have `[ABSORPTION:0]`
    #[serde(alias = "DOES_NOT_ABSORB")]
    pub does_not_absorb: Option<()>,
    /// Performing a reaction with large stacks of inputs can allow multiple sets of outputs to be
    /// produced. Setting this flag causes the reagent to be ignored in this process -
    /// for example, with the reaction "1 plant + 1 barrel -> 5 alcohol (into barrel)",
    /// using this on the barrel allows the reaction to be performed as
    /// "5 plant + 1 barrel -> 25 alcohol" instead of "5 plant + 5 barrel -> 25 alcohol".
    #[serde(alias = "DOES_NOT_DETERMINE_PRODUCT_AMOUNT")]
    pub does_not_determine_product_amount: Option<()>,
    /// If the reagent is a container, it must be empty.
    #[serde(alias = "EMPTY")]
    pub empty: Option<()>,
    /// Reagent must be considered fire-safe (stable temperature below 11000 °U )
    /// - i.e. not wood, and not coal.
    #[serde(alias = "FIRE_BUILD_SAFE")]
    pub fire_build_safe: Option<()>,
    /// Reagent must be a barrel or any non-absorbing tool with `[TOOL_USE:FOOD_STORAGE]`
    #[serde(alias = "FOOD_STORAGE_CONTAINER")]
    pub food_storage_container: Option<()>,
    /// Reagent material has `[IS_GLASS]`.
    #[serde(alias = "GLASS_MATERIAL")]
    pub glass_material: Option<()>,
    /// Similar to `HAS_MATERIAL_REACTION_PRODUCT`, but requires the reagent's material to
    /// have a matching `ITEM_REACTION_PRODUCT` entry.
    #[serde(alias = "HAS_ITEM_REACTION_PRODUCT")]
    pub has_item_reaction_product: Option<Reference>, // TODO See REACTION_CLASS in MaterialDefinitionToken
    /// Similar to `REACTION_CLASS`, but requires the reagents material to have a matching
    /// `MATERIAL_REACTION_PRODUCT` entry. Intended for reactions which transform one class of
    /// material into another, such as skin->leather and fat->tallow.
    #[serde(alias = "HAS_MATERIAL_REACTION_PRODUCT")]
    pub has_material_reaction_product: Option<Reference>, // TODO See REACTION_CLASS in MaterialDefinitionToken
    /// Reagent must be a tool with the specific `TOOL_USE` value.
    /// The reagents item type must be `TOOL:NONE` for this to make any sense.
    #[serde(alias = "HAS_TOOL_USE")]
    /// Reference to `ITEM_TOOL` but the category like: `LIQUID_CONTAINER`.
    /// The item type must be `TOOL:NONE` for this to make any sense.
    pub has_tool_use: Option<Reference>, // TODO
    /// Reagent must contain writing.
    #[serde(alias = "HAS_WRITING_IMPROVEMENT")]
    pub has_writing_improvement: Option<()>,
    /// *Currently broken*
    ///
    /// Reagent must be considered magma-safe (stable temperature below 12000 °U ).
    #[serde(alias = "MAGMA_BUILD_SAFE")]
    pub magma_build_safe: Option<()>,
    /// Reagent material must be an ore of the specified metal.
    #[serde(alias = "METAL_ORE")]
    pub metal_ore: Option<Reference>, // TODO reference to Inorganic material
    /// Reagent's item dimension must be at least this large. The reagent's item type must be
    /// `BAR`, `POWDER_MISC`, `LIQUID_MISC`, `DRINK`, `THREAD`, `CLOTH`, or `GLOB` for this to work.
    #[serde(alias = "MIN_DIMENSION")]
    pub min_dimension: Option<u32>,
    /// Item must not have an edge, so must be blunt.
    /// Sharp stones (produced using knapping) and most types of weapon/ammo
    /// can not be used with this token.
    #[serde(alias = "NO_EDGE_ALLOWED")]
    pub no_edge_allowed: Option<()>,
    /// Reagent must be sharpened (used for carving).
    #[serde(alias = "HAS_EDGE")]
    pub has_edge: Option<()>,
    /// If the item is a container, it must not contain lye or milk.
    /// Not necessary if specifying `[EMPTY]`.
    #[serde(alias = "NOT_CONTAIN_BARREL_ITEM")]
    pub not_contain_barrel_item: Option<()>,
    /// Reagent can not be engraved. For example, a memorial slab can not be engraved.
    #[serde(alias = "NOT_ENGRAVED")]
    pub not_engraved: Option<()>,
    /// Reagent has not been decorated.
    #[serde(alias = "NOT_IMPROVED")]
    pub not_improved: Option<()>,
    /// Reagent must not be in the `SOLID_PRESSED` state.
    #[serde(alias = "NOT_PRESSED")]
    pub not_pressed: Option<()>,
    /// Reagent must be "collected" - used with `THREAD:NONE` to exclude webs.
    #[serde(alias = "NOT_WEB")]
    pub not_web: Option<()>,
    /// Reagent is not destroyed, which is the normal effect, at the completion of the reaction.
    /// Typically used for containers.
    #[serde(alias = "PRESERVE_REAGENT")]
    pub preserve_reagent: Option<()>,
    /// Requires the reagents material to have a matching `REACTION_CLASS` entry.
    /// Intended for reactions which accept a variety of materials but where the input material
    /// does not determine the output material, such as `FLUX` (for making pig iron and steel)
    /// and GYPSUM (for producing plaster powder).
    #[serde(alias = "REACTION_CLASS")]
    pub reaction_class: Option<Reference>, // TODO
    /// Reagent must not be rotten, mainly for organic materials.
    #[serde(alias = "UNROTTEN")]
    pub unrotten: Option<()>,
    /// Reagent material must come off a creature's body (`CORPSE` or `CORPSEPIECE`).
    #[serde(alias = "USE_BODY_COMPONENT")]
    pub use_body_component: Option<()>,
    /// Reagent must be "undisturbed" - used with `THREAD:NONE` to gather webs.
    #[serde(alias = "WEB_ONLY")]
    pub web_only: Option<()>,
    /// Reagent is made of an non-economic stone.
    #[serde(alias = "WORTHLESS_STONE_ONLY")]
    pub worthless_stone_only: Option<()>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ProductToken {
    /// Defines a thing that comes out of the reaction. `GET_MATERIAL_FROM_REAGENT` and
    /// `GET_ITEM_DATA_FROM_REAGENT` can be used to defer the choice of material and/or item to
    /// the appropriate tag in a given reagent's material - the former comes in place of the
    /// material token, the latter replaces both the item and material tokens.
    /// Arguments are:
    ///    `<probability of success(0-100)>:<quantity>:<item token>:<item subtype>:<mat tokens>`
    /// `<mat tokens>` might consists of 3 arguments,
    /// for example: `GET_MATERIAL_FROM_REAGENT:A:TAN_MAT`
    /// Other times it has 2 extra arguments like: `INORGANIC:BRONZE_COATING`
    // Tarn comment:
    // > In the product, if you want to use the reagent's material itself,
    // > use NONE instead of a reaction product class (TAN_MAT in this example).
    // |---0---|-1-|2|-----------------3--------------------|---------------------4-------------------|
    // [PRODUCT:100:1:SKIN_TANNED:NONE                      :GET_MATERIAL_FROM_REAGENT:A     :TAN_MAT ]
    // [PRODUCT:100:1:BAR        :NONE                      :GET_MATERIAL_FROM_REAGENT:tallow:SOAP_MAT]
    // [PRODUCT:100:4:BAR        :NO_SUBTYPE                :METAL                    :STERLING_SILVER]
    // [PRODUCT:100:1:WEAPON     :ITEM_WEAPON_SPEAR_TRAINING:GET_MATERIAL_FROM_REAGENT:log   :NONE    ]
    #[serde(alias = "PRODUCT")]
    pub reference: Option<(
        u8,               // probability_success
        u32,              // quantity
        ItemReferenceArg, // item_token
        MaterialTokenArg, // material_token
    )>,
    /// Product is given a sharp edge. Used for knapping.
    #[serde(alias = "FORCE_EDGE")]
    pub force_edge: Option<()>,
    /// Specifies the size of the product.
    /// A size of 150 is typical for `BAR`, `POWDER_MISC`, `LIQUID_MISC`, `DRINK`, and `GLOB`.
    /// A size of 15000 is typical for `THREAD`,
    /// and a size of 10000 is typical for `CLOTH`.
    #[serde(alias = "PRODUCT_DIMENSION")]
    pub product_dimension: Option<u32>,
    /// Product is created in the `SOLID_PASTE` state.
    #[serde(alias = "PRODUCT_PASTE")]
    pub product_paste: Option<()>,
    /// Product is created in the `SOLID_PRESSED` state.
    #[serde(alias = "PRODUCT_PRESSED")]
    pub product_pressed: Option<()>,
    /// Places the product in a container; `<id>` must be the name of a reagent with
    /// the `PRESERVE_REAGENT` token and a container item type.
    #[serde(alias = "PRODUCT_TO_CONTAINER")]
    pub product_to_container: Option<Reference>, // TODO refer to `[REAGENT:<This_value>:..]`
    /// Allows the product to be referred to by the given name, for the purpose of being passed
    /// down as argument in other tokens.
    #[serde(alias = "PRODUCT_TOKEN")]
    pub product_token: Option<Reference>, // TODO refer to `[REAGENT:<This_value>:..]`
    /// Transfers artifact status from the reagent to the product.
    #[serde(alias = "TRANSFER_ARTIFACT_STATUS")]
    pub transfer_artifact_status: Option<()>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ReactionCategoryToken {
    /// Argument 1 of `[CATEGORY:...]`
    #[serde(alias = "CATEGORY")]
    pub reference: Option<ReferenceTo<Self>>,
    /// The name of the category as displayed in-game.
    #[serde(alias = "CATEGORY_NAME")]
    pub category_name: Option<String>,
    /// If present, when the category is highlighted in a building menu, this string will be
    /// displayed in the Helpful Hint box.
    #[serde(alias = "CATEGORY_DESCRIPTION")]
    pub category_description: Option<String>,
    /// If present, when the category is highlighted in a building menu, this string will be
    /// displayed in the Helpful Hint box.
    #[serde(alias = "CATEGORY_PARENT")]
    pub category_parent: Option<ReferenceTo<Self>>,
    /// If present, this category can be selected from its parent menu (whether a building or a
    /// parent category) using the given hotkey.
    #[serde(alias = "CATEGORY_KEY")]
    pub category_key: Option<Choose<KeyBindEnum, NoneEnum>>, // param is KeyBind
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ImprovementType {
    #[serde(alias = "COVERED")]
    Covered,
    #[serde(alias = "GLAZED")]
    Glazed,
    #[serde(alias = "BANDS")]
    Bands,
    #[serde(alias = "RINGS_HANGING")]
    RingsHanging,
    #[serde(alias = "SPIKES")]
    Spikes,
    #[serde(alias = "INSTRUMENT_PIECE")]
    InstrumentPiece,
}

impl Default for ImprovementType {
    fn default() -> Self {
        Self::Covered
    }
}
