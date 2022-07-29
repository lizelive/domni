use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum UnitTypeEnum {
    /// https://dwarffortresswiki.org/index.php/Miner
    #[serde(alias = "MINER")]
    Miner,
    /// https://dwarffortresswiki.org/index.php/Woodworker
    #[serde(alias = "WOODWORKER")]
    Woodworker,
    /// https://dwarffortresswiki.org/index.php/Carpenter
    #[serde(alias = "CARPENTER")]
    Carpenter,
    /// https://dwarffortresswiki.org/index.php/Bowyer
    #[serde(alias = "BOWYER")]
    Bowyer,
    /// https://dwarffortresswiki.org/index.php/Woodcutter
    #[serde(alias = "WOODCUTTER")]
    Woodcutter,
    /// https://dwarffortresswiki.org/index.php/Stoneworker
    #[serde(alias = "STONEWORKER")]
    Stoneworker,
    /// https://dwarffortresswiki.org/index.php/Engraver
    #[serde(alias = "ENGRAVER")]
    Engraver,
    /// https://dwarffortresswiki.org/index.php/Mason
    #[serde(alias = "MASON")]
    Mason,
    /// https://dwarffortresswiki.org/index.php/Ranger
    #[serde(alias = "RANGER")]
    Ranger,
    /// https://dwarffortresswiki.org/index.php/Animal_caretaker
    #[serde(alias = "ANIMAL_CARETAKER")]
    AnimalCaretaker,
    /// https://dwarffortresswiki.org/index.php/Animal_trainer
    #[serde(alias = "ANIMAL_TRAINER")]
    AnimalTrainer,
    /// https://dwarffortresswiki.org/index.php/Hunter
    #[serde(alias = "HUNTER")]
    Hunter,
    /// https://dwarffortresswiki.org/index.php/Trapper
    #[serde(alias = "TRAPPER")]
    Trapper,
    /// https://dwarffortresswiki.org/index.php/Animal_dissector
    #[serde(alias = "ANIMAL_DISSECTOR")]
    AnimalDissector,
    /// https://dwarffortresswiki.org/index.php/Metalsmith
    #[serde(alias = "METALSMITH")]
    Metalsmith,
    /// https://dwarffortresswiki.org/index.php/Furnace_operator
    #[serde(alias = "FURNACE_OPERATOR")]
    FurnaceOperator,
    /// https://dwarffortresswiki.org/index.php/Weaponsmith
    #[serde(alias = "WEAPONSMITH")]
    Weaponsmith,
    /// https://dwarffortresswiki.org/index.php/Armorsmith
    #[serde(alias = "ARMORER")]
    Armorer,
    /// https://dwarffortresswiki.org/index.php/Blacksmith
    #[serde(alias = "BLACKSMITH")]
    Blacksmith,
    /// https://dwarffortresswiki.org/index.php/Metalcrafter
    #[serde(alias = "METALCRAFTER")]
    Metalcrafter,
    /// https://dwarffortresswiki.org/index.php/Jeweler
    #[serde(alias = "JEWELER")]
    Jeweler,
    /// https://dwarffortresswiki.org/index.php/Gem_cutter
    #[serde(alias = "GEM_CUTTER")]
    GemCutter,
    /// https://dwarffortresswiki.org/index.php/Gem_setter
    #[serde(alias = "GEM_SETTER")]
    GemSetter,
    /// https://dwarffortresswiki.org/index.php/Craftsdwarf
    #[serde(alias = "CRAFTSMAN")]
    Craftsman,
    /// https://dwarffortresswiki.org/index.php/Woodcrafter
    #[serde(alias = "WOODCRAFTER")]
    Woodcrafter,
    /// https://dwarffortresswiki.org/index.php/Stonecrafter
    #[serde(alias = "STONECRAFTER")]
    Stonecrafter,
    /// https://dwarffortresswiki.org/index.php/Leatherworker
    #[serde(alias = "LEATHERWORKER")]
    Leatherworker,
    /// https://dwarffortresswiki.org/index.php/Bone_carver
    #[serde(alias = "BONE_CARVER")]
    BoneCarver,
    /// https://dwarffortresswiki.org/index.php/Weaver
    #[serde(alias = "WEAVER")]
    Weaver,
    /// https://dwarffortresswiki.org/index.php/Clothier
    #[serde(alias = "CLOTHIER")]
    Clothier,
    /// https://dwarffortresswiki.org/index.php/Glassmaker
    #[serde(alias = "GLASSMAKER")]
    Glassmaker,
    /// https://dwarffortresswiki.org/index.php/Potter
    #[serde(alias = "POTTER")]
    Potter,
    /// https://dwarffortresswiki.org/index.php/Glazer
    #[serde(alias = "GLAZER")]
    Glazer,
    /// https://dwarffortresswiki.org/index.php/Wax_worker
    #[serde(alias = "WAX_WORKER")]
    WaxWorker,
    /// https://dwarffortresswiki.org/index.php/Strand_extractor
    #[serde(alias = "STRAND_EXTRACTOR")]
    StrandExtractor,
    /// https://dwarffortresswiki.org/index.php/Fishery_worker
    #[serde(alias = "FISHERY_WORKER")]
    FisheryWorker,
    /// https://dwarffortresswiki.org/index.php/Fisherdwarf
    #[serde(alias = "FISHERMAN")]
    Fisherman,
    /// https://dwarffortresswiki.org/index.php/Fish_dissector
    #[serde(alias = "FISH_DISSECTOR")]
    FishDissector,
    /// https://dwarffortresswiki.org/index.php/Fish_cleaner
    #[serde(alias = "FISH_CLEANER")]
    FishCleaner,
    /// https://dwarffortresswiki.org/index.php/Farmer
    #[serde(alias = "FARMER")]
    Farmer,
    /// https://dwarffortresswiki.org/index.php/Cheese_maker
    #[serde(alias = "CHEESE_MAKER")]
    CheeseMaker,
    /// https://dwarffortresswiki.org/index.php/Milker
    #[serde(alias = "MILKER")]
    Milker,
    /// https://dwarffortresswiki.org/index.php/Cook
    #[serde(alias = "COOK")]
    Cook,
    /// https://dwarffortresswiki.org/index.php/Thresher
    #[serde(alias = "THRESHER")]
    Thresher,
    /// https://dwarffortresswiki.org/index.php/Miller
    #[serde(alias = "MILLER")]
    Miller,
    /// https://dwarffortresswiki.org/index.php/Butcher
    #[serde(alias = "BUTCHER")]
    Butcher,
    /// https://dwarffortresswiki.org/index.php/Tanner
    #[serde(alias = "TANNER")]
    Tanner,
    /// https://dwarffortresswiki.org/index.php/Dyer
    #[serde(alias = "DYER")]
    Dyer,
    /// https://dwarffortresswiki.org/index.php/Grower
    #[serde(alias = "PLANTER")]
    Planter,
    /// https://dwarffortresswiki.org/index.php/Herbalist
    #[serde(alias = "HERBALIST")]
    Herbalist,
    /// https://dwarffortresswiki.org/index.php/Brewer
    #[serde(alias = "BREWER")]
    Brewer,
    /// https://dwarffortresswiki.org/index.php/Soaper
    #[serde(alias = "SOAP_MAKER")]
    SoapMaker,
    /// https://dwarffortresswiki.org/index.php/Potash_maker
    #[serde(alias = "POTASH_MAKER")]
    PotashMaker,
    /// https://dwarffortresswiki.org/index.php/Lye_maker
    #[serde(alias = "LYE_MAKER")]
    LyeMaker,
    /// https://dwarffortresswiki.org/index.php/Wood_burner
    #[serde(alias = "WOOD_BURNER")]
    WoodBurner,
    /// https://dwarffortresswiki.org/index.php/Shearer
    #[serde(alias = "SHEARER")]
    Shearer,
    /// https://dwarffortresswiki.org/index.php/Spinner
    #[serde(alias = "SPINNER")]
    Spinner,
    /// https://dwarffortresswiki.org/index.php/Presser
    #[serde(alias = "PRESSER")]
    Presser,
    /// https://dwarffortresswiki.org/index.php/Beekeeper
    #[serde(alias = "BEEKEEPER")]
    Beekeeper,
    /// https://dwarffortresswiki.org/index.php/Engineer
    #[serde(alias = "ENGINEER")]
    Engineer,
    /// https://dwarffortresswiki.org/index.php/Mechanic
    #[serde(alias = "MECHANIC")]
    Mechanic,
    /// https://dwarffortresswiki.org/index.php/Siege_engineer
    #[serde(alias = "SIEGE_ENGINEER")]
    SiegeEngineer,
    /// https://dwarffortresswiki.org/index.php/Siege_operator
    #[serde(alias = "SIEGE_OPERATOR")]
    SiegeOperator,
    /// https://dwarffortresswiki.org/index.php/Pump_operator
    #[serde(alias = "PUMP_OPERATOR")]
    PumpOperator,
    /// https://dwarffortresswiki.org/index.php/Clerk
    #[serde(alias = "CLERK")]
    Clerk,
    /// https://dwarffortresswiki.org/index.php/Administrator
    #[serde(alias = "ADMINISTRATOR")]
    Administrator,
    /// https://dwarffortresswiki.org/index.php/Trader
    #[serde(alias = "TRADER")]
    Trader,
    /// https://dwarffortresswiki.org/index.php/Architect
    #[serde(alias = "ARCHITECT")]
    Architect,
    /// https://dwarffortresswiki.org/index.php/Alchemist
    #[serde(alias = "ALCHEMIST")]
    Alchemist,
    /// https://dwarffortresswiki.org/index.php/Doctor
    #[serde(alias = "DOCTOR")]
    Doctor,
    /// https://dwarffortresswiki.org/index.php/Diagnostician
    #[serde(alias = "DIAGNOSER")]
    Diagnoser,
    /// https://dwarffortresswiki.org/index.php/Bone_doctor
    #[serde(alias = "BONE_SETTER")]
    BoneSetter,
    /// https://dwarffortresswiki.org/index.php/Suturer
    #[serde(alias = "SUTURER")]
    Suturer,
    /// https://dwarffortresswiki.org/index.php/Surgeon
    #[serde(alias = "SURGEON")]
    Surgeon,
    /// https://dwarffortresswiki.org/index.php/Merchant
    #[serde(alias = "MERCHANT")]
    Merchant,
    /// https://dwarffortresswiki.org/index.php/Hammerman
    #[serde(alias = "HAMMERMAN")]
    Hammerman,
    /// https://dwarffortresswiki.org/index.php/Hammer_lord
    #[serde(alias = "MASTER_HAMMERMAN")]
    MasterHammerman,
    /// https://dwarffortresswiki.org/index.php/Spearman
    #[serde(alias = "SPEARMAN")]
    Spearman,
    /// https://dwarffortresswiki.org/index.php/Spearmaster
    #[serde(alias = "MASTER_SPEARMAN")]
    MasterSpearman,
    /// https://dwarffortresswiki.org/index.php/Crossbowman
    #[serde(alias = "CROSSBOWMAN")]
    Crossbowman,
    /// https://dwarffortresswiki.org/index.php/Elite_crossbowman
    #[serde(alias = "MASTER_CROSSBOWMAN")]
    MasterCrossbowman,
    /// https://dwarffortresswiki.org/index.php/Wrestler
    #[serde(alias = "WRESTLER")]
    Wrestler,
    /// https://dwarffortresswiki.org/index.php/Elite_wrestler
    #[serde(alias = "MASTER_WRESTLER")]
    MasterWrestler,
    /// https://dwarffortresswiki.org/index.php/Axeman
    #[serde(alias = "AXEMAN")]
    Axeman,
    /// https://dwarffortresswiki.org/index.php/Axe_lord
    #[serde(alias = "MASTER_AXEMAN")]
    MasterAxeman,
    /// https://dwarffortresswiki.org/index.php/Swordsman
    #[serde(alias = "SWORDSMAN")]
    Swordsman,
    /// https://dwarffortresswiki.org/index.php/Swordmaster
    #[serde(alias = "MASTER_SWORDSMAN")]
    MasterSwordsman,
    /// https://dwarffortresswiki.org/index.php/Maceman
    #[serde(alias = "MACEMAN")]
    Maceman,
    /// https://dwarffortresswiki.org/index.php/Mace_lord
    #[serde(alias = "MASTER_MACEMAN")]
    MasterMaceman,
    /// https://dwarffortresswiki.org/index.php/Pikeman
    #[serde(alias = "PIKEMAN")]
    Pikeman,
    /// https://dwarffortresswiki.org/index.php/Pikemaster
    #[serde(alias = "MASTER_PIKEMAN")]
    MasterPikeman,
    /// https://dwarffortresswiki.org/index.php/Bowman
    #[serde(alias = "BOWMAN")]
    Bowman,
    /// https://dwarffortresswiki.org/index.php/Elite_bowman
    #[serde(alias = "MASTER_BOWMAN")]
    MasterBowman,
    /// https://dwarffortresswiki.org/index.php/Blowgunner
    #[serde(alias = "BLOWGUNMAN")]
    Blowgunman,
    /// https://dwarffortresswiki.org/index.php/Master_blowgunner
    #[serde(alias = "MASTER_BLOWGUNMAN")]
    MasterBlowgunman,
    /// https://dwarffortresswiki.org/index.php/Lasher
    #[serde(alias = "LASHER")]
    Lasher,
    /// https://dwarffortresswiki.org/index.php/Master_lasher
    #[serde(alias = "MASTER_LASHER")]
    MasterLasher,
    /// https://dwarffortresswiki.org/index.php/Recruit
    #[serde(alias = "RECRUIT")]
    Recruit,
    /// https://dwarffortresswiki.org/index.php/Hunting_animal
    #[serde(alias = "TRAINED_HUNTER")]
    TrainedHunter,
    /// https://dwarffortresswiki.org/index.php/War_animal
    #[serde(alias = "TRAINED_WAR")]
    TrainedWar,
    /// https://dwarffortresswiki.org/index.php/Master_thief
    #[serde(alias = "MASTER_THIEF")]
    MasterThief,
    /// https://dwarffortresswiki.org/index.php/Thief
    #[serde(alias = "THIEF")]
    Thief,
    /// https://dwarffortresswiki.org/index.php/Peasant
    #[serde(alias = "STANDARD")]
    Standard,
    /// https://dwarffortresswiki.org/index.php/Child
    #[serde(alias = "CHILD")]
    Child,
    /// https://dwarffortresswiki.org/index.php/Baby
    #[serde(alias = "BABY")]
    Baby,
    /// https://dwarffortresswiki.org/index.php/Drunk
    #[serde(alias = "DRUNK")]
    Drunk,
    /// https://dwarffortresswiki.org/index.php/Monster_slayer
    #[serde(alias = "MONSTER_SLAYER")]
    MonsterSlayer,
    /// https://dwarffortresswiki.org/index.php/Scout
    #[serde(alias = "SCOUT")]
    Scout,
    /// https://dwarffortresswiki.org/index.php/Beast_hunter
    #[serde(alias = "BEAST_HUNTER")]
    BeastHunter,
    /// https://dwarffortresswiki.org/index.php/Snatcher
    #[serde(alias = "SNATCHER")]
    Snatcher,
    /// https://dwarffortresswiki.org/index.php/Mercenary
    #[serde(alias = "MERCENARY")]
    Mercenary,
    /// https://dwarffortresswiki.org/index.php/Gelder
    #[serde(alias = "GELDER")]
    Gelder,
    /// https://dwarffortresswiki.org/index.php/Performer
    #[serde(alias = "PERFORMER")]
    Performer,
    /// https://dwarffortresswiki.org/index.php/Poet
    #[serde(alias = "POET")]
    Poet,
    /// https://dwarffortresswiki.org/index.php/Bard
    #[serde(alias = "BARD")]
    Bard,
    /// https://dwarffortresswiki.org/index.php/Dancer
    #[serde(alias = "DANCER")]
    Dancer,
    /// https://dwarffortresswiki.org/index.php/Sage
    #[serde(alias = "SAGE")]
    Sage,
    /// https://dwarffortresswiki.org/index.php/Scholar
    #[serde(alias = "SCHOLAR")]
    Scholar,
    /// https://dwarffortresswiki.org/index.php/Philosopher
    #[serde(alias = "PHILOSOPHER")]
    Philosopher,
    /// https://dwarffortresswiki.org/index.php/Mathematician
    #[serde(alias = "MATHEMATICIAN")]
    Mathematician,
    /// https://dwarffortresswiki.org/index.php/Historian
    #[serde(alias = "HISTORIAN")]
    Historian,
    /// https://dwarffortresswiki.org/index.php/Astronomer
    #[serde(alias = "ASTRONOMER")]
    Astronomer,
    /// https://dwarffortresswiki.org/index.php/Naturalist
    #[serde(alias = "NATURALIST")]
    Naturalist,
    /// https://dwarffortresswiki.org/index.php/Chemist
    #[serde(alias = "CHEMIST")]
    Chemist,
    /// https://dwarffortresswiki.org/index.php/Geographer
    #[serde(alias = "GEOGRAPHER")]
    Geographer,
    /// https://dwarffortresswiki.org/index.php/Scribe
    #[serde(alias = "SCRIBE")]
    Scribe,
    /// https://dwarffortresswiki.org/index.php/Papermaker
    #[serde(alias = "PAPERMAKER")]
    Papermaker,
    /// https://dwarffortresswiki.org/index.php/Bookbinder
    #[serde(alias = "BOOKBINDER")]
    Bookbinder,
    /// https://dwarffortresswiki.org/index.php/Tavern_keeper
    #[serde(alias = "TAVERN_KEEPER")]
    TavernKeeper,
    /// https://dwarffortresswiki.org/index.php/Criminal
    #[serde(alias = "CRIMINAL")]
    Criminal,
    /// https://dwarffortresswiki.org/index.php/Peddler
    #[serde(alias = "PEDDLER")]
    Peddler,
    /// https://dwarffortresswiki.org/index.php/Prophet
    #[serde(alias = "PROPHET")]
    Prophet,
    /// https://dwarffortresswiki.org/index.php/Pilgrim
    #[serde(alias = "PILGRIM")]
    Pilgrim,
    /// https://dwarffortresswiki.org/index.php/Monk
    #[serde(alias = "MONK")]
    Monk,
    /// https://dwarffortresswiki.org/index.php/Messenger
    #[serde(alias = "MESSENGER")]
    Messenger,
}

impl Default for UnitTypeEnum {
    fn default() -> Self {
        Self::Miner
    }
}
