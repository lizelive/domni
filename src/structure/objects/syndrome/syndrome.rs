use crate::core::{Reference, ReferenceTo, Flag};
use crate::structure::{
    CeAddTag, CeBleeding, CeBlisters, CeBodyAppearanceModifier, CeBodyMatInteraction,
    CeBodyTransformation, CeBpAppearanceModifier, CeBruising, CeCanDoInteraction,
    CeChangePersonality, CeCloseOpenWounds, CeCoughBlood, CeCureInfection, CeDisplayName,
    CeDisplayTile, CeDizziness, CeDrowsiness, CeErraticBehavior, CeFeelEmotion, CeFever,
    CeFlashTile, CeHealNerves, CeHealTissues, CeImpairFunction, CeMaterialForceMultiplier,
    CeMentAttChange, CeNausea, CeNecrosis, CeNumbness, CeOozing, CePain, CeParalysis,
    CePhysAttChange, CeReduceDizziness, CeReduceFever, CeReduceNausea, CeReducePain,
    CeReduceParalysis, CeReduceSwelling, CeRegrowParts, CeRemoveTag, CeSenseCreatureClass,
    CeSkillRollAdjust, CeSpecialAttackInteraction, CeSpeedChange, CeStopBleeding, CeSwelling,
    CeUnconsciousness, CeVomitBlood, CreatureToken,
};

use serde::{Deserialize, Serialize};

/// Used to begin defining a new syndrome.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SyndromeToken {
    #[serde(alias = "SYNDROME")]
    pub syndrome: Flag,
    // region: Basic Tokens =======================================================================
    /// Used to specify the name of the syndrome as it appears in-game. Names don't have to be
    /// unique, it's perfectly acceptable to have multiple syndromes with identical names.
    #[serde(alias = "SYN_NAME")]
    pub syn_name: Option<String>,
    /// Can be included to create a syndrome class and assign the syndrome to it, for use with the
    /// `IT_CANNOT_HAVE_SYNDROME_CLASS` interaction token. Can be specified more than once to assign
    /// the syndrome to multiple classes. Other syndromes can also be assigned to the same class.
    #[serde(alias = "SYN_CLASS")]
    pub syn_class: Vec<Reference>,
    /// If the syndrome is tied to a material, creatures who come into contact with this material
    /// will contract the syndrome if this token is included in the syndrome definition. Contact
    /// transmission occurs when a creature's body becomes contaminated with the material (visible
    /// as `material name` 'smear', 'dusting' or 'covering' over body parts when viewing the
    /// creature's inventory). Note that contact with items made of a syndrome-inducing material
    /// currently doesn't result in transmission.  
    ///
    /// Methods of getting a material contaminant onto a creature's body include:
    /// - secretions
    /// - liquid projectiles (contaminate struck body parts if exposed)
    /// - vapor and dust clouds (contaminate all external body parts, even if covered)
    /// - puddles and dust piles (`STANCE` body parts become contaminated if the creature walks into
    /// them barefoot, and all uncovered external body parts are contaminated if the creature is
    /// prone)
    /// - freakish rain (contaminates all external body parts, even if covered, if the
    /// creature is outside)
    /// - unprotected bodily contact with a contaminated creature (including performing or receiving
    /// body part attacks such as punches and wrestling moves, creature collisions, as well as
    ///  `CONTACT` interaction effects, if the involved body parts are exposed)
    /// - items melting whilst equipped or hauled (this contaminates the body part that was
    /// holding them if exposed)
    /// - striking the creature's body with a contaminated item (see below)
    ///
    /// It is possible to use this token for syndromes intended to be applied via envenomed weapons
    /// (but also check out `SYN_INJECTED`). When a creature's body is struck with an item which is
    /// contaminated with a contact syndrome-inducing material, the syndrome will be transmitted to
    /// the struck creature, even if the attack doesn't pierce the flesh. Syndrome transmission in
    /// this context often occurs in the absence of a visible contaminant on the body.
    ///
    /// Contact transmission only appears to occur at the moment of contamination (which is to say,
    /// when a new bodily spatter is created). If the syndrome ends (once all its creature effects reach
    /// their `END` point, at which point it will be removed from the creature), it will NOT be
    /// reapplied by the original syndrome-inducing contaminant (assuming it hasn't been cleaned off
    /// yet); the creature will need to be recontaminated with the causative material for this to
    /// occur. (Note that in the case of secretions, the secreted contaminants are continuously
    /// reapplied to the secretory body parts, so any associated short-lasting contact syndromes
    /// allowed to target the secreting creature can potentially be reapplied at the rate of
    /// secretion; this may work differently in adventurer mode).
    #[serde(alias = "SYN_CONTACT")]
    pub syn_contact: Flag,
    /// If the syndrome is tied to a material, creatures who eat or drink substances comprising,
    /// containing or contaminated with this material will contract the syndrome if this token is
    /// included. This includes prepared meals when any of the constituent ingredients contains the
    /// material in question.
    ///
    /// This also applies to grazing creatures which happen to munch on a grass that has an
    /// ingestion-triggered syndrome tied to any of its constituent materials.
    #[serde(alias = "SYN_INGESTED")]
    pub syn_ingested: Flag,
    /// If the syndrome is tied to a material, creatures who inhale the material will contract the
    /// syndrome if this token is included. Materials can only be inhaled in their gaseous state,
    /// which is attainable by boiling, or in the form of a `TRAILING_GAS_FLOW`, `UNDIRECTED_GAS` or
    /// `WEATHER_CREEPING_GAS`. Creatures can also be made to leak gaseous tissue when damaged.
    ///
    /// Note that `[AQUATIC]` creatures never inhale gaseous materials, and creatures which do
    /// breathe air aren't guaranteed to inhale gases when exposed to them for a short time.
    /// Contrary to what one might expect, creatures with `[NOBREATHE]` are in fact capable of
    /// contracting inhalation syndromes; this is presumably a bug.
    #[serde(alias = "SYN_INHALED")]
    pub syn_inhaled: Flag,
    /// If the syndrome is tied to a material, the injection of this material into a creature's
    /// bloodstream will cause it to contract the syndrome if this token is included. Injection can
    /// be carried out as part of a creature attack via `SPECIALATTACK_INJECT_EXTRACT`, or by
    /// piercing the flesh of a creature with an item that has been contaminated with the material.
    /// Thus, this token can be used as a more specific alternative to `SYN_CONTACT` for syndromes
    /// intended to be administered by envenomed weapons.
    ///
    /// For injection to work, the material definition must include `ENTERS_BLOOD`, the attacked body
    /// part needs to have `VASCULAR` tissue, and the intended victim must have `BLOOD` (so it won't
    /// work on creatures with the `CE_REMOVE_TAG:HAS_BLOOD` syndrome effect). Getting the weapon
    /// "lodged into the wound" isn't a requirement.
    #[serde(alias = "SYN_INJECTED")]
    pub syn_injected: Flag,
    /// If this is included, only creatures which belong to the specified creature class (as well as
    /// creatures which pass the `SYN_AFFECTED_CREATURE` check if this is included) will be able to
    /// contract the syndrome. This token can be specified multiple times per syndrome, in which
    /// case creatures which have at least one matching class will be considered susceptible. If
    /// `SYN_IMMUNE_CLASS` and/or `SYN_IMMUNE_CREATURE` are included, creatures which fail these
    /// checks will be unable to contract the syndrome even if they pass this class check.
    #[serde(alias = "SYN_AFFECTED_CLASS")]
    pub syn_affected_class: Vec<Reference>,
    /// If this is included, creatures which belong to the specified creature class will be unable
    /// to contract the syndrome. This token can be specified multiple times per syndrome, in which
    /// case creatures with at least one matching class will be considered immune (unless overridden
    /// by `SYN_AFFECTED_CREATURE`).
    #[serde(alias = "SYN_IMMUNE_CLASS")]
    pub syn_immune_class: Vec<Reference>,
    /// If this is included, only the specified creature (and, if `SYN_AFFECTED_CLASS` is included,
    /// also creatures which pass this check as explained above) will be able to contract the
    /// syndrome. This token can be used multiple times per syndrome. If used alongside
    /// `SYN_IMMUNE_CLASS`, the specified creature will be able to contract the syndrome regardless
    /// of this class check.
    ///
    /// `DWARF:FEMALE` is an example of a valid `<creature>:<caste>` combination;
    /// "`ALL`" can be used in place of a specific caste so as to indicate that this applies to all
    /// castes of the specified creature.
    #[serde(alias = "SYN_AFFECTED_CREATURE")]
    pub syn_affected_creature: Vec<(ReferenceTo<CreatureToken>, Reference)>,
    /// If this is included, the specified creature will be unable to contract the syndrome (even if
    /// it matches `SYN_AFFECTED_CLASS`). It can be specified multiple times per syndrome.
    ///
    /// `DWARF:FEMALE` is an example of a valid `<creature>:<caste>` combination;
    /// "`ALL`" can be used in place of a specific caste so as to indicate that this applies to all
    /// castes of the specified creature.
    #[serde(alias = "SYN_IMMUNE_CREATURE")]
    pub syn_immune_creature: Vec<(ReferenceTo<CreatureToken>, Reference)>,
    /// Prevents creatures from being admitted to hospital for problems arising directly as a result
    /// of the syndrome's effects, no matter how bad they get.
    #[serde(alias = "SYN_NO_HOSPITAL")]
    pub syn_no_hospital: Flag,
    /// This token can be included to give a syndrome an identifier which can be shared between
    /// multiple syndromes. Only one identifier may be specified per syndrome.
    ///
    /// Syndrome identifiers can be used in conjunction with the `SYNDROME_DILUTION_FACTOR` creature
    /// token to alter a creature’s innate resistance to the relevant effects of any syndromes that
    /// possess the specified identifier. For example, every alcoholic beverage in unmodded games
    /// comes with its own copy of an intoxicating syndrome, each of which has a `[SYN_IDENTIFIER:INEBRIATION]`
    /// token. All dwarves have `[SYNDROME_DILUTION_FACTOR:INEBRIATION:150]`, which decreases the
    /// severity of any effects derived from a syndrome with the `INEBRIATION` identifier, thus
    /// enabling them to better handle all forms of alcohol.
    ///
    /// A creature can only possess a single syndrome with a particular identifier at any given time.
    /// As such, if a creature contracts a syndrome with a particular identifier and is subsequently
    /// exposed to another syndrome with a matching identifier whilst that first syndrome is still
    /// active, then this later obtained syndrome will not be contracted. If the latter comes with
    /// a `SYN_CONCENTRATION_ADDED` token, it will instead adjust the first syndrome's concentration
    /// as described below.
    ///
    /// Taking the above example once again, a sober dwarf drinking their first alcoholic beverage
    /// would be exposed to an `INEBRIATION` syndrome, contracting it and having its effects manifest
    /// normally. If the dwarf were to have another drink before the effects of this first syndrome
    /// have all worn off (by reaching their `END` point), then exposure to the second `INEBRIATION`
    /// syndrome would only increase the severity of the original syndrome's effects, making the
    /// dwarf progressively more intoxicated.
    #[serde(alias = "SYN_IDENTIFIER")]
    pub syn_identifier: Option<Reference>,
    /// Syndrome concentration is essentially a quantity which impacts the severity of the
    /// syndrome's relevant effects. The higher the syndrome's concentration, the greater its
    /// severity. When a syndrome is contracted, the value specified in the first argument
    /// is its initial concentration level.
    ///
    /// If a creature is exposed to a syndrome with a particular `SYN_IDENTIFIER` when already
    /// possessing an active syndrome with the same identifier, then this later syndrome isn't
    /// contracted, instead contributing to the original syndrome's concentration as indicated by
    /// its `SYN_CONCENTRATION_ADDED` token, if present.
    ///
    /// The syndrome in question will increase the original syndrome's concentration by `<amount>`
    /// whenever the creature is exposed to it, until its specified `<max>` concentration is reached
    /// by the original syndrome, causing subsequent exposure to this particular syndrome to do
    /// nothing (that is, until the original syndrome ends, at which point a new one may be
    /// contracted normally). Should the creature be exposed to a different syndrome with the same
    /// identifier and a higher <max> value, the concentration will of course increase further.
    ///
    /// For example, all forms of alcohol in the vanilla game have a syndrome with
    /// `[SYN_IDENTIFIER:INEBRIATION]` and `[SYN_CONCENTRATION_ADDED:100:1000]`. When alcohol is
    /// first drunk, the creature contracts the relevant inebriating syndrome at a concentration
    /// level of 100. Every subsequent drink will increase the concentration of this first syndrome
    /// by a further 100, intensifying its effects, until it plateaus at concentration level 1000.
    /// Once all the effects of the original syndrome have ended, the cycle can be started anew
    /// (assuming the drinker hasn't died of alcohol poisoning yet).
    ///
    /// As described by Toady, "Each 100 of `<amount>` will contribute `SEV` in general to each effect
    /// (before dilution), `<max>` goes up to 1000. The concentration does not decrease, but will
    /// stay at the maximum attained until the syndrome wears off."
    ///
    /// Some of the generated interaction-derived syndromes come with `[SYN_CONCENTRATION_ADDED:1000:0]`.
    /// According to Toady, this "was a precaution after I had one bug with effects not fully
    /// manifesting due to low levels. It may not be necessary, but I decided to give everybody a
    /// full dose of the juice until I could get a closer look at it."
    /// [Source](http://www.bay12forums.com/smf/index.php?topic=169696.msg8083872#msg8083872)
    #[serde(alias = "SYN_CONCENTRATION_ADDED")]
    pub syn_concentration_added: Option<(u32, u32)>,
    // endregion ==================================================================================
    // region: Symptomatic Effects ================================================================
    /// A list of `CE_BRUISING` tokens for this syndrome.
    #[serde(alias = "CE_BRUISING")]
    pub ce_bruising: Vec<CeBruising>,
    /// A list of `CE_BLISTERS` tokens for this syndrome.
    #[serde(alias = "CE_BLISTERS")]
    pub ce_blisters: Vec<CeBlisters>,
    /// A list of `CE_OOZING` tokens for this syndrome.
    #[serde(alias = "CE_OOZING")]
    pub ce_oozing: Vec<CeOozing>,
    /// A list of `CE_BLEEDING` tokens for this syndrome.
    #[serde(alias = "CE_BLEEDING")]
    pub ce_bleeding: Vec<CeBleeding>,
    /// A list of `CE_SWELLING` tokens for this syndrome.
    #[serde(alias = "CE_SWELLING")]
    pub ce_swelling: Vec<CeSwelling>,
    /// A list of `CE_NECROSIS` tokens for this syndrome.
    #[serde(alias = "CE_NECROSIS")]
    pub ce_necrosis: Vec<CeNecrosis>,
    /// A list of `CE_NUMBNESS` tokens for this syndrome.
    #[serde(alias = "CE_NUMBNESS")]
    pub ce_numbness: Vec<CeNumbness>,
    /// A list of `CE_PAIN` tokens for this syndrome.
    #[serde(alias = "CE_PAIN")]
    pub ce_pain: Vec<CePain>,
    /// A list of `CE_PARALYSIS` tokens for this syndrome.
    #[serde(alias = "CE_PARALYSIS")]
    pub ce_paralysis: Vec<CeParalysis>,
    /// A list of `CE_IMPAIR_FUNCTION` tokens for this syndrome.
    #[serde(alias = "CE_IMPAIR_FUNCTION")]
    pub ce_impair_function: Vec<CeImpairFunction>,
    /// A list of `CE_DIZZINESS` tokens for this syndrome.
    #[serde(alias = "CE_DIZZINESS")]
    pub ce_dizziness: Vec<CeDizziness>,
    /// A list of `CE_DROWSINESS` tokens for this syndrome.
    #[serde(alias = "CE_DROWSINESS")]
    pub ce_drowsiness: Vec<CeDrowsiness>,
    /// A list of `CE_UNCONSCIOUSNESS` tokens for this syndrome.
    #[serde(alias = "CE_UNCONSCIOUSNESS")]
    pub ce_unconsciousness: Vec<CeUnconsciousness>,
    /// A list of `CE_FEVER` tokens for this syndrome.
    #[serde(alias = "CE_FEVER")]
    pub ce_fever: Vec<CeFever>,
    /// A list of `CE_NAUSEA` tokens for this syndrome.
    #[serde(alias = "CE_NAUSEA")]
    pub ce_nausea: Vec<CeNausea>,
    /// A list of `CE_COUGH_BLOOD` tokens for this syndrome.
    #[serde(alias = "CE_COUGH_BLOOD")]
    pub ce_cough_blood: Vec<CeCoughBlood>,
    /// A list of `CE_VOMIT_BLOOD` tokens for this syndrome.
    #[serde(alias = "CE_VOMIT_BLOOD")]
    pub ce_vomit_blood: Vec<CeVomitBlood>,
    // endregion ==================================================================================
    // region: Healing Effects ====================================================================
    /// A list of `CE_REDUCE_PAIN` tokens for this syndrome.
    #[serde(alias = "CE_REDUCE_PAIN")]
    pub ce_reduce_pain: Vec<CeReducePain>,
    /// A list of `CE_REDUCE_SWELLING` tokens for this syndrome.
    #[serde(alias = "CE_REDUCE_SWELLING")]
    pub ce_reduce_swelling: Vec<CeReduceSwelling>,
    /// A list of `CE_REDUCE_PARALYSIS` tokens for this syndrome.
    #[serde(alias = "CE_REDUCE_PARALYSIS")]
    pub ce_reduce_paralysis: Vec<CeReduceParalysis>,
    /// A list of `CE_REDUCE_DIZZINESS` tokens for this syndrome.
    #[serde(alias = "CE_REDUCE_DIZZINESS")]
    pub ce_reduce_dizziness: Vec<CeReduceDizziness>,
    /// A list of `CE_REDUCE_NAUSEA` tokens for this syndrome.
    #[serde(alias = "CE_REDUCE_NAUSEA")]
    pub ce_reduce_nausea: Vec<CeReduceNausea>,
    /// A list of `CE_REDUCE_FEVER` tokens for this syndrome.
    #[serde(alias = "CE_REDUCE_FEVER")]
    pub ce_reduce_fever: Vec<CeReduceFever>,
    /// A list of `CE_STOP_BLEEDING` tokens for this syndrome.
    #[serde(alias = "CE_STOP_BLEEDING")]
    pub ce_stop_bleeding: Vec<CeStopBleeding>,
    /// A list of `CE_CLOSE_OPEN_WOUNDS` tokens for this syndrome.
    #[serde(alias = "CE_CLOSE_OPEN_WOUNDS")]
    pub ce_close_open_wounds: Vec<CeCloseOpenWounds>,
    /// A list of `CE_CURE_INFECTION` tokens for this syndrome.
    #[serde(alias = "CE_CURE_INFECTION")]
    pub ce_cure_infection: Vec<CeCureInfection>,
    /// A list of `CE_HEAL_TISSUES` tokens for this syndrome.
    #[serde(alias = "CE_HEAL_TISSUES")]
    pub ce_heal_tissues: Vec<CeHealTissues>,
    /// A list of `CE_HEAL_NERVES` tokens for this syndrome.
    #[serde(alias = "CE_HEAL_NERVES")]
    pub ce_heal_nerves: Vec<CeHealNerves>,
    /// A list of `CE_REGROW_PARTS` tokens for this syndrome.
    #[serde(alias = "CE_REGROW_PARTS")]
    pub ce_regrow_parts: Vec<CeRegrowParts>,
    // endregion ==================================================================================
    // region: Special Effects ====================================================================
    /// A list of `CE_ADD_TAG` tokens for this syndrome.
    #[serde(alias = "CE_ADD_TAG")]
    pub ce_add_tag: Vec<CeAddTag>,
    /// A list of `CE_REMOVE_TAG` tokens for this syndrome.
    #[serde(alias = "CE_REMOVE_TAG")]
    pub ce_remove_tag: Vec<CeRemoveTag>,
    /// A list of `CE_DISPLAY_NAME` tokens for this syndrome.
    #[serde(alias = "CE_DISPLAY_NAME")]
    pub ce_display_name: Vec<CeDisplayName>,
    /// A list of `CE_DISPLAY_TILE` tokens for this syndrome.
    #[serde(alias = "CE_DISPLAY_TILE")]
    pub ce_display_tile: Vec<CeDisplayTile>,
    /// A list of `CE_FLASH_TILE` tokens for this syndrome.
    #[serde(alias = "CE_FLASH_TILE")]
    pub ce_flash_tile: Vec<CeFlashTile>,
    /// A list of `CE_PHYS_ATT_CHANGE` tokens for this syndrome.
    #[serde(alias = "CE_PHYS_ATT_CHANGE")]
    pub ce_phys_att_change: Vec<CePhysAttChange>,
    /// A list of `CE_MENT_ATT_CHANGE` tokens for this syndrome.
    #[serde(alias = "CE_MENT_ATT_CHANGE")]
    pub ce_ment_att_change: Vec<CeMentAttChange>,
    /// A list of `CE_SPEED_CHANGE` tokens for this syndrome.
    #[serde(alias = "CE_SPEED_CHANGE")]
    pub ce_speed_change: Vec<CeSpeedChange>,
    /// A list of `CE_SKILL_ROLL_ADJUST` tokens for this syndrome.
    #[serde(alias = "CE_SKILL_ROLL_ADJUST")]
    pub ce_skill_roll_adjust: Vec<CeSkillRollAdjust>,
    /// A list of `CE_BODY_APPEARANCE_MODIFIER` tokens for this syndrome.
    #[serde(alias = "CE_BODY_APPEARANCE_MODIFIER")]
    pub ce_body_appearance_modifier: Vec<CeBodyAppearanceModifier>,
    /// A list of `CE_BP_APPEARANCE_MODIFIER` tokens for this syndrome.
    #[serde(alias = "CE_BP_APPEARANCE_MODIFIER")]
    pub ce_bp_appearance_modifier: Vec<CeBpAppearanceModifier>,
    /// A list of `CE_BODY_TRANSFORMATION` tokens for this syndrome.
    #[serde(alias = "CE_BODY_TRANSFORMATION")]
    pub ce_body_transformation: Vec<CeBodyTransformation>,
    /// A list of `CE_MATERIAL_FORCE_MULTIPLIER` tokens for this syndrome.
    #[serde(alias = "CE_MATERIAL_FORCE_MULTIPLIER")]
    pub ce_material_force_multiplier: Vec<CeMaterialForceMultiplier>,
    /// A list of `CE_CAN_DO_INTERACTION` tokens for this syndrome.
    #[serde(alias = "CE_CAN_DO_INTERACTION")]
    pub ce_can_do_interaction: Vec<CeCanDoInteraction>,
    /// A list of `CE_SPECIAL_ATTACK_INTERACTION` tokens for this syndrome.
    #[serde(alias = "CE_SPECIAL_ATTACK_INTERACTION")]
    pub ce_special_attack_interaction: Vec<CeSpecialAttackInteraction>,
    /// A list of `CE_BODY_MAT_INTERACTION` tokens for this syndrome.
    #[serde(alias = "CE_BODY_MAT_INTERACTION")]
    pub ce_body_mat_interaction: Vec<CeBodyMatInteraction>,
    /// A list of `CE_SENSE_CREATURE_CLASS` tokens for this syndrome.
    #[serde(alias = "CE_SENSE_CREATURE_CLASS")]
    pub ce_sense_creature_class: Vec<CeSenseCreatureClass>,
    /// A list of `CE_FEEL_EMOTION` tokens for this syndrome.
    #[serde(alias = "CE_FEEL_EMOTION")]
    pub ce_feel_emotion: Vec<CeFeelEmotion>,
    /// A list of `CE_CHANGE_PERSONALITY` tokens for this syndrome.
    #[serde(alias = "CE_CHANGE_PERSONALITY")]
    pub ce_change_personality: Vec<CeChangePersonality>,
    /// A list of `CE_ERRATIC_BEHAVIOR` tokens for this syndrome.
    #[serde(alias = "CE_ERRATIC_BEHAVIOR")]
    pub ce_erratic_behavior: Vec<CeErraticBehavior>,
    // endregion ==================================================================================
}
