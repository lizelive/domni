use crate::core::{Reference, ReferenceTo};
use crate::structure::{
    BodyAttributeEnum, BpCriteriaTokenArg, CasteFlagEnum, CreatureFlagEnum, CreatureToken,
    FacetEnum, InteractionToken, PersonalityTraitEnum, SoulAttributeEnum,
    SynTransmittionMethodEnum, TargetPropertyEnum,
};

use serde::{Deserialize, Serialize};

// region: CE args for CE_BODY_MAT_INTERACTION ====================================================
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CeBodyMatInteractionTokenArg {
    /// Used to specify what must be done with the body material to trigger the interaction.
    /// Multiple instances of this tag may be used to specify different valid transmission routes.
    ///
    /// However, even though it should accept `SYN_INGESTED`,`SYN_INJECTED`, `SYN_CONTACT`, or
    /// `SYN_INHALED`, `SYN_INGESTED` appears to be the only one that works at present.
    SyndromeTag(SynTransmittionMethodEnum),
    /// Used to specify which interaction is to be run. Appropriate interaction effects
    /// with a creature target (such as `ADD_SYNDROME`) will be inflicted upon the unit who interacts
    /// with the body material as specified above.
    ///
    /// Note that the linked interaction must have an `[I_SOURCE:INGESTION]` token for this to work.
    Interaction(ReferenceTo<InteractionToken>),
}
impl Default for CeBodyMatInteractionTokenArg {
    fn default() -> Self {
        Self::SyndromeTag(SynTransmittionMethodEnum::SynIngested)
    }
}

// region: CE args for CE_BODY_TRANSFORMATION =====================================================
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CeBodyTransformationTokenArg {
    /// This can be used to specify a specific target creature of a specific caste, to transform
    /// into (for example, `[CE:CREATURE:DWARF:FEMALE]`). `ALL` or `ANY` can be used in place of
    /// a specific caste to randomise the caste for every transformation.
    ///
    /// Do note that using `ALL` or `ANY` for transformation castes will make the creature transform
    /// over and over again with the interval depending on the `START` token. This can lead to an
    /// unending transformation loop.
    /// [However, there is a way to get around this](https://dwarffortresswiki.org/index.php/Syndrome#looping_problem).
    Creature((ReferenceTo<CreatureToken>, Reference)), // TODO: this ref is `ANY`, `ALL`, or a caste for the creature
    /// Narrows down the selection to creatures which have the specified creature flag. May be used
    /// multiple times per transformation effect; creatures which lack any of the indicated flags
    /// will never be transformed into.
    CreatureFlag(CreatureFlagEnum),
    /// Excludes creatures with the specified creature flag from the random selection pool. May be
    /// used multiple times per transformation effect; creatures which possess any of the indicated
    /// flags will never be transformed into.
    ForbiddenCreatureFlag(CreatureFlagEnum),
    /// Narrows down the selection to creatures which have the specified caste flag. May be used
    /// multiple times per transformation effect; creatures which lack any of the indicated flags
    /// will never be transformed into.
    CreatureCasteFlag(CasteFlagEnum),
    /// Excludes any creature with the specified caste flag from the random selection pool. May be
    /// used multiple times per transformation effect; creatures which possess any of the indicated
    /// flags will never be transformed into.
    ForbiddenCreatureCasteFlag(CasteFlagEnum),
    /// Narrows down the selection to creatures which have at least one gait with an `<energy expenditure>` of
    /// 0 and a `<max speed>` less than or equal to the specified `<minimum gait speed>` ("less than"
    /// because lower is faster in the scale used for gait speed). This is used in generated
    /// divination curses to prevent the player from being transformed into a creature that is
    /// frustratingly slow to play as.
    HaveFastEffortlessGaitSpeed(u32),
    /// Excludes any creatures which have at least one gait with an `<energy expenditure>` of 0 and
    /// a `<max speed>` value less than or equal to the specified `<maximum gait speed>` (note that
    /// larger values are slower in the scale used for gait speed).
    AllSlowEffortlessGaitSpeed(u32),
}
impl Default for CeBodyTransformationTokenArg {
    fn default() -> Self {
        Self::Creature((
            ReferenceTo::<CreatureToken>::default(),
            Reference::default(),
        ))
    }
}

// region: Full CE_X args =========================================================================
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeXTokenArg {
    /// The severity of the effect. Higher values appear to be worse, with `SEV:1000` `CE_NECROSIS`
    /// causing a part to near-instantly become rotten.
    pub sev: Option<u32>,
    /// The probability of the effect actually manifesting in the victim, as a percentage. 100 means
    /// always, 1 means a 1 in 100 chance.
    pub prob: Option<u8>,
    /// This tag overrides any `BP` tokens (so it is incompatible with them) and then forces the game to attempt to
    /// apply the effect to the limb that came into contact with the contagion - i.e. the part that
    /// was bitten by the creature injecting the syndrome material, or the one that was splattered
    /// by a contact contagion.
    ///
    /// If an effect can not be applied to the contacted limb (such as `IMPAIR_FUNCTION` on a
    /// non-organ) then this token makes the syndrome have no effect. This token also makes
    /// inhaled syndromes have no effect.
    pub localized: Option<()>,
    /// Specifies which body parts and tissues are to be affected by the syndrome. You can specify
    /// by category, by type, or by token, and then specify a specific tissue within that category,
    /// type or token.
    ///
    /// For example, if you wanted to target the lungs of a creature, you would use
    /// `BP:BY_CATEGORY:LUNG:ALL`. The syndrome would act on all bodyparts within the creature with
    /// the `CATEGORY` tag `LUNG`, and the `ALL` means it would affect all tissue layers. For another
    /// example, say you wanted to cause the skin to rot off a creature - you could use
    /// `BP:BY_CATEGORY:ALL:SKIN`, targeting the `SKIN` tissue on all bodyparts.
    ///
    /// This is one of the most powerful and useful aspects of the syndrome system, as it allows you
    /// to selectively target bodyparts relevant to the contagion, like lungs for coal dust inhalation,
    /// or the eyes for exposure to an acid gas. Multiple targets can be given in one syndrome by
    /// placing the `BP` tokens end to end.
    ///
    /// This tag is overidden by (and therefore incompatible with) `LOCALIZED`.
    pub bp: Vec<(BpCriteriaTokenArg, Reference)>,
    /// This tag makes the syndrome only affect tissue layers with the `VASCULAR` token.
    pub vascular_only: Option<()>,
    /// This tag makes the syndrome only affects tissue layers with the `MUSCULAR` token.
    pub muscular_only: Option<()>,
    /// This argument presumably causes the effects of the syndrome to scale with the size of the
    /// creature compared to the size of the dose of contagion they received, but has yet to be
    /// extensively tested.
    pub size_dilutes: Option<()>,
    /// This argument has yet to be tested fully, but presumably delays the onset of a syndrome
    /// according to the size of the victim.
    pub size_delays: Option<()>,
    /// Multiplies the duration of the syndrome by X in Fortress mode. If X is 144, syndrome
    /// will last the same amount of ticks in fortress and adventure mode.
    pub dwf_stretch: Option<u32>,
    /// Makes the symptom begin immediately rather than ramping up.
    pub abrupt: Option<()>,
    /// Can be hidden by a unit assuming a secret identity, such as a
    /// [vampire](https://dwarffortresswiki.org/index.php/Vampire).
    pub can_be_hidden: Option<()>,
    /// Determines if the effect can be hindered by the target creature's [disease resistance
    /// attribute](https://dwarffortresswiki.org/index.php/Attribute#Disease_Resistance). Without
    /// this argument, disease resistance is ignored.
    pub resistable: Option<()>,
    /// Determines the number of ticks after which this syndrome effect begins to affect a creature.
    pub start: Option<u32>,
    /// Determines the number of ticks this syndrome lasts.
    pub end: Option<u32>,
    /// Determines when this syndrome effect "peaks", reaching its highest severity.
    pub peak: Option<u32>,
}

// region: CE_X args with no target params ========================================================
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeXNoTargetTokenArg {
    /// The severity of the effect. Higher values appear to be worse, with `SEV:1000` `CE_NECROSIS`
    /// causing a part to near-instantly become rotten.
    pub sev: Option<u32>,
    /// The probability of the effect actually manifesting in the victim, as a percentage. 100 means
    /// always, 1 means a 1 in 100 chance.
    pub prob: Option<u8>,
    /// This argument presumably causes the effects of the syndrome to scale with the size of the
    /// creature compared to the size of the dose of contagion they received, but has yet to be
    /// extensively tested.
    pub size_dilutes: Option<()>,
    /// This argument has yet to be tested fully, but presumably delays the onset of a syndrome
    /// according to the size of the victim.
    pub size_delays: Option<()>,
    /// Multiplies the duration of the syndrome by X in Fortress mode. If X is 144, syndrome
    /// will last the same amount of ticks in fortress and adventure mode.
    pub dwf_stretch: Option<u32>,
    /// Makes the symptom begin immediately rather than ramping up.
    pub abrupt: Option<()>,
    /// Can be hidden by a unit assuming a secret identity, such as a
    /// [vampire](https://dwarffortresswiki.org/index.php/Vampire).
    pub can_be_hidden: Option<()>,
    /// Determines if the effect can be hindered by the target creature's [disease resistance
    /// attribute](https://dwarffortresswiki.org/index.php/Attribute#Disease_Resistance). Without
    /// this argument, disease resistance is ignored.
    pub resistable: Option<()>,
    /// Determines the number of ticks after which this syndrome effect begins to affect a creature.
    pub start: Option<u32>,
    /// Determines the number of ticks this syndrome lasts.
    pub end: Option<u32>,
    /// Determines when this syndrome effect "peaks", reaching its highest severity.
    pub peak: Option<u32>,
}

// region: CE_X args with no SEV or target parameters =============================================
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeXNoSevTokenArg {
    /// The probability of the effect actually manifesting in the victim, as a percentage. 100 means
    /// always, 1 means a 1 in 100 chance.
    pub prob: Option<u8>,
    /// This argument presumably causes the effects of the syndrome to scale with the size of the
    /// creature compared to the size of the dose of contagion they received, but has yet to be
    /// extensively tested.
    pub size_dilutes: Option<()>,
    /// This argument has yet to be tested fully, but presumably delays the onset of a syndrome
    /// according to the size of the victim.
    pub size_delays: Option<()>,
    /// Multiplies the duration of the syndrome by X in Fortress mode. If X is 144, syndrome
    /// will last the same amount of ticks in fortress and adventure mode.
    pub dwf_stretch: Option<u32>,
    /// Makes the symptom begin immediately rather than ramping up.
    pub abrupt: Option<()>,
    /// Can be hidden by a unit assuming a secret identity, such as a
    /// [vampire](https://dwarffortresswiki.org/index.php/Vampire).
    pub can_be_hidden: Option<()>,
    /// Determines if the effect can be hindered by the target creature's [disease resistance
    /// attribute](https://dwarffortresswiki.org/index.php/Attribute#Disease_Resistance). Without
    /// this argument, disease resistance is ignored.
    pub resistable: Option<()>,
    /// Determines the number of ticks after which this syndrome effect begins to affect a creature.
    pub start: Option<u32>,
    /// Determines the number of ticks this syndrome lasts.
    pub end: Option<u32>,
    /// Determines when this syndrome effect "peaks", reaching its highest severity.
    pub peak: Option<u32>,
}

// region: CE_CHANGE_PERSONALITY args =============================================================
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeChangePersonalityTokenArg {
    pub facets: Vec<(FacetEnum, PersonalityTraitEnum, i8)>,
    pub general_cex: Option<CeXNoSevTokenArg>,
}

// region: CE_(ADD/REMOVE)_TAG args ===============================================================
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeTagsTokenArg {
    pub tags: Vec<TargetPropertyEnum>,
    pub general_cex: Option<CeXNoSevTokenArg>,
}

// region: CE_PHYS_ATT_CHANGE args ================================================================
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CePhysAttChangeTokenArg {
    pub body_attributes: Vec<(BodyAttributeEnum, u32, u32)>,
    pub general_cex: Option<CeXNoSevTokenArg>,
}

// region: CE_MENT_ATT_CHANGE args ================================================================
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeMentAttChangeTokenArg {
    pub soul_attributes: Vec<(SoulAttributeEnum, u32, u32)>,
    pub general_cex: Option<CeXNoSevTokenArg>,
}
