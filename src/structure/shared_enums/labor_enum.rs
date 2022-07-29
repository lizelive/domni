use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum LaborEnum {
    /// [Mining](https://dwarffortresswiki.org/index.php/Mining)
    #[serde(alias = "MINE")]
    Mine,
    /// [Stone hauling](https://dwarffortresswiki.org/index.php/Stone_hauling)
    #[serde(alias = "HAUL_STONE")]
    HaulStone,
    /// [Wood hauling](https://dwarffortresswiki.org/index.php/Wood_hauling)
    #[serde(alias = "HAUL_WOOD")]
    HaulWood,
    /// [Burial](https://dwarffortresswiki.org/index.php/Burial)
    #[serde(alias = "HAUL_BODY")]
    HaulBody,
    /// [Food hauling](https://dwarffortresswiki.org/index.php/Food_hauling)
    #[serde(alias = "HAUL_FOOD")]
    HaulFood,
    /// [Refuse hauling](https://dwarffortresswiki.org/index.php/Refuse_hauling)
    #[serde(alias = "HAUL_REFUSE")]
    HaulRefuse,
    /// [Item hauling](https://dwarffortresswiki.org/index.php/Item_hauling)
    #[serde(alias = "HAUL_ITEM")]
    HaulItem,
    /// [Furniture hauling](https://dwarffortresswiki.org/index.php/Furniture_hauling)
    #[serde(alias = "HAUL_FURNITURE")]
    HaulFurniture,
    /// [Animal hauling](https://dwarffortresswiki.org/index.php/Animal_hauling)
    #[serde(alias = "HAUL_ANIMALS")]
    HaulAnimals,
    /// [Cleaning](https://dwarffortresswiki.org/index.php/Cleaning)
    #[serde(alias = "CLEAN")]
    Clean,
    /// [Wood cutting](https://dwarffortresswiki.org/index.php/Wood_cutting)
    #[serde(alias = "CUTWOOD")]
    CutWood,
    /// [Carpentry](https://dwarffortresswiki.org/index.php/Carpentry)
    #[serde(alias = "CARPENTER")]
    Carpenter,
    /// [Stone detailing](https://dwarffortresswiki.org/index.php/Stone_detailing)
    #[serde(alias = "DETAIL")]
    Detail,
    /// [Masonry](https://dwarffortresswiki.org/index.php/Masonry)
    #[serde(alias = "MASON")]
    Mason,
    /// [Architecture](https://dwarffortresswiki.org/index.php/Architecture)
    #[serde(alias = "ARCHITECT")]
    Architect,
    /// [Animal training](https://dwarffortresswiki.org/index.php/Animal_training)
    #[serde(alias = "ANIMALTRAIN")]
    AnimalTrainer,
    /// [Animal care](https://dwarffortresswiki.org/index.php/Animal_care)
    #[serde(alias = "ANIMALCARE")]
    AnimalCare,
    /// [Diagnosis](https://dwarffortresswiki.org/index.php/Diagnosis)
    #[serde(alias = "DIAGNOSE")]
    Diagnoser,
    /// [Surgery](https://dwarffortresswiki.org/index.php/Surgery)
    #[serde(alias = "SURGERY")]
    Surgery,
    /// [Setting bones](https://dwarffortresswiki.org/index.php/Setting_bones)
    #[serde(alias = "BONE_SETTING")]
    BoneSetting,
    /// [Suturing](https://dwarffortresswiki.org/index.php/Suturing)
    #[serde(alias = "SUTURING")]
    Suturing,
    /// [Dressing wounds](https://dwarffortresswiki.org/index.php/Dressing_wounds)
    #[serde(alias = "DRESSING_WOUNDS")]
    DressingWounds,
    /// [Feed patients/prisoners](https://dwarffortresswiki.org/index.php/Feed_patients/prisoners)
    #[serde(alias = "FEED_WATER_CIVILIANS")]
    FeedWaterCivilians,
    /// [Recovering wounded](https://dwarffortresswiki.org/index.php/Recovering_wounded)
    #[serde(alias = "RECOVER_WOUNDED")]
    RecoveringWounded,
    /// [Butchery](https://dwarffortresswiki.org/index.php/Butchery)
    #[serde(alias = "BUTCHER")]
    Butchery,
    /// [Trapper](https://dwarffortresswiki.org/index.php/Trapper)
    #[serde(alias = "TRAPPER")]
    Trapper,
    /// [Small animal dissection](https://dwarffortresswiki.org/index.php/Small_animal_dissection)
    #[serde(alias = "DISSECT_VERMIN")]
    SmallAnimalDissection,
    /// [Leatherworking](https://dwarffortresswiki.org/index.php/Leatherworking)
    #[serde(alias = "LEATHER")]
    Leatherworker,
    /// [Tanning](https://dwarffortresswiki.org/index.php/Tanning)
    #[serde(alias = "TANNER")]
    Tanning,
    /// [Brewing](https://dwarffortresswiki.org/index.php/Brewing)
    #[serde(alias = "BREWER")]
    Brewing,
    /// [Alchemy](https://dwarffortresswiki.org/index.php/Alchemy)
    #[serde(alias = "ALCHEMIST")]
    Alchemy,
    /// [Soap making](https://dwarffortresswiki.org/index.php/Soap_making)
    #[serde(alias = "SOAP_MAKER")]
    SoapMaking,
    /// [Weaving](https://dwarffortresswiki.org/index.php/Weaving)
    #[serde(alias = "WEAVER")]
    Weaving,
    /// [Clothesmaking](https://dwarffortresswiki.org/index.php/Clothesmaking)
    #[serde(alias = "CLOTHESMAKER")]
    ClothesMaking,
    /// [Milling](https://dwarffortresswiki.org/index.php/Milling)
    #[serde(alias = "MILLER")]
    Milling,
    /// [Plant processing](https://dwarffortresswiki.org/index.php/Plant_processing)
    #[serde(alias = "PROCESS_PLANT")]
    PlantProcessing,
    /// [Cheese making](https://dwarffortresswiki.org/index.php/Cheese_making)
    #[serde(alias = "MAKE_CHEESE")]
    CheeseMaking,
    /// [Milking](https://dwarffortresswiki.org/index.php/Milking)
    #[serde(alias = "MILK")]
    Milking,
    /// [Cooking](https://dwarffortresswiki.org/index.php/Cooking)
    #[serde(alias = "COOK")]
    Cooking,
    /// [Farming (fields)](https://dwarffortresswiki.org/index.php/Farming_(fields))
    #[serde(alias = "PLANT")]
    Farming,
    /// [Plant gathering](https://dwarffortresswiki.org/index.php/Plant_gathering)
    #[serde(alias = "HERBALIST")]
    PlantGathering,
    /// [Fishing](https://dwarffortresswiki.org/index.php/Fishing)
    #[serde(alias = "FISH")]
    Fishing,
    /// [Fish cleaning](https://dwarffortresswiki.org/index.php/Fish_cleaning)
    #[serde(alias = "CLEAN_FISH")]
    FishCleaning,
    /// [Fish dissection](https://dwarffortresswiki.org/index.php/Fish_dissection)
    #[serde(alias = "DISSECT_FISH")]
    FishDissection,
    /// [Hunting](https://dwarffortresswiki.org/index.php/Hunting)
    #[serde(alias = "HUNT")]
    Hunting,
    /// [Furnace operating](https://dwarffortresswiki.org/index.php/Furnace_operating)
    #[serde(alias = "SMELT")]
    FurnaceOperating,
    /// [Weaponsmithing](https://dwarffortresswiki.org/index.php/Weaponsmithing)
    #[serde(alias = "FORGE_WEAPON")]
    WeaponSmithing,
    /// [Armoring](https://dwarffortresswiki.org/index.php/Armoring)
    #[serde(alias = "FORGE_ARMOR")]
    Armoring,
    /// [Blacksmithing](https://dwarffortresswiki.org/index.php/Blacksmithing)
    #[serde(alias = "FORGE_FURNITURE")]
    Blacksmithing,
    /// [Metalcrafting](https://dwarffortresswiki.org/index.php/Metalcrafting)
    #[serde(alias = "METAL_CRAFT")]
    MetalCrafting,
    /// [Gem cutting](https://dwarffortresswiki.org/index.php/Gem_cutting)
    #[serde(alias = "CUT_GEM")]
    GemCutting,
    /// [Gem setting](https://dwarffortresswiki.org/index.php/Gem_setting)
    #[serde(alias = "ENCRUST_GEM")]
    GemSetting,
    /// [Woodcrafting](https://dwarffortresswiki.org/index.php/Woodcrafting)
    #[serde(alias = "WOOD_CRAFT")]
    WoodCrafting,
    /// [Stonecrafting](https://dwarffortresswiki.org/index.php/Stonecrafting)
    #[serde(alias = "STONE_CRAFT")]
    StoneCrafting,
    /// [Bone carving](https://dwarffortresswiki.org/index.php/Bone_carving)
    #[serde(alias = "BONE_CARVE")]
    BoneCarving,
    /// [Glass making](https://dwarffortresswiki.org/index.php/Glassmaking)
    #[serde(alias = "GLASSMAKER")]
    GlassMaking,
    /// [Strand extraction](https://dwarffortresswiki.org/index.php/Strand_extraction)
    #[serde(alias = "EXTRACT_STRAND")]
    StrandExtraction,
    /// [Siege engineering](https://dwarffortresswiki.org/index.php/Siege_engineering)
    #[serde(alias = "SIEGECRAFT")]
    SiegeEngineering,
    /// [Siege operating](https://dwarffortresswiki.org/index.php/Siege_operating)
    #[serde(alias = "SIEGEOPERATE")]
    SiegeOperating,
    /// [Bowyer](https://dwarffortresswiki.org/index.php/Bowyer)
    #[serde(alias = "BOWYER")]
    Bowyer,
    /// [Mechanics](https://dwarffortresswiki.org/index.php/Mechanics)
    #[serde(alias = "MECHANIC")]
    Mechanics,
    /// [Potash making](https://dwarffortresswiki.org/index.php/Potash_making)
    #[serde(alias = "POTASH_MAKING")]
    PotashMaking,
    /// [Lye making](https://dwarffortresswiki.org/index.php/Lye_making)
    #[serde(alias = "LYE_MAKING")]
    LyeMaking,
    /// [Dyeing](https://dwarffortresswiki.org/index.php/Dyeing)
    #[serde(alias = "DYER")]
    Dyeing,
    /// [Wood burning](https://dwarffortresswiki.org/index.php/Wood_burning)
    #[serde(alias = "BURN_WOOD")]
    WoodBurning,
    /// [Pump operating](https://dwarffortresswiki.org/index.php/Pump_operating)
    #[serde(alias = "OPERATE_PUMP")]
    PumpOperating,
    /// [Shearing](https://dwarffortresswiki.org/index.php/Shearing)
    #[serde(alias = "SHEARER")]
    Shearing,
    /// [Spinning](https://dwarffortresswiki.org/index.php/Spinning)
    #[serde(alias = "SPINNER")]
    Spinning,
    /// [Pottery](https://dwarffortresswiki.org/index.php/Pottery)
    #[serde(alias = "POTTERY")]
    Pottery,
    /// [Glazing](https://dwarffortresswiki.org/index.php/Glazing)
    #[serde(alias = "GLAZING")]
    Glazing,
    /// [Pressing](https://dwarffortresswiki.org/index.php/Pressing)
    #[serde(alias = "PRESSING")]
    Pressing,
    /// [Beekeeping](https://dwarffortresswiki.org/index.php/Beekeeping)
    #[serde(alias = "BEEKEEPING")]
    Beekeeping,
    /// [Wax working](https://dwarffortresswiki.org/index.php/Wax_working)
    #[serde(alias = "WAX_WORKING")]
    WaxWorking,
    /// [Push/haul vehicles](https://dwarffortresswiki.org/index.php/Push/haul_vehicles)
    #[serde(alias = "HANDLE_VEHICLES")]
    HandleVehicles,
    /// [Trade](https://dwarffortresswiki.org/index.php/Trade)
    #[serde(alias = "HAUL_TRADE")]
    HaulTrade,
    /// [Lever](https://dwarffortresswiki.org/index.php/Lever)
    #[serde(alias = "PULL_LEVER")]
    PullLever,
    /// [Teardown and building disassembly](https://dwarffortresswiki.org/index.php/Construction)
    #[serde(alias = "REMOVE_CONSTRUCTION")]
    RemoveConstruction,
    /// Haul water from [activity zone](https://dwarffortresswiki.org/index.php/Activity_zone)
    #[serde(alias = "HAUL_WATER")]
    HaulWater,
    /// [Gelding](https://dwarffortresswiki.org/index.php/Gelding)
    #[serde(alias = "GELD")]
    Gelding,
    /// [Road](https://dwarffortresswiki.org/index.php/Road)
    #[serde(alias = "BUILD_ROAD")]
    BuildRoad,
    /// [Construction](https://dwarffortresswiki.org/index.php/Construction)
    #[serde(alias = "BUILD_CONSTRUCTION")]
    BuildConstruction,
}

impl Default for LaborEnum {
    fn default() -> Self {
        Self::Mine
    }
}
