use crate::core::{AllowEmpty, Any, Choose, Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

use crate::structure::{
    AllEnum, AnyHardStoneEnum, AppGeneticModelEnum, AppModTypeEnum, BodyPartTypeEnum,
    BpAppModifersEnum, BpCriteriaTokenArg, BpRelationEnum, CdiTokenArg, ColorToken,
    ItemReferenceArg, MaterialTokenArg, NoEndEnum, NormalEnum, PositionEnum, RootEnum, ShapeToken,
    SingularOrPluralEnum, StylingEnum, TimescaleEnum, TissueModifierEnum,
};

/// Makes the creature able to perform this interaction. Follow this effect token with
/// various `CDI` tokens to specify its properties.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CanDoInteraction {
    /// Arguments for `[CAN_DO_INTERACTION:...]`
    #[serde(alias = "CAN_DO_INTERACTION")]
    pub reference: Option<Reference>,
    /// Specifies details for the preceding `[CAN_DO_INTERACTION:...]` token.
    /// See [interaction token](https://dwarffortresswiki.org/index.php/Interaction_token).
    #[serde(alias = "CDI")]
    pub cdi: Vec<CdiTokenArg>,
}

/// The creature drops an additional object from the specified body parts when butchered.
/// The items dropped are defined by `[EBO_ITEM]` and `[EBO_SHAPE]`.
///
/// Used for gizzard stones in vanilla creatures.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExtraButcherObject {
    /// Arguments for `[EXTRA_BUTCHER_OBJECT:...]`
    #[serde(alias = "EXTRA_BUTCHER_OBJECT")]
    pub reference: Option<BpCriteriaTokenArg>,
    /// A list of `EBO_ITEM` tokens in this `EXTRA_BUTCHER_OBJECT`
    #[serde(alias = "EBO_ITEM")]
    pub ebo_item: Option<EboItem>,
}

/// Defines the item that the creature drops upon being butchered.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EboItem {
    /// Arguments for `[EBO_ITEM:...]`
    #[serde(alias = "EBO_ITEM")]
    // TODO check duplicate
    pub reference: Option<(ItemReferenceArg, Choose<AnyHardStoneEnum, MaterialTokenArg>)>,
    /// The shape of the creature's extra butchering drop.
    #[serde(alias = "EBO_SHAPE")]
    pub ebo_shape: Option<ReferenceTo<ShapeToken>>,
}

/// These body modifiers give individual creatures different characteristics. In the case of
/// `HEIGHT`, `BROADNESS` and `LENGTH`, the modifier is also a percentage change to the
/// `BODY_SIZE` of the individual creature. The seven numbers afterward give a distribution of
/// ranges. Each interval has an equal chance of occurring.
///
/// Example:
///
/// `[BODY_APPEARANCE_MODIFIER:HEIGHT:90:95:98:100:102:105:110]`
///
/// `HEIGHT` : marks the height to be changed
///
/// `90:95:98:100:102:105:110` : sets the range from the shortest (90% of the average height) to
/// the tallest (110% of the average height) creature variation.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BodyAppearanceModifier {
    /// Arguments for `[BODY_APPEARANCE_MODIFIER:...]`
    #[serde(alias = "BODY_APPEARANCE_MODIFIER")]
    pub body_appearance_modifier: Option<(AppModTypeEnum, u32, u32, u32, u32, u32, u32, u32)>,
    // region: APP_MOD tokens =====================================================================
    /// This token determines how appearance modifiers will be described. That is, a preceding
    /// `APPPEARANCE_MODIFIER` token will specify a range of values for given body part or tissue;
    /// for example, the length of a creatures hair, whereas *this* token specifies when to *describe*
    /// the hair as being extremely short, very short, slightly short, typical, long,
    /// very long etc etc.
    ///
    /// For example:
    ///
    /// `[APP_MOD_DESC_RANGE:30:60:90:110:150:190]`
    ///
    /// This would mean that 0-30 will be described as extremely short, 31-60 as very short,
    /// and so on (the last value will range from 191 to any higher value at all).
    ///
    /// Note that this token does not alter how often the associated body part will actually have
    /// a given value, only what ranges are described a particular way. For example, if you specied
    /// the maximum hair length as 150, then the above example with a top level `APP_MOD_DESC_RANGE`
    /// of 191+ would never describe that creatures hair as "extremely long", or even as "very long"
    /// (as that descriptor is for hair of length 151-190).
    #[serde(alias = "APP_MOD_DESC_RANGE")]
    pub app_mod_desc_range: Option<(u32, u32, u32, u32, u32, u32)>,
    /// Defines a genetic model for the relevant appearance modifier(s). May or may not do anything
    /// significant at present.
    #[serde(alias = "APP_MOD_GENETIC_MODEL")]
    pub app_mod_genetic_model: Option<AppGeneticModelEnum>,
    /// Determines how important the appearance modifier is, for determining whether it shows up in
    /// the creature description (unverified).
    #[serde(alias = "APP_MOD_IMPORTANCE")]
    pub app_mod_importance: Option<u32>,
    /// Creates a noun for the appearance, and whether it is singular or plural.
    #[serde(alias = "APP_MOD_NOUN")]
    pub app_mod_noun: Option<(String, SingularOrPluralEnum)>,
    /// Setting the growth rate of the modifier. The last two tokens can be replaced by `NO_END` to
    /// have growth continue indefinitely.
    #[serde(alias = "APP_MOD_RATE")]
    pub app_mod_rate: Option<(
        u32,
        TimescaleEnum,
        u32,
        u32,
        u32,
        u32,
        Choose<NoEndEnum, (u32, u32)>,
    )>,
    // endregion ==================================================================================
}

/// Begins a selection of body parts.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SetBpGroup {
    /// Arguments for `[SET_BP_GROUP:...]`
    #[serde(alias = "SET_BP_GROUP")]
    pub set_bp_group: Option<BpCriteriaTokenArg>,
    /// Adds a body part group to selected body part group. Presumably used immediately after
    /// `[SET_BP_GROUP]`.
    #[serde(alias = "PLUS_BP_GROUP")]
    pub plus_bp_group: Vec<BpCriteriaTokenArg>,
    /// A list of `BP_APPEARANCE_MODIFIER` tokens in this `SET_BP_GROUP`
    #[serde(alias = "BP_APPEARANCE_MODIFIER")]
    pub bp_appearance_modifier: Vec<BpAppearanceModifier>,
    /// Adds a type to a body part. In vanilla DF, this is used for adding the type 'GELDABLE'
    /// to the lower body of certain creatures.
    #[serde(alias = "BP_ADD_TYPE")]
    pub bp_add_type: Option<BodyPartTypeEnum>,
    /// Removes a type from a body part.
    #[serde(alias = "BP_REMOVE_TYPE")]
    pub bp_remove_type: Option<BodyPartTypeEnum>,
}

/// Sets up the breadth of possibilities for appearance qualities for a selected `BP` group.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpAppearanceModifier {
    /// Arguments for `[BP_APPEARANCE_MODIFIER:...]`
    #[serde(alias = "BP_APPEARANCE_MODIFIER")]
    pub bp_appearance_modifier: Option<(BpAppModifersEnum, u32, u32, u32, u32, u32, u32, u32)>,
    // region: APP_MOD tokens =====================================================================
    /// This token determines how appearance modifiers will be described. That is, a preceding
    /// `APPPEARANCE_MODIFIER` token will specify a range of values for given body part or tissue;
    /// for example, the length of a creatures hair, whereas *this* token specifies when to *describe*
    /// the hair as being extremely short, very short, slightly short, typical, long,
    /// very long etc etc.
    ///
    /// For example:
    ///
    /// `[APP_MOD_DESC_RANGE:30:60:90:110:150:190]`
    ///
    /// This would mean that 0-30 will be described as extremely short, 31-60 as very short,
    /// and so on (the last value will range from 191 to any higher value at all).
    ///
    /// Note that this token does not alter how often the associated body part will actually have
    /// a given value, only what ranges are described a particular way. For example, if you specied
    /// the maximum hair length as 150, then the above example with a top level `APP_MOD_DESC_RANGE`
    /// of 191+ would never describe that creatures hair as "extremely long", or even as "very long"
    /// (as that descriptor is for hair of length 151-190).
    #[serde(alias = "APP_MOD_DESC_RANGE")]
    pub app_mod_desc_range: Option<(u32, u32, u32, u32, u32, u32)>,
    /// Defines a genetic model for the relevant appearance modifier(s). May or may not do anything
    /// significant at present.
    #[serde(alias = "APP_MOD_GENETIC_MODEL")]
    pub app_mod_genetic_model: Option<AppGeneticModelEnum>,
    /// Determines how important the appearance modifier is, for determining whether it shows up in
    /// the creature description (unverified).
    #[serde(alias = "APP_MOD_IMPORTANCE")]
    pub app_mod_importance: Option<u32>,
    /// Creates a noun for the appearance, and whether it is singular or plural.
    #[serde(alias = "APP_MOD_NOUN")]
    pub app_mod_noun: Option<(String, SingularOrPluralEnum)>,
    /// Setting the growth rate of the modifier. The last two tokens can be replaced by `NO_END` to
    /// have growth continue indefinitely.
    #[serde(alias = "APP_MOD_RATE")]
    pub app_mod_rate: Option<(
        u32,
        TimescaleEnum,
        u32,
        u32,
        u32,
        u32,
        Choose<NoEndEnum, (u32, u32)>,
    )>,
    // endregion ==================================================================================
}

/// Begins a selection of tissue layers.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SetTlGroup {
    /// Arguments for `[SET_TL_GROUP:...]`
    #[serde(alias = "SET_TL_GROUP")]
    pub set_tl_group: Option<(BpCriteriaTokenArg, Reference)>,
    /// Continues a selection of tissue layers.
    #[serde(alias = "PLUS_TL_GROUP")]
    pub plus_tl_group: Vec<(BpCriteriaTokenArg, Reference)>,
    /// A list of `TL_COLOR_MODIFIER` tokens in this `SET_TL_GROUP`
    #[serde(alias = "TL_COLOR_MODIFIER")]
    pub tl_color_modifier: Vec<TlColorModifier>,
    /// A list of `TISSUE_LAYER_APPEARANCE_MODIFIER` tokens in this `SET_TL_GROUP`
    #[serde(alias = "TISSUE_LAYER_APPEARANCE_MODIFIER")]
    pub tissue_layer_appearance_modifier: Vec<TissueLayerAppearanceModifier>,
    /// A list of `TISSUE_STYLE_UNIT` tokens in this `SET_TL_GROUP`
    #[serde(alias = "TISSUE_STYLE_UNIT")]
    pub tissue_style_unit: Option<TissueStyleUnit>,
    /// Tissue layer can be sheared for its component material. The specified modifier must be at
    /// least of the desired value for shearing to be possible (for example, a llama's wool must
    /// have a `LENGTH` of 300 before it is shearable).
    #[serde(alias = "SHEARABLE_TISSUE_LAYER")]
    pub shearable_tissue_layer: Option<(TissueModifierEnum, u32)>,
}

/// Sets the range of qualities, including `LENGTH`, `DENSE`, `HIGH_POSITION`, `CURLY`,
/// `GREASY`, WRINKLY
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TissueLayerAppearanceModifier {
    /// Arguments for `[TISSUE_LAYER_APPEARANCE_MODIFIER:...]`
    #[serde(alias = "TISSUE_LAYER_APPEARANCE_MODIFIER")]
    pub tissue_layer_appearance_modifier:
        Option<(TissueModifierEnum, u32, u32, u32, u32, u32, u32, u32)>,
    // region: APP_MOD tokens =====================================================================
    /// This token determines how appearance modifiers will be described. That is, a preceding
    /// `APPPEARANCE_MODIFIER` token will specify a range of values for given body part or tissue;
    /// for example, the length of a creatures hair, whereas *this* token specifies when to *describe*
    /// the hair as being extremely short, very short, slightly short, typical, long,
    /// very long etc etc.
    ///
    /// For example:
    ///
    /// `[APP_MOD_DESC_RANGE:30:60:90:110:150:190]`
    ///
    /// This would mean that 0-30 will be described as extremely short, 31-60 as very short,
    /// and so on (the last value will range from 191 to any higher value at all).
    ///
    /// Note that this token does not alter how often the associated body part will actually have
    /// a given value, only what ranges are described a particular way. For example, if you specied
    /// the maximum hair length as 150, then the above example with a top level `APP_MOD_DESC_RANGE`
    /// of 191+ would never describe that creatures hair as "extremely long", or even as "very long"
    /// (as that descriptor is for hair of length 151-190).
    #[serde(alias = "APP_MOD_DESC_RANGE")]
    pub app_mod_desc_range: Option<(u32, u32, u32, u32, u32, u32)>,
    /// Defines a genetic model for the relevant appearance modifier(s). May or may not do anything
    /// significant at present.
    #[serde(alias = "APP_MOD_GENETIC_MODEL")]
    pub app_mod_genetic_model: Option<AppGeneticModelEnum>,
    /// Determines how important the appearance modifier is, for determining whether it shows up in
    /// the creature description (unverified).
    #[serde(alias = "APP_MOD_IMPORTANCE")]
    pub app_mod_importance: Option<u32>,
    /// Creates a noun for the appearance, and whether it is singular or plural.
    #[serde(alias = "APP_MOD_NOUN")]
    pub app_mod_noun: Option<(String, SingularOrPluralEnum)>,
    /// Setting the growth rate of the modifier. The last two tokens can be replaced by `NO_END` to
    /// have growth continue indefinitely.
    #[serde(alias = "APP_MOD_RATE")]
    pub app_mod_rate: Option<(
        u32,
        TimescaleEnum,
        u32,
        u32,
        u32,
        u32,
        Choose<NoEndEnum, (u32, u32)>,
    )>,
    // endregion ==================================================================================
}

/// Creates a list of colors/color patterns, giving each a relative frequency. If the given
/// color or pattern does not exist, the tissue is described as being "transparent".
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TlColorModifier {
    /// Arguments for `[TL_COLOR_MODIFIER:...]`
    #[serde(alias = "TL_COLOR_MODIFIER")]
    pub tl_color_modifier: Option<(Vec<(ReferenceTo<ColorToken>, u32)>,)>,
    /// The way the color modifier is passed on to offspring. May or may not work in the current
    /// version of Dwarf Fortress.
    #[serde(alias = "TLCM_GENETIC_MODEL")]
    pub tlcm_genetic_model: Option<()>,
    /// Presumably modifies the importance of the tissue layer color modifier, for description
    /// purposes.
    ///
    /// HOWEVER, using this appears to remove all mention of colour from creature descriptions. It
    /// does not appear in any default creatures.
    #[serde(alias = "TLCM_IMPORTANCE")]
    pub tlcm_importance: Option<u32>,
    /// Names the tissue layer color modifier, and determines the noun. Also used by stonesense for
    /// colouring body parts.
    #[serde(alias = "TLCM_NOUN")]
    pub tlcm_noun: Option<(String, SingularOrPluralEnum)>,
    /// Determines the point in the creature's life when the color change begins and ends.
    #[serde(alias = "TLCM_TIMING")]
    pub tlcm_timing: Option<(RootEnum, u32, u32, u32, u32)>,
}

/// Sets tissue layer to be the target of `TISSUE_STYLE` token specified for an entity, works
/// only on entity members. Mostly used with tissues `HAIR`, `BEARD`, `MOUSTACHE`, `SIDEBURNS`.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TissueStyleUnit {
    /// Arguments for `[TISSUE_STYLE_UNIT:...]`
    #[serde(alias = "TISSUE_STYLE_UNIT")]
    pub tissue_style_unit: Option<(Reference, StylingEnum)>,
    /// Noun for the `[TISSUE_STYLE_UNIT]`, used in the description of the tissue layer's style.
    #[serde(alias = "TSU_NOUN")]
    pub tsu_noun: Option<(String, SingularOrPluralEnum)>,
}

/// Adds the tissue layer to wherever it is required.
///
/// Non-argument Locations can be `FRONT`, `RIGHT`, `LEFT`, `TOP`, `BOTTOM`. Argument locations
/// are `AROUND` and `CLEANS`, requiring a further body part and a % of coverage/cleansing
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TissueLayer {
    #[serde(alias = "TISSUE_LAYER")]
    pub tissue_layer: Option<(
        BpCriteriaTokenArg,
        Reference,
        // TODO: investigate how all these tokens dealing with selecting tissues and BPs actually work,
        // this is too many edge cases, it's ridiculous.
        Option<
            Choose<
                NormalEnum,
                Choose<PositionEnum, (BpRelationEnum, BpCriteriaTokenArg, Option<u8>)>,
            >,
        >,
    )>,
    /// Adds a tissue to those selected
    #[serde(alias = "PLUS_TISSUE_LAYER")]
    pub plus_tissue_layer: Vec<(Reference, BpCriteriaTokenArg)>,
    /// Sets a selected tissue layer to be made of a different tissue.
    #[serde(alias = "SET_LAYER_TISSUE")]
    pub set_layer_tissue: Vec<Reference>,
    /// Gives the `CONNECTS` attribute to selected layers.
    #[serde(alias = "TL_CONNECTS")]
    pub tl_connects: Option<()>,
    /// Changes the `HEALING_RATE` of the selected tissue layers.
    #[serde(alias = "TL_HEALING_RATE")]
    pub tl_healing_rate: Option<u32>,
    /// Gives the "major arteries" attribute to selected layers. Used to add massive bleeding
    /// properties to the throat, made from skin.
    #[serde(alias = "TL_MAJOR_ARTERIES")]
    pub tl_major_arteries: Option<()>,
    /// Changes the number of pain receptors for selected tissue layers.
    #[serde(alias = "TL_PAIN_RECEPTORS")]
    pub tl_pain_receptors: Option<u32>,
    /// Changes the relative thickness for selected tissue layers.
    #[serde(alias = "TL_RELATIVE_THICKNESS")]
    pub tl_relative_thickness: Option<u32>,
    /// Sets a new `VASCULAR` value (which modulates bleeding) for selected tissue layers.
    #[serde(alias = "TL_VASCULAR")]
    pub tl_vascular: Option<u32>,
}

/// Selects a tissue layer for descriptor and cosmetic purposes.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SelectTissueLayer {
    /// Arguments for `[SELECT_TISSUE_LAYER:...]`
    #[serde(alias = "SELECT_TISSUE_LAYER")]
    // TODO: investigate how all these tokens dealing with selecting tissues and BPs actually work,
    // this is too many edge cases, it's ridiculous.
    pub select_tissue_layer: Option<
        Choose<
            (AllEnum, Option<(BpCriteriaTokenArg, Option<PositionEnum>)>),
            (Reference, BpCriteriaTokenArg, Option<PositionEnum>),
        >,
    >,
    /// Adds a tissue to those selected
    #[serde(alias = "PLUS_TISSUE_LAYER")] // TODO nest tissue select
    pub plus_tissue_layer: Vec<(Reference, BpCriteriaTokenArg)>,
    /// Sets a selected tissue layer to be made of a different tissue.
    #[serde(alias = "SET_LAYER_TISSUE")]
    pub set_layer_tissue: Vec<Reference>,
    /// Gives the `CONNECTS` attribute to selected layers.
    #[serde(alias = "TL_CONNECTS")]
    pub tl_connects: Option<()>,
    /// Changes the `HEALING_RATE` of the selected tissue layers.
    #[serde(alias = "TL_HEALING_RATE")]
    pub tl_healing_rate: Option<u32>,
    /// Gives the "major arteries" attribute to selected layers. Used to add massive bleeding
    /// properties to the throat, made from skin.
    #[serde(alias = "TL_MAJOR_ARTERIES")]
    pub tl_major_arteries: Option<()>,
    /// Changes the number of pain receptors for selected tissue layers.
    #[serde(alias = "TL_PAIN_RECEPTORS")]
    pub tl_pain_receptors: Option<u32>,
    /// Changes the relative thickness for selected tissue layers.
    #[serde(alias = "TL_RELATIVE_THICKNESS")]
    pub tl_relative_thickness: Option<u32>,
    /// Sets a new `VASCULAR` value (which modulates bleeding) for selected tissue layers.
    #[serde(alias = "TL_VASCULAR")]
    pub tl_vascular: Option<u32>,
}

/// Starts a "conversion block" to modify the arguments of existing tokens on a creature.
/// A conversion block contains one `CVCT_MASTER`, one `CVCT_TARGET`, and one `CVCT_REPLACEMENT`
/// (note, `CVCT_REPLACEMENT` is optional, and leaving it out may be used to erase parts of
/// arguments instead of replacing them).
///
/// Note that if a creature contains multiple `CV_CONVERT_TAG` blocks altering the same token,
/// the replacements will be applied in reverse order
/// (see [Application](https://dwarffortresswiki.org/index.php/Creature_variation_token#Application)
/// for more detail).
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CreatureCvConvertTag {
    /// `CV_CONVERT_TAG`; has no arguments.
    #[serde(alias = "CV_CONVERT_TAG")]
    pub cv_convert_tag: Option<()>,
    // TODO: properly implement the 3 below tokens:
    // --------------------------------------------------------------------------------------------
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Specifies which tokens of the creature may be modified, by looking at full arguments
    /// at the start of said token.
    ///
    /// For example, `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]` would target
    /// `[BODY:HUMANOID_SIMPLE:3_EYES]`, but `[CVCT_MASTER:BODY:3_EYES]`, or `[CVCT_MASTER:3_EYES]`
    /// would not. Neither would `[CVCT_MASTER:BODY:HUMANOID_SIMPLE]` be able to target
    /// `[BODY:HUMANOID_SIMPLE_NECK]`, as it looks for whole arguments.
    ///
    /// `[CVCT_MASTER:BODY]` would target both examples above, as they both start with `BODY`.
    ///
    /// If no `CVCT_MASTER` is given in the conversion block, or it is given no arguments,
    /// or the only argument it is given is blank (i.e. `[CVCT_MASTER:]`), *all* tokens of
    /// the creature are selected.
    ///
    /// Note that if a creature contains multiple `CV_CONVERT_TAG` blocks altering the same token,
    /// the replacements will be applied in reverse order
    /// (see [Application](https://dwarffortresswiki.org/index.php/Creature_variation_token#Application)
    /// for more detail).
    #[serde(alias = "CVCT_MASTER")]
    pub cvct_master: Option<
        AllowEmpty<(
            Reference, // TODO: ref can be name of any token that nests under CREATURE
            Option<(Vec<Any>,)>,
        )>,
    >,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Locates the specified parameters or portions of parameters, within all tokens specified
    /// by `CVCT_MASTER`.
    ///
    /// For example, this conversion block:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:STATE_NAME]
    ///     [CVCT_TARGET:adder]
    /// ```
    /// Will select every instance of `adder` in all arguments of all `STATE_NAME` tokens
    /// in that creature, and *only* the portion saying `adder`:
    ///
    /// ```df_raw
    /// [STATE_NAME:ALL_SOLID:frozen adder venom]
    /// [STATE_NAME:LIQUID:adder venom]
    /// ```
    /// You may target references and integers as well as strings, though be warned that integers
    /// cannot be targeted exactly; for instance, `[CVCT_TARGET:1]` will select the `1` in `10`
    /// or `101` as well.
    ///
    /// If no `CVCT_TARGET` is given in the conversion block, or it is given no arguments,
    /// or the only argument it is given is blank (i.e. `[CVCT_TARGET:]`), the game will freeze
    /// when loading the creature.
    #[serde(alias = "CVCT_TARGET")]
    pub cvct_target: Option<(Vec<Any>,)>,
    /// **Warning: Incomplete token. This token is not yet properly implemented, and so you will
    /// not get any hover text information from the arguments, autocomplete will not work, and
    /// you will not be alerted to any errors.**
    ///
    /// ---
    /// Replaces the string specified by `CVCT_TARGET` within the tokens specified by `CVCT_MASTER`.
    /// This means the targeted part of a token can be changed anywhere in the token, e.g:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:BODY]
    ///     [CVCT_TARGET:2EYES]
    ///     [CVCT_REPLACEMENT:2EYESTALKS]
    /// ```
    /// Would affect both of these:
    ///
    /// ```df_raw
    /// [BODY:QUADRUPED_NECK:NOSE:2LUNGS:BRAIN:2EYES]
    /// [BODY:INSECT:2EYES:HEART:GUTS:BRAIN:MOUTH:2WINGS]
    /// ```
    /// Converting them into:
    ///
    /// ```df_raw
    /// [BODY:QUADRUPED_NECK:NOSE:2LUNGS:BRAIN:2EYESTALKS]
    /// [BODY:INSECT:2EYESTALKS:HEART:GUTS:BRAIN:MOUTH:2WINGS]
    /// ```
    /// Colons can be included as part of both the target and the replacement string, for example:
    ///
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:BODY]
    ///     [CVCT_TARGET:BASIC_1PARTBODY:BASIC_HEAD]
    ///     [CVCT_REPLACEMENT:HUMANOID:3FINGERS]
    /// ```
    /// Will convert `[BODY:BASIC_1PARTBODY:BASIC_HEAD:HEART:GUTS:BRAIN:MOUTH:2EYESTALKS]`, into
    /// `[BODY:HUMANOID:3FINGERS:HEART:GUTS:BRAIN:MOUTH:2EYESTALKS]`. All occurrences of the target
    ///  string are replaced, for example:
    /// ```df_raw
    /// [CV_CONVERT_TAG]
    ///     [CVCT_MASTER:DESCRIPTION]
    ///     [CVCT_TARGET:TRAIT]
    ///     [CVCT_REPLACEMENT:modderiffic]
    /// ```
    /// Will convert `[DESCRIPTION:This is an example creature. It is TRAIT, very very TRAIT.]`,
    /// into `[DESCRIPTION:This is an example creature. It is modderiffic, very very modderiffic.]`.
    ///
    /// If no `CVCT_REPLACEMENT` is given, the target string is simply removed.
    #[serde(alias = "CVCT_REPLACEMENT")]
    pub cvct_replacement: Option<(Vec<Any>,)>,
}
