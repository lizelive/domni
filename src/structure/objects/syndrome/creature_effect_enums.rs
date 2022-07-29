use serde::{Deserialize, Serialize};

// region: Unique single CE_X enums ===============================================================
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BpEnum {
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
    #[serde(alias = "BP")]
    Bp,
}
impl Default for BpEnum {
    fn default() -> Self {
        Self::Bp
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NameEnum {
    #[serde(alias = "NAME")]
    Name,
}
impl Default for NameEnum {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TileEnum {
    #[serde(alias = "TILE")]
    Tile,
}
impl Default for TileEnum {
    fn default() -> Self {
        Self::Tile
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum FrequencyEnum {
    /// How many frames to use the normal tile, and how many frames to use the "syndrome" tile for.
    #[serde(alias = "FREQUENCY")]
    Frequency,
}
impl Default for FrequencyEnum {
    fn default() -> Self {
        Self::Frequency
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SpeedEnum {
    /// Modifies a creature's in-game speed by a specified percentage; higher numbers are faster.
    #[serde(alias = "SPEED_PERC")]
    SpeedPerc,
    /// Adds to a creature's `[SPEED:XX]` token, with higher numbers being slower.
    /// Negative numbers are accepted, though will only reduce a creature's speed to zero.
    #[serde(alias = "SPEED_ADD")]
    SpeedAdd,
}
impl Default for SpeedEnum {
    fn default() -> Self {
        Self::SpeedPerc
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum PercEnum {
    /// Specifies a percentage of the creature's current skill.
    #[serde(alias = "PERC")]
    Perc,
}
impl Default for PercEnum {
    fn default() -> Self {
        Self::Perc
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum PercOnEnum {
    /// Specifies the probability of the effect being applied on a particular skill roll.
    #[serde(alias = "PERC_ON")]
    PercOn,
}
impl Default for PercOnEnum {
    fn default() -> Self {
        Self::PercOn
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AppearanceModifierEnum {
    /// Selects a type of appearance modifier to be altered.
    #[serde(alias = "APPEARANCE_MODIFIER")]
    AppearanceModifier,
}
impl Default for AppearanceModifierEnum {
    fn default() -> Self {
        Self::AppearanceModifier
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AppModTypeEnum {
    /// The height of the body part.
    #[serde(alias = "HEIGHT")]
    Height,
    /// The length of the body part.
    #[serde(alias = "LENGTH")]
    Length,
    /// The broadness of the body part.
    #[serde(alias = "BROADNESS")]
    Broadness,
}
impl Default for AppModTypeEnum {
    fn default() -> Self {
        Self::Height
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MatMultEnum {
    #[serde(alias = "MAT_MULT")]
    MatMult,
}
impl Default for MatMultEnum {
    fn default() -> Self {
        Self::MatMult
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum InteractionEnum {
    /// References the interaction ID to be used for this special attack.
    #[serde(alias = "INTERACTION")]
    Interaction,
}
impl Default for InteractionEnum {
    fn default() -> Self {
        Self::Interaction
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MatTokenEnum {
    /// Specifies the name/ID of a local creature material.
    #[serde(alias = "MAT_TOKEN")]
    MatToken,
}
impl Default for MatTokenEnum {
    fn default() -> Self {
        Self::MatToken
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ReservedBloodEnum {
    /// `RESERVED_BLOOD` is a special body material token which can be used to
    /// specify the `[BLOOD]` material of any creature, regardless of the material's actual ID.
    #[serde(alias = "RESERVED_BLOOD")]
    ReservedBlood,
}
impl Default for ReservedBloodEnum {
    fn default() -> Self {
        Self::ReservedBlood
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ClassEnum {
    /// Specifies a creature class (for instance, `GENERAL_POISON`).
    #[serde(alias = "CLASS")]
    Class,
}
impl Default for ClassEnum {
    fn default() -> Self {
        Self::Class
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum EmotionEnum {
    /// Specifies the emotion.
    #[serde(alias = "EMOTION")]
    Emotion,
}
impl Default for EmotionEnum {
    fn default() -> Self {
        Self::Emotion
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum FacetEnum {
    /// Specifies a [personality trait/facet](https://dwarffortresswiki.org/index.php/Personality_trait#Facets).
    #[serde(alias = "FACET")]
    Facet,
}
impl Default for FacetEnum {
    fn default() -> Self {
        Self::Facet
    }
}
// endregion ======================================================================================

// region: General CE enums =======================================================================
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum PeriodicTriggerEnum {
    /// When this token is placed after a syndrome effect, it will prevent that effect from working
    /// unless within the specified period range.
    ///
    /// For example, generated werebeast syndromes have a body transformation effect with
    /// `[CE:PERIODIC:MOON_PHASE:27:0]`, which makes the transformation active only throughout moon
    /// phases 27 to 0 (the full moon period). Once the moon phase changes from 0 to 1, the
    /// transformation will end and remain inactive until phase 27 is reached again (unless of
    /// course the effect has an `END` time which is reached before this happens. On that note, keep
    /// in mind that the `START` time of the effect needs to have been reached for activation to
    /// have become possible).
    ///
    /// Only one periodic trigger may currently be specified per effect. Counter triggers can also
    /// be specified for the same effect, in which case both the periodic trigger and at least one
    /// counter trigger will need to have its conditions met for the effect to be allowed to work.
    ///
    /// `MOON_PHASE` is currently the only valid period type.
    #[serde(alias = "PERIODIC")]
    Periodic,
}
impl Default for PeriodicTriggerEnum {
    fn default() -> Self {
        Self::Periodic
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum CounterTriggerEnum {
    /// Creatures in Dwarf Fortress possess internal counters which keep track of their various
    /// activities and statuses. When this token is placed after a syndrome effect, it will prevent
    /// the effect from working unless the affected creature has the indicated counter, and its
    /// value lies within the specified range.
    ///
    /// For example, generated vampire syndromes use
    /// `[CE:COUNTER_TRIGGER:DRINKING_BLOOD:1:NONE:REQUIRED]` with an appearance modifier to make
    /// the vampire's teeth temporarily lengthen whilst leeching blood.
    ///
    /// Note that `NONE` can be used in place of `<max_value>` to indicate that any value above
    /// `<min_value>` is valid. `NONE` can also be used in place of `<min_value>`, which is
    /// equivalent to the lowest value attainable by a counter.
    ///
    /// Most counters only exist temporarily, so their use as triggers is
    /// somewhat more restricted than intuition suggests. For example, specifying 0 or `NONE` as the
    /// `<min_value>` for a `CAVE_ADAPT` trigger wouldn't permit the effect to work when the
    /// affected creature is outside, since this counter is removed from the unit as soon as its
    /// value decreases past 1. Similarly, `MILK_COUNTER` is only present for some time after a
    /// creature is milked.
    ///
    /// Multiple counter triggers can be specified per effect, in which case the effect will be
    /// permitted to work if at least one of the trigger conditions is met. A periodic trigger can
    /// also be specified for the same effect, in which case both the periodic trigger and at least
    /// one counter trigger will need to have their conditions met for the effect to work.
    #[serde(alias = "COUNTER_TRIGGER")]
    CounterTrigger,
}
impl Default for CounterTriggerEnum {
    fn default() -> Self {
        Self::CounterTrigger
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MoonPhaseEnum {
    /// The lunar cycle in Dwarf Fortress is composed of 28 segments (each slightly shorter than a
    /// day in duration), with each segment represented by a value ranging from 0 to 27. These
    /// correspond to moon phases as follows:
    /// - 0 = full moon
    /// - 1-4 = waning gibbous
    /// - 5-8 = waning half
    /// - 9-12 = waning crescent
    /// - 13-14 = new moon
    /// - 15-18 = waxing crescent
    /// - 19-22 = waxing half
    /// - 23-26 = waxing gibbous
    /// - 27 = full moon
    #[serde(alias = "MOON_PHASE")]
    MoonPhase,
}
impl Default for MoonPhaseEnum {
    fn default() -> Self {
        Self::MoonPhase
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum CounterTypesEnum {
    /// For `[ALCOHOL_DEPENDENT]` creatures, this counter increases by 1 each tick, and is reset to
    /// 0 when the creature drinks alcohol. The following messages are added after "needs alcohol to
    /// get through the working day" in the creature's description when the counter reaches the
    /// specified values:
    /// - 100800 (3 months) = and is starting to work slowly due to its scarcity
    /// - 201600 (6 months) = and really wants a drink
    /// - 302400 (9 months) = and has gone without a drink for far, far too long
    /// - 403200 (1 year) = and can't even remember the last time (s)he had some
    #[serde(alias = "ALCOHOLIC")]
    Alcoholic,
    /// For creatures with the `[CAVE_ADAPT]` token, this counter is created and increases by 1 each
    /// tick when the creature is in the Dark, and decreases by 10 each tick when Outside.
    /// The counter is removed if it decreases to 0.
    /// See [cave adaptation](https://dwarffortresswiki.org/index.php/Cave_adaptation) for more information.
    /// - 403200 (1 year) = going outside causes irritation
    /// - 604800 (1.5 years) = going outside causes nausea
    #[serde(alias = "CAVE_ADAPT")]
    CaveAdapt,
    /// When a creature is milked, this counter is created and set to the frequency value specified
    /// in the creature's `[MILKABLE]` token, and subsequently decreases by 1 each tick until it
    /// reaches 0, at which point it is immediately removed, making the creature available for
    /// milking again.
    #[serde(alias = "MILK_COUNTER")]
    MilkCounter,
    /// This counter is created and set to 100800 (3 months' worth of ticks in fortress mode) when a
    /// creature lays eggs, and thereafter decreases by 1 each tick until it reaches 0, at which
    /// point it is removed and the creature regains the ability to lay eggs.
    #[serde(alias = "EGG_SPENT")]
    EggSpent,
    /// How angry (and likely to attack) an animal is from being in an overcrowded location. The
    /// counter is created and set to 200 when the animal is forced to lie on the ground whilst
    /// sharing a tile with another creature. It subsequently decreases by 1 each tick, but this is
    /// overcome by the addition of 200 every so often (with a variable delay between each spike) if
    /// the creature remains grounded. The counter is removed if it decreases to 0.
    #[serde(alias = "GROUNDED_ANIMAL_ANGER")]
    GroundedAnimalAnger,
    /// This counter rises by 1 every tick for creatures with the `[BLOODSUCKER]` token. When it
    /// rises high enough (generally around 100800; 3 months in fortress mode time), the creature
    /// will seek an unconscious victim to leech off of. Blood-sucking causes the counter to
    /// decrease, and will continue until either the victim is dead or the counter reaches 0. Note
    /// that this counter isn't removed when 0 is reached.
    ///
    /// When playing as a bloodsucker in adventure mode, the following bloodthirst indicators are
    /// displayed when this counter reaches the specified values:
    /// - 172800 (1 day in adventure mode time) = Thirsty
    /// - 1209600 (1 week) = Thirsty!
    /// - 2419200 (2 weeks) = Thirsty!
    ///
    /// Various penalties are applied as bloodthirst increases; see the
    /// [vampire](https://dwarffortresswiki.org/index.php/Vampire) article for more information.
    #[serde(alias = "TIME_SINCE_SUCKED_BLOOD")]
    TimeSinceSuckedBlood,
    /// This appears to be created and set to a fixed value of 20 whilst the creature is sucking
    /// blood, and begins to decrease by 1 each tick once blood-sucking ceases (as described above)
    /// until it reaches 0, at which point the counter is removed.
    #[serde(alias = "DRINKING_BLOOD")]
    DrinkingBlood,
    /// How long before the creature will decide to attend another party. The counter is set to the
    /// tick equivalent of around 3 months when the party being attended ends, and subsequently
    /// counts down to 0. Redundant as of 0.42.01, since parties no longer occur.
    #[serde(alias = "PARTIED_OUT")]
    PartiedOut,
}
impl Default for CounterTypesEnum {
    fn default() -> Self {
        Self::Alcoholic
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum RequiredEnum {
    /// `REQUIRED` implies that the effect won't proceed if the counter exists but doesn't lie within
    /// the range provided. However, it's actually redunant as `COUNTER_TRIGGER` always checks for
    /// both of these conditions; replacing it with `NONE` doesn't alter the way the trigger
    /// functions, though it will fail to work if this slot is left empty instead.
    #[serde(alias = "REQUIRED")]
    Required,
}
impl Default for RequiredEnum {
    fn default() -> Self {
        Self::Required
    }
}
// endregion ======================================================================================

// TODO: append this to the bottom of each enum value when hovering for a description
/// Creatures have an emotional response to certain circumstances. Different creatures have
/// differing responses to the same circumstance due to their personalities.
/// Recent emotion/circumstance pairs are listed in the
/// [Thoughts and Preferences](https://dwarffortresswiki.org/index.php/Thoughts_and_Preferences)
/// screen. Different thoughts can have different strengths, depending on time elapsed dwarven
/// personality. This thought strength is then divided by the "divisor" number given above.
///
/// Note that positive thoughts have a negative number, as they _reduce_ stress. Numbers closer
/// to 1 or -1 have the strongest effect on stress (remember, these are *divisors*).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum EmotionTypeEnum {
    /// Emotion: Anything
    ///
    /// Divisor: 0
    #[serde(alias = "ANYTHING")]
    Anything,
    /// Emotion: Acceptance
    ///
    /// Divisor: -8
    #[serde(alias = "ACCEPTANCE")]
    Acceptance,
    /// Emotion: Adoration
    ///
    /// Divisor: -1
    #[serde(alias = "ADORATION")]
    Adoration,
    /// Emotion: Affection
    ///
    /// Divisor: -2
    #[serde(alias = "AFFECTION")]
    Affection,
    /// Emotion: Agitation
    ///
    /// Divisor: 4
    #[serde(alias = "AGITATION")]
    Agitation,
    /// Emotion: Aggravation
    ///
    /// Divisor: 4
    #[serde(alias = "AGGRAVATION")]
    Aggravation,
    /// Emotion: Agony
    ///
    /// Divisor: 1
    #[serde(alias = "AGONY")]
    Agony,
    /// Emotion: Alarm
    ///
    /// Divisor: 4
    #[serde(alias = "ALARM")]
    Alarm,
    /// Emotion: Alienation
    ///
    /// Divisor: 8
    #[serde(alias = "ALIENATION")]
    Alienation,
    /// Emotion: Amazement
    ///
    /// Divisor: 0
    #[serde(alias = "AMAZEMENT")]
    Amazement,
    /// Emotion: Ambivalence
    ///
    /// Divisor: 0
    #[serde(alias = "AMBIVALENCE")]
    Ambivalence,
    /// Emotion: Amusement
    ///
    /// Divisor: -4
    #[serde(alias = "AMUSEMENT")]
    Amusement,
    /// Emotion: Anger
    ///
    /// Divisor: 2
    #[serde(alias = "ANGER")]
    Anger,
    /// Emotion: Angst
    ///
    /// Divisor: 1
    #[serde(alias = "ANGST")]
    Angst,
    /// Emotion: Anguish
    ///
    /// Divisor: 1
    #[serde(alias = "ANGUISH")]
    Anguish,
    /// Emotion: Annoyance
    ///
    /// Divisor: 8
    #[serde(alias = "ANNOYANCE")]
    Annoyance,
    /// Emotion: Anxiety
    ///
    /// Divisor: 4
    #[serde(alias = "ANXIETY")]
    Anxiety,
    /// Emotion: Apathy
    ///
    /// Divisor: 0
    #[serde(alias = "APATHY")]
    Apathy,
    /// Emotion: Arousal
    ///
    /// Divisor: -8
    #[serde(alias = "AROUSAL")]
    Arousal,
    /// Emotion: Astonishment
    ///
    /// Divisor: 0
    #[serde(alias = "ASTONISHMENT")]
    Astonishment,
    /// Emotion: Aversion
    ///
    /// Divisor: 4
    #[serde(alias = "AVERSION")]
    Aversion,
    /// Emotion: Awe
    ///
    /// Divisor: 0
    #[serde(alias = "AWE")]
    Awe,
    /// Emotion: Bitterness
    ///
    /// Divisor: 2
    #[serde(alias = "BITTERNESS")]
    Bitterness,
    /// Emotion: Bliss
    ///
    /// Divisor: -1
    #[serde(alias = "BLISS")]
    Bliss,
    /// Emotion: Boredom
    ///
    /// Divisor: 8
    #[serde(alias = "BOREDOM")]
    Boredom,
    /// Emotion: Caring
    ///
    /// Divisor: -2
    #[serde(alias = "CARING")]
    Caring,
    /// Emotion: Confusion
    ///
    /// Divisor: 8
    #[serde(alias = "CONFUSION")]
    Confusion,
    /// Emotion: Contempt
    ///
    /// Divisor: 4
    #[serde(alias = "CONTEMPT")]
    Contempt,
    /// Emotion: Contentment
    ///
    /// Divisor: -8
    #[serde(alias = "CONTENTMENT")]
    Contentment,
    /// Emotion: Defeat
    ///
    /// Divisor: 2
    #[serde(alias = "DEFEAT")]
    Defeat,
    /// Emotion: Dejection
    ///
    /// Divisor: 4
    #[serde(alias = "DEJECTION")]
    Dejection,
    /// Emotion: Delight
    ///
    /// Divisor: -1
    #[serde(alias = "DELIGHT")]
    Delight,
    /// Emotion: Despair
    ///
    /// Divisor: 1
    #[serde(alias = "DESPAIR")]
    Despair,
    /// Emotion: Disappointment
    ///
    /// Divisor: 8
    #[serde(alias = "DISAPPOINTMENT")]
    Disappointment,
    /// Emotion: Disgust
    ///
    /// Divisor: 4
    #[serde(alias = "DISGUST")]
    Disgust,
    /// Emotion: Disillusionment
    ///
    /// Divisor: 8
    #[serde(alias = "DISILLUSIONMENT")]
    Disillusionment,
    /// Emotion: Dislike
    ///
    /// Divisor: 8
    #[serde(alias = "DISLIKE")]
    Dislike,
    /// Emotion: Dismay
    ///
    /// Divisor: 2
    #[serde(alias = "DISMAY")]
    Dismay,
    /// Emotion: Displeasure
    ///
    /// Divisor: 8
    #[serde(alias = "DISPLEASURE")]
    Displeasure,
    /// Emotion: Distress
    ///
    /// Divisor: 2
    #[serde(alias = "DISTRESS")]
    Distress,
    /// Emotion: Doubt
    ///
    /// Divisor: 8
    #[serde(alias = "DOUBT")]
    Doubt,
    /// Emotion: Eagerness
    ///
    /// Divisor: -4
    #[serde(alias = "EAGERNESS")]
    Eagerness,
    /// Emotion: Elation
    ///
    /// Divisor: -2
    #[serde(alias = "ELATION")]
    Elation,
    /// Emotion: Embarrassment
    ///
    /// Divisor: 8
    #[serde(alias = "EMBARRASSMENT")]
    Embarrassment,
    /// Emotion: Empathy
    ///
    /// Divisor: -2
    #[serde(alias = "EMPATHY")]
    Empathy,
    /// Emotion: Emptiness
    ///
    /// Divisor: 4
    #[serde(alias = "EMPTINESS")]
    Emptiness,
    /// Emotion: Enjoyment
    ///
    /// Divisor: -8
    #[serde(alias = "ENJOYMENT")]
    Enjoyment,
    /// Emotion: Enthusiasm
    ///
    /// Divisor: -8
    #[serde(alias = "ENTHUSIASM")]
    Enthusiasm,
    /// Emotion: Euphoria
    ///
    /// Divisor: -1
    #[serde(alias = "EUPHORIA")]
    Euphoria,
    /// Emotion: Exasperation
    ///
    /// Divisor: 8
    #[serde(alias = "EXASPERATION")]
    Exasperation,
    /// Emotion: Excitement
    ///
    /// Divisor: -2
    #[serde(alias = "EXCITEMENT")]
    Excitement,
    /// Emotion: Exhilaration
    ///
    /// Divisor: -2
    #[serde(alias = "EXHILARATION")]
    Exhilaration,
    /// Emotion: Expectancy
    ///
    /// Divisor: -8
    #[serde(alias = "EXPECTANCY")]
    Expectancy,
    /// Emotion: Fear
    ///
    /// Divisor: 1
    #[serde(alias = "FEAR")]
    Fear,
    /// Emotion: Ferocity
    ///
    /// Divisor: 2
    #[serde(alias = "FEROCITY")]
    Ferocity,
    /// Emotion: Fondness
    ///
    /// Divisor: -8
    #[serde(alias = "FONDNESS")]
    Fondness,
    /// Emotion: Freedom
    ///
    /// Divisor: -4
    #[serde(alias = "FREEDOM")]
    Freedom,
    /// Emotion: Fright
    ///
    /// Divisor: 2
    #[serde(alias = "FRIGHT")]
    Fright,
    /// Emotion: Frustration
    ///
    /// Divisor: 8
    #[serde(alias = "FRUSTRATION")]
    Frustration,
    /// Emotion: Gaiety
    ///
    /// Divisor: -2
    #[serde(alias = "GAIETY")]
    Gaiety,
    /// Emotion: Glee
    ///
    /// Divisor: -2
    #[serde(alias = "GLEE")]
    Glee,
    /// Emotion: Gloom
    ///
    /// Divisor: 4
    #[serde(alias = "GLOOM")]
    Gloom,
    /// Emotion: Glumness
    ///
    /// Divisor: 8
    #[serde(alias = "GLUMNESS")]
    Glumness,
    /// Emotion: Gratitude
    ///
    /// Divisor: -4
    #[serde(alias = "GRATITUDE")]
    Gratitude,
    /// Emotion: Grief
    ///
    /// Divisor: 2
    #[serde(alias = "GRIEF")]
    Grief,
    /// Emotion: Grim Satisfaction
    ///
    /// Divisor: 0
    #[serde(alias = "GRIM_SATISFACTION")]
    GrimSatisfaction,
    /// Emotion: Grouchiness
    ///
    /// Divisor: 8
    #[serde(alias = "GROUCHINESS")]
    Grouchiness,
    /// Emotion: Grumpiness
    ///
    /// Divisor: 8
    #[serde(alias = "GRUMPINESS")]
    Grumpiness,
    /// Emotion: Guilt
    ///
    /// Divisor: 4
    #[serde(alias = "GUILT")]
    Guilt,
    /// Emotion: Happiness
    ///
    /// Divisor: -2
    #[serde(alias = "HAPPINESS")]
    Happiness,
    /// Emotion: Hatred
    ///
    /// Divisor: 2
    #[serde(alias = "HATRED")]
    Hatred,
    /// Emotion: Hope
    ///
    /// Divisor: -2
    #[serde(alias = "HOPE")]
    Hope,
    /// Emotion: Hopelessness
    ///
    /// Divisor: 2
    #[serde(alias = "HOPELESSNESS")]
    Hopelessness,
    /// Emotion: Horror
    ///
    /// Divisor: 1
    #[serde(alias = "HORROR")]
    Horror,
    /// Emotion: Humiliation
    ///
    /// Divisor: 4
    #[serde(alias = "HUMILIATION")]
    Humiliation,
    /// Emotion: Insult
    ///
    /// Divisor: 4
    #[serde(alias = "INSULT")]
    Insult,
    /// Emotion: Interest
    ///
    /// Divisor: -8
    #[serde(alias = "INTEREST")]
    Interest,
    /// Emotion: Irritation
    ///
    /// Divisor: 8
    #[serde(alias = "IRRITATION")]
    Irritation,
    /// Emotion: Isolation
    ///
    /// Divisor: 4
    #[serde(alias = "ISOLATION")]
    Isolation,
    /// Emotion: Jolliness
    ///
    /// Divisor: -4
    #[serde(alias = "JOLLINESS")]
    Jolliness,
    /// Emotion: Joviality
    ///
    /// Divisor: -2
    #[serde(alias = "JOVIALITY")]
    Joviality,
    /// Emotion: Joy
    ///
    /// Divisor: -1
    #[serde(alias = "JOY")]
    Joy,
    /// Emotion: Jubilation
    ///
    /// Divisor: -1
    #[serde(alias = "JUBILATION")]
    Jubilation,
    /// Emotion: Loathing
    ///
    /// Divisor: 2
    #[serde(alias = "LOATHING")]
    Loathing,
    /// Emotion: Loneliness
    ///
    /// Divisor: 4
    #[serde(alias = "LONELINESS")]
    Loneliness,
    /// Emotion: Love
    ///
    /// Divisor: -1
    #[serde(alias = "LOVE")]
    Love,
    /// Emotion: Lust
    ///
    /// Divisor: -8
    #[serde(alias = "LUST")]
    Lust,
    /// Emotion: Misery
    ///
    /// Divisor: 1
    #[serde(alias = "MISERY")]
    Misery,
    /// Emotion: Mortification
    ///
    /// Divisor: 2
    #[serde(alias = "MORTIFICATION")]
    Mortification,
    /// Emotion: Nervousness
    ///
    /// Divisor: 8
    #[serde(alias = "NERVOUSNESS")]
    Nervousness,
    /// Emotion: Nostalgia
    ///
    /// Divisor: -8
    #[serde(alias = "NOSTALGIA")]
    Nostalgia,
    /// Emotion: Optimism
    ///
    /// Divisor: -4
    #[serde(alias = "OPTIMISM")]
    Optimism,
    /// Emotion: Outrage
    ///
    /// Divisor: 2
    #[serde(alias = "OUTRAGE")]
    Outrage,
    /// Emotion: Panic
    ///
    /// Divisor: 1
    #[serde(alias = "PANIC")]
    Panic,
    /// Emotion: Patience
    ///
    /// Divisor: -8
    #[serde(alias = "PATIENCE")]
    Patience,
    /// Emotion: Passion
    ///
    /// Divisor: -2
    #[serde(alias = "PASSION")]
    Passion,
    /// Emotion: Pessimism
    ///
    /// Divisor: 8
    #[serde(alias = "PESSIMISM")]
    Pessimism,
    /// Emotion: Pleasure
    ///
    /// Divisor: -4
    #[serde(alias = "PLEASURE")]
    Pleasure,
    /// Emotion: Pride
    ///
    /// Divisor: -4
    #[serde(alias = "PRIDE")]
    Pride,
    /// Emotion: Rage
    ///
    /// Divisor: 1
    #[serde(alias = "RAGE")]
    Rage,
    /// Emotion: Rapture
    ///
    /// Divisor: -1
    #[serde(alias = "RAPTURE")]
    Rapture,
    /// Emotion: Rejection
    ///
    /// Divisor: 4
    #[serde(alias = "REJECTION")]
    Rejection,
    /// Emotion: Relief
    ///
    /// Divisor: -2
    #[serde(alias = "RELIEF")]
    Relief,
    /// Emotion: Regret
    ///
    /// Divisor: 8
    #[serde(alias = "REGRET")]
    Regret,
    /// Emotion: Remorse
    ///
    /// Divisor: 4
    #[serde(alias = "REMORSE")]
    Remorse,
    /// Emotion: Repentance
    ///
    /// Divisor: -2
    #[serde(alias = "REPENTANCE")]
    Repentance,
    /// Emotion: Resentment
    ///
    /// Divisor: 8
    #[serde(alias = "RESENTMENT")]
    Resentment,
    /// Emotion: Righteous Indignation
    ///
    /// Divisor: 8
    #[serde(alias = "RIGHTEOUS_INDIGNATION")]
    RighteousIndignation,
    /// Emotion: Sadness
    ///
    /// Divisor: 4
    #[serde(alias = "SADNESS")]
    Sadness,
    /// Emotion: Satisfaction
    ///
    /// Divisor: -8
    #[serde(alias = "SATISFACTION")]
    Satisfaction,
    /// Emotion: Self Pity
    ///
    /// Divisor: 8
    #[serde(alias = "SELF_PITY")]
    SelfPity,
    /// Emotion: Servile
    ///
    /// Divisor: 0
    #[serde(alias = "SERVILE")]
    Servile,
    /// Emotion: Shaken
    ///
    /// Divisor: 1
    #[serde(alias = "SHAKEN")]
    Shaken,
    /// Emotion: Shame
    ///
    /// Divisor: 4
    #[serde(alias = "SHAME")]
    Shame,
    /// Emotion: Shock
    ///
    /// Divisor: 1
    #[serde(alias = "SHOCK")]
    Shock,
    /// Emotion: Suspicion
    ///
    /// Divisor: 8
    #[serde(alias = "SUSPICION")]
    Suspicion,
    /// Emotion: Sympathy
    ///
    /// Divisor: -8
    #[serde(alias = "SYMPATHY")]
    Sympathy,
    /// Emotion: Tenderness
    ///
    /// Divisor: -2
    #[serde(alias = "TENDERNESS")]
    Tenderness,
    /// Emotion: Terror
    ///
    /// Divisor: 1
    #[serde(alias = "TERROR")]
    Terror,
    /// Emotion: Thrill
    ///
    /// Divisor: -2
    #[serde(alias = "THRILL")]
    Thrill,
    /// Emotion: Triumph
    ///
    /// Divisor: -2
    #[serde(alias = "TRIUMPH")]
    Triumph,
    /// Emotion: Uneasiness
    ///
    /// Divisor: 8
    #[serde(alias = "UNEASINESS")]
    Uneasiness,
    /// Emotion: Unhappiness
    ///
    /// Divisor: 4
    #[serde(alias = "UNHAPPINESS")]
    Unhappiness,
    /// Emotion: Vengefulness
    ///
    /// Divisor: 4
    #[serde(alias = "VENGEFULNESS")]
    Vengefulness,
    /// Emotion: Wonder
    ///
    /// Divisor: -8
    #[serde(alias = "WONDER")]
    Wonder,
    /// Emotion: Worry
    ///
    /// Divisor: 8
    #[serde(alias = "WORRY")]
    Worry,
    /// Emotion: Wrath
    ///
    /// Divisor: 1
    #[serde(alias = "WRATH")]
    Wrath,
    /// Emotion: Zeal
    ///
    /// Divisor: -4
    #[serde(alias = "ZEAL")]
    Zeal,
    /// Emotion: Restless
    ///
    /// Divisor: 8
    #[serde(alias = "RESTLESS")]
    Restless,
    /// Emotion: Admiration
    ///
    /// Divisor: -8
    #[serde(alias = "ADMIRATION")]
    Admiration,
}
impl Default for EmotionTypeEnum {
    fn default() -> Self {
        Self::Anything
    }
}
