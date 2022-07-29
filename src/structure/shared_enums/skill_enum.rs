use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum WeaponSkillEnum {
    #[serde(alias = "AXE")]
    Axe,
    #[serde(alias = "SWORD")]
    Sword,
    #[serde(alias = "DAGGER")]
    Dagger,
    #[serde(alias = "MACE")]
    Mace,
    #[serde(alias = "HAMMER")]
    Hammer,
    #[serde(alias = "SPEAR")]
    Spear,
    #[serde(alias = "CROSSBOW")]
    Crossbow,
    #[serde(alias = "SHIELD")]
    Shield,
    #[serde(alias = "PIKE")]
    Pike,
    #[serde(alias = "WHIP")]
    Whip,
    #[serde(alias = "BOW")]
    Bow,
    #[serde(alias = "BLOWGUN")]
    Blowgun,
}
impl Default for WeaponSkillEnum {
    fn default() -> Self {
        Self::Axe
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MusicSkillEnum {
    #[serde(alias = "PLAY_KEYBOARD_INSTRUMENT")]
    PlayKeyboardInstrument,
    #[serde(alias = "PLAY_STRINGED_INSTRUMENT")]
    PlayStringedInstrument,
    #[serde(alias = "PLAY_WIND_INSTRUMENT")]
    PlayWindInstrument,
    #[serde(alias = "PLAY_PERCUSSION_INSTRUMENT")]
    PlayPercussionInstrument,
}
impl Default for MusicSkillEnum {
    fn default() -> Self {
        Self::PlayKeyboardInstrument
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SkillEnum {
    #[serde(alias = "MINING")]
    Mining,
    #[serde(alias = "WOODCUTTING")]
    Woodcutting,
    #[serde(alias = "CARPENTRY")]
    Carpentry,
    #[serde(alias = "DETAILSTONE")]
    Detailstone,
    #[serde(alias = "MASONRY")]
    Masonry,
    #[serde(alias = "ANIMALTRAIN")]
    Animaltrain,
    #[serde(alias = "ANIMALCARE")]
    Animalcare,
    #[serde(alias = "DISSECT_FISH")]
    DissectFish,
    #[serde(alias = "DISSECT_VERMIN")]
    DissectVermin,
    #[serde(alias = "PROCESSFISH")]
    Processfish,
    #[serde(alias = "BUTCHER")]
    Butcher,
    #[serde(alias = "TRAPPING")]
    Trapping,
    #[serde(alias = "TANNER")]
    Tanner,
    #[serde(alias = "WEAVING")]
    Weaving,
    #[serde(alias = "BREWING")]
    Brewing,
    #[serde(alias = "ALCHEMY")]
    Alchemy,
    #[serde(alias = "CLOTHESMAKING")]
    Clothesmaking,
    #[serde(alias = "MILLING")]
    Milling,
    #[serde(alias = "PROCESSPLANTS")]
    Processplants,
    #[serde(alias = "CHEESEMAKING")]
    Cheesemaking,
    #[serde(alias = "MILK")]
    Milk,
    #[serde(alias = "COOK")]
    Cook,
    #[serde(alias = "PLANT")]
    Plant,
    #[serde(alias = "HERBALISM")]
    Herbalism,
    #[serde(alias = "FISH")]
    Fish,
    #[serde(alias = "SMELT")]
    Smelt,
    #[serde(alias = "EXTRACT_STRAND")]
    ExtractStrand,
    #[serde(alias = "FORGE_WEAPON")]
    ForgeWeapon,
    #[serde(alias = "FORGE_ARMOR")]
    ForgeArmor,
    #[serde(alias = "FORGE_FURNITURE")]
    ForgeFurniture,
    #[serde(alias = "CUTGEM")]
    Cutgem,
    #[serde(alias = "ENCRUSTGEM")]
    Encrustgem,
    #[serde(alias = "WOODCRAFT")]
    Woodcraft,
    #[serde(alias = "STONECRAFT")]
    Stonecraft,
    #[serde(alias = "METALCRAFT")]
    Metalcraft,
    #[serde(alias = "GLASSMAKER")]
    Glassmaker,
    #[serde(alias = "LEATHERWORK")]
    Leatherwork,
    #[serde(alias = "BONECARVE")]
    Bonecarve,
    #[serde(alias = "AXE")]
    Axe,
    #[serde(alias = "SWORD")]
    Sword,
    #[serde(alias = "DAGGER")]
    Dagger,
    #[serde(alias = "MACE")]
    Mace,
    #[serde(alias = "HAMMER")]
    Hammer,
    #[serde(alias = "SPEAR")]
    Spear,
    #[serde(alias = "CROSSBOW")]
    Crossbow,
    #[serde(alias = "SHIELD")]
    Shield,
    #[serde(alias = "ARMOR")]
    Armor,
    #[serde(alias = "SIEGECRAFT")]
    Siegecraft,
    #[serde(alias = "SIEGEOPERATE")]
    Siegeoperate,
    #[serde(alias = "BOWYER")]
    Bowyer,
    #[serde(alias = "PIKE")]
    Pike,
    #[serde(alias = "WHIP")]
    Whip,
    #[serde(alias = "BOW")]
    Bow,
    #[serde(alias = "BLOWGUN")]
    Blowgun,
    #[serde(alias = "THROW")]
    Throw,
    #[serde(alias = "MECHANICS")]
    Mechanics,
    #[serde(alias = "MAGIC_NATURE")]
    MagicNature,
    #[serde(alias = "SNEAK")]
    Sneak,
    #[serde(alias = "DESIGNBUILDING")]
    Designbuilding,
    #[serde(alias = "DRESS_WOUNDS")]
    DressWounds,
    #[serde(alias = "DIAGNOSE")]
    Diagnose,
    #[serde(alias = "SURGERY")]
    Surgery,
    #[serde(alias = "SET_BONE")]
    SetBone,
    #[serde(alias = "SUTURE")]
    Suture,
    #[serde(alias = "CRUTCH_WALK")]
    CrutchWalk,
    #[serde(alias = "WOOD_BURNING")]
    WoodBurning,
    #[serde(alias = "LYE_MAKING")]
    LyeMaking,
    #[serde(alias = "SOAP_MAKING")]
    SoapMaking,
    #[serde(alias = "POTASH_MAKING")]
    PotashMaking,
    #[serde(alias = "DYER")]
    Dyer,
    #[serde(alias = "OPERATE_PUMP")]
    OperatePump,
    #[serde(alias = "SWIMMING")]
    Swimming,
    #[serde(alias = "PERSUASION")]
    Persuasion,
    #[serde(alias = "NEGOTIATION")]
    Negotiation,
    #[serde(alias = "JUDGING_INTENT")]
    JudgingIntent,
    #[serde(alias = "APPRAISAL")]
    Appraisal,
    #[serde(alias = "ORGANIZATION")]
    Organization,
    #[serde(alias = "RECORD_KEEPING")]
    RecordKeeping,
    #[serde(alias = "LYING")]
    Lying,
    #[serde(alias = "INTIMIDATION")]
    Intimidation,
    #[serde(alias = "CONVERSATION")]
    Conversation,
    #[serde(alias = "COMEDY")]
    Comedy,
    #[serde(alias = "FLATTERY")]
    Flattery,
    #[serde(alias = "CONSOLE")]
    Console,
    #[serde(alias = "PACIFY")]
    Pacify,
    #[serde(alias = "TRACKING")]
    Tracking,
    #[serde(alias = "KNOWLEDGE_ACQUISITION")]
    KnowledgeAcquisition,
    #[serde(alias = "CONCENTRATION")]
    Concentration,
    #[serde(alias = "DISCIPLINE")]
    Discipline,
    #[serde(alias = "SITUATIONAL_AWARENESS")]
    SituationalAwareness,
    #[serde(alias = "WRITING")]
    Writing,
    #[serde(alias = "PROSE")]
    Prose,
    #[serde(alias = "POETRY")]
    Poetry,
    #[serde(alias = "READING")]
    Reading,
    #[serde(alias = "SPEAKING")]
    Speaking,
    #[serde(alias = "COORDINATION")]
    Coordination,
    #[serde(alias = "BALANCE")]
    Balance,
    #[serde(alias = "LEADERSHIP")]
    Leadership,
    #[serde(alias = "TEACHING")]
    Teaching,
    #[serde(alias = "MELEE_COMBAT")]
    MeleeCombat,
    #[serde(alias = "RANGED_COMBAT")]
    RangedCombat,
    #[serde(alias = "WRESTLING")]
    Wrestling,
    #[serde(alias = "BITE")]
    Bite,
    #[serde(alias = "GRASP_STRIKE")]
    GraspStrike,
    #[serde(alias = "STANCE_STRIKE")]
    StanceStrike,
    #[serde(alias = "DODGING")]
    Dodging,
    #[serde(alias = "MISC_WEAPON")]
    MiscWeapon,
    #[serde(alias = "KNAPPING")]
    Knapping,
    #[serde(alias = "MILITARY_TACTICS")]
    MilitaryTactics,
    #[serde(alias = "SHEARING")]
    Shearing,
    #[serde(alias = "SPINNING")]
    Spinning,
    #[serde(alias = "POTTERY")]
    Pottery,
    #[serde(alias = "GLAZING")]
    Glazing,
    #[serde(alias = "PRESSING")]
    Pressing,
    #[serde(alias = "BEEKEEPING")]
    Beekeeping,
    #[serde(alias = "WAX_WORKING")]
    WaxWorking,
    #[serde(alias = "CLIMBING")]
    Climbing,
    #[serde(alias = "GELD")]
    Geld,
    #[serde(alias = "DANCE")]
    Dance,
    #[serde(alias = "MAKE_MUSIC")]
    MakeMusic,
    #[serde(alias = "SING")]
    Sing,
    #[serde(alias = "PLAY_KEYBOARD_INSTRUMENT")]
    PlayKeyboardInstrument,
    #[serde(alias = "PLAY_STRINGED_INSTRUMENT")]
    PlayStringedInstrument,
    #[serde(alias = "PLAY_WIND_INSTRUMENT")]
    PlayWindInstrument,
    #[serde(alias = "PLAY_PERCUSSION_INSTRUMENT")]
    PlayPercussionInstrument,
    #[serde(alias = "CRITICAL_THINKING")]
    CriticalThinking,
    #[serde(alias = "LOGIC")]
    Logic,
    #[serde(alias = "MATHEMATICS")]
    Mathematics,
    #[serde(alias = "ASTRONOMY")]
    Astronomy,
    #[serde(alias = "CHEMISTRY")]
    Chemistry,
    #[serde(alias = "GEOGRAPHY")]
    Geography,
    #[serde(alias = "OPTICS_ENGINEER")]
    OpticsEngineer,
    #[serde(alias = "FLUID_ENGINEER")]
    FluidEngineer,
    #[serde(alias = "PAPERMAKING")]
    Papermaking,
    #[serde(alias = "BOOKBINDING")]
    Bookbinding,
    #[serde(alias = "INTRIGUE")]
    Intrigue,
    #[serde(alias = "RIDING")]
    Riding,
}
impl Default for SkillEnum {
    fn default() -> Self {
        Self::Mining
    }
}
