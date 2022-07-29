use crate::core::{Choose, Clamp, Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

use crate::structure::CreatureToken;
use crate::structure::{AllEnum, MaleOrFemaleEnum, WeaponSkillEnum};

/// Defines a leader/noble position for a civilization. These replace previous tags such as
/// `[MAYOR]` and `[CAN_HAVE_SITE_LEADER]` and so on.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EntityPosition {
    /// argument 1 of `POSITION`
    #[serde(alias = "POSITION")]
    pub reference: Option<Reference>,
    /// A list of responsibilties this position has.
    #[serde(alias = "RESPONSIBILITY")]
    pub responsibility: Vec<Responsibility>,
    /// The position holder is not subjected to the economy. Less than relevant right now.
    #[serde(alias = "ACCOUNT_EXEMPT")]
    pub account_exempt: Option<()>,
    /// Only creatures with the specified class token can be appointed to this position.
    #[serde(alias = "ALLOWED_CLASS")]
    pub allowed_class: Option<Reference>, // TODO: ref is creature class
    /// Restricts the position to only the defined caste. Only works with a caste of the entity's
    /// current race. (If the entity had multiple `CREATURE` tokens)
    #[serde(alias = "ALLOWED_CREATURE")]
    pub allowed_creature: Vec<(ReferenceTo<CreatureToken>, Option<Reference>)>, // TODO ref is caste
    /// This position can only be chosen for the task from the nobles screen, and is available only
    /// if there is an *argument* present. For example, the `GENERAL` is `[APPOINTED_BY:MONARCH]`.
    /// Contrast `[ELECTED]`. Being appointed by a `MONARCH` seems to handle a lot of worldgen
    /// stuff, and interferes with fort mode titles.
    #[serde(alias = "APPOINTED_BY")]
    pub appointed_by: Vec<Reference>, // TODO: ref here is EntityPosition
    /// A creature that kills a member of this position will be sure to talk about it a lot.
    #[serde(alias = "BRAG_ON_KILL")]
    pub brag_on_kill: Option<()>,
    /// In adventure mode, when referencing locations, an `NPC` may mention this position holder
    /// living there or having done some deed there, it also means that the position exists in
    /// world-gen, rather than being created only at the end of world-gen.
    ///
    /// Dark Fortress civs cannot have this tag on anybody but their Law Maker, or the game will
    /// crash without leaving an errorlog.
    #[serde(alias = "CHAT_WORTHY")]
    pub chat_worthy: Option<()>,
    /// Creatures of this position will have this color, instead of their profession color, e.g.
    /// `[COLOR:5:0:1]`.
    #[serde(alias = "COLOR")]
    pub color: Option<(u8, u8, u8)>,
    /// This position will act as a commander of the specified position (presumably).
    /// E.g. `GENERAL` is `[COMMANDER:LIEUTENANT:ALL]`. Unknown if values other than `ALL` work.
    #[serde(alias = "COMMANDER")]
    pub commander: Option<(Reference, AllEnum)>, // TODO: ref here is an EntityPosition
    /// This position is a puppet ruler left behind in a conquered site.
    #[serde(alias = "CONQUERED_SITE")]
    pub conquered_site: Option<()>,
    /// How many demands the position can make of the population at one time.
    #[serde(alias = "DEMAND_MAX")]
    pub demand_max: Option<Clamp<u8, 0, 100>>,
    /// The site's (or civ's) minted coins, if any, will have images that reflect the personality of
    /// this position holder.
    #[serde(alias = "DETERMINES_COIN_DESIGN")]
    pub determines_coin_design: Option<()>,
    /// The position won't be culled from Legends as "unimportant" during world generation.
    #[serde(alias = "DO_NOT_CULL")]
    pub do_not_cull: Option<()>,
    /// Members of this position will never agree to 'join' your character during adventure mode.
    #[serde(alias = "DUTY_BOUND")]
    pub duty_bound: Option<()>,
    /// The population will periodically select the most skill-eligible creature to fill this
    /// position. Contrast `[APPOINTED_BY]`.
    #[serde(alias = "ELECTED")]
    pub elected: Option<()>,
    /// The various members who have filled this role will be listed in the civilisation's history.
    #[serde(alias = "EXPORTED_IN_LEGENDS")]
    pub exported_in_legends: Option<()>,
    /// The creature holding this position will visibly flash, like legendary citizens. Represents a
    /// properly noble station by default.
    #[serde(alias = "FLASHES")]
    pub flashes: Option<()>,
    /// The position can only be held by the specified gender. Currently bugged:
    /// [Bug:2714](https://www.bay12games.com/dwarves/mantisbt/view.php?id=2714).
    #[serde(alias = "GENDER")]
    pub gender: Option<MaleOrFemaleEnum>,
    /// The position can assign quests to adventurers.
    #[serde(alias = "KILL_QUEST")]
    pub kill_quest: Option<()>,
    /// This is an alternative to `SITE`. What it does is allow positions to be created at civ-level
    /// 'as needed' for all sites that meet the requirements to have them, which are the values set
    /// in `LAND_HOLDER_TRIGGER`. The character is tied permanently to a particular site but also
    /// operates at the civ-level.
    #[serde(alias = "LAND_HOLDER")]
    pub land_holder: Option<Clamp<u8, 1, 3>>,
    /// The name the area takes on when under the control of a `LAND_HOLDER`. E.g. for the `DUKE`,
    /// `[LAND_NAME:a duchy]`.
    #[serde(alias = "LAND_NAME")]
    pub land_name: Option<String>,
    /// The maximum number of mandates the position can make at once.
    #[serde(alias = "MANDATE_MAX")]
    pub mandate_max: Option<u8>,
    /// The position holder cannot be assigned labors. Currently nonfunctional.
    /// [Bug:3721](https://www.bay12games.com/dwarves/mantisbt/view.php?id=3721).
    #[serde(alias = "MENIAL_WORK_EXEMPTION")]
    pub menial_work_exemption: Option<()>,
    /// The spouse of the position holder doesn't have to work/cannot be assigned labors.
    /// Currently nonfunctional.
    /// [Bug:3721](https://www.bay12games.com/dwarves/mantisbt/view.php?id=3721).
    #[serde(alias = "MENIAL_WORK_EXEMPTION_SPOUSE")]
    pub menial_work_exemption_spouse: Option<()>,
    /// This position cannot be appointed from the nobles screen. Intended for militia captains and
    /// other squad leaders to reduce clutter. Currently nonfunctional:
    /// [Bug:8965](https://www.bay12games.com/dwarves/mantisbt/view.php?id=8965).
    #[serde(alias = "MILITARY_SCREEN_ONLY")]
    pub military_screen_only: Option<()>,
    /// The name of the position.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// If the creature holding the position is male, this is the position's name. E.g. for
    /// `MONARCH`, `[NAME_MALE:king:kings]`.
    #[serde(alias = "NAME_MALE")]
    pub name_male: Option<(String, String)>,
    /// If the creature holding the position is female, this is the position's name. E.g. for
    /// `MONARCH`, `[NAME_FEMALE:queen:queens]`.
    #[serde(alias = "NAME_FEMALE")]
    pub name_female: Option<(String, String)>,
    /// How many of the position there should be. If the `[SITE]` token exists, this is per site,
    /// otherwise this is per civilisation.
    #[serde(alias = "NUMBER")]
    pub number: Option<Choose<u8, AsNeededEnum>>,
    /// How important the position is in society; a lower number is more important and displayed
    /// higher in the Nobles menu. For `MONARCH` it's 1, for `MILITIA_CAPTAIN` it's 200.
    #[serde(alias = "PRECEDENCE")]
    pub precedence: Option<Clamp<u32, 0, 30_000>>,
    /// The position holder will not be held accountable for his or her crimes. Currently
    /// nonfunctional. [Bug:4589](http://www.bay12games.com/dwarves/mantisbt/view.php?id=4589).
    #[serde(alias = "PUNISHMENT_EXEMPTION")]
    pub punishment_exemption: Option<()>,
    /// The position holder can give quests in Adventure mode. Functionality in 0.31.13 and later is
    /// uncertain.
    #[serde(alias = "QUEST_GIVER")]
    pub quest_giver: Option<()>,
    /// Creatures of the specified class cannot be appointed to this position.
    #[serde(alias = "REJECTED_CLASS")]
    pub rejected_class: Vec<Reference>, // TODO: ref is creature class
    /// Restricts position holders by `CREATURE` type.
    #[serde(alias = "REJECTED_CREATURE")]
    pub rejected_creature: Vec<(ReferenceTo<CreatureToken>, Option<Reference>)>, // TODO ref is caste
    /// This position is absorbed by another down the line. For example, expedition leader is
    /// `[REPLACED_BY:MAYOR]`.
    #[serde(alias = "REPLACED_BY")]
    pub replaced_by: Option<Reference>, // TODO: semantic check; this should be a POSITION
    /// The position holder requires a bedroom with at least this value.
    #[serde(alias = "REQUIRED_BEDROOM")]
    pub required_bedroom: Option<Clamp<u32, 0, 1_000_000>>,
    /// The position holder requires at least this many boxes.
    #[serde(alias = "REQUIRED_BOXES")]
    pub required_boxes: Option<Clamp<u8, 0, 100>>,
    /// The position holder requires at least this many cabinets.
    #[serde(alias = "REQUIRED_CABINETS")]
    pub required_cabinets: Option<Clamp<u8, 0, 100>>,
    /// The position holder requires a dining room with at least this value.
    #[serde(alias = "REQUIRED_DINING")]
    pub required_dining: Option<Clamp<u32, 0, 1_000_000>>,
    /// The position holder requires an office with at least this value.
    #[serde(alias = "REQUIRED_OFFICE")]
    pub required_office: Option<Clamp<u32, 0, 1_000_000>>,
    /// The position holder requires at least this many weapon racks.
    #[serde(alias = "REQUIRED_RACKS")]
    pub required_racks: Option<Clamp<u8, 0, 100>>,
    /// The position holder requires at least this many armour stands.
    #[serde(alias = "REQUIRED_STANDS")]
    pub required_stands: Option<Clamp<u8, 0, 100>>,
    /// The position holder requires a tomb with at least this value.
    #[serde(alias = "REQUIRED_TOMB")]
    pub required_tomb: Option<Clamp<u32, 0, 1_000_000>>,
    /// Does not have anything directly to do with markets. It means that in minor sites (such as
    /// hillocks) the position will not appear, while in major sites (such as dwarf fortresses) it
    /// will.
    #[serde(alias = "REQUIRES_MARKET")]
    pub requires_market: Option<()>,
    /// The position requires the population to be at least this number before it becomes available,
    /// or before the position holder will move in.
    #[serde(alias = "REQUIRES_POPULATION")]
    pub requires_population: Option<u32>,
    /// If there is a special location set aside for rulers, such as a human castle/mead hall, the
    /// position holder will always be found at that particular location. Does nothing for dwarven
    /// nobles, because at present, dwarves have no such special locations.
    #[serde(alias = "RULES_FROM_LOCATION")]
    pub rules_from_location: Option<()>,
    /// Every site government will have the defined number of this position instead of the whole
    /// civilization; provided that other criteria (if any) are met. Unless `LAND_HOLDER` is present
    /// instead, the defined number of the position will be created only for the civilization as a
    /// whole.
    #[serde(alias = "SITE")]
    pub site: Option<()>,
    /// The position holder will get upset if someone with a higher `PRECEDENCE` holds quarters with
    /// a greater value than their own.
    #[serde(alias = "SLEEP_PRETENSION")]
    pub sleep_pretension: Option<()>,
    /// The civilization will inter the corpse of the position holder in a special grave, either in
    /// catacombs or in monuments. If that grave is disturbed, the position holder can return as a
    /// mummy (unverified).
    #[serde(alias = "SPECIAL_BURIAL")]
    pub special_burial: Option<()>,
    /// The name of the position holder's spouse.
    #[serde(alias = "SPOUSE")]
    pub spouse: Option<(String, String)>,
    /// If the spouse of the creature holding the position is female, this is the spouse's position
    /// name.
    #[serde(alias = "SPOUSE_FEMALE")]
    pub spouse_female: Option<(String, String)>,
    /// If the spouse of the creature holding the position is male, this is the spouse's position
    /// name.
    #[serde(alias = "SPOUSE_MALE")]
    pub spouse_male: Option<(String, String)>,
    /// The position holder is authorized to form a military squad, led by themselves.
    ///
    /// The number denotes the maximum headcount. The noun used to describe the subordinates
    /// (e.g. royal guard) is used in adventure mode for the adventurer.
    #[serde(alias = "SQUAD")]
    pub squad: Option<(Clamp<u8, 0, 10>, String, String)>,
    /// How a new position holder is chosen. A single position can have multiple `BY_POSITION`
    /// tokens. If the type is `BY_POSITION`, the position must be specified.
    #[serde(alias = "SUCCESSION")]
    pub succession: Option<(SuccessionTypeEnum, Option<Reference>)>, // TODO: ref is position
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SuccessionTypeEnum {
    #[serde(alias = "BY_HEIR")]
    ByHeir,
    #[serde(alias = "BY_POSITION")]
    ByPosition,
}
impl Default for SuccessionTypeEnum {
    fn default() -> Self {
        Self::ByHeir
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AsNeededEnum {
    #[serde(alias = "AS_NEEDED")]
    AsNeeded,
}
impl Default for AsNeededEnum {
    fn default() -> Self {
        Self::AsNeeded
    }
}

/// The position holder does a thing. See each enum value for what each responsibility does.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Responsibility {
    /// argument 1 of `Responsibility`
    #[serde(alias = "RESPONSIBILITY")]
    pub reference: Option<ResponsibilityEnum>,
    /// A mandatory sub-tag of `[RESPONSIBILITY:EXECUTIONS]`. Determines the weapon chosen by the
    /// executioner for their work.
    #[serde(alias = "EXECUTION_SKILL")]
    pub execution_skill: Option<WeaponSkillEnum>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ResponsibilityEnum {
    /// Found on bookkeeper. Position will use record keeper skill to keep track of stocks.
    #[serde(alias = "ACCOUNTING")]
    Accounting,
    ///
    #[serde(alias = "ADVISE_LEADERS")]
    AdviseLeaders,
    /// Found on elven ranger captain and human warrior. Effect unknown.
    #[serde(alias = "ATTACK_ENEMIES")]
    AttackEnemies,
    /// Found on champion. Position will lead military training exercises.
    #[serde(alias = "BUILD_MORALE")]
    BuildMorale,
    ///
    #[serde(alias = "BUILDING_SAFETY")]
    BuildingSafety,
    /// Currently unused - was originally assigned to the tax collector.
    #[serde(alias = "COLLECT_TAXES")]
    CollectTaxes,
    ///
    #[serde(alias = "CONSTRUCTION_PERMITS")]
    ConstructionPermits,
    /// Currently unused - was originally assigned to the arsenal dwarf.
    #[serde(alias = "EQUIPMENT_MANIFESTS")]
    EquipmentManifests,
    /// Currently unused - was originally assigned to the Royal Guards (squad members beneath the
    /// Hammerer).
    #[serde(alias = "ESCORT_TAX_COLLECTOR")]
    EscortTaxCollector,
    /// Found on dungeon master and princess.
    #[serde(alias = "ESPIONAGE")]
    Espionage,
    /// Found on outpost liaison. Position travels with the caravan to make trade agreements with
    /// any settlements that it visits, provided they are domestic, report the news and promote
    /// `LAND_HOLDER` positions. Also reports recent news. Presumably has no effect at site level.
    ///
    /// Crucially, it does not visit foreign settlements, but the civ-level `TRADE` position does
    /// the exact same thing in its position.
    #[serde(alias = "ESTABLISH_COLONY_TRADE_AGREEMENTS")]
    EstablishColonyTradeAgreements,
    /// Found on hammerer. Position executes death penalty judgements with a weapon of the
    /// appropriate skill.
    #[serde(alias = "EXECUTIONS")]
    Executions,
    ///
    #[serde(alias = "FIRE_SAFETY")]
    FireSafety,
    ///
    #[serde(alias = "FOOD_SUPPLY")]
    FoodSupply,
    /// Found on chief medical dwarf. Position will use diagnostician skill to enable the z-menu
    /// health screen.
    #[serde(alias = "HEALTH_MANAGEMENT")]
    HealthManagement,
    ///
    #[serde(alias = "JUDGE")]
    Judge,
    /// Found on sheriff/captain of the guard. Position and its subordinates are in charge of
    /// punishing criminals.
    ///
    /// A position holding this responsibility plus the `SQUAD` token (or allowing the entity to
    /// have a `SITE_VARIABLE_POSITION` with this responsibility) is required for an adventurer
    /// from a given civilization to start as a hearthperson, fortress guard, etc.
    #[serde(alias = "LAW_ENFORCEMENT")]
    LawEnforcement,
    /// Found on monarch/landholders. Will be referred to as the leader of the site in adventure
    /// mode and they may be informed as to the site being the capital city for civ-level positions.
    #[serde(alias = "LAW_MAKING")]
    LawMaking,
    ///
    #[serde(alias = "MAINTAIN_BRIDGES")]
    MaintainBridges,
    ///
    #[serde(alias = "MAINTAIN_ROADS")]
    MaintainRoads,
    ///
    #[serde(alias = "MAINTAIN_SEWERS")]
    MaintainSewers,
    ///
    #[serde(alias = "MAINTAIN_TUNNELS")]
    MaintainTunnels,
    /// Position will make a 'social call' to an established foreign settlement, complimenting or
    /// insulting them depending on relations and reporting the news.
    #[serde(alias = "MAKE_INTRODUCTIONS")]
    MakeIntroductions,
    /// Found on diplomat. Position negotiates peace treaties in order to end wars.
    #[serde(alias = "MAKE_PEACE_AGREEMENTS")]
    MakePeaceAgreements,
    /// Found on diplomat. Position negotiates special agreements, such as tree cutting quotas.
    #[serde(alias = "MAKE_TOPIC_AGREEMENTS")]
    MakeTopicAgreements,
    /// Found on dungeon master.
    #[serde(alias = "MANAGE_ANIMALS")]
    ManageAnimals,
    ///
    #[serde(alias = "MANAGE_LEADER_HOUSEHOLD_CLEANLINESS")]
    ManageLeaderHouseholdCleanliness,
    ///
    #[serde(alias = "MANAGE_LEADER_HOUSEHOLD_DRINKS")]
    ManageLeaderHouseholdDrinks,
    ///
    #[serde(alias = "MANAGE_LEADER_HOUSEHOLD_FOOD")]
    ManageLeaderHouseholdFood,
    /// Found on manager. Position enables the use of workshop profiles and uses the organizer skill
    /// to process work orders entered in the job manager after the fortress population reaches 20.
    #[serde(alias = "MANAGE_PRODUCTION")]
    ManageProduction,
    /// Found on expedition leader/mayor. Position uses the various social skills to hold meetings
    /// with unhappy citizens and try to pacify them with happy thoughts.
    #[serde(alias = "MEET_WORKERS")]
    MeetWorkers,
    /// Found on monarch/landholder/leaders. Character is in charge of going to war and making peace
    /// for the government they work for.
    ///
    /// Without a position with this responsibility at civ level the civilization will not be able
    /// to make peace and its sites will wage war on each other constantly, and as a result, all
    /// viable civilizations must have one leader with this tag at civ level. This appears not to
    /// be a problem for kobolds, presumably due to either the `SKULKING` or the `UTTERANCES` tokens.
    #[serde(alias = "MILITARY_GOALS")]
    MilitaryGoals,
    /// Found on general/militia commander. During worldgen, position will go on expeditions to tame
    /// exotic creatures. Means that they will command the armies of their site or civilization.
    ///
    /// Issues the orders for the teams conducting raids or other operations away from the
    /// fortress.
    #[serde(alias = "MILITARY_STRATEGY")]
    MilitaryStrategy,
    ///
    #[serde(alias = "OVERSEE_LEADER_HOUSEHOLD")]
    OverseeLeaderHousehold,
    /// Found on elven ranger captain. Effect unknown.
    #[serde(alias = "PATROL_TERRITORY")]
    PatrolTerritory,
    ///
    #[serde(alias = "PREPARE_LEADER_MEALS")]
    PrepareLeaderMeals,
    /// Found on monarch/landholder/leaders. Position uses the various social skills to hold
    /// meetings with incoming diplomats and liaisons.
    #[serde(alias = "RECEIVE_DIPLOMATS")]
    ReceiveDiplomats,
    /// Found on elven druid. Position informs you about worship cults.
    #[serde(alias = "RELIGION")]
    Religion,
    /// Currently unused - was originally assigned to the arsenal dwarf.
    #[serde(alias = "SORT_AMMUNITION")]
    SortAmmunition,
    /// Position will tame animals with the `[PET_EXOTIC]` token. Currently unused - was originally
    /// assigned to the dungeon master.
    #[serde(alias = "TAME_EXOTICS")]
    TameExotics,
    /// Found on broker. Position will use Appraisal skill to display value estimates and the
    /// various Social skills to trade at the depot.
    ///
    /// When applied to other civilizations, this position will arrive with the caravan to make
    /// trade agreements (like the Human Guild Representative from older versions) and behaves
    /// otherwise like the civ's own `ESTABLISH_COLONY_TRADE_AGREEMENTS` position holder.
    #[serde(alias = "TRADE")]
    Trade,
    /// Currently unused - was originally assigned to the arsenal dwarf.
    #[serde(alias = "UPGRADE_SQUAD_EQUIPMENT")]
    UpgradeSquadEquipment,
}
impl Default for ResponsibilityEnum {
    fn default() -> Self {
        Self::Accounting
    }
}
