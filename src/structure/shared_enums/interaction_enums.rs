use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum EffectLocationEnum {
    /// A depth of 1/7 is sufficient for `IN_WATER`.
    #[serde(alias = "IN_WATER")]
    InWater,
    /// A depth of 1/7 is sufficient for `IN_MAGMA`.
    #[serde(alias = "IN_MAGMA")]
    InMagma,
    #[serde(alias = "NO_WATER")]
    NoWater,
    #[serde(alias = "NO_MAGMA")]
    NoMagma,
    #[serde(alias = "NO_THICK_FOG")]
    NoThickFog,
    #[serde(alias = "OUTSIDE")]
    Outside,
}
impl Default for EffectLocationEnum {
    fn default() -> Self {
        Self::InWater
    }
}

// TODO: most of these are creature tokens; check if ANY creature token can be used for this
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TargetPropertyEnum {
    /// Any corpse or body part that can become a zombie (heads, hands, etc.)
    ///
    /// Adding or removing this tag via a syndrome seems to have no effect.
    #[serde(alias = "FIT_FOR_ANIMATION")]
    FitForAnimation,
    /// The target corpse's `UPPERBODY` must be attached.
    ///
    /// Adding or removing this tag via a syndrome seems to have no effect.
    #[serde(alias = "FIT_FOR_RESURRECTION")]
    FitForResurrection,
    /// If this tag is removed via syndrome, the creature behaves as though it has no blood; it doesn't bleed
    /// when `VASCULAR` tissues are damaged (and thus cannot die of blood loss), and substances
    /// cannot be injected into it.
    ///
    /// Adding this tag (via syndrome) appears to have no effect.
    #[serde(alias = "HAS_BLOOD")]
    HasBlood,
    /// Adding or removing this tag via a syndrome seems to have no effect.
    #[serde(alias = "MORTAL")]
    Mortal,
    /// Halts the creature's aging process and prevents death by old age.
    ///
    /// Has no apparent effect if removed from a creature, unless it was added already, in which
    /// case it will be negated.
    #[serde(alias = "NO_AGING")]
    NoAging,
    /// Makes the creature unable to produce [offspring](https://dwarffortresswiki.org/index.php/Children).
    ///
    /// Has no apparent effect if removed from a creature, unless it was added already, in which
    /// case it will be negated.
    #[serde(alias = "STERILE")]
    Sterile,
    // TODO: when the creature tokens are done, copy the descriptions to here.
    #[serde(alias = "BLOODSUCKER")]
    Bloodsucker,
    #[serde(alias = "CAN_LEARN")]
    CanLearn,
    #[serde(alias = "CAN_SPEAK")]
    CanSpeak,
    #[serde(alias = "CRAZED")]
    Crazed,
    #[serde(alias = "EXTRAVISION")]
    ExtraVision,
    #[serde(alias = "LIKES_FIGHTING")]
    LikesFighting,
    #[serde(alias = "MISCHIEVOUS", alias = "MISCHIEVIOUS")]
    Mischievous,
    #[serde(alias = "NO_CONNECTIONS_FOR_MOVEMENT")]
    NoConnectionsForMovement,
    #[serde(alias = "NO_DIZZINESS")]
    NoDizziness,
    #[serde(alias = "NO_DRINK")]
    NoDrink,
    #[serde(alias = "NO_EAT")]
    NoEat,
    #[serde(alias = "NO_FEVERS")]
    NoFevers,
    #[serde(alias = "NO_PHYS_ATT_GAIN")]
    NoPhysAttGain,
    #[serde(alias = "NO_PHYS_ATT_RUST")]
    NoPhysAttRust,
    #[serde(alias = "NO_SLEEP")]
    NoSleep,
    #[serde(alias = "NO_THOUGHT_CENTER_FOR_MOVEMENT")]
    NoThoughtCenterForMovement,
    #[serde(alias = "NOBREATHE")]
    NoBreathe,
    #[serde(alias = "NOEMOTION")]
    NoEmotion,
    #[serde(alias = "NOEXERT")]
    NoExert,
    #[serde(alias = "NOFEAR")]
    NoFear,
    #[serde(alias = "NONAUSEA")]
    NoNausea,
    #[serde(alias = "NOPAIN")]
    NoPain,
    #[serde(alias = "NOSTUN")]
    NoStun,
    #[serde(alias = "NOT_LIVING")]
    NotLiving,
    #[serde(alias = "NOTHOUGHT")]
    NoThought,
    #[serde(alias = "OPPOSED_TO_LIFE")]
    OpposedToLife,
    #[serde(alias = "PARALYZEIMMUNE")]
    ParalyzeImmune,
    #[serde(alias = "SUPERNATURAL")]
    Supernatural,
    #[serde(alias = "TRANCES")]
    Trances,
    #[serde(alias = "UTTERANCES")]
    Utterances,
}
impl Default for TargetPropertyEnum {
    fn default() -> Self {
        Self::FitForAnimation
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SynTransmittionMethodEnum {
    /// If the syndrome is tied to a material, creatures who eat or drink substances comprising,
    /// containing or contaminated with this material will contract the syndrome if this token is
    /// included. This includes prepared meals when any of the constituent ingredients contains the
    /// material in question.
    ///
    /// This also applies to grazing creatures which happen to munch on a grass that has an
    /// ingestion-triggered syndrome tied to any of its constituent materials.
    #[serde(alias = "SYN_INGESTED")]
    SynIngested,
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
    #[serde(alias = "SYN_INJECTED")] // TODO mark as broken, see #83
    SynInjected,
    /// If the syndrome is tied to a material, creatures who come into contact with this material
    /// will contract the syndrome if this token is included in the syndrome definition. Contact
    /// transmission occurs when a creature's body becomes contaminated with the material (visible
    /// as "`<material name> <smear/dusting/covering>`" over body parts when viewing the
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
    #[serde(alias = "SYN_CONTACT")] // TODO mark as broken, see #83
    SynContact,
    /// If the syndrome is tied to a material, creatures who inhale the material will contract the
    /// syndrome if this token is included. Materials can only be inhaled in their gaseous state,
    /// which is attainable by boiling, or in the form of a `TRAILING_GAS_FLOW`, `UNDIRECTED_GAS` or
    /// `WEATHER_CREEPING_GAS`. Creatures can also be made to leak gaseous tissue when damaged.
    ///
    /// Note that `[AQUATIC]` creatures never inhale gaseous materials, and creatures which do
    /// breathe air aren't guaranteed to inhale gases when exposed to them for a short time.
    /// Contrary to what one might expect, creatures with `[NOBREATHE]` are in fact capable of
    /// contracting inhalation syndromes; this is presumably a bug.
    #[serde(alias = "SYN_INHALED")] // TODO mark as broken, see #83
    SynInhaled,
}
impl Default for SynTransmittionMethodEnum {
    fn default() -> Self {
        Self::SynIngested
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BreathFlowEnum {
    /// Emits a wide cone of dragon fire that burns target creatures at a scorching 50000 °U .
    #[serde(alias = "DRAGONFIRE")]
    Dragonfire,
    /// Emits a narrow cone of fire that burns target creatures at 11000 °U .
    #[serde(alias = "FIREJET")]
    Firejet,
    /// Emits a fireball that burns the target creature.
    #[serde(alias = "FIREBALL")]
    Fireball,
}
impl Default for BreathFlowEnum {
    fn default() -> Self {
        Self::Dragonfire
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BreathMaterialEnum {
    /// Shoots a trail of solid dust at the target. Appears to use cave-in dust physics, as the
    /// dust cloud will fling around anything it comes in contact with (including the creature who
    /// emitted it), making it capable of smashing creatures into the ground and flinging them
    /// over walls. Creatures caught in the dust cloud will be covered with dust; this will
    /// trigger any associated contact syndromes.
    #[serde(alias = "TRAILING_DUST_FLOW")]
    TrailingDustFlow,
    /// Shoots a trail of liquid mist at the target. Creatures caught in the vapor will be covered
    /// with the condensed liquid; this will trigger any associated contact syndromes.
    #[serde(alias = "TRAILING_VAPOR_FLOW")]
    TrailingVaporFlow,
    /// Shoots a trail of gaseous substance at the target. This can be inhaled, triggering any
    /// associated inhalation syndromes.
    #[serde(alias = "TRAILING_GAS_FLOW")]
    TrailingGasFlow,
    /// Shoots a "cloud" of items at the target, leaving piles of this item on the floor.
    ///
    /// Note that this does not create the actual items or use falling item mechanics (meaning no
    /// flying daggers or Touhou-style barrages, unfortunately).
    ///
    /// Instead, this token acts as `TRAILING_GAS_FLOW`, except that the material will use its
    /// normal temperature - for example, a breath attack of steel anvils will envelop the target
    /// in a "burst of steel".
    #[serde(alias = "TRAILING_ITEM_FLOW")]
    TrailingItemFlow,
    /// The creature releases a cloud of solid dust which spreads and dissipates. Similar to
    /// `TRAILING_DUST_FLOW`, but undirected, thus affecting a larger area but losing the distance;
    /// range is roughly the same as that of a cave-in. Creature will attack as normal.
    ///
    /// DO NOT USE THIS TAG UNLESS YOU WANT TO KILL THE CREATURE AND EVERYTHING NEAR IT AND SEND
    /// PEOPLE FLYING. You know you want to.
    #[serde(alias = "UNDIRECTED_DUST")]
    UndirectedDust,
    /// The creature releases a cloud of liquid mist which spreads and dissipates. Similar to
    /// `TRAILING_VAPOR_FLOW`, but undirected, thus affecting a larger area but losing the
    /// distance.
    #[serde(alias = "UNDIRECTED_VAPOR")]
    UndirectedVapor,
    /// The creature releases a cloud of gaseous material which spreads and dissipates. Similar to
    /// `TRAILING_GAS_FLOW`, but undirected, thus affecting a larger area but losing the distance.
    #[serde(alias = "UNDIRECTED_GAS")]
    UndirectedGas,
    /// The creature releases a "cloud" of items at the target, leaving piles of this item on the
    /// floor.
    ///
    /// The same comments apply as `TRAILING_ITEM_FLOW`.
    #[serde(alias = "UNDIRECTED_ITEM_CLOUD")]
    UndirectedItemCloud,
    /// Creates a cloud of creeping dust. Not usable by creatures.
    #[serde(alias = "WEATHER_CREEPING_DUST")]
    WeatherCreepingDust,
    /// Creates a cloud of creeping vapor. Not usable by creatures.
    #[serde(alias = "WEATHER_CREEPING_VAPOR")]
    WeatherCreepingVapor,
    /// Creates a cloud of gas that appears at the edge of the map and slowly creeps across the
    /// map. Not usable by creatures.
    #[serde(alias = "WEATHER_CREEPING_GAS")]
    WeatherCreepingGas,
    /// Causes it to start raining a particular material. If the material is solid at the outdoor
    /// temperatures, it will snow the material instead. Can transfer contact syndromes.
    /// Regardless of the nature of the material, being caught in it will give dwarves the
    /// negative thought of being 'caught in freakish weather lately'. Not usable by creatures.
    #[serde(alias = "WEATHER_FALLING_MATERIAL")]
    WeatherFallingMaterial,
    /// Shoots a solid glob of spinning substance at the target, leaving a symbol similar to
    /// broken arrows, if it misses. Essentially a projectile weapon. If the cooldown rate is
    /// short enough, some creatures with this breath attack will not move, preferring instead to
    /// hold position and shoot globs at their enemies, even when they are right next to them.
    #[serde(alias = "SOLID_GLOB")]
    SolidGlob,
    /// Just like `SOLID_GLOB`, but more harmful, as it shoots a sharpened solid chunk of material
    /// instead.
    #[serde(alias = "SHARP_ROCK")]
    SharpRock,
    /// Shoots a liquid glob of substance at the target. Contact syndromes will take effect if the glob hits the target's exposed skin.
    #[serde(alias = "LIQUID_GLOB")]
    LiquidGlob,
    /// Creates a pile of powder at the specified location.
    #[serde(alias = "SPATTER_POWDER")]
    SpatterPowder,
    /// Creates a pool of liquid at the specified location.
    #[serde(alias = "SPATTER_LIQUID")]
    SpatterLiquid,
    /// Emits a burst of webs that entangle target creatures.
    #[serde(alias = "WEB_SPRAY")]
    WebSpray,
}
impl Default for BreathMaterialEnum {
    fn default() -> Self {
        Self::TrailingDustFlow
    }
}
