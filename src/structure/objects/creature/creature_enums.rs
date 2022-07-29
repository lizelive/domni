use crate::core::ReferenceTo;

use serde::{Deserialize, Serialize};

use crate::structure::{CreatureToken, PlantToken};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SingularOrPluralEnum {
    #[serde(alias = "SINGULAR")]
    Singular,
    #[serde(alias = "PLURAL")]
    Plural,
}
impl Default for SingularOrPluralEnum {
    fn default() -> Self {
        Self::Singular
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AlertOrPeacefulIntermittentEnum {
    #[serde(alias = "ALERT")]
    Alert,
    #[serde(alias = "PEACEFUL_INTERMITTENT")]
    PeacefulIntermittent,
}
impl Default for AlertOrPeacefulIntermittentEnum {
    fn default() -> Self {
        Self::Alert
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum VocalizationEnum {
    #[serde(alias = "VOCALIZATION")]
    Vocalization,
}
impl Default for VocalizationEnum {
    fn default() -> Self {
        Self::Vocalization
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TissueModifierEnum {
    #[serde(alias = "LENGTH")]
    Length,
    #[serde(alias = "DENSE")]
    Dense,
    #[serde(alias = "HIGH_POSITION")]
    HighPosition,
    #[serde(alias = "CURLY")]
    Curly,
    #[serde(alias = "GREASY")]
    Greasy,
    #[serde(alias = "WRINKLY")]
    Wrinkly,
}
impl Default for TissueModifierEnum {
    fn default() -> Self {
        Self::Length
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AppGeneticModelEnum {
    #[serde(alias = "DOMINANT_MORE")]
    DominantMore,
    #[serde(alias = "DOMINANT_LESS")]
    DominantLess,
    #[serde(alias = "MIX")]
    Mix,
}
impl Default for AppGeneticModelEnum {
    fn default() -> Self {
        Self::DominantMore
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SecretionTriggerEnum {
    /// Secretion occurs once every 40 ticks in fortress mode, and every tick in adventurer mode.
    #[serde(alias = "CONTINUOUS")]
    Continuous,
    /// Secretion occurs continuously (every 40 ticks in fortress mode, and every tick in adventurer
    /// mode) whilst the creature is at minimum `Tired` following physical exertion. Note that this
    /// cannot occur if the creature has `[NOEXERT]`.
    #[serde(alias = "EXERTION")]
    Exertion,
    /// Secretion occurs continuously (every 40 ticks in fortress mode, and every tick in adventurer
    /// mode) whilst the creature is distressed. Cannot occur in creatures with `[NOEMOTION]`.
    #[serde(alias = "EXTREME_EMOTION")]
    ExtremeEmotion,
}
impl Default for SecretionTriggerEnum {
    fn default() -> Self {
        Self::Continuous
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum LairCharacteristicEnum {
    #[serde(alias = "HAS_DOORS")]
    HasDoors,
}
impl Default for LairCharacteristicEnum {
    fn default() -> Self {
        Self::HasDoors
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum LairTypeEnum {
    #[serde(alias = "SIMPLE_BURROW")]
    SimpleBurrow,
    #[serde(alias = "SIMPLE_MOUND")]
    SimpleMound,
    #[serde(alias = "WILDERNESS_LOCATION")]
    WildernessLocation,
    #[serde(alias = "SHRINE")]
    Shrine,
    #[serde(alias = "LABYRINTH")]
    Labyrinth,
}
impl Default for LairTypeEnum {
    fn default() -> Self {
        Self::SimpleBurrow
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TestAllEnum {
    #[serde(alias = "TEST_ALL")]
    TestAll,
}
impl Default for TestAllEnum {
    fn default() -> Self {
        Self::TestAll
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum HabitTypeEnum {
    #[serde(alias = "COLLECT_TROPHIES")]
    CollectTrophies,
    #[serde(alias = "COOK_PEOPLE")]
    CookPeople,
    #[serde(alias = "COOK_VERMIN")]
    CookVermin,
    #[serde(alias = "GRIND_VERMIN")]
    GrindVermin,
    #[serde(alias = "COOK_BLOOD")]
    CookBlood,
    #[serde(alias = "GRIND_BONE_MEAL")]
    GrindBoneMeal,
    #[serde(alias = "EAT_BONE_PORRIDGE")]
    EatBonePorridge,
    #[serde(alias = "USE_ANY_MELEE_WEAPON")]
    UseAnyMeleeWeapon,
    #[serde(alias = "GIANT_NEST")]
    GiantNest,
    #[serde(alias = "COLLECT_WEALTH")]
    CollectWealth,
}
impl Default for HabitTypeEnum {
    fn default() -> Self {
        Self::CollectTrophies
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AnyHardStoneEnum {
    #[serde(alias = "ANY_HARD_STONE")]
    AnyHardStone,
}
impl Default for AnyHardStoneEnum {
    fn default() -> Self {
        Self::AnyHardStone
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum RootEnum {
    #[serde(alias = "ROOT")]
    Root,
}
impl Default for RootEnum {
    fn default() -> Self {
        Self::Root
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TimescaleEnum {
    #[serde(alias = "DAILY")]
    Daily,
    #[serde(alias = "YEARLY")]
    Yearly,
}
impl Default for TimescaleEnum {
    fn default() -> Self {
        Self::Daily
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NoEndEnum {
    #[serde(alias = "NO_END")]
    NoEnd,
}
impl Default for NoEndEnum {
    fn default() -> Self {
        Self::NoEnd
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NormalEnum {
    /// _"Normal just skips the same was as none does. Could be an old compatibility thing? No idea."_
    ///
    /// -- [Toady](http://www.bay12forums.com/smf/index.php?topic=169696.msg8292042#msg8292042)
    #[serde(alias = "NORMAL")]
    Normal,
}
impl Default for NormalEnum {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PlantOrCreatureTokenArg {
    Plant(ReferenceTo<PlantToken>),
    Creature(ReferenceTo<CreatureToken>),
}
impl Default for PlantOrCreatureTokenArg {
    fn default() -> Self {
        Self::Plant(ReferenceTo::new(String::default()))
    }
}
