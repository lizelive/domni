use crate::core::{Choose, Reference, ReferenceTo};
use crate::structure::{
    BpCriteriaTokenArg, BreathFlowEnum, BreathMaterialEnum, EffectLocationEnum, InteractionToken,
    MaterialTokenArg, NotApplicableEnum,
};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum CdiTargetTypeEnum {
    /// The source needs to be able to see the target
    #[serde(alias = "LINE_OF_SIGHT")]
    LineOfSight,
    /// The source needs to be able to touch the target
    #[serde(alias = "TOUCHABLE")]
    Touchable,
    /// The target must be whoever disturbed the source (this is currently only relevant to mummies,
    /// allowing them to curse solely the unit who disturbed their resting place)
    #[serde(alias = "DISTURBER_ONLY")]
    DisturberOnly,
    /// The target can be the source
    #[serde(alias = "SELF_ALLOWED")]
    SelfAllowed,
    /// The target is required to be the source
    #[serde(alias = "SELF_ONLY")]
    SelfOnly,
}
impl Default for CdiTargetTypeEnum {
    fn default() -> Self {
        Self::LineOfSight
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum UsageHintEnum {
    /// Creatures will target 'friendly' creatures, usually at random.
    #[serde(alias = "GREETING")]
    Greeting,
    /// Targets enemy creatures in combat. If the interaction specifies `SELF_ONLY`, CPU-controlled
    /// creatures will never use it.
    #[serde(alias = "ATTACK")]
    Attack,
    /// Used in combat. Creature will target itself.
    #[serde(alias = "DEFEND")]
    Defend,
    /// Used when fleeing an enemy. Creature will target itself.
    #[serde(alias = "FLEEING")]
    Fleeing,
    /// Creature targets itself when its body is covered with contaminants.
    #[serde(alias = "CLEAN_SELF")]
    CleanSelf,
    /// As above, but targets other friendly units instead.
    #[serde(alias = "CLEAN_FRIEND")]
    CleanFriend,
    /// Used when a creature is expressing contempt (for example, to a murderer). This is used in
    /// the default spitting interaction, for example.
    #[serde(alias = "NEGATIVE_SOCIAL_RESPONSE")]
    NegativeSocialResponse,
    /// This is also used in the default spitting interaction, and is presumably used in a similar
    /// context.
    #[serde(alias = "TORMENT")]
    Torment,
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
impl Default for UsageHintEnum {
    fn default() -> Self {
        Self::Greeting
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CdiTokenArg {
    /// Specifies which interaction can be performed. This is only needed following
    /// `CE_CAN_DO_INTERACTION`; there's no need to include this after `CAN_DO_INTERACTION`
    /// as the latter allows you to specify the interaction directly.
    Interaction(ReferenceTo<InteractionToken>),
    /// Specifies the name of the interaction as it will appear on the adventure mode
    /// 'powers/abilities' menu.
    AdvName(String),
    /// Specifies how the creature chooses what to target. Target `ID` refers to an `I_TARGET`
    /// defined in the interaction itself. Multiple target types can be specified. If no target is
    /// specified, creature will target any available target within range, even through walls.
    Target((Reference, Vec<CdiTargetTypeEnum>)),
    /// Determines the maximum distance from the interaction user (in tiles) at which something
    /// can be considered a valid target. For `SOLID_GLOB`, `SHARP_ROCK`, `LIQUID_GLOB` and
    /// `FIREBALL` breath attacks, also determines how far the projectiles can fly before falling
    /// to the ground.
    TargetRange((Reference, u32)),
    /// Specifies the maximum number of things that can be selected for a particular `I_TARGET`.
    MaxTargetNumber(Choose<(Reference, u32), u32>),
    /// Prevents CPU-controlled creatures from using the interaction unless they are in a location
    /// which meets the specified criteria.
    LocationHint(EffectLocationEnum),
    /// Indicates when and how CPU-controlled creatures will use the interaction. If no hint is
    /// specified, the interaction will be used whenever a valid target is available.
    /// Multiple usage hints may be specified.
    UsageHint(UsageHintEnum),
    /// The creature must wait the specified number of ticks before being able to use the
    /// interaction again. The delay defaults to 20 ticks if this is omitted. Setting it to 0
    /// removes the delay altogether.
    ///
    /// Note: A `WAIT_PERIOD` of 10 is 10 ticks long in fortress mode, but only 1 tick long in
    /// adventurer mode.
    WaitPeriod(u32),
    /// Indicates that performing the interaction doesn't take any time.
    FreeAction,
    /// Indicates that a particular body part must be present in order to perform the interaction.
    /// Criteria are `BY_CATEGORY:category`, `BY_TYPE:type` (`GRASP`, for example), or `BY_TOKEN:token`
    /// (uses the body part `ID`).
    BpRequired(BpCriteriaTokenArg),
    /// Only creatures that can speak will be able to use the interaction. Might also be needed
    /// for `VERBAL_SPEECH`.
    Verbal,
    /// Specifies what the creature says when they perform the interaction. Filename path is
    /// relative to /data/speech.
    VerbalSpeech(String),
    /// Presumably allows two creatures with the same interaction to use it on each other
    /// simultaneously, for example, cats cleaning each other.
    CanBeMutual,
    /// When a creature uses the interaction, a combat report message will be displayed in the
    /// form:
    ///
    /// `[interaction user(s)] [VERB text] [target creature (if applicable)]`
    ///
    /// The 'self' text is presented when describing the interaction in the second person (that
    /// is, when the interaction is carried out by the player character in adventure mode), the
    /// 'other' text is presented when describing it in the third person, and the 'mutual' text is
    /// presented when the interaction is carried out in a mutual fashion.
    ///
    /// Examples of this token being used:
    /// - `[CDI:VERB:lick:licks:lick each other]`
    /// - `[CDI:VERB:gesture:gestures:NA]`
    Verb((String, String, Choose<NotApplicableEnum, String>)),
    /// **This token's purpose is unknown; if you know what it does,
    /// please open an issue on the issue tracker.**
    VerbReverse,
    /// When a creature uses the interaction, a message will display, describing the target as
    /// doing this.
    ///
    /// The first argument is the message when the effect is applied to onself, the
    /// second is when describing another creature using the interaction, for example:
    /// `[CDI:TARGET_VERB:shudder and begin to move:shudders and begins to move]`
    TargetVerb((String, String)), // TODO: the second one MIGHT possibly be `NotApplicableEnum`, haven't seen it anywhere
    /// Can be used with interactions that have `[I_EFFECT:MATERIAL_EMISSION]` and
    /// `[IT_LOCATION:CONTEXT_MATERIAL]` (or `[IT_MATERIAL:CONTEXT_MATERIAL]`) to make the
    /// interaction emit a special flow type.
    Flow(BreathFlowEnum),
    /// Can be used with interactions that have `[I_EFFECT:MATERIAL_EMISSION]` and
    /// `[IT_LOCATION:CONTEXT_MATERIAL]` (or `[IT_MATERIAL:CONTEXT_MATERIAL]`) to make the
    /// interaction emit the specified material in the manner described by the breath attack token
    /// used.
    Material((MaterialTokenArg, BreathMaterialEnum)),
}
impl Default for CdiTokenArg {
    fn default() -> Self {
        Self::AdvName(String::default())
    }
}
