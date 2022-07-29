use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BiomeEnum {
    /// Mountain (ID: 0)
    #[serde(alias = "MOUNTAIN", alias = "MOUNTAINS")]
    Mountain,
    /// Glacier (ID: 1)
    #[serde(alias = "GLACIER")]
    Glacier,
    /// Tundra (ID: 2)
    #[serde(alias = "TUNDRA")]
    Tundra,
    /// Temperate Freshwater Swamp (ID: 3)
    #[serde(alias = "SWAMP_TEMPERATE_FRESHWATER")]
    SwampTemperateFreshwater,
    /// Temperate Saltwater Swamp (ID: 4)
    #[serde(alias = "SWAMP_TEMPERATE_SALTWATER")]
    SwampTemperateSaltwater,
    /// Temperate Freshwater Marsh (ID: 5)
    #[serde(alias = "MARSH_TEMPERATE_FRESHWATER")]
    MarshTemperateFreshwater,
    /// Temperate Saltwater Marsh (ID: 6)
    #[serde(alias = "MARSH_TEMPERATE_SALTWATER")]
    MarshTemperateSaltwater,
    /// Tropical Freshwater Swamp (ID: 7)
    #[serde(alias = "SWAMP_TROPICAL_FRESHWATER")]
    SwampTropicalFreshwater,
    /// Tropical Saltwater Swamp (ID: 8)
    #[serde(alias = "SWAMP_TROPICAL_SALTWATER")]
    SwampTropicalSaltwater,
    /// Mangrove Swamp (ID: 9)
    #[serde(alias = "SWAMP_MANGROVE")]
    SwampMangrove,
    /// Tropical Freshwater Marsh (ID: 10)
    #[serde(alias = "MARSH_TROPICAL_FRESHWATER")]
    MarshTropicalFreshwater,
    /// Tropical Saltwater Marsh (ID: 11)
    #[serde(alias = "MARSH_TROPICAL_SALTWATER")]
    MarshTropicalSaltwater,
    /// Taiga (ID: 12)
    #[serde(alias = "FOREST_TAIGA", alias = "TAIGA")]
    ForestTaiga,
    /// Temperate Coniferous Forest (ID: 13)
    #[serde(alias = "FOREST_TEMPERATE_CONIFER")]
    ForestTemperateConifer,
    /// Temperate Broadleaf Forest (ID: 14)
    #[serde(alias = "FOREST_TEMPERATE_BROADLEAF")]
    ForestTemperateBroadleaf,
    /// Tropical Coniferous Forest (ID: 15)
    #[serde(alias = "FOREST_TROPICAL_CONIFER")]
    ForestTropicalConifer,
    /// Tropical Dry Broadleaf Forest (ID: 16)
    #[serde(alias = "FOREST_TROPICAL_DRY_BROADLEAF")]
    ForestTropicalDryBroadleaf,
    /// Tropical Moist Broadleaf Forest (ID: 17)
    #[serde(alias = "FOREST_TROPICAL_MOIST_BROADLEAF")]
    ForestTropicalMoistBroadleaf,
    /// Temperate Grassland (ID: 18)
    #[serde(alias = "GRASSLAND_TEMPERATE")]
    GrasslandTemperate,
    /// Temperate Savanna (ID: 19)
    #[serde(alias = "SAVANNA_TEMPERATE")]
    SavannaTemperate,
    /// Temperate Shrubland (ID: 20)
    #[serde(alias = "SHRUBLAND_TEMPERATE")]
    ShrublandTemperate,
    /// Tropical Grassland (ID: 21)
    #[serde(alias = "GRASSLAND_TROPICAL")]
    GrasslandTropical,
    /// Tropical Savanna (ID: 22)
    #[serde(alias = "SAVANNA_TROPICAL")]
    SavannaTropical,
    /// Tropical Shrubland (ID: 23)
    #[serde(alias = "SHRUBLAND_TROPICAL")]
    ShrublandTropical,
    /// Badlands (ID: 24)
    #[serde(alias = "DESERT_BADLAND")]
    DesertBadland,
    /// Rocky Wasteland (ID: 25)
    #[serde(alias = "DESERT_ROCK")]
    DesertRock,
    /// Sand Desert (ID: 26)
    #[serde(alias = "DESERT_SAND")]
    DesertSand,
    /// Tropical Ocean (ID: 27)
    #[serde(alias = "OCEAN_TROPICAL")]
    OceanTropical,
    /// Temperate Ocean (ID: 28)
    #[serde(alias = "OCEAN_TEMPERATE")]
    OceanTemperate,
    /// Arctic Ocean (ID: 29)
    #[serde(alias = "OCEAN_ARCTIC")]
    OceanArctic,
    /// Temperate Freshwater Pool (ID: 30)
    #[serde(alias = "POOL_TEMPERATE_FRESHWATER")]
    PoolTemperateFreshwater,
    /// Temperate Brackish Pool (ID: 31)
    #[serde(alias = "POOL_TEMPERATE_BRACKISHWATER")]
    PoolTemperateBrackishwater,
    /// Temperate Saltwater Pool (ID: 32)
    #[serde(alias = "POOL_TEMPERATE_SALTWATER")]
    PoolTemperateSaltwater,
    /// Tropical Freshwater Pool (ID: 33)
    #[serde(alias = "POOL_TROPICAL_FRESHWATER")]
    PoolTropicalFreshwater,
    /// Tropical Brackish Pool (ID: 34)
    #[serde(alias = "POOL_TROPICAL_BRACKISHWATER")]
    PoolTropicalBrackishwater,
    /// Tropical Saltwater Pool (ID: 35)
    #[serde(alias = "POOL_TROPICAL_SALTWATER")]
    PoolTropicalSaltwater,
    /// Temperate Freshwater Lake (ID: 36)
    #[serde(alias = "LAKE_TEMPERATE_FRESHWATER")]
    LakeTemperateFreshwater,
    /// Temperate Brackish Lake (ID: 37)
    #[serde(alias = "LAKE_TEMPERATE_BRACKISHWATER")]
    LakeTemperateBrackishwater,
    /// Temperate Saltwater Lake (ID: 38)
    #[serde(alias = "LAKE_TEMPERATE_SALTWATER")]
    LakeTemperateSaltwater,
    /// Tropical Freshwater Lake (ID: 39)
    #[serde(alias = "LAKE_TROPICAL_FRESHWATER")]
    LakeTropicalFreshwater,
    /// Tropical Brackish Lake (ID: 40)
    #[serde(alias = "LAKE_TROPICAL_BRACKISHWATER")]
    LakeTropicalBrackishwater,
    /// Tropical Saltwater Lake (ID: 41)
    #[serde(alias = "LAKE_TROPICAL_SALTWATER")]
    LakeTropicalSaltwater,
    /// Temperate Freshwater River (ID: 42)
    #[serde(alias = "RIVER_TEMPERATE_FRESHWATER")]
    RiverTemperateFreshwater,
    /// Temperate Brackish River (ID: 43)
    #[serde(alias = "RIVER_TEMPERATE_BRACKISHWATER")]
    RiverTemperateBrackishwater,
    /// Temperate Saltwater River (ID: 44)
    #[serde(alias = "RIVER_TEMPERATE_SALTWATER")]
    RiverTemperateSaltwater,
    /// Tropical Freshwater River (ID: 45)
    #[serde(alias = "RIVER_TROPICAL_FRESHWATER")]
    RiverTropicalFreshwater,
    /// Tropical Brackish River (ID: 46)
    #[serde(alias = "RIVER_TROPICAL_BRACKISHWATER")]
    RiverTropicalBrackishwater,
    /// Tropical Saltwater River (ID: 47)
    #[serde(alias = "RIVER_TROPICAL_SALTWATER")]
    RiverTropicalSaltwater,
    /// Underground caverns (in water) (ID: 48)
    #[serde(alias = "SUBTERRANEAN_WATER")]
    SubterraneanWater,
    /// Underground caverns (out of water) (ID: 49)
    #[serde(alias = "SUBTERRANEAN_CHASM")]
    SubterraneanChasm,
    /// Magma sea (ID: 50)
    #[serde(alias = "SUBTERRANEAN_LAVA")]
    SubterraneanLava,
    /// All biomes excluding pools, rivers, and underground features (ID: 0-29, 36-41)
    #[serde(alias = "ALL_MAIN")]
    AllMain,
    /// All main biomes excluding oceans and lakes (ID: 0-26)
    #[serde(alias = "ANY_LAND")]
    AnyLand,
    /// All ocean biomes (ID: 27-29)
    #[serde(alias = "ANY_OCEAN")]
    AnyOcean,
    /// All lake biomes (ID: 36-41)
    #[serde(alias = "ANY_LAKE")]
    AnyLake,
    /// All temperate lake biomes (ID: 36-38)
    #[serde(alias = "ANY_TEMPERATE_LAKE")]
    AnyTemperateLake,
    /// All tropical lake biomes (ID: 39-41)
    #[serde(alias = "ANY_TROPICAL_LAKE")]
    AnyTropicalLake,
    /// All river biomes (ID: 42-47)
    #[serde(alias = "ANY_RIVER")]
    AnyRiver,
    /// All temperate river biomes (ID: 42-44)
    #[serde(alias = "ANY_TEMPERATE_RIVER")]
    AnyTemperateRiver,
    /// All tropical river biomes (ID: 45-47)
    #[serde(alias = "ANY_TROPICAL_RIVER")]
    AnyTropicalRiver,
    /// All pool biomes (ID: 30-35)
    #[serde(alias = "ANY_POOL")]
    AnyPool,
    /// All land biomes excluding Mountain, Glacier, and Tundra (ID: 3-26)
    #[serde(alias = "NOT_FREEZING")]
    NotFreezing,
    /// All Temperate land biomes - marshes, swamps, forests, grassland, savanna, and shrubland
    /// (ID: 3-6, 13-14, 18-20)
    #[serde(alias = "ANY_TEMPERATE")]
    AnyTemperate,
    /// All Tropical land biomes - marshes, swamps (including Mangrove), forests, grassland,
    /// savanna, and shrubland (ID: 7-11, 15-17, 21-23)
    #[serde(alias = "ANY_TROPICAL")]
    AnyTropical,
    /// All Forest biomes (excluding Taiga) (ID: 13-17)
    #[serde(alias = "ANY_FOREST")]
    AnyForest,
    /// Temperate and Tropical Shrubland (ID: 20, 23)
    #[serde(alias = "ANY_SHRUBLAND")]
    AnyShrubland,
    /// Temperate and Tropical Grassland (ID: 18, 21)
    #[serde(alias = "ANY_GRASSLAND")]
    AnyGrassland,
    /// Temperate and Tropical Savanna (ID: 19, 22)
    #[serde(alias = "ANY_SAVANNA")]
    AnySavanna,
    /// Temperate Coniferous and Broadleaf Forests (ID: 13-14)
    #[serde(alias = "ANY_TEMPERATE_FOREST")]
    AnyTemperateForest,
    /// Tropical Coniferous and Dry/Moist Broadleaf Forests (ID: 15-17)
    #[serde(alias = "ANY_TROPICAL_FOREST")]
    AnyTropicalForest,
    /// Temperate Broadleaf Forest, Grassland/Savanna/Shrubland, Swamps, and Marshes (ID: 3-6, 14, 18-20)
    #[serde(alias = "ANY_TEMPERATE_BROADLEAF")]
    AnyTemperateBroadleaf,
    /// Tropical Dry/Moist Broadleaf Forest, Grassland/Savanna/Shrubland, Swamps (including Mangrove),
    /// and Marshes (ID: 7-11, 16-17, 21-23)
    #[serde(alias = "ANY_TROPICAL_BROADLEAF")]
    AnyTropicalBroadleaf,
    /// All swamps and marshes (ID: 3-11)
    #[serde(alias = "ANY_WETLAND")]
    AnyWetland,
    /// All temperate swamps and marshes (ID: 3-6)
    #[serde(alias = "ANY_TEMPERATE_WETLAND")]
    AnyTemperateWetland,
    /// All tropical swamps and marshes (ID: 7-11)
    #[serde(alias = "ANY_TROPICAL_WETLAND")]
    AnyTropicalWetland,
    /// All tropical marshes (ID: 10-11)
    #[serde(alias = "ANY_TROPICAL_MARSH")]
    AnyTropicalMarsh,
    /// All temperate marshes (ID: 5-6)
    #[serde(alias = "ANY_TEMPERATE_MARSH")]
    AnyTemperateMarsh,
    /// All tropical swamps (including Mangrove) (ID: 7-9)
    #[serde(alias = "ANY_TROPICAL_SWAMP")]
    AnyTropicalSwamp,
    /// All temperate swamps (ID: 3-4)
    #[serde(alias = "ANY_TEMPERATE_SWAMP")]
    AnyTemperateSwamp,
    /// Badlands, Rocky Wasteland, and Sand Desert (ID: 24-26)
    #[serde(alias = "ANY_DESERT")]
    AnyDesert,
}

impl Default for BiomeEnum {
    fn default() -> Self {
        Self::Mountain
    }
}
