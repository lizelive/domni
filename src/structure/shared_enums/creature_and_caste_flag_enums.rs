use serde::{Deserialize, Serialize};

// TODO: maybe add descriptions for these where possible/practical;
// copy from applicable creature tokens.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum CreatureFlagEnum {
    #[serde(alias = "ALL_CASTES_ALIVE")]
    AllCastesAlive,
    #[serde(alias = "ARTIFICIAL_HIVEABLE")]
    ArtificialHiveable,
    #[serde(alias = "BIOME_DESERT_BADLAND")]
    BiomeDesertBadland,
    #[serde(alias = "BIOME_DESERT_ROCK")]
    BiomeDesertRock,
    #[serde(alias = "BIOME_DESERT_SAND")]
    BiomeDesertSand,
    #[serde(alias = "BIOME_FOREST_TAIGA")]
    BiomeForestTaiga,
    #[serde(alias = "BIOME_FOREST_TEMPERATE_BROADLEAF")]
    BiomeForestTemperateBroadleaf,
    #[serde(alias = "BIOME_FOREST_TEMPERATE_CONIFER")]
    BiomeForestTemperateConifer,
    #[serde(alias = "BIOME_FOREST_TROPICAL_CONIFER")]
    BiomeForestTropicalConifer,
    #[serde(alias = "BIOME_FOREST_TROPICAL_DRY_BROADLEAF")]
    BiomeForestTropicalDryBroadleaf,
    #[serde(alias = "BIOME_FOREST_TROPICAL_MOIST_BROADLEAF")]
    BiomeForestTropicalMoistBroadleaf,
    #[serde(alias = "BIOME_GLACIER")]
    BiomeGlacier,
    #[serde(alias = "BIOME_GRASSLAND_TEMPERATE")]
    BiomeGrasslandTemperate,
    #[serde(alias = "BIOME_GRASSLAND_TROPICAL")]
    BiomeGrasslandTropical,
    #[serde(alias = "BIOME_LAKE_TEMPERATE_BRACKISHWATER")]
    BiomeLakeTemperateBrackishwater,
    #[serde(alias = "BIOME_LAKE_TEMPERATE_FRESHWATER")]
    BiomeLakeTemperateFreshwater,
    #[serde(alias = "BIOME_LAKE_TEMPERATE_SALTWATER")]
    BiomeLakeTemperateSaltwater,
    #[serde(alias = "BIOME_LAKE_TROPICAL_BRACKISHWATER")]
    BiomeLakeTropicalBrackishwater,
    #[serde(alias = "BIOME_LAKE_TROPICAL_FRESHWATER")]
    BiomeLakeTropicalFreshwater,
    #[serde(alias = "BIOME_LAKE_TROPICAL_SALTWATER")]
    BiomeLakeTropicalSaltwater,
    #[serde(alias = "BIOME_MARSH_TEMPERATE_FRESHWATER")]
    BiomeMarshTemperateFreshwater,
    #[serde(alias = "BIOME_MARSH_TEMPERATE_SALTWATER")]
    BiomeMarshTemperateSaltwater,
    #[serde(alias = "BIOME_MARSH_TROPICAL_FRESHWATER")]
    BiomeMarshTropicalFreshwater,
    #[serde(alias = "BIOME_MARSH_TROPICAL_SALTWATER")]
    BiomeMarshTropicalSaltwater,
    #[serde(alias = "BIOME_MOUNTAIN")]
    BiomeMountain,
    #[serde(alias = "BIOME_OCEAN_ARCTIC")]
    BiomeOceanArctic,
    #[serde(alias = "BIOME_OCEAN_TEMPERATE")]
    BiomeOceanTemperate,
    #[serde(alias = "BIOME_OCEAN_TROPICAL")]
    BiomeOceanTropical,
    #[serde(alias = "BIOME_POOL_TEMPERATE_BRACKISHWATER")]
    BiomePoolTemperateBrackishwater,
    #[serde(alias = "BIOME_POOL_TEMPERATE_FRESHWATER")]
    BiomePoolTemperateFreshwater,
    #[serde(alias = "BIOME_POOL_TEMPERATE_SALTWATER")]
    BiomePoolTemperateSaltwater,
    #[serde(alias = "BIOME_POOL_TROPICAL_BRACKISHWATER")]
    BiomePoolTropicalBrackishwater,
    #[serde(alias = "BIOME_POOL_TROPICAL_FRESHWATER")]
    BiomePoolTropicalFreshwater,
    #[serde(alias = "BIOME_POOL_TROPICAL_SALTWATER")]
    BiomePoolTropicalSaltwater,
    #[serde(alias = "BIOME_RIVER_TEMPERATE_BRACKISHWATER")]
    BiomeRiverTemperateBrackishwater,
    #[serde(alias = "BIOME_RIVER_TEMPERATE_FRESHWATER")]
    BiomeRiverTemperateFreshwater,
    #[serde(alias = "BIOME_RIVER_TEMPERATE_SALTWATER")]
    BiomeRiverTemperateSaltwater,
    #[serde(alias = "BIOME_RIVER_TROPICAL_BRACKISHWATER")]
    BiomeRiverTropicalBrackishwater,
    #[serde(alias = "BIOME_RIVER_TROPICAL_FRESHWATER")]
    BiomeRiverTropicalFreshwater,
    #[serde(alias = "BIOME_RIVER_TROPICAL_SALTWATER")]
    BiomeRiverTropicalSaltwater,
    #[serde(alias = "BIOME_SAVANNA_TEMPERATE")]
    BiomeSavannaTemperate,
    #[serde(alias = "BIOME_SAVANNA_TROPICAL")]
    BiomeSavannaTropical,
    #[serde(alias = "BIOME_SHRUBLAND_TEMPERATE")]
    BiomeShrublandTemperate,
    #[serde(alias = "BIOME_SHRUBLAND_TROPICAL")]
    BiomeShrublandTropical,
    #[serde(alias = "BIOME_SUBTERRANEAN_CHASM")]
    BiomeSubterraneanChasm,
    #[serde(alias = "BIOME_SUBTERRANEAN_LAVA")]
    BiomeSubterraneanLava,
    #[serde(alias = "BIOME_SUBTERRANEAN_WATER")]
    BiomeSubterraneanWater,
    #[serde(alias = "BIOME_SWAMP_MANGROVE")]
    BiomeSwampMangrove,
    #[serde(alias = "BIOME_SWAMP_TEMPERATE_FRESHWATER")]
    BiomeSwampTemperateFreshwater,
    #[serde(alias = "BIOME_SWAMP_TEMPERATE_SALTWATER")]
    BiomeSwampTemperateSaltwater,
    #[serde(alias = "BIOME_SWAMP_TROPICAL_FRESHWATER")]
    BiomeSwampTropicalFreshwater,
    #[serde(alias = "BIOME_SWAMP_TROPICAL_SALTWATER")]
    BiomeSwampTropicalSaltwater,
    #[serde(alias = "BIOME_TUNDRA")]
    BiomeTundra,
    #[serde(alias = "DOES_NOT_EXIST")]
    DoesNotExist,
    #[serde(alias = "EQUIPMENT")]
    Equipment,
    #[serde(alias = "EQUIPMENT_WAGON")]
    EquipmentWagon,
    #[serde(alias = "EVIL")]
    Evil,
    #[serde(alias = "FANCIFUL")]
    Fanciful,
    #[serde(alias = "GENERATED")]
    Generated,
    #[serde(alias = "GOOD")]
    Good,
    #[serde(alias = "HAS_ANY_BENIGN")]
    HasAnyBenign,
    #[serde(alias = "HAS_ANY_CANNOT_BREATHE_AIR")]
    HasAnyCannotBreatheAir,
    #[serde(alias = "HAS_ANY_CANNOT_BREATHE_WATER")]
    HasAnyCannotBreatheWater,
    #[serde(alias = "HAS_ANY_CAN_SWIM")]
    HasAnyCanSwim,
    #[serde(alias = "HAS_ANY_CARNIVORE")]
    HasAnyCarnivore,
    #[serde(alias = "HAS_ANY_COMMON_DOMESTIC")]
    HasAnyCommonDomestic,
    #[serde(alias = "HAS_ANY_CURIOUS_BEAST")]
    HasAnyCuriousBeast,
    #[serde(alias = "HAS_ANY_DEMON")]
    HasAnyDemon,
    #[serde(alias = "HAS_ANY_FEATURE_BEAST")]
    HasAnyFeatureBeast,
    #[serde(alias = "HAS_ANY_FLIER")]
    HasAnyFlier,
    #[serde(alias = "HAS_ANY_FLY_RACE_GAIT")]
    HasAnyFlyRaceGait,
    #[serde(alias = "HAS_ANY_GRASP")]
    HasAnyGrasp,
    #[serde(alias = "HAS_ANY_GRAZER")]
    HasAnyGrazer,
    #[serde(alias = "HAS_ANY_HAS_BLOOD")]
    HasAnyHasBlood,
    #[serde(alias = "HAS_ANY_IMMOBILE")]
    HasAnyImmobile,
    #[serde(alias = "HAS_ANY_INTELLIGENT_LEARNS")]
    HasAnyIntelligentLearns,
    #[serde(alias = "HAS_ANY_INTELLIGENT_SPEAKS")]
    HasAnyIntelligentSpeaks,
    #[serde(alias = "HAS_ANY_LARGE_PREDATOR")]
    HasAnyLargePredator,
    #[serde(alias = "HAS_ANY_LOCAL_POPS_CONTROLLABLE")]
    HasAnyLocalPopsControllable,
    #[serde(alias = "HAS_ANY_LOCAL_POPS_PRODUCE_HEROES")]
    HasAnyLocalPopsProduceHeroes,
    #[serde(alias = "HAS_ANY_MEGABEAST")]
    HasAnyMegabeast,
    #[serde(alias = "HAS_ANY_MISCHIEVIOUS")]
    HasAnyMischievious,
    #[serde(alias = "HAS_ANY_NATURAL_ANIMAL")]
    HasAnyNaturalAnimal,
    #[serde(alias = "HAS_ANY_NIGHT_CREATURE")]
    HasAnyNightCreature,
    #[serde(alias = "HAS_ANY_NIGHT_CREATURE_BOGEYMAN")]
    HasAnyNightCreatureBogeyman,
    #[serde(alias = "HAS_ANY_NIGHT_CREATURE_EXPERIMENTER")]
    HasAnyNightCreatureExperimenter,
    #[serde(alias = "HAS_ANY_NIGHT_CREATURE_HUNTER")]
    HasAnyNightCreatureHunter,
    #[serde(alias = "HAS_ANY_NIGHT_CREATURE_NIGHTMARE")]
    HasAnyNightCreatureNightmare,
    #[serde(alias = "HAS_ANY_NOT_FIREIMMUNE")]
    HasAnyNotFireimmune,
    #[serde(alias = "HAS_ANY_NOT_FLIER")]
    HasAnyNotFlier,
    #[serde(alias = "HAS_ANY_NOT_LIVING")]
    HasAnyNotLiving,
    #[serde(alias = "HAS_ANY_OUTSIDER_CONTROLLABLE")]
    HasAnyOutsiderControllable,
    #[serde(alias = "HAS_ANY_POWER")]
    HasAnyPower,
    #[serde(alias = "HAS_ANY_RACE_GAIT")]
    HasAnyRaceGait,
    #[serde(alias = "HAS_ANY_SEMIMEGABEAST")]
    HasAnySemimegabeast,
    #[serde(alias = "HAS_ANY_SLOW_LEARNER")]
    HasAnySlowLearner,
    #[serde(alias = "HAS_ANY_SUPERNATURAL")]
    HasAnySupernatural,
    #[serde(alias = "HAS_ANY_TITAN")]
    HasAnyTitan,
    #[serde(alias = "HAS_ANY_UNIQUE_DEMON")]
    HasAnyUniqueDemon,
    #[serde(alias = "HAS_ANY_UTTERANCES")]
    HasAnyUtterances,
    #[serde(alias = "HAS_ANY_VERMIN_HATEABLE")]
    HasAnyVerminHateable,
    #[serde(alias = "HAS_ANY_VERMIN_MICRO")]
    HasAnyVerminMicro,
    #[serde(alias = "HAS_FEMALE")]
    HasFemale,
    #[serde(alias = "HAS_MALE")]
    HasMale,
    #[serde(alias = "LARGE_ROAMING")]
    LargeRoaming,
    #[serde(alias = "LOOSE_CLUSTERS")]
    LooseClusters,
    #[serde(alias = "MATES_TO_BREED")]
    MatesToBreed,
    #[serde(alias = "MUNDANE")]
    Mundane,
    #[serde(alias = "OCCURS_AS_ENTITY_RACE")]
    OccursAsEntityRace,
    #[serde(alias = "SAVAGE")]
    Savage,
    /// Applies to any vermin creature.
    #[serde(alias = "SMALL_RACE")]
    SmallRace,
    #[serde(alias = "TWO_GENDERS")]
    TwoGenders,
    #[serde(alias = "UBIQUITOUS")]
    Ubiquitous,
    #[serde(alias = "VERMIN_EATER")]
    VerminEater,
    #[serde(alias = "VERMIN_FISH")]
    VerminFish,
    #[serde(alias = "VERMIN_GROUNDER")]
    VerminGrounder,
    #[serde(alias = "VERMIN_ROTTER")]
    VerminRotter,
    #[serde(alias = "VERMIN_SOIL")]
    VerminSoil,
    #[serde(alias = "VERMIN_SOIL_COLONY")]
    VerminSoilColony,
}
impl Default for CreatureFlagEnum {
    fn default() -> Self {
        Self::AllCastesAlive
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum CasteFlagEnum {
    #[serde(alias = "ADOPTS_OWNER")]
    AdoptsOwner,
    #[serde(alias = "ALCOHOL_DEPENDENT")]
    AlcoholDependent,
    #[serde(alias = "ALL_ACTIVE")]
    AllActive,
    #[serde(alias = "AMBUSHPREDATOR")]
    Ambushpredator,
    #[serde(alias = "AQUATIC_UNDERSWIM")]
    AquaticUnderswim,
    #[serde(alias = "ARENA_RESTRICTED")]
    ArenaRestricted,
    #[serde(alias = "AT_PEACE_WITH_WILDLIFE")]
    AtPeaceWithWildlife,
    #[serde(alias = "BENIGN")]
    Benign,
    #[serde(alias = "BLOODSUCKER")]
    Bloodsucker,
    #[serde(alias = "BONECARN")]
    BoneCarn,
    #[serde(alias = "CANNOT_BREATHE_AIR")]
    CannotBreatheAir,
    #[serde(alias = "CANNOT_CLIMB")]
    CannotClimb,
    #[serde(alias = "CANNOT_JUMP")]
    CannotJump,
    #[serde(alias = "CANOPENDOORS")]
    CanOpenDoors,
    #[serde(alias = "CAN_BREATHE_WATER")]
    CanBreatheWater,
    #[serde(alias = "CAN_LEARN", alias = "INTELLIGENT_LEARNS")]
    CanLearn,
    #[serde(alias = "CAN_SPEAK", alias = "INTELLIGENT_SPEAKS")]
    CanSpeak,
    #[serde(alias = "CAN_SWIM")]
    CanSwim,
    #[serde(alias = "CAN_SWIM_INNATE")]
    CanSwimInnate,
    #[serde(alias = "CARNIVORE")]
    Carnivore,
    #[serde(alias = "CAVE_ADAPT")]
    CaveAdapt,
    #[serde(alias = "COLONY_EXTERNAL")]
    ColonyExternal,
    #[serde(alias = "COMMON_DOMESTIC")]
    CommonDomestic,
    #[serde(alias = "CONVERTED_SPOUSE")]
    ConvertedSpouse,
    #[serde(alias = "COOKABLE_LIVE")]
    CookableLive,
    #[serde(alias = "CRAZED")]
    Crazed,
    #[serde(alias = "CREPUSCULAR")]
    Crepuscular,
    #[serde(alias = "CURIOUS_BEAST")]
    CuriousBeast,
    #[serde(alias = "CURIOUS_BEAST_EATER")]
    CuriousBeastEater,
    #[serde(alias = "CURIOUS_BEAST_GUZZLER")]
    CuriousBeastGuzzler,
    #[serde(alias = "CURIOUS_BEAST_ITEM")]
    CuriousBeastItem,
    #[serde(alias = "DEMON")]
    Demon,
    #[serde(alias = "DIE_WHEN_VERMIN_BITE")]
    DieWhenVerminBite,
    #[serde(alias = "DIURNAL")]
    Diurnal,
    #[serde(alias = "DIVE_HUNTS_VERMIN")]
    DiveHuntsVermin,
    #[serde(alias = "EQUIPS")]
    Equips,
    #[serde(alias = "EXTRAVISION")]
    Extravision,
    #[serde(alias = "FEATURE_ATTACK_GROUP")]
    FeatureAttackGroup,
    #[serde(alias = "FEATURE_BEAST")]
    FeatureBeast,
    #[serde(alias = "FIREIMMUNE")]
    Fireimmune,
    #[serde(alias = "FIREIMMUNE_SUPER")]
    FireimmuneSuper,
    #[serde(alias = "FISHITEM")]
    FishItem,
    #[serde(alias = "FLEEQUICK")]
    FleeQuick,
    #[serde(alias = "FLIER")]
    Flier,
    #[serde(alias = "GELDABLE")]
    Geldable,
    #[serde(alias = "GETS_INFECTIONS_FROM_ROT")]
    GetsInfectionsFromRot,
    #[serde(alias = "GETS_WOUND_INFECTIONS")]
    GetsWoundInfections,
    #[serde(alias = "GNAWER")]
    Gnawer,
    #[serde(alias = "GRAZER")]
    Grazer,
    #[serde(alias = "HASSHELL")]
    HasShell,
    #[serde(alias = "HAS_BABYSTATE")]
    HasBabystate,
    #[serde(alias = "HAS_BLOOD")]
    HasBlood,
    #[serde(alias = "HAS_CHILDSTATE")]
    HasChildstate,
    #[serde(alias = "HAS_COLOR")]
    HasColor,
    #[serde(alias = "HAS_FLY_RACE_GAIT")]
    HasFlyRaceGait,
    #[serde(alias = "HAS_GLOW_COLOR")]
    HasGlowColor,
    #[serde(alias = "HAS_GLOW_TILE")]
    HasGlowTile,
    #[serde(alias = "HAS_GRASP")]
    HasGrasp,
    #[serde(alias = "HAS_NERVES")]
    HasNerves,
    #[serde(alias = "HAS_PUS")]
    HasPus,
    #[serde(alias = "HAS_RACE_GAIT")]
    HasRaceGait,
    #[serde(alias = "HAS_ROTTABLE")]
    HasRottable,
    #[serde(alias = "HAS_SECRETION")]
    HasSecretion,
    #[serde(alias = "HAS_SOLDIER_TILE")]
    HasSoldierTile,
    #[serde(alias = "HAS_SOUND_ALERT")]
    HasSoundAlert,
    #[serde(alias = "HAS_SOUND_PEACEFUL_INTERMITTENT")]
    HasSoundPeacefulIntermittent,
    #[serde(alias = "HAS_TILE")]
    HasTile,
    #[serde(alias = "HUNTS_VERMIN")]
    HuntsVermin,
    #[serde(alias = "IMMOBILE")]
    Immobile,
    #[serde(alias = "IMMOBILE_LAND")]
    ImmobileLand,
    #[serde(alias = "IMMOLATE")]
    Immolate,
    #[serde(alias = "ITEMCORPSE")]
    ItemCorpse,
    #[serde(alias = "LAIR_HUNTER")]
    LairHunter,
    #[serde(alias = "LARGE_PREDATOR")]
    LargePredator,
    #[serde(alias = "LAYS_EGGS")]
    LaysEggs,
    #[serde(alias = "LAYS_UNUSUAL_EGGS")]
    LaysUnusualEggs,
    #[serde(alias = "LIGAMENTS")]
    Ligaments,
    #[serde(alias = "LIGHT_GEN")]
    LightGen,
    #[serde(alias = "LISP")]
    Lisp,
    #[serde(alias = "LOCAL_POPS_CONTROLLABLE")]
    LocalPopsControllable,
    #[serde(alias = "LOCAL_POPS_PRODUCE_HEROES")]
    LocalPopsProduceHeroes,
    #[serde(alias = "LOCKPICKER")]
    Lockpicker,
    #[serde(alias = "MAGICAL")]
    Magical,
    #[serde(alias = "MAGMA_VISION")]
    MagmaVision,
    #[serde(alias = "MANNERISM_BREATH")]
    MannerismBreath,
    #[serde(alias = "MANNERISM_EYELIDS")]
    MannerismEyelids,
    #[serde(alias = "MANNERISM_LAUGH")]
    MannerismLaugh,
    #[serde(alias = "MANNERISM_POSTURE")]
    MannerismPosture,
    #[serde(alias = "MANNERISM_SIT")]
    MannerismSit,
    #[serde(alias = "MANNERISM_SMILE")]
    MannerismSmile,
    #[serde(alias = "MANNERISM_STRETCH")]
    MannerismStretch,
    #[serde(alias = "MANNERISM_WALK")]
    MannerismWalk,
    #[serde(alias = "MATUTINAL")]
    Matutinal,
    #[serde(alias = "MEANDERER")]
    Meanderer,
    #[serde(alias = "MEGABEAST")]
    Megabeast,
    #[serde(alias = "MILKABLE")]
    Milkable,
    #[serde(alias = "MISCHIEVIOUS")]
    Mischievious,
    #[serde(alias = "MOUNT")]
    Mount,
    #[serde(alias = "MOUNT_EXOTIC")]
    MountExotic,
    #[serde(alias = "MULTIPART_FULL_VISION")]
    MultipartFullVision,
    #[serde(alias = "MULTIPLE_LITTER_RARE")]
    MultipleLitterRare,
    #[serde(alias = "NATURAL_ANIMAL")]
    NaturalAnimal,
    #[serde(alias = "NIGHT_CREATURE")]
    NightCreature,
    #[serde(alias = "NIGHT_CREATURE_BOGEYMAN")]
    NightCreatureBogeyman,
    #[serde(alias = "NIGHT_CREATURE_EXPERIMENTER")]
    NightCreatureExperimenter,
    #[serde(alias = "NIGHT_CREATURE_HUNTER")]
    NightCreatureHunter,
    #[serde(alias = "NIGHT_CREATURE_NIGHTMARE")]
    NightCreatureNightmare,
    #[serde(alias = "NOBONES")]
    NoBones,
    #[serde(alias = "NOBREATHE")]
    NoBreathe,
    #[serde(alias = "NOCTURNAL")]
    Nocturnal,
    #[serde(alias = "NOEMOTION")]
    NoEmotion,
    #[serde(alias = "NOEXERT")]
    NoExert,
    #[serde(alias = "NOFEAR")]
    NoFear,
    #[serde(alias = "NOMEAT")]
    NoMeat,
    #[serde(alias = "NONAUSEA")]
    NoNausea,
    #[serde(alias = "NOPAIN")]
    NoPain,
    #[serde(alias = "NOSKIN")]
    NoSkin,
    #[serde(alias = "NOSKULL")]
    NoSkull,
    #[serde(alias = "NOSMELLYROT")]
    NoSmellyRot,
    #[serde(alias = "NOSTUCKINS")]
    NoStuckins,
    #[serde(alias = "NOSTUN")]
    NoStun,
    #[serde(alias = "NOTHOUGHT")]
    NoThought,
    #[serde(alias = "NOT_BUTCHERABLE")]
    NotButcherable,
    #[serde(alias = "NOT_LIVING")]
    NotLiving,
    #[serde(alias = "NO_AUTUMN")]
    NoAutumn,
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
    #[serde(alias = "NO_SPRING")]
    NoSpring,
    #[serde(alias = "NO_SUMMER")]
    NoSummer,
    #[serde(alias = "NO_THOUGHT_CENTER_FOR_MOVEMENT")]
    NoThoughtCenterForMovement,
    #[serde(alias = "NO_UNIT_TYPE_COLOR")]
    NoUnitTypeColor,
    #[serde(alias = "NO_VEGETATION_PERTURB")]
    NoVegetationPerturb,
    #[serde(alias = "NO_WINTER")]
    NoWinter,
    #[serde(alias = "OPPOSED_TO_LIFE")]
    OpposedToLife,
    #[serde(alias = "OUTSIDER_CONTROLLABLE")]
    OutsiderControllable,
    #[serde(alias = "PACK_ANIMAL")]
    PackAnimal,
    #[serde(alias = "PARALYZEIMMUNE")]
    Paralyzeimmune,
    #[serde(alias = "PATTERNFLIER")]
    Patternflier,
    #[serde(alias = "PEARL")]
    Pearl,
    #[serde(alias = "PET")]
    Pet,
    #[serde(alias = "PET_EXOTIC")]
    PetExotic,
    #[serde(alias = "POWER")]
    Power,
    #[serde(alias = "REMAINS_ON_VERMIN_BITE_DEATH")]
    RemainsOnVerminBiteDeath,
    #[serde(alias = "REMAINS_UNDETERMINED")]
    RemainsUndetermined,
    #[serde(alias = "RETURNS_VERMIN_KILLS_TO_OWNER")]
    ReturnsVerminKillsToOwner,
    #[serde(alias = "SEMIMEGABEAST")]
    SemiMegabeast,
    #[serde(alias = "SLOW_LEARNER")]
    SlowLearner,
    #[serde(alias = "SPOUSE_CONVERSION_TARGET")]
    SpouseConversionTarget,
    #[serde(alias = "SPOUSE_CONVERTER")]
    SpouseConverter,
    #[serde(alias = "SPREAD_EVIL_SPHERES_IF_RULER")]
    SpreadEvilSpheresIfRuler,
    #[serde(alias = "STANCE_CLIMBER")]
    StanceClimber,
    #[serde(alias = "STRANGE_MOODS")]
    StrangeMoods,
    #[serde(alias = "SUPERNATURAL")]
    Supernatural,
    #[serde(alias = "TENDONS")]
    Tendons,
    #[serde(alias = "THICKWEB")]
    Thickweb,
    #[serde(alias = "TITAN")]
    Titan,
    #[serde(alias = "TRAINABLE_HUNTING")]
    TrainableHunting,
    #[serde(alias = "TRAINABLE_WAR")]
    TrainableWar,
    #[serde(alias = "TRANCES")]
    Trances,
    #[serde(alias = "TRAPAVOID")]
    Trapavoid,
    #[serde(alias = "UNIQUE_DEMON")]
    UniqueDemon,
    #[serde(alias = "UTTERANCES")]
    Utterances,
    #[serde(alias = "VEGETATION")]
    Vegetation,
    #[serde(alias = "VERMIN_GOBBLER")]
    VerminGobbler,
    #[serde(alias = "VERMIN_HATEABLE")]
    VerminHateable,
    #[serde(alias = "VERMIN_MICRO")]
    VerminMicro,
    #[serde(alias = "VERMIN_NOFISH")]
    VerminNoFish,
    #[serde(alias = "VERMIN_NOROAM")]
    VerminNoRoam,
    #[serde(alias = "VERMIN_NOTRAP")]
    VerminNoTrap,
    #[serde(alias = "VESPERTINE")]
    Vespertine,
    #[serde(alias = "WAGON_PULLER")]
    WagonPuller,
    #[serde(alias = "WEBBER")]
    Webber,
    #[serde(alias = "WEBIMMUNE")]
    Webimmune,
}
impl Default for CasteFlagEnum {
    fn default() -> Self {
        Self::AdoptsOwner
    }
}
