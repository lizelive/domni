use crate::core::{Choose, DFChar, Reference, ReferenceTo};
use crate::structure::{
    AppModTypeEnum, AppearanceModifierEnum, BpAppModifersEnum, BpCriteriaTokenArg, BpEnum,
    CdiTokenArg, CeBodyMatInteractionTokenArg, CeBodyTransformationTokenArg,
    CeChangePersonalityTokenArg, CeMentAttChangeTokenArg, CePhysAttChangeTokenArg, CeTagsTokenArg,
    CeXNoSevTokenArg, CeXNoTargetTokenArg, CeXTokenArg, ClassEnum, CounterTriggerEnum,
    CounterTypesEnum, EmotionEnum, EmotionTypeEnum, FrequencyEnum, InteractionEnum,
    InteractionToken, MatMultEnum, MatTokenEnum, MaterialTokenArg, MoonPhaseEnum, NameEnum,
    NoneEnum, PercEnum, PercOnEnum, PeriodicTriggerEnum, RequiredEnum, ReservedBloodEnum,
    SpeedEnum, TileEnum,
};

use serde::{Deserialize, Serialize};

type CeTokenArgs = Choose<
    (PeriodicTriggerEnum, MoonPhaseEnum, u32, u32),
    (
        CounterTriggerEnum,
        CounterTypesEnum,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
        RequiredEnum,
    ),
>;

// region: Symptomatic Effects ====================================================================
/// Causes the targeted bodypart to undergo bruising.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeBruising {
    /// Arguments for `[CE_BRUISING:...]`
    #[serde(alias = "CE_BRUISING")]
    pub ce_bruising: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Covers the targeted bodypart with blisters.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeBlisters {
    /// Arguments for `[CE_BLISTERS:...]`
    #[serde(alias = "CE_BLISTERS")]
    pub ce_blisters: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes pus to ooze from the afflicted bodypart.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeOozing {
    /// Arguments for `[CE_OOZING:...]`
    #[serde(alias = "CE_OOZING")]
    pub ce_oozing: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the targeted bodypart to start bleeding, with heavy enough bleeding resulting in the
/// death of the sufferer. Some conditions seem to cause bleeding to be fatal no matter how weak.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeBleeding {
    /// Arguments for `[CE_BLEEDING:...]`
    #[serde(alias = "CE_BLEEDING")]
    pub ce_bleeding: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the targeted bodypart to swell up. Extreme swelling may lead to necrosis.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeSwelling {
    /// Arguments for `[CE_SWELLING:...]`
    #[serde(alias = "CE_SWELLING")]
    pub ce_swelling: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the targeted bodypart to rot, with associated tissue damage, miasma emission and
/// bleeding. The victim slowly bleeds to death if the wound is not treated. Badly necrotic limbs
/// will require amputation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeNecrosis {
    /// Arguments for `[CE_NECROSIS:...]`
    #[serde(alias = "CE_NECROSIS")]
    pub ce_necrosis: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes numbness in the affected body part, blocking pain. Extreme numbness may lead to sensory
/// nerve damage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeNumbness {
    /// Arguments for `[CE_NUMBNESS:...]`
    #[serde(alias = "CE_NUMBNESS")]
    pub ce_numbness: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Afflicts the targeted bodypart with intense pain.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CePain {
    /// Arguments for `[CE_PAIN:...]`
    #[serde(alias = "CE_PAIN")]
    pub ce_pain: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes paralysis. Paralysis is complete paralysis and will cause suffocation in smaller
/// creatures. Paralysis on a limb may lead to motor nerve damage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeParalysis {
    /// Arguments for `[CE_PARALYSIS:...]`
    #[serde(alias = "CE_PARALYSIS")]
    pub ce_paralysis: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// An organ afflicted with this `CE` is rendered inoperable - for example, if both lungs are
/// impaired the creature can't breathe and will suffocate. This token only affects organs, not
/// limbs. Note that this effect is currently bugged, and will not "turn off" until the creature
/// receives a wound to cause its body parts to update.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeImpairFunction {
    /// Arguments for `[CE_IMPAIR_FUNCTION:...]`
    #[serde(alias = "CE_IMPAIR_FUNCTION")]
    pub ce_impair_function: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Inflicts the Dizziness condition, occasional fainting and a general slowdown in movement and
/// work speed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeDizziness {
    /// Arguments for `[CE_DIZZINESS:...]`
    #[serde(alias = "CE_DIZZINESS")]
    pub ce_dizziness: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the Drowsiness condition.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeDrowsiness {
    /// Arguments for `[CE_DROWSINESS:...]`
    #[serde(alias = "CE_DROWSINESS")]
    pub ce_drowsiness: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Renders unconscious.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeUnconsciousness {
    /// Arguments for `[CE_UNCONSCIOUSNESS:...]`
    #[serde(alias = "CE_UNCONSCIOUSNESS")]
    pub ce_unconsciousness: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the Fever condition.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeFever {
    /// Arguments for `[CE_FEVER:...]`
    #[serde(alias = "CE_FEVER")]
    pub ce_fever: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the Nausea condition, and heavy vomiting. Can eventually lead to dehydration and death.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeNausea {
    /// Arguments for `[CE_NAUSEA:...]`
    #[serde(alias = "CE_NAUSEA")]
    pub ce_nausea: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// This effect results in the sufferer periodically coughing blood, which stains the tile they're
/// on and requires cleanup. It doesn't appear to be lethal, but may cause minor bleeding damage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeCoughBlood {
    /// Arguments for `[CE_COUGH_BLOOD:...]`
    #[serde(alias = "CE_COUGH_BLOOD")]
    pub ce_cough_blood: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// This effect results in the sufferer periodically vomiting blood, which stains the tile they're
/// on and requires cleanup. It doesn't appear to be lethal, but may cause minor bleeding damage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeVomitBlood {
    /// Arguments for `[CE_VOMIT_BLOOD:...]`
    #[serde(alias = "CE_VOMIT_BLOOD")]
    pub ce_vomit_blood: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
// endregion ======================================================================================
// region: Healing Effects ========================================================================
/// Decreases the severity of pain produced by wounds or syndrome effects on the targeted bodypart.
/// The `SEV` value probably controls by how much the pain is decreased.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeReducePain {
    /// Arguments for `[CE_REDUCE_PAIN:...]`
    #[serde(alias = "CE_REDUCE_PAIN")]
    pub ce_reduce_pain: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Decreases the severity of swelling on the targeted bodypart.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeReduceSwelling {
    /// Arguments for `[CE_REDUCE_SWELLING:...]`
    #[serde(alias = "CE_REDUCE_SWELLING")]
    pub ce_reduce_swelling: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Decreases the severity of any paralysis effects on the targeted bodypart.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeReduceParalysis {
    /// Arguments for `[CE_REDUCE_PARALYSIS:...]`
    #[serde(alias = "CE_REDUCE_PARALYSIS")]
    pub ce_reduce_paralysis: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Decreases the severity of any dizziness the creature has.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeReduceDizziness {
    /// Arguments for `[CE_REDUCE_DIZZINESS:...]`
    #[serde(alias = "CE_REDUCE_DIZZINESS")]
    pub ce_reduce_dizziness: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Decreases the severity of any nausea the creature has.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeReduceNausea {
    /// Arguments for `[CE_REDUCE_NAUSEA:...]`
    #[serde(alias = "CE_REDUCE_NAUSEA")]
    pub ce_reduce_nausea: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Decreases the severity of any fever the creature has.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeReduceFever {
    /// Arguments for `[CE_REDUCE_FEVER:...]`
    #[serde(alias = "CE_REDUCE_FEVER")]
    pub ce_reduce_fever: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Decreases the severity of the bleeding of any wounds or syndrome effects on the targeted
/// bodypart. The `SEV` value probably controls by how much the bleeding is decreased.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeStopBleeding {
    /// Arguments for `[CE_STOP_BLEEDING:...]`
    #[serde(alias = "CE_STOP_BLEEDING")]
    pub ce_stop_bleeding: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Closes any wounds on the targeted bodypart with speed depending on the `SEV` value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeCloseOpenWounds {
    /// Arguments for `[CE_CLOSE_OPEN_WOUNDS:...]`
    #[serde(alias = "CE_CLOSE_OPEN_WOUNDS")]
    pub ce_close_open_wounds: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Probably decreases the severity of the infection from infected wounds over time.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeCureInfection {
    /// Arguments for `[CE_CURE_INFECTION:...]`
    #[serde(alias = "CE_CURE_INFECTION")]
    pub ce_cure_infection: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Heals the tissues of the targeted bodypart with speed depending on the `SEV` value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeHealTissues {
    /// Arguments for `[CE_HEAL_TISSUES:...]`
    #[serde(alias = "CE_HEAL_TISSUES")]
    pub ce_heal_tissues: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Heals the nerves of the targeted bodypart with speed depending on the `SEV` value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeHealNerves {
    /// Arguments for `[CE_HEAL_NERVES:...]`
    #[serde(alias = "CE_HEAL_NERVES")]
    pub ce_heal_nerves: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes missing bodyparts to regrow. `SEV` controls how quickly bodyparts are regrown.
///
/// In adventure, parts will be regrown until you travel or wait/sleep.
/// [Bug:0011396.](https://www.bay12games.com/dwarves/mantisbt/view.php?id=0011396)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeRegrowParts {
    /// Arguments for `[CE_REGROW_PARTS:...]`
    #[serde(alias = "CE_REGROW_PARTS")]
    pub ce_regrow_parts: Option<CeXTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
// endregion ======================================================================================
// region: Special Effects ========================================================================
/// Adds the specified tag(s) to the affected creature. Multiple tags can be specified sequentially
/// within a single effect token.
///
/// Adding tags will cause the creature to pass/fail any relevant `IT_REQUIRES`/`IT_FORBIDDEN` checks
/// (with the apparent exceptions of `FIT_FOR_ANIMATION` and `FIT_FOR_RESURRECTION`). Note that
/// `CE_REMOVE_TAG` overrides this effect.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeAddTag {
    /// Arguments for `[CE_ADD_TAG:...]`
    #[serde(alias = "CE_ADD_TAG")]
    pub ce_add_tag: Option<CeTagsTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Removes the specified tag(s) from the affected creature. Multiple tags can be specified
/// sequentially within a single effect token.
///
/// If a particular tag is targeted by both `CE_REMOVE_TAG` and `CE_ADD_TAG`, and both effects are
/// active simultaneously, `CE_REMOVE_TAG` takes precedence (i.e. the overall effect is that of tag
/// removal for as long as `CE_REMOVE_TAG` remains active). The order in which the effects activate
/// doesn't affect this, not even if `CE_ADD_TAG` is added later/earlier via a different syndrome.
///
/// Removing tags will cause the creature to fail/pass any relevant `IT_REQUIRES`/`IT_FORBIDDEN` checks
/// (with the apparent exceptions of `FIT_FOR_ANIMATION` and `FIT_FOR_RESURRECTION`).
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeRemoveTag {
    /// Arguments for `[CE_REMOVE_TAG:...]`
    #[serde(alias = "CE_REMOVE_TAG")]
    pub ce_remove_tag: Option<CeTagsTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Attaches the specified name to the creature's normal name.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeDisplayName {
    /// Arguments for `[CE_DISPLAY_NAME:...]`
    #[serde(alias = "CE_DISPLAY_NAME")]
    pub ce_display_name: Option<(NameEnum, String, String, String, CeXNoSevTokenArg)>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the creature to display the specified tile instead of its normal one.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeDisplayTile {
    /// Arguments for `[CE_DISPLAY_TILE:...]`
    #[serde(alias = "CE_DISPLAY_TILE")]
    pub ce_display_tile: Option<(TileEnum, DFChar, u8, u8, u8, CeXNoSevTokenArg)>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes the creature to flash between its normal tile and the one specified here.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeFlashTile {
    /// Arguments for `[CE_FLASH_TILE:...]`
    #[serde(alias = "CE_FLASH_TILE")]
    pub ce_flash_tile: Option<(
        TileEnum,
        DFChar,
        u8,
        u8,
        u8,
        FrequencyEnum,
        u32,
        u32,
        CeXNoSevTokenArg,
    )>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Alters the creature's specified physical attribute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CePhysAttChange {
    /// Arguments for `[CE_PHYS_ATT_CHANGE:...]`
    #[serde(alias = "CE_PHYS_ATT_CHANGE")]
    pub ce_phys_att_change: Option<CePhysAttChangeTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Alters the creature's specified mental attribute.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeMentAttChange {
    /// Arguments for `[CE_MENT_ATT_CHANGE:...]`
    #[serde(alias = "CE_MENT_ATT_CHANGE")]
    pub ce_ment_att_change: Option<CeMentAttChangeTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Changes the creature's speed. The minimum and maximum speeds able to be created by
/// `CE_SPEED_CHANGE` are 99 and 10,000 respectively.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeSpeedChange {
    /// Arguments for `[CE_SPEED_CHANGE:...]`
    #[serde(alias = "CE_SPEED_CHANGE")]
    pub ce_speed_change: Option<(SpeedEnum, i32, CeXNoSevTokenArg)>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Alters the creature's specified skill level. The argument `PERC` specifies a percentage of the
/// creature's current skill, and `PERC_ON` the probability of the effect being applied on a
/// particular roll.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeSkillRollAdjust {
    /// Arguments for `[CE_SKILL_ROLL_ADJUST:...]`
    #[serde(alias = "CE_SKILL_ROLL_ADJUST")]
    pub ce_skill_roll_adjust: Option<(PercEnum, u8, PercOnEnum, u8, CeXNoSevTokenArg)>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Alters the size of the creature.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeBodyAppearanceModifier {
    /// Arguments for `[CE_BODY_APPEARANCE_MODIFIER:...]`
    #[serde(alias = "CE_BODY_APPEARANCE_MODIFIER")]
    pub ce_body_appearance_modifier: Option<(
        AppearanceModifierEnum,
        AppModTypeEnum,
        u32,
        CeXNoSevTokenArg,
    )>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Alters the characteristics (height, width etc.) of a body part.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeBpAppearanceModifier {
    /// Arguments for `[CE_BP_APPEARANCE_MODIFIER:...]`
    #[serde(alias = "CE_BP_APPEARANCE_MODIFIER")]
    pub ce_bp_appearance_modifier: Option<(
        BpEnum,
        BpCriteriaTokenArg,
        AppearanceModifierEnum,
        BpAppModifersEnum,
        u32,
        CeXNoTargetTokenArg,
    )>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Makes the affected unit transform into a different creature. The target creature may either
/// be specified directly by following this with a `CE:CREATURE` token, or else set to be randomly
/// selected from a "pool" defined by additional `[CE:...]` tokens (of which multiple may be
/// specified).
///
/// Note that transformation into or out of the target form causes the creature to
/// drop all items in its inventory and instantly heals all of its wounds. If an undead limb
/// happens to be transformed, its entire body will regenerate upon transforming back.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeBodyTransformation {
    /// Arguments for `[CE_BODY_TRANSFORMATION:...]`
    #[serde(alias = "CE_BODY_TRANSFORMATION")]
    pub ce_body_transformation: Option<CeXNoSevTokenArg>,
    /// Either specifies the details of what the creature transforms into, or prevents the syndrome
    /// effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<Choose<CeTokenArgs, CeBodyTransformationTokenArg>>,
}
/// When the affected creature is struck with a weapon made of the specified material, the force
/// exerted will be multiplied by A/B, thus making the creature more or less susceptible to this
/// material. For example, if A is 2 and B is 1, the force exerted by the defined material will
/// be doubled. If A is 1 and B is 2, it will be halved instead. `NONE:NONE` can be used in place
/// of a specific material token so as to make the effect applicable to all materials. Note that
/// this syndrome effect is equivalent to the `MATERIAL_FORCE_MULTIPLIER` creature token.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeMaterialForceMultiplier {
    /// Arguments for `[CE_MATERIAL_FORCE_MULTIPLIER:...]`
    #[serde(alias = "CE_MATERIAL_FORCE_MULTIPLIER")]
    pub ce_material_force_multiplier: Option<(
        MatMultEnum,
        Choose<(NoneEnum, NoneEnum), MaterialTokenArg>,
        u32,
        u32,
        CeXNoSevTokenArg,
    )>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Makes the creature able to perform an interaction. Follow this effect token with
/// `[CDI:INTERACTION:<interaction name>]` to specify the desired interaction, and add other `CDI`
/// tokens as required.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeCanDoInteraction {
    /// Arguments for `[CE_CAN_DO_INTERACTION:...]`
    #[serde(alias = "CE_CAN_DO_INTERACTION")]
    pub ce_can_do_interaction: Option<CeXNoSevTokenArg>,
    ///
    #[serde(alias = "CDI")]
    pub cdi: Vec<CdiTokenArg>,
}
/// Makes the creature able to perform an interaction when using an attack with a designated body
/// part/parts.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeSpecialAttackInteraction {
    /// Arguments for `[CE_SPECIAL_ATTACK_INTERACTION:...]`
    #[serde(alias = "CE_SPECIAL_ATTACK_INTERACTION")]
    pub ce_special_attack_interaction: Option<(
        InteractionEnum,
        ReferenceTo<InteractionToken>,
        CeXNoSevTokenArg,
    )>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// This is used to tie an interaction to one of the creature’s body materials. Generated vampire
/// syndromes use this effect to make vampire blood pass on the vampirism curse when consumed.
///
/// The target body material is specified by inserting its `ID` as defined in the creature raws. For
/// example, when a syndrome with "`CE_BODY_MAT_INTERACTION:MAT_TOKEN:SWEAT`" is gained by a unit, the
/// effect will apply to any material defined as "SWEAT" in the creature raws of that unit, if such
/// a material is present.
///
/// This currently only works on materials obtained from historical figures. That is to say, the
/// material must bear the source unit's name, such as "Urist McVampire's dwarf blood" as opposed to
/// mere "dwarf blood".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeBodyMatInteraction {
    /// Arguments for `[CE_BODY_MAT_INTERACTION:...]`
    #[serde(alias = "CE_BODY_MAT_INTERACTION")]
    pub ce_body_mat_interaction: Option<(
        MatTokenEnum,
        Choose<ReservedBloodEnum, Reference>,
        CeXNoSevTokenArg,
    )>,
    /// Used to either specify how the body material triggers the interaction, or what interaction
    /// is actually used.
    #[serde(alias = "CE")]
    pub ce: Vec<CeBodyMatInteractionTokenArg>,
}
/// Provides the ability to sense creatures belonging to the specified creature class even when such
/// creatures lie far beyond line of sight, including through walls and floors. It also appears to
/// reduce or negate the combat penalty of blind units when fighting creatures they can sense. In
/// adventure mode, the specified tile will be used to represent sensed creatures when they cannot
/// be seen directly.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeSenseCreatureClass {
    /// Arguments for `[CE_SENSE_CREATURE_CLASS:...]`
    #[serde(alias = "CE_SENSE_CREATURE_CLASS")]
    pub ce_sense_creature_class: Option<(
        ClassEnum,
        Reference,
        DFChar,
        u8,
        u8,
        u8,
        CeXNoTargetTokenArg,
    )>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Makes the creature feel a specific emotion. The effect's `SEV` value determines how intense
/// the emotion is. The creature also receives a thought in the following format: "`[creature]`
/// feels `[emotion]` due to `[syndrome name]`".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeFeelEmotion {
    /// Arguments for `[CE_FEEL_EMOTION:...]`
    #[serde(alias = "CE_FEEL_EMOTION")]
    pub ce_feel_emotion: Option<(EmotionEnum, EmotionTypeEnum, CeXNoTargetTokenArg)>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Changes a personality trait by the given amount. Multiple `FACET:<trait>:<amount>` sets may be
/// used, and `<amount>` can be negative. For example, generated necromancer syndromes come with
/// the following effect:
///
/// `[CE_CHANGE_PERSONALITY:FACET:ANXIETY_PROPENSITY:50:FACET:TRUST:-50:START:0:ABRUPT]`
///
/// According to Toady, `CE_CHANGE_PERSONALITY` effects can cause creatures to re-evaluate their
/// goals in worldgen; the boost to anxiety and distrust given to necromancers makes it more
/// likely for them to develop the goal of ruling the world.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeChangePersonality {
    /// Arguments for `[CE_CHANGE_PERSONALITY:...]`
    #[serde(alias = "CE_CHANGE_PERSONALITY")]
    pub ce_change_personality: Option<CeChangePersonalityTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
/// Causes erratic behavior, meaning "People that like to brawl have a chance of starting a brawl-
/// level fight with any nearby adult." -Toady
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CeErraticBehavior {
    /// Arguments for `[CE_ERRATIC_BEHAVIOR:...]`
    #[serde(alias = "CE_ERRATIC_BEHAVIOR")]
    pub ce_erratic_behavior: Option<CeXNoTargetTokenArg>,
    /// Used to prevent the syndrome effect being active unless certain conditions apply.
    #[serde(alias = "CE")]
    pub ce: Vec<CeTokenArgs>,
}
// endregion ======================================================================================
