use crate::core::{Choose, Reference, ReferenceTo, Flag};
use crate::structure::{
    BreathFlowEnum, BreathMaterialEnum, CasteFlagEnum, CreatureFlagEnum, CreatureToken,
    EffectLocationEnum, MaterialTokenArg, SphereEnum, SynTransmittionMethodEnum, SyndromeToken,
    TargetPropertyEnum,
};

use serde::{Deserialize, Serialize};

/// Define a new interaction.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InteractionToken {
    /// Argument 1 of `[INTERACTION:...]`
    #[serde(alias = "INTERACTION")]
    pub reference: Option<ReferenceTo<Self>>,
    /// This disallows use of the interaction in play, and also encourages usage specifically to
    /// create experimental populations.
    #[serde(alias = "EXPERIMENT_ONLY")]
    pub experiment_only: Flag,
    /// Defines what things are capable of triggering this interaction - multiple sources may be
    /// specified.
    #[serde(alias = "I_SOURCE")]
    pub i_source: Vec<ISource>,
    /// Defines the targets available for subsequent use with `[I_EFFECT]` tokens. Multiple targets
    /// may be specified; the precise target(s) used with each interaction effect are indicated via
    /// their ID as specified in `IE_TARGET`.
    #[serde(alias = "I_TARGET")]
    pub i_target: Vec<ITarget>, // ref here is an "ID" used in other parts of the interaction
    /// Specifies what the interaction does to the targets. Multiple `[I_EFFECT]`s may be specified in
    /// a single interaction, and the same type may be used more than once.
    #[serde(alias = "I_EFFECT")]
    pub i_effect: Vec<IEffect>,
    /// Indicates that this is a generated interaction. Cannot be specified in user-defined raws.
    #[serde(alias = "GENERATED")]
    pub generated: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ISource {
    /// Argument 1 of `[I_SOURCE:...]`
    #[serde(alias = "I_SOURCE")]
    pub reference: Option<SourceEnum>,
    /// Describes what the interaction did to a historical figure; this is displayed in legends mode
    /// following the name of the historical figure who performed the interaction and preceding the
    /// name of the targeted historical figure (or, in the case of `[I_SOURCE:INGESTION]`, the
    /// historical figure from whom the consumed material was extracted).
    ///
    /// You must include a space at the start of the string for the game to format it properly.
    ///
    /// Example: `[IS_HIST_STRING_1: cursed]`
    #[serde(alias = "IS_HIST_STRING_1")]
    pub is_hist_string_1: Option<String>,
    /// Describes what the interaction did to a historical figure; this is displayed in legends mode
    /// after the name of the historical figure who was targeted by the interaction. In the case of
    /// `[I_SOURCE:INGESTION]`, it is displayed after the name of the historical figure from whom
    /// the consumed material was extracted.
    ///
    /// You must include a space at the start of the string for the game to format it properly.
    ///
    /// Example: `[IS_HIST_STRING_2: to assume the form of a lizard-like monster every full moon]`
    #[serde(alias = "IS_HIST_STRING_2")]
    pub is_hist_string_2: Option<String>,
    /// Displayed as an announcement when the interaction is carried out during play. The text
    /// follows the name of the target unit, and is preceded by `IS_TRIGGER_STRING_SECOND` or
    /// `IS_TRIGGER_STRING_THIRD`.
    ///
    /// May be limited to `[I_SOURCE:DEITY]` and `[I_SOURCE:EXPERIMENT]` interactions at present;
    /// this still needs to be tested.
    ///
    /// You must include a space at the start of the string for the game to format it properly.
    ///
    /// Example: `[IS_TRIGGER_STRING: been infected with a contagious ghoulish condition]`
    #[serde(alias = "IS_TRIGGER_STRING")]
    pub is_trigger_string: Option<String>,
    /// Presented before the `IS_TRIGGER_STRING` when describing the event in the second person.
    ///
    /// You must include a space at the start of the string for the game to format it properly.
    ///
    /// Example: `[IS_TRIGGER_STRING_SECOND: have]`
    #[serde(alias = "IS_TRIGGER_STRING_SECOND")]
    pub is_trigger_string_second: Option<String>,
    /// Presented before the `IS_TRIGGER_STRING` when describing the event in the third person.
    ///
    /// You must include a space at the start of the string for the game to format it properly.
    ///
    /// Example: `[IS_TRIGGER_STRING_THIRD: has]`
    #[serde(alias = "IS_TRIGGER_STRING_THIRD")]
    pub is_trigger_string_third: Option<String>,
    /// Generally used with `[I_SOURCE:SECRET]` interactions to describe what the secret is about
    /// (though it may be used to name any `I_SOURCE`). This name is engraved onto the appropriate
    /// secret-containing slabs from worldgen, and is used in legends mode when describing the
    /// secret being learnt by historical figures.
    ///
    /// Example: `[IS_NAME:the secrets of life and death]`
    #[serde(alias = "IS_NAME")]
    pub is_name: Option<String>,
    /// Indicates the sphere to which this secret pertains. Only one sphere can be defined for each
    /// `[I_SOURCE:SECRET]` token, so several `[I_SOURCE:SECRET]` tokens are required to make a
    /// secret belong to multiple spheres.
    #[serde(alias = "IS_SPHERE")]
    pub is_sphere: Option<SphereEnum>,
    /// Indicates why somebody would want to learn the secret. It has many allowed values currently,
    /// but only `IMMORTALITY` will result in a secret being pursued during world-gen.
    #[serde(alias = "IS_SECRET_GOAL")]
    pub is_secret_goal: Vec<SecretGoalEnum>,
    /// Indicates how the secret can be learned.
    #[serde(alias = "IS_SECRET")]
    pub is_secret: Vec<(SecretLearnMethodEnum, Option<(String, String)>)>, // TODO: the strings are filepaths, add a new type for filepaths
    /// Indicates why a deity would choose to perform this interaction.
    #[serde(alias = "IS_USAGE_HINT")]
    pub is_usage_hint: Option<LimitedUsageHintEnum>,
    /// Indicates what types of regions are capable of performing this interaction. This token may
    /// be specified several times per `I_SOURCE` to permit multiple terrain/alignment types.
    #[serde(alias = "IS_REGION")]
    pub is_region: Vec<RegionTypeEnum>,
    /// When used with `[I_SOURCE:REGION]`, determines how likely it is for the region(s) specified
    /// via `[IS_REGION]` to possess this interaction.
    ///
    /// Note: it appears that regions aren't allowed to possess more than a single regional
    /// interaction at present.
    #[serde(alias = "IS_FREQUENCY")]
    pub is_frequency: Option<i32>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ITarget {
    /// Arguments of `[I_TARGET:...]`
    #[serde(alias = "I_TARGET")]
    pub reference: Option<(Reference, TargetTypeEnum)>,
    /// This is often included after `[I_TARGET]` token to add more detail about the target.
    #[serde(alias = "IT_LOCATION")]
    pub it_location: Option<TargetLocationEnum>,
    /// Tells the adventure mode player what they should be selecting. If not specified, the player
    /// will only be able to target themselves.
    #[serde(alias = "IT_MANUAL_INPUT")]
    pub it_manual_input: Option<String>,
    /// Specifies specific creatures the interaction can target.
    #[serde(alias = "IT_AFFECTED_CREATURE")]
    pub it_affected_creature: Vec<(ReferenceTo<CreatureToken>, Reference)>, // ref here is creature caste
    /// Specifies creature classes the interaction can target.
    #[serde(alias = "IT_AFFECTED_CLASS")]
    pub it_affected_class: Vec<Reference>,
    /// Specifies specific creatures the interaction cannot target.
    #[serde(alias = "IT_IMMUNE_CREATURE")]
    pub it_immune_creature: Vec<(ReferenceTo<CreatureToken>, Reference)>, // ref here is creature caste
    /// Specifies creature classes the interaction cannot target.
    #[serde(alias = "IT_IMMUNE_CLASS")]
    pub it_immune_class: Vec<Reference>,
    /// Indicates that the target must have the specified property.
    #[serde(alias = "IT_REQUIRES")]
    pub it_requires: Vec<(Vec<TargetPropertyEnum>,)>,
    /// Indicates that the target must not have the specified property.
    #[serde(alias = "IT_FORBIDDEN")]
    pub it_forbidden: Vec<(Vec<TargetPropertyEnum>,)>,
    /// Prevents the interaction from targeting a creature that's already under the effect of the
    /// same interaction.
    #[serde(alias = "IT_CANNOT_TARGET_IF_ALREADY_AFFECTED")]
    pub it_cannot_target_if_already_affected: Flag,
    /// Prevents the interaction from targeting a creature under the effects of a syndrome having
    /// the specified `SYN_CLASS` value.
    #[serde(alias = "IT_CANNOT_HAVE_SYNDROME_CLASS")]
    pub it_cannot_have_syndrome_class: Vec<Reference>,
    /// Specifies the type of material the interaction targets; currently only used for
    /// `MATERIAL_EMISSION` interaction effects.
    #[serde(alias = "IT_MATERIAL")]
    pub it_material: Option<InteractionMaterialEmissionTypeTokenArg>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IEffect {
    /// Argument 1 of `[I_EFFECT:...]`
    #[serde(alias = "I_EFFECT")]
    pub reference: Option<EffectEnum>, // TODO semantic: restrict allowed tokens based on effect
    /// List of syndromes added by the interaction.
    #[serde(alias = "SYNDROME")]
    pub syndrome: Vec<SyndromeToken>,
    /// Allows the interaction effect to be applied directly to newly spawned creatures in arena
    /// mode. The specified name is used to represent it within the creature creation effects list.
    #[serde(alias = "IE_ARENA_NAME")]
    pub ie_arena_name: Option<String>,
    /// Specifies which `I_TARGET` a particular interaction effect will be applied to. For example,
    /// in an interaction with the token `[I_TARGET:B:CREATURE]`, 'B' is the `ID` used to indicate
    /// this target option. `[I_EFFECT:ADD_SYNDROME]` followed by `[IE_TARGET:B]` would therefore
    /// apply the syndrome to this target. Certain types of interaction effects require multiple
    /// `IE_TARGET` tokens in a specific order to function properly. A few effects do not require
    /// a target at all.
    #[serde(alias = "IE_TARGET")]
    pub ie_target: Vec<Reference>, // TODO some kind of semantic stuff using this I think
    /// Only appears to work with `[I_SOURCE:REGION]` interactions. Indicates that the effect
    /// happens intermittently and specifies roughly how often. Regional interactions aren't able to
    /// use effects which lack this token.
    ///
    /// `WEEKLY` is the only value that seems to work right now, despite `DAILY`, `MONTHLY` and
    /// `YEARLY` also existing in the string dump.
    #[serde(alias = "IE_INTERMITTENT")]
    pub ie_intermittent: Option<IntermittentFrequencyEnum>,
    /// Indicates that the effect happens immediately.
    #[serde(alias = "IE_IMMEDIATE")]
    pub ie_immediate: Flag,
    /// Prevents the interaction effect from manifesting unless the target is in a location which
    /// meets the specified criteria.
    ///
    /// Note: `NO_THICK_FOG` and `OUTSIDE` are accepted as valid location hints when specified with
    /// `IE_LOCATION`, but don't appear to work. It's possible that they're currently only
    /// implemented for use with `[CDI:LOCATION_HINT]`.
    #[serde(alias = "IE_LOCATION")]
    pub ie_location: Option<EffectLocationEnum>,
    /// Indicates what type of weather is added.
    #[serde(alias = "IE_ADD_WEATHER")]
    pub ie_add_weather: Option<WeatherEnum>,
    /// Indicates what type of weather is removed.
    #[serde(alias = "IE_REMOVE_WEATHER")]
    pub ie_remove_weather: Option<WeatherEnum>,
    /// `[IE_GRIME_LEVEL:2]` appears in the default cleaning interaction, and may indicate amount of
    /// grime cleaned, but this isn't clear.
    #[serde(alias = "IE_GRIME_LEVEL")]
    pub ie_grime_level: Option<u32>,
    /// When a creature cleans off a contaminant which is associated with a syndrome, the syndrome
    /// will be contracted if it has a matching trigger flag. This is what enables cats to become
    /// slightly inebriated when licking off alcohol. `SYN_INGESTED` appears to be the only syndrome
    /// trigger flag that works in this context.
    #[serde(alias = "IE_SYNDROME_TAG")]
    pub ie_syndrome_tag: Option<SynTransmittionMethodEnum>,
    /// Indicates the amount of force that the target will be propelled with.
    #[serde(alias = "IE_PROPEL_FORCE")]
    pub ie_propel_force: Option<u32>,
    /// Defines what item will be created.
    #[serde(alias = "IE_ITEM")]
    pub ie_item: Option<(u8, u16, Reference, MaterialTokenArg)>, // ref here is an item token
    /// Defines what quality the created item shall have. Can either be specified in the form of a
    /// single, fixed quality (it seems that `ARTIFACT` can only be used in this manner), or a
    /// minimum and maximum level (in which case the quality is selected randomly).
    ///
    /// Valid values (numerals only except for `ARTIFACT`) are:
    /// - 0 = ordinary
    /// - 1 = well-crafted
    /// - 2 = finely crafted
    /// - 3 = superior quality
    /// - 4 = exceptional
    /// - 5 = masterwork
    /// - `ARTIFACT`
    #[serde(alias = "IE_ITEM_QUALITY")]
    pub ie_item_quality: Option<Choose<u8, ArtifactEnum>>,
    /// Defines a fixed quality level which the affected item(s) will be set to (decreasing or
    /// increasing in quality as necessary).
    ///
    /// Valid values are:
    /// - 0 = ordinary
    /// - 1 = well-crafted
    /// - 2 = finely crafted
    /// - 3 = superior quality
    /// - 4 = exceptional
    /// - 5 = masterwork
    #[serde(alias = "IE_SET_QUALITY")]
    pub ie_set_quality: Option<u8>,
    /// Determines how much the quality of the item(s) will be changed. For instance, improving a
    /// well-crafted `-item-` (quality level 1) by 2 will turn it into a superior-quality `*item*`
    /// (quality level 3). A negative value can be used to decrease quality. Quality cannot be
    /// increased beyond level 5 (masterwork) or decreased below level 0 (ordinary).
    #[serde(alias = "IE_CHANGE_QUALITY")]
    pub ie_change_quality: Option<i8>,
    /// Indicates which specific creature and caste will be created when using this interaction.
    /// `ANY` can be used in place of a specific caste token. Only one `[CREATURE]` may currently
    /// be specified per interaction effect.
    #[serde(alias = "CREATURE")]
    pub creature: Option<(ReferenceTo<CreatureToken>, Reference)>, // this ref is `ANY`, `ALL`, or a caste for the creature
    /// When this token is added to a random creature summoning effect, it narrows down the
    /// selection to creatures which have the specified creature flag. This token may be used
    /// multiple times per interaction effect; creatures which lack any of the indicated flags will
    /// never be summoned.
    #[serde(alias = "IE_CREATURE_FLAG")]
    pub ie_creature_flag: Vec<CreatureFlagEnum>,
    /// When this token is added to a random creature summoning effect, any creature with the
    /// specified creature flag will be excluded from the selection. This token may be used multiple
    /// times per interaction effect; creatures which possess any of the indicated flags will never
    /// be summoned.
    #[serde(alias = "IE_FORBIDDEN_CREATURE_FLAG")]
    pub ie_forbidden_creature_flag: Vec<CreatureFlagEnum>,
    /// When this token is added to a random creature summoning effect, it narrows down the
    /// selection to creatures which have the specified caste flag. This token may be used multiple
    /// times per interaction effect; creatures which lack any of the indicated flags will never be
    /// summoned.
    #[serde(alias = "IE_CREATURE_CASTE_FLAG")]
    pub ie_creature_caste_flag: Vec<CasteFlagEnum>,
    /// When this token is added to a random creature summoning effect, it excludes any creature
    /// with the specified caste flag. This token may be used multiple times per interaction effect;
    /// creatures which possess any of the indicated flags will never be summoned.
    #[serde(alias = "IE_FORBIDDEN_CREATURE_CASTE_FLAG")]
    pub ie_forbidden_creature_caste_flag: Vec<CasteFlagEnum>,
    /// When this token is added to a random creature summoning effect, it narrows down the
    /// selection to creatures which have at least one gait with an `<energy expenditure>` of 0 and a
    /// `<max speed>` less than or equal to the specified `<minimum gait speed>` ("less than" because
    /// lower is faster in the scale used for gait speed).
    #[serde(alias = "IE_HAVE_FAST_EFFORTLESS_GAIT_SPEED")]
    pub ie_have_fast_effortless_gait_speed: Option<u32>,
    /// When this token is added to a random creature summoning effect, it excludes any creatures
    /// which have at least one gait with an `<energy expenditure>` of 0 and a `<max speed>` value less
    /// than or equal to the specified `<maximum gait speed>` (note that larger values are slower in
    /// the scale used for gait speed).
    #[serde(alias = "IE_ALL_SLOW_EFFORTLESS_GAIT_SPEED")]
    pub ie_all_slow_effortless_gait_speed: Option<u32>,
    /// The summoned unit vanishes in a puff of smoke once a certain amount of time has elapsed. The
    /// time limit is a randomly selected number of ticks within the specified minimum-maximum time
    /// range. The unit will persist indefinitely if this token is omitted.
    #[serde(alias = "IE_TIME_RANGE")]
    pub ie_time_range: Option<(u32, u32)>,
    /// Makes the summoned unit behave as a pet of the unit who performed the summoning interaction.
    #[serde(alias = "IE_MAKE_PET_IF_POSSIBLE")]
    pub ie_make_pet_if_possible: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InteractionMaterialEmissionTypeTokenArg {
    /// Indicates the emission details should be obtained from `[CDI:MATERIAL]` or `[CDI:FLOW]`.
    ContextMaterial,
    /// The emission will consist of the specified special flow type.
    Flow(BreathFlowEnum),
    /// The emission will consist of the specified material dispersed in the specified manner.
    Material((MaterialTokenArg, BreathMaterialEnum)),
}
impl Default for InteractionMaterialEmissionTypeTokenArg {
    fn default() -> Self {
        Self::ContextMaterial
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SourceEnum {
    /// Specifies that the interaction may be used in conjunction with `[CAN_DO_INTERACTION]` and
    /// `[CE_CAN_DO_INTERACTION]`, but it isn't actually necessary for this. It might exist simply
    /// to allow for the inclusion of `IS_` tokens (detailed below) to be applied when the
    /// interaction is used in this context.
    #[serde(alias = "CREATURE_ACTION")]
    CreatureAction,
    /// Allows the interaction to be used in conjunction with `[SPECIALATTACK_INTERACTION]` and
    /// `[CE_SPECIAL_ATTACK_INTERACTION]`.
    #[serde(alias = "ATTACK")]
    Attack,
    /// Allows the interaction to be used in conjunction with `[CE_BODY_MAT_INTERACTION]`.
    #[serde(alias = "INGESTION")]
    Ingestion,
    /// Allows the interaction to be inflicted upon mortals by the gods, for reasons dictated by
    /// `[IS_USAGE_HINT]`.
    #[serde(alias = "DEITY")]
    Deity,
    /// Allows the interaction to act as a secret which can be learnt and passed on to others, as
    /// specified via `[IS_SECRET]`. Appropriate interaction effects with a creature target will be
    /// applied to individuals who learn the secret. It is possible to set restrictions on who may
    /// learn the secret by using creature target tokens as described below. Also see
    /// `[IS_SECRET_GOAL]` and `[IS_SPHERE]`.
    #[serde(alias = "SECRET")]
    Secret,
    /// Allows the interaction to take place spontaneously in regions specified using `[IS_REGION]`.
    /// Also see `[IS_FREQUENCY]` and `[IE_INTERMITTENT]`.
    #[serde(alias = "REGION")]
    Region,
    /// Allows the interaction to take place spontaneously in disturbed tombs
    #[serde(alias = "DISTURBANCE")]
    Disturbance,
    /// Allows the interaction to take place spontaneously in curious underground structures (which
    /// have since been removed in `v0.40.01`)
    #[serde(alias = "UNDERGROUND_SPECIAL")]
    UndergroundSpecial,
    /// Allows the interaction to be used when experimenting on creatures.
    #[serde(alias = "EXPERIMENT")]
    Experiment,
}
impl Default for SourceEnum {
    fn default() -> Self {
        Self::CreatureAction
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SecretGoalEnum {
    /// Ingame description: Unknown; if you know the ingame description, please open an issue on the
    /// issue tracker, or tell us on the Discord server.
    ///
    /// Gameplay effects: Unknown, possibly none.
    #[serde(alias = "STAY_ALIVE")]
    StayAlive,
    /// Ingame description: Unknown; if you know the ingame description, please open an issue on the
    /// issue tracker, or tell us on the Discord server.
    ///
    /// Gameplay effects: Unknown, possibly none.
    #[serde(alias = "MAINTAIN_ENTITY_STATUS")]
    MaintainEntityStatus,
    /// Ingame description: "dreams of raising a family"
    ///
    /// Gameplay effects: Goal completed upon giving birth or fathering an infant.
    #[serde(alias = "START_A_FAMILY")]
    StartAFamily,
    /// Ingame description: "dreams of ruling the world"
    ///
    /// Gameplay effects: Unknown, possibly none.
    #[serde(alias = "RULE_THE_WORLD")]
    RuleTheWorld,
    /// Ingame description: "dreams of creating a great work of art"
    ///
    /// Gameplay effects: Goal completed upon creation of Artifact or Masterpiece.
    #[serde(alias = "CREATE_A_GREAT_WORK_OF_ART")]
    CreateAGreatWorkOfArt,
    /// Ingame description: "dreams of crafting a masterwork someday "
    ///
    /// Gameplay effects: Goal completed upon creation of Artifact or Masterpiece.
    #[serde(alias = "CRAFT_A_MASTERWORK")]
    CraftAMasterwork,
    /// Ingame description: "dreams of bringing lasting peace to the world"
    ///
    /// Gameplay effects: Unknown, possibly none.
    #[serde(alias = "BRING_PEACE_TO_THE_WORLD")]
    BringPeaceToTheWorld,
    /// Ingame description: "dreams of becoming a legendary warrior"
    ///
    /// Gameplay effects: Unknown, possibly none.
    #[serde(alias = "BECOME_A_LEGENDARY_WARRIOR")]
    BecomeALegendaryWarrior,
    /// Ingame description: "dreams of mastering a skill"
    ///
    /// Gameplay effects: Goal completed upon reaching Legendary skill status.
    #[serde(alias = "MASTER_A_SKILL")]
    MasterASkill,
    /// Ingame description: "dreams of falling in love"
    ///
    /// Gameplay effects: Unknown, possibly none.
    #[serde(alias = "FALL_IN_LOVE")]
    FallInLove,
    /// Ingame description: "dreams of seeing the great natural places of the world"
    ///
    /// Gameplay effects: Unknown, possibly none.
    #[serde(alias = "SEE_THE_GREAT_NATURAL_SITES")]
    SeeTheGreatNaturalSites,
    /// Ingame description: "has become obsessed with his/her own mortality"
    ///
    /// Gameplay effects: Leads to [necromancy](https://dwarffortresswiki.org/index.php/Necromancer).
    #[serde(alias = "IMMORTALITY")]
    Immortality,
}
impl Default for SecretGoalEnum {
    fn default() -> Self {
        Self::StayAlive
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SecretLearnMethodEnum {
    /// Gods may gift the secret to their worshippers. Secrets with `[IS_SPHERE]` specified may only
    /// be granted by gods who have at least one matching sphere.
    #[serde(alias = "SUPERNATURAL_LEARNING_POSSIBLE")]
    SupernaturalLearningPossible,
    /// The secret can be researched by mundane means. This doesn't do anything at present
    /// ([source](http://www.bay12forums.com/smf/index.php?topic=169696.msg8243222#msg8243222)).
    #[serde(alias = "MUNDANE_RESEARCH_POSSIBLE")]
    MundaneResearchPossible,
    /// The secret can be taught to apprentices
    #[serde(alias = "MUNDANE_TEACHING_POSSIBLE")]
    MundaneTeachingPossible,
    /// The secret can be written in books with the specified title. If this tag is present, a slab
    /// will be created upon learning the secret by supernatural means.
    #[serde(alias = "MUNDANE_RECORDING_POSSIBLE")]
    MundaneRecordingPossible,
}
impl Default for SecretLearnMethodEnum {
    fn default() -> Self {
        Self::SupernaturalLearningPossible
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum LimitedUsageHintEnum {
    /// Used in divination dice blessings. Targets the roller.
    #[serde(alias = "MINOR_BLESSING")]
    MinorBlessing,
    /// Used in divination dice blessings. Targets the roller.
    #[serde(alias = "MEDIUM_BLESSING")]
    MediumBlessing,
    /// Used in divination dice curses. Targets the roller.
    #[serde(alias = "MINOR_CURSE")]
    MinorCurse,
    /// Used in divination dice curses. Targets the roller.
    #[serde(alias = "MEDIUM_CURSE")]
    MediumCurse,
    /// Used in disturbance and deity curses. Targets the tomb disturber/temple defiler.
    #[serde(alias = "MAJOR_CURSE")]
    MajorCurse,
}
impl Default for LimitedUsageHintEnum {
    fn default() -> Self {
        Self::MinorBlessing
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum RegionTypeEnum {
    #[serde(alias = "ANY")]
    Any,
    #[serde(alias = "DESERT")]
    Desert,
    #[serde(alias = "FOREST")]
    Forest,
    #[serde(alias = "GLACIER")]
    Glacier,
    #[serde(alias = "GRASSLAND")]
    Grassland,
    #[serde(alias = "HILLS")]
    Hills,
    #[serde(alias = "LAKE")]
    Lake,
    #[serde(alias = "MOUNTAINS")]
    Mountains,
    #[serde(alias = "OCEAN")]
    Ocean,
    #[serde(alias = "SWAMP")]
    Swamp,
    #[serde(alias = "TUNDRA")]
    Tundra,
    /// Permits the interaction to occur in all regions which meet alignment specifications.
    #[serde(alias = "ANY_TERRAIN")]
    AnyTerrain,
    #[serde(alias = "NORMAL_ALLOWED")]
    NormalAllowed,
    #[serde(alias = "EVIL_ALLOWED")]
    EvilAllowed,
    #[serde(alias = "GOOD_ALLOWED")]
    GoodAllowed,
    #[serde(alias = "SAVAGE_ALLOWED")]
    SavageAllowed,
    #[serde(alias = "EVIL_ONLY")]
    EvilOnly,
    #[serde(alias = "GOOD_ONLY")]
    GoodOnly,
    #[serde(alias = "SAVAGE_ONLY")]
    SavageOnly,
}
impl Default for RegionTypeEnum {
    fn default() -> Self {
        Self::Any
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TargetTypeEnum {
    /// The target is a `CORPSE` or `CORPSEPIECE` item.
    #[serde(alias = "CORPSE")]
    Corpse,
    /// The target is a unit.
    #[serde(alias = "CREATURE")]
    Creature,
    /// This is a valid target for use in `[I_EFFECT:MATERIAL_EMISSION]` interaction effects, and is
    /// used to set the material or flow type of the emission. This information, in turn, is either
    /// obtained from an `[IT_MATERIAL]` token or by using `[IT_LOCATION:CONTEXT_MATERIAL]`. Using
    /// the latter implies that the precise emission info will be provided when defining how an
    /// interaction user can use the interaction in question via `CDI` tokens, enabling one to
    /// create 'template' material emission interactions such as the `MATERIAL_EMISSION` and
    /// `MATERIAL_EMISSION_WITH_HIDE` interactions included in the vanilla raws.
    #[serde(alias = "MATERIAL")]
    Material,
    /// The target is a local map tile. If used with `[IT_LOCATION:CONTEXT_CREATURE_OR_LOCATION]`,
    /// creatures at the target tile are also valid targets.
    #[serde(alias = "LOCATION")]
    Location,
}
impl Default for TargetTypeEnum {
    fn default() -> Self {
        Self::Corpse
    }
}

// TODO semantic analysis to limit which values are allowed based on the parent `I_TARGET`
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TargetLocationEnum {
    /// Used with `CREATURE` to target the whole unit.
    ContextCreature,
    /// Used with `CREATURE` to target the body part specified in `[CDI:BP_REQUIRED]`.
    ContextBp,
    /// Used with `LOCATION` to target only a tile.
    ContextLocation,
    /// Used with `LOCATION` to allow for targeting of both creatures and tiles.
    ContextCreatureOrLocation,
    /// Used with `CORPSE`.
    ContextItem,
    /// Can only be used by `[I_SOURCE:REGION]` interactions.
    ContextRegion,
    /// Used with `MATERIAL` if you want an `[I_EFFECT:MATERIAL_EMISSION]` to obtain the emission
    /// material/flow type from `[CDI:MATERIAL]` or `[CDI:FLOW]`.
    ContextMaterial,
    /// Used with `LOCATION`. Targets a location from somewhere random within a number of squares
    /// from another `LOCATION` target specified by its target ID. For example,
    /// `[I_TARGET:B:LOCATION]` with `[IT_LOCATION:RANDOM_NEARBY_LOCATION:A:5]` will randomly select
    /// a tile lying somewhere within a radius of 5 tiles from `[I_TARGET:A:LOCATION]`. A walkable
    /// path between the two locations must exist.
    RandomNearbyLocation((Reference, i32)),
}
impl Default for TargetLocationEnum {
    fn default() -> Self {
        Self::ContextCreature
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum EffectEnum {
    /// Adds one or more syndromes to a valid creature target. You must specify the syndrome details
    /// just below this interaction effect using the `[SYNDROME]` tag followed by the relevant
    /// syndrome tokens. See [here](https://dwarffortresswiki.org/index.php/Syndrome#The_anatomy_of_a_syndrome)
    /// for more information.
    #[serde(alias = "ADD_SYNDROME")]
    AddSyndrome,
    /// Raises the target corpse/bodypart as an undead unit. The zombie will always be hostile to
    /// life and will retain no information about its original personality/loyalties. Syndromes can
    /// also be specified within this tag. If a regional interaction contains this effect, affected
    /// regions will have undead wildlife.
    #[serde(alias = "ANIMATE")]
    Animate,
    /// Takes a target corpse and returns the creature to life. This can be used on parts that are
    /// not `FIT_FOR_RESURRECTION`, but only the main part (with an `UPPERBODY` attached) will
    /// remain loyal to its original faction. Syndromes can also be specified within this tag.
    #[serde(alias = "RESURRECT")]
    Resurrect,
    /// Removes contaminants from a valid creature target. See `[IE_GRIME_LEVEL]` and
    /// `[IE_SYNDROME_TAG]`.
    #[serde(alias = "CLEAN")]
    Clean,
    /// Causes the creatures to touch.
    #[serde(alias = "CONTACT")]
    Contact,
    /// Causes a particular material to be emitted. Used by evil weather and the `MATERIAL_EMISSION`
    /// interaction.
    #[serde(alias = "MATERIAL_EMISSION")]
    MaterialEmission,
    /// Allows the creature to hide even if another creature can see it.
    #[serde(alias = "HIDE")]
    Hide,
    /// Creates an item as described by `[IE_ITEM]` and `[IE_ITEM_QUALITY]`.
    #[serde(alias = "CREATE_ITEM")]
    CreateItem,
    /// Alters an item's quality level as indicated by either `[IE_CHANGE_QUALITY]` or
    /// `[IE_SET_QUALITY]`. When targeting a unit, all items equipped by that unit will be affected.
    #[serde(alias = "CHANGE_ITEM_QUALITY")]
    ChangeItemQuality,
    /// Creates a new unit at the target. The type of unit can either be specified using the
    /// `[CREATURE]` token, or made to be randomly selected as indicated by a variety of flag-based
    /// tokens: `[IE_CREATURE_FLAG]`, `[IE_FORBIDDEN_CREATURE_FLAG]`, `[IE_CREATURE_CASTE_FLAG]`,
    /// `[IE_FORBIDDEN_CREATURE_CASTE_FLAG]`, `[IE_HAVE_FAST_EFFORTLESS_GAIT_SPEED]` and/or
    /// `[IE_ALL_SLOW_EFFORTLESS_GAIT_SPEED]`. See also `[IE_TIME_RANGE]` and
    /// `[IE_MAKE_PET_IF_POSSIBLE]`.
    #[serde(alias = "SUMMON_UNIT")]
    SummonUnit,
    /// Applies a force specified using `[IE_PROPEL_FORCE]` to a unit to knock it back.
    #[serde(alias = "PROPEL_UNIT")]
    PropelUnit,
    /// Changes the weather as specified by `[IE_ADD_WEATHER]` and/or `[IE_REMOVE_WEATHER]`.
    #[serde(alias = "CHANGE_WEATHER")]
    ChangeWeather,
    /// Present in version `0.47.01` and accepted as a valid `I_EFFECT` token, but does not have an
    /// effect currently.
    #[serde(alias = "RAISE_GHOST")]
    RaiseGhost,
}
impl Default for EffectEnum {
    fn default() -> Self {
        Self::AddSyndrome
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum IntermittentFrequencyEnum {
    #[serde(alias = "WEEKLY")]
    Weekly,
    #[serde(alias = "DAILY")] // TODO mark as broken, see #83
    Daily,
    #[serde(alias = "MONTHLY")] // TODO mark as broken, see #83
    Monthly,
    #[serde(alias = "YEARLY")] // TODO mark as broken, see #83
    Yearly,
}
impl Default for IntermittentFrequencyEnum {
    fn default() -> Self {
        Self::Weekly
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum WeatherEnum {
    #[serde(alias = "FOG_MIST")]
    FogMist,
    #[serde(alias = "FOG_NORMAL")]
    FogNormal,
    #[serde(alias = "FOG_THICK")]
    FogThick,
    #[serde(alias = "FRONT_WARM")]
    FrontWarm,
    #[serde(alias = "FRONT_COLD")]
    FrontCold,
    #[serde(alias = "FRONT_OCCLUDED")]
    FrontOccluded,
    #[serde(alias = "STRATUS_ALTO")]
    StratusAlto,
    #[serde(alias = "STRATUS_PROPER")]
    StratusProper,
    #[serde(alias = "STRATUS_NIMBUS")]
    StratusNimbus,
    #[serde(alias = "CUMULUS_MED")]
    CumulusMed,
    #[serde(alias = "CUMULUS_MULTI")]
    CumulusMulti,
    #[serde(alias = "CUMULUS_NIMBUS")]
    CumulusNimbus,
    #[serde(alias = "CIRRUS")]
    Cirrus,
}
impl Default for WeatherEnum {
    fn default() -> Self {
        Self::FogMist
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ArtifactEnum {
    #[serde(alias = "ARTIFACT")]
    Artifact,
}
impl Default for ArtifactEnum {
    fn default() -> Self {
        Self::Artifact
    }
}
