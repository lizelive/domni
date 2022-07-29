use crate::core::{Choose, Clamp, Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

mod entity_enums;
mod entity_position;

use crate::structure::{AllEnum, BiomeEnum, NoneEnum, SphereEnum, UnitTypeEnum};
use crate::structure::{
    AmmoToken, ArmorToken, BuildingToken, CreatureToken, GlovesToken, HelmToken, InorganicToken,
    InstrumentToken, PantsToken, ReactionToken, ShapeToken, ShieldToken, ShoesToken,
    SiegeAmmoToken, SymbolToken, TissueToken, ToolToken, ToyToken, TranslationToken, TrapCompToken,
    WeaponToken,
};

pub use entity_enums::*;
pub use entity_position::*;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EntityToken {
    /// Argument 1 of `[ENTITY:...]`
    #[serde(alias = "ENTITY")]
    pub reference: Option<ReferenceTo<Self>>,
    /// A list of animal definitions for the entity.
    #[serde(alias = "ANIMAL")]
    pub animal: Vec<Animal>,
    /// A list of Tissue Styles for the entity.
    #[serde(alias = "TISSUE_STYLE")]
    pub tissue_style: Vec<TissueStyle>,
    // region: Gameplay ============================================================
    /// Allows creatures from this entity to be playable in adventure mode.
    #[serde(alias = "ALL_MAIN_POPS_CONTROLLABLE")]
    pub all_main_pops_controllable: Option<()>,
    /// Allows playing this entity in fortress mode, and at least one civilization must have this
    /// token. If multiple entities have the `SITE_CONTROLLABLE` token, then at embark the specific
    /// civs can be chosen on the civ list screen, though it will not state what entity the civs
    /// belong to. To check which one, go to the neighbors screen: the entity's race will be at the top.
    #[serde(alias = "SITE_CONTROLLABLE")]
    pub site_controllable: Option<()>,
    /// The type of creature that will inhabit the civilization. If multiple creature types are
    /// specified, each civilization of this entity type will randomly choose one of the creatures.
    ///
    /// In entities with multiple possible creatures, you can manipulate the chance of one creature
    /// being chosen by adding multiple identical creature tags.
    /// For instance, adding `[CREATURE:DWARF][CREATURE:DWARF][CREATURE:DWARF][CREATURE:ELF]` to the
    /// same entity will make the civs created about 75% dwarven, 25% elven.
    #[serde(alias = "CREATURE")]
    pub creature: Vec<ReferenceTo<CreatureToken>>,
    /// Found on generated angel entities. Appears to draw from creatures with this `HFID`, which
    /// associates the entity with a historical figure of the same `ID` corresponding to a deity.
    #[serde(alias = "SOURCE_HFID")]
    // `TODO` mark this; since it's generated, I don't think it's used
    pub source_hfid: Option<u32>,
    // endregion ===================================================================
    // region: Placement ===========================================================
    /// Controls the expansion of the civilization's territory.
    /// The higher the number is relative to other `BIOME_SUPPORT` tokens in the entity, the faster
    /// it can spread through the biome.
    ///
    /// These numbers are evaluated relative to each other, i.e. if one biome is 1 and the other is
    /// 2, the spread will be the same as if one was 100 and the other was 200.
    ///
    /// Civs can spread out over biomes they cannot actually build in; for example, humans spread
    /// quickly over oceans but cannot actually build in them.
    #[serde(alias = "BIOME_SUPPORT")]
    pub biome_support: Vec<(BiomeEnum, u32)>,
    /// If the civ's territory crosses over this biome, it can build settlements here.
    #[serde(alias = "SETTLEMENT_BIOME")]
    pub settlement_biome: Vec<BiomeEnum>,
    /// Combination of `EXCLUSIVE_START_BIOME` and SETTLEMENT_BIOME; allows the civ to start in and
    /// create settlements in the biome.
    #[serde(alias = "START_BIOME")]
    pub start_biome: Vec<BiomeEnum>,
    /// The birth of the civilization can occur in this biome, but cannot (necessarily) build in it.
    /// If the civ does not have `SETTLEMENT_BIOME` or `START_BIOME` for the biome in question, it
    /// will only construct a single settlement there.
    #[serde(alias = "EXCLUSIVE_START_BIOME")]
    pub exclusive_start_biome: Option<BiomeEnum>, // TODO: check if this can be Vec
    /// Valid site types are `DARK_FORTRESS` (π), `CAVE` (•), `CAVE_DETAILED` (Ω), `TREE_CITY` (î),
    /// and `CITY` (#). Defaults to `CITY`. Also recognizes `PLAYER_FORTRESS` (creates a civ
    /// of hillocks only), and `MONUMENT` (creates a civ without visible sites (except tombs
    /// and castles), but may cause worldgen crashes).
    ///
    /// Selecting `CAVE` causes the classic kobold behavior of not showing up on the "neighbors"
    /// section of the site selection screen. Selecting `DARK_FORTRESS` also allows generation of
    /// certain other structures. `CAVE_DETAILED` civilizations will create fortresses in
    /// mountainous regions and hillocks in non-mountainous regions.
    #[serde(alias = "DEFAULT_SITE_TYPE")]
    pub default_site_type: Option<SiteTypeEnum>,
    /// Most residents will try to move to this site type, unless already at one.
    #[serde(alias = "LIKES_SITE")]
    pub likes_site: Vec<SiteTypeEnum>,
    /// Some residents will try to move to this site type, unless already at one.
    #[serde(alias = "TOLERATES_SITE")]
    pub tolerates_site: Vec<SiteTypeEnum>,
    /// Controls which constructions the civ will build on the world map.
    /// Valid constructions are `ROAD`, `TUNNEL`, `BRIDGE`, and `WALL`.
    #[serde(alias = "WORLD_CONSTRUCTION")]
    pub world_construction: Vec<ConstructionEnum>, // TODO: semantic, check no duplicate second args
    // endregion ===================================================================
    // region: Population ==========================================================
    /// Max historical population per entity.
    /// Multiply this by max starting civ to get the total maximum historical population of the
    /// species. Defaults to 500.
    #[serde(alias = "MAX_POP_NUMBER")]
    pub max_pop_number: Option<u32>,
    /// Max historical population per individual site. Defaults to 50.
    #[serde(alias = "MAX_SITE_POP_NUMBER")]
    pub max_site_pop_number: Option<u32>,
    /// Max number of civ to spawn at world generation. Defaults to 3. Worldgen picks entities in
    /// some sequential order from the raws, and once it reaches the end of the list, it will begin
    /// again at the top.
    ///
    /// Setting this number lower than 100, like say, 7, will cause worldgen to skip over the civ
    /// for placement if there are already 7 civs of this type. Note that if all civs are set to
    /// lower numbers, and the number of starting civs is set higher than the maximum possible
    /// amount of civs in total, it will gracefully stop placing civs and get down to the history
    /// aspect of worldgen.
    #[serde(alias = "MAX_STARTING_CIV_NUMBER")]
    pub max_starting_civ_number: Option<u32>,
    // endregion ===================================================================
    // region: Flavor ==============================================================
    /// The named, custom building can be built by a civilization in Fortress Mode.
    #[serde(alias = "PERMITTED_BUILDING")]
    pub permitted_building: Vec<ReferenceTo<BuildingToken>>,
    /// Allows this job type to be selected by this entity. This applies to worldgen creatures, in
    /// the embark screen, and in play. Certain professions also influence the availability
    /// of materials for trade.
    #[serde(alias = "PERMITTED_JOB")]
    pub permitted_job: Vec<UnitTypeEnum>, // TODO: prevent duplicate args
    /// Allows this reaction to be used by a civilization. It is used primarily in Fortress Mode,
    /// but also allows certain resources, such as steel, to be available to a race.
    ///
    /// When creating custom reactions, this token must be present or the player will not be able to
    /// use the reaction in Fortress Mode.
    #[serde(alias = "PERMITTED_REACTION")]
    pub permitted_reaction: Vec<ReferenceTo<ReactionToken>>,
    /// Causes the civ's currency to be numbered with the year it was minted.
    #[serde(alias = "CURRENCY_BY_YEAR")]
    pub currency_by_year: Option<()>,
    /// What kind of metals the civ uses for coin minting as well as the value of the coin.
    #[serde(alias = "CURRENCY")] // TODO: the way material tokens are referred to is weird
    pub currency: Vec<(ReferenceTo<InorganicToken>, u32)>,
    /// Accepts values `OWN_RACE`, `FANCIFUL`, `EVIL`, and `GOOD`. The default value is 256.
    #[serde(alias = "ART_FACET_MODIFIER")]
    pub art_facet_modifier: Vec<(ArtFacetEnum, Clamp<u32, 0, 25_600>)>,
    /// Accepts values `CREATURE`, `PLANT`, `TREE`, `SHAPE`, and `ITEM`. The default value is 256.
    ///
    /// Determines the chance of each image occurring in that entity's artwork, such as engravings
    /// and on artifacts, for default (non-historical) artwork.
    #[serde(alias = "ART_IMAGE_ELEMENT_MODIFIER")]
    pub art_image_element_modifier: Vec<(ArtImageModifierEnum, Clamp<u32, 0, 25_600>)>,
    /// Accepts values `ART_IMAGE`, `COVERED` or `GLAZED`, `RINGS_HANGING`, `BANDS`, `SPIKES`,
    /// `ITEMSPECIFIC`, `THREAD`, `CLOTH`, and `SEWN_IMAGE`. The default value is 256.
    ///
    /// Determines the chance of the entity using that particular artwork method, such as "encircled
    /// with bands" or "menaces with spikes". This also seems to change the amount that the entity
    /// will pay for items that are improved in these ways in their tokens.
    #[serde(alias = "ITEM_IMPROVEMENT_MODIFIER")]
    pub item_improvement_modifier: Vec<(ItemImprovementModifierEnum, Clamp<u32, 0, 25_600>)>,
    /// What language raw the entity uses.
    /// - If an entity lacks this tag, translations are drawn randomly from all translation files.
    /// Multiple translation tags will only result in the last one being used.
    /// - If `GEN_DIVINE` is entered, the entity will use a generated divine language, that is, the
    /// same language that is used for the names of angels.
    #[serde(alias = "TRANSLATION")]
    pub translation: Option<ReferenceTo<TranslationToken>>,
    /// Causes the entity to more often use these symbols in the particular `SYM` set.
    #[serde(alias = "SELECT_SYMBOL")]
    pub select_symbol: Vec<(SymbolNounEnum, ReferenceTo<SymbolToken>)>,
    /// Causes the symbol set to be preferred as adjectives by the civilization. Used in vanilla to
    /// put violent names in sieges and batttles.
    #[serde(alias = "SUBSELECT_SYMBOL")]
    pub subselect_symbol: Vec<(SymbolNounEnum, ReferenceTo<SymbolToken>)>,
    /// Causes the entity to not use the words in these `SYM` sets.
    #[serde(alias = "CULL_SYMBOL")]
    pub cull_symbol: Vec<(SymbolNounEnum, ReferenceTo<SymbolToken>)>,
    /// The color of this entity's civilization settlements in the world gen and embark screens,
    /// also used when announcing arrival of their caravan. Defaults to `7:0:1`.
    #[serde(alias = "FRIENDLY_COLOR")]
    pub friendly_color: Option<(u8, u8, u8)>,
    // endregion ===================================================================
    // region: Religion ============================================================
    /// Determines what the creatures in the entity will worship (gods vs regional forces).
    #[serde(alias = "RELIGION")]
    pub religion: Option<ReligionTypeEnum>,
    /// Can be any available sphere - multiple entries are possible.
    ///
    /// Choosing a religious sphere will automatically make its opposing spheres not possible for
    /// the species to have; adding `WATER`, for example, means civs of this entity will never get
    /// `FIRE` as a religious sphere.
    ///
    /// Note that the `DEATH` sphere favours the appearance of necromancers (and therefore, towers)
    /// "in" the entity.
    #[serde(alias = "RELIGION_SPHERE")]
    pub religion_sphere: Vec<SphereEnum>,
    /// This token forces an entity to favor or disfavor particular religious spheres,
    /// causing them to acquire those spheres more often when generating a pantheon.
    #[serde(alias = "SPHERE_ALIGNMENT")]
    pub sphere_alignment: Vec<(SphereEnum, u32)>,
    // endregion ===================================================================
    // region: Leadership ==========================================================
    /// A list of positions this entity has.
    #[serde(alias = "POSITION")]
    pub position: Vec<EntityPosition>,
    /// Defines when a particular land-holding noble (baron, count, duke in vanilla) will arrive at
    /// a fortress. As of version `0.44.11`, however, this is obsolete due to the changes in how sites
    /// are elevated in status.
    #[serde(alias = "LAND_HOLDER_TRIGGER")]
    pub land_holder_trigger: Vec<(u32, u32, u32, u32)>,
    /// Allows a site responsibility to be taken up by a dynamically generated position (lords,
    /// hearthpersons, etc).
    ///
    /// Any defined positions holding a given responsibility will take precedence over generated
    /// positions for that responsibility. Also appears to cause site disputes.
    #[serde(alias = "SITE_VARIABLE_POSITIONS")]
    pub site_variable_positions: Option<Choose<ResponsibilityEnum, AllEnum>>,
    /// Allows a responsibility to be taken up by a dynamically generated position (such as Law-
    /// maker). Any defined positions holding a given responsibility will take precedence over
    /// generated positions for that responsibility.
    #[serde(alias = "VARIABLE_POSITIONS")]
    pub variable_positions: Option<Choose<ResponsibilityEnum, AllEnum>>,
    // endregion ===================================================================
    // region: Behavior ============================================================
    /// Sets the civ's view of ethics (certain behaviors), from capital punishment to completely
    /// acceptable.
    ///
    /// This also causes the civ to look upon opposing ethics with disfavor if their reaction to it
    /// is opposing, and when at extremes (one `ACCEPTABLE`, another civ `UNTHINKABLE`; for example)
    /// they will often go to war over it.
    #[serde(alias = "ETHIC")]
    pub ethic: Vec<(EthicTypeEnum, EthicReactionEnum)>,
    /// Sets the civ's cultural values. Numbers range from -50 (complete anathema) to 0 (neutral) to
    /// 50 (highly valued). Certain values must be set to 15 or more for civs to create structures
    /// and form entities during history gen:
    ///
    /// - 15+ `KNOWLEDGE` for libraries
    /// - 15+ `COOPERATION` and 15+ `CRAFTSMANSHIP` for craft guilds
    /// - Guilds also need guild-valid professions (see `PERMITTED_JOB`)
    #[serde(alias = "VALUE")]
    pub value: Vec<(CulturalValueEnum, Clamp<i8, -50, 50>)>,
    /// Makes values randomized rather than specified. This tag overrides the `VALUE` tag.
    /// Using `[VARIABLE_VALUE:ALL:x:y]` and then overwriting single values with further
    /// `[VARIABLE_VALUE:value:x:y]` tags works.
    #[serde(alias = "VARIABLE_VALUE")]
    pub variable_value: Vec<(
        Choose<CulturalValueEnum, AllEnum>,
        Clamp<i8, -50, 50>,
        Clamp<i8, -50, 50>,
    )>,
    /// Makes the civ's traders accept offered goods.
    #[serde(alias = "WILL_ACCEPT_TRIBUTE")]
    pub will_accept_tribute: Option<()>,
    /// The civ will send out this sorts of adventurer in worldgen, which seems to increase Tracker
    /// skill.
    ///
    /// This type of adventurer will sometimes be seen leading a battle (instead of war leaders or
    /// generals) in remote locations during world-gen, in charge of the defenders.
    #[serde(alias = "WANDERER")]
    pub wanderer: Option<()>,
    /// The civ will send out this sorts of adventurer in worldgen, which seems to increase Tracker
    /// skill.
    ///
    /// This type of adventurer will sometimes be seen leading a battle (instead of war leaders or
    /// generals) in remote locations during world-gen, in charge of the defenders.
    #[serde(alias = "BEAST_HUNTER")]
    pub beast_hunter: Option<()>,
    /// The civ will send out this sorts of adventurer in worldgen, which seems to increase Tracker
    /// skill.
    ///
    /// This type of adventurer will sometimes be seen leading a battle (instead of war leaders or
    /// generals) in remote locations during world-gen, in charge of the defenders.
    #[serde(alias = "SCOUT")]
    pub scout: Option<()>,
    /// The civ will send out this sort of adventurer in worldgen, which seems to increase Tracker
    /// skill.
    ///
    /// This type of adventurer will sometimes be seen leading a battle (instead of war leaders or
    /// generals) in remote locations during world-gen, in charge of the defenders.
    ///
    /// If the civ sends out mercenaries, they may come to the player's fort to enlist in the
    /// military.
    #[serde(alias = "MERCENARY")]
    pub mercenary: Option<()>,
    /// The civilization will mutilate bodies when they are the victors in history-gen warfare,
    /// such as hanging bodies from trees, putting them on spikes, and so forth.
    ///
    /// Adventurers killed in Adventurer mode will sometimes be impaled on spikes wherever they
    /// died, with or without this token, and regardless of whether they actually antagonized the
    /// townspeople.
    #[serde(alias = "ABUSE_BODIES")]
    pub abuse_bodies: Option<()>,
    /// The season when the civ is most active: when they will trade, interact with you via
    /// diplomats, and/or invade you. Civs can have multiple season entries.
    ///
    /// Note: If multiple caravans arrive at the same time, you are able to select which civ to
    /// trade with at the depot menu.
    ///
    /// `ACTIVE_SEASON` tags may be changed for a currently active fort.
    #[serde(alias = "ACTIVE_SEASON")]
    pub active_season: Vec<SeasonEnum>,
    /// When invading, sneaks around and shoots at straggling members of your society.
    /// They will spawn on the edge of the map and will only be visible when one of their party are
    /// spotted; this can be quite dangerous to undefended trade depots.
    ///
    /// If the civilization also has the `SIEGER` token, they will eventually ramp it up to less
    /// subtle means of warfare.
    #[serde(alias = "AMBUSHER")]
    pub ambusher: Option<()>,
    /// Will not attack wildlife, and will not be attacked by them, even if you have them in your
    /// party. This can be somewhat disconcerting when attacked by bears in the forest, and your
    /// elven ally sits back and does nothing.
    #[serde(alias = "AT_PEACE_WITH_WILDLIFE")]
    pub at_peace_with_wildlife: Option<()>,
    /// Sends thieves to steal babies. Without this tag (or `AMBUSHER`, or `ITEM_THIEF`), enemy civs
    /// will only siege (if capable), and will siege as early as they would otherwise babysnatch.
    /// This can happen as early as the first year of the fort!
    ///
    /// In addition, babysnatcher civs will snatch children during worldgen, allowing them to become
    /// part of the civ if they do not escape.
    ///
    /// Note: If the playable civ in fortress mode has this tag (e.g. you add `BABYSNATCHER` to
    /// the dwarf entity) then the roles will be reversed and elves and humans will siege and
    /// ambush and goblins will be friendly to you. However, animals traded away to one's own
    /// caravan will count as snatched, reported upon the animal leaving the map, and the animal
    /// will not count as having been exported.
    #[serde(alias = "BABYSNATCHER")]
    pub babysnatcher: Option<()>,
    /// Makes the civilization build castles from mead halls, and (unlike older versions) only
    /// functions when the type of site built is a hamlet/town.
    ///
    /// This, combined with the correct type of position associated with a site, is why
    /// adventurers can only lay claim to human sites.
    /// [Bug:8001](https://www.bay12games.com/dwarves/mantisbt/view.php?id=8001).
    #[serde(alias = "BUILDS_OUTDOOR_FORTIFICATIONS")]
    pub builds_outdoor_fortifications: Option<()>,
    /// Makes the civilization build tombs.
    #[serde(alias = "BUILDS_OUTDOOR_TOMBS")]
    pub builds_outdoor_tombs: Option<()>,
    /// Sets a percentage of the entity population to be used as bandits.
    #[serde(alias = "BANDITRY")]
    pub banditry: Option<Clamp<u8, 0, 100>>,
    /// Visiting diplomats are accompanied by a pair of soldiers.
    #[serde(alias = "DIPLOMAT_BODYGUARDS")]
    pub diplomat_bodyguards: Option<()>,
    /// Found on generated divine "HF Guardian Entities". Cannot be used in user-defined raws.
    #[serde(alias = "GENERATED")]
    pub generated: Option<()>, // TODO: mark this as unusable, or maybe remove it?
    /// Causes invaders to ignore visiting caravans and other neutral creatures.
    #[serde(alias = "INVADERS_IGNORE_NEUTRALS")]
    pub invaders_ignore_neutrals: Option<()>,
    /// Sends thieves to steal items. This will also occur in history generation, and thieves will
    /// have the "thief" profession. Items stolen in history gen will be scattered around that
    /// creature's home. Also causes that civ to be hostile to any entity without this token.
    ///
    /// Without this tag (or `AMBUSHER`, or BABYSNATCHER), enemy civs will only siege (if capable),
    /// and will siege as early as they would otherwise steal.
    ///
    /// Note: If the playable civ in Fortress Mode has this tag (e.g. you add `ITEM_THIEF` to the
    /// Dwarf entity) then the roles will be reversed and elves and humans will siege and ambush
    /// and kobolds will be friendly to you. However, ALL items traded away to one's own caravan
    /// will count as stolen, reported when the items leave the map, and the stolen items will
    /// not count as exported.
    #[serde(alias = "ITEM_THIEF")]
    pub item_thief: Option<()>,
    /// Causes the entity to send out patrols that can ambush adventurers. Said patrols will be
    /// hostile to any adventurers they encounter, regardless of race or nationality.
    #[serde(alias = "LOCAL_BANDITRY")]
    pub local_banditry: Option<()>,
    /// Caravan merchants are accompanied by soldiers.
    #[serde(alias = "MERCHANT_BODYGUARDS")]
    pub merchant_bodyguards: Option<()>,
    /// Merchants will engage in cross-civ trading and form companies.
    #[serde(alias = "MERCHANT_NOBILITY")]
    pub merchant_nobility: Option<()>,
    /// 0 to 5, civ will come to site once population at site has reached that level. If multiple
    /// progress triggers exist for a civ, it will come when any one of them is fulfilled instead
    /// of waiting for all of them to be reached. Each value corresponds to some level of
    /// population:
    /// - 0: disables the trigger.
    /// - 1: population of 20.
    /// - 2: population of 50.
    /// - 3: population of 80.
    /// - 4: population of 110.
    /// - 5: population of 140.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    ///
    /// Note: hostile civs require that this be fulfilled as well as at least one other non-siege
    /// trigger before visiting for non-siege activities.
    #[serde(alias = "PROGRESS_TRIGGER_POPULATION")]
    pub progress_trigger_population: Option<Clamp<u8, 0, 5>>,
    /// 0 to 5, civ will come to site once created wealth has reached that level. If multiple
    /// progress triggers exist for a civ, it will come when any one of them is fulfilled instead
    /// of waiting for all of them to be reached. Each value corresponds to some amount of
    /// created wealth:
    /// - 0: disables the trigger.
    /// - 1: 5,000☼.
    /// - 2: 25,000☼.
    /// - 3: 100,000☼.
    /// - 4: 200,000☼.
    /// - 5: 300,000☼.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    #[serde(alias = "PROGRESS_TRIGGER_PRODUCTION")]
    pub progress_trigger_production: Option<Clamp<u8, 0, 5>>,
    /// 0 to 5, civ will come to site once exported goods has reached that level. If multiple
    /// progress triggers exist for a civ, it will come when any one of them is fulfilled instead
    /// of waiting for all of them to be reached. Each value corresponds to some amount of
    /// exported goods:
    /// - 0: disables the trigger.
    /// - 1: 500☼.
    /// - 2: 2,500☼.
    /// - 3: 10,000☼.
    /// - 4: 20,000☼.
    /// - 5: 30,000☼.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    #[serde(alias = "PROGRESS_TRIGGER_TRADE")]
    pub progress_trigger_trade: Option<Clamp<u8, 0, 5>>,
    /// 0 to 5, civ will begin to send sieges against the player civ when this level is reached if
    /// it is hostile. If multiple siege progress triggers exist for a civ, it will come when any
    /// one of them is fulfilled instead of waiting for all of them to be reached.
    /// - 0: disables the trigger.
    /// - 1: population of 20.
    /// - 2: population of 50.
    /// - 3: population of 80.
    /// - 4: population of 110.
    /// - 5: population of 140.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    #[serde(alias = "PROGRESS_TRIGGER_POP_SIEGE")]
    pub progress_trigger_pop_siege: Option<Clamp<u8, 0, 5>>,
    /// 0 to 5, civ will begin to send sieges against the player civ when this level is reached if
    /// it is hostile. If multiple siege progress triggers exist for a civ, it will come when any
    /// one of them is fulfilled instead of waiting for all of them to be reached.
    /// - 0: disables the trigger.
    /// - 1: 5,000☼.
    /// - 2: 25,000☼.
    /// - 3: 100,000☼.
    /// - 4: 200,000☼.
    /// - 5: 300,000☼.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    #[serde(alias = "PROGRESS_TRIGGER_PROD_SIEGE")]
    pub progress_trigger_prod_siege: Option<Clamp<u8, 0, 5>>,
    /// 0 to 5, civ will begin to send sieges against the player civ when this level is reached if
    /// it is hostile. If multiple siege progress triggers exist for a civ, it will come when any
    /// one of them is fulfilled instead of waiting for all of them to be reached.
    /// - 0: disables the trigger.
    /// - 1: 500☼.
    /// - 2: 2,500☼.
    /// - 3: 10,000☼.
    /// - 4: 20,000☼.
    /// - 5: 30,000☼.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    #[serde(alias = "PROGRESS_TRIGGER_TRADE_SIEGE")]
    pub progress_trigger_trade_siege: Option<Clamp<u8, 0, 5>>,
    /// Will start campfires and wait around at the edge of your map for a month or two before
    /// rushing in to attack. This will occur when the progress triggers for sieging are reached.
    ///
    /// If the civ lacks smaller methods of conflict (`AMBUSHER`, `BABYSNATCHER`, `ITEM_THIEF`),
    /// they will instead send smaller-scale sieges when their triggers for "first contact" are
    /// reached.
    #[serde(alias = "SIEGER")]
    pub sieger: Option<()>,
    /// Guards certain special sites, such as a vault belonging to a demon allied with a deity. Used
    /// in generated divine entities.
    #[serde(alias = "SITE_GUARDIAN")]
    pub site_guardian: Option<()>,
    /// This makes the severity of attacks depend on the extent of item/baby thievery rather than
    /// the passage of time. Designed to go with `ITEM_THIEF`, may or may not work with `BABYSNATCHER`.
    ///
    /// Prevents the civ from engaging in diplomacy or ending up at war.
    #[serde(alias = "SKULKING")]
    pub skulking: Option<()>,
    /// Visiting diplomats impose tree cutting quotas; without this, they will simply compliment
    /// your fortress and leave.
    ///
    /// Also causes the diplomat to make unannounced first contact at the very beginning of the
    /// first Spring after your fortress becomes a land holder.
    #[serde(alias = "TREE_CAP_DIPLOMACY")]
    pub tree_cap_diplomacy: Option<()>,
    /// Defines if a civilization is a hidden subterranean entity, such as bat man civilizations.
    /// May spawn in any of the three caverns, in groups of 5-10 soldiers who will hunt down nearby
    /// cavern creatures. If you embark as this civ, you have access to pets and trees from all
    /// three layers, not only the first.
    #[serde(alias = "LAYER_LINKED")]
    pub layer_linked: Option<()>,
    /// Unknown. Possibly makes this creature available as a "Corpse" (e.g. Human Corpse) for
    /// necromancers in adventure mode.
    #[serde(alias = "UNDEAD_CANDIDATE")]
    pub undead_candidate: Option<()>,
    /// Makes civilizations generate keyboard instruments.
    #[serde(alias = "GENERATE_KEYBOARD_INSTRUMENTS")]
    pub generate_keyboard_instruments: Option<()>,
    /// Makes civilizations generate percussion instruments.
    #[serde(alias = "GENERATE_PERCUSSION_INSTRUMENTS")]
    pub generate_percussion_instruments: Option<()>,
    /// Makes civilizations generate stringed instruments.
    #[serde(alias = "GENERATE_STRINGED_INSTRUMENTS")]
    pub generate_stringed_instruments: Option<()>,
    /// Makes civilizations generate wind instruments.
    #[serde(alias = "GENERATE_WIND_INSTRUMENTS")]
    pub generate_wind_instruments: Option<()>,
    /// Makes civilizations generate dance forms.
    #[serde(alias = "GENERATE_DANCE_FORMS")]
    pub generate_dance_forms: Option<()>,
    /// Makes civilizations generate musical forms.
    #[serde(alias = "GENERATE_MUSICAL_FORMS")]
    pub generate_musical_forms: Option<()>,
    /// Makes civilizations generate poetic forms.
    #[serde(alias = "GENERATE_POETIC_FORMS")]
    pub generate_poetic_forms: Option<()>,
    /// Define a type of scholar this entity will generate.
    #[serde(alias = "SCHOLAR")]
    pub scholar: Vec<ScholarTypeEnum>,
    /// Generates scholars based on the values generated with the `VARIABLE_VALUE` tag.
    #[serde(alias = "SET_SCHOLARS_ON_VALUES_AND_JOBS")]
    pub set_scholars_on_values_and_jobs: Option<()>,
    /// Used for kobolds.
    #[serde(alias = "NO_ARTIFACT_CLAIMS")]
    pub no_artifact_claims: Option<()>,
    /// The civilization can breach the Underworld during world generation.
    #[serde(alias = "MINING_UNDERWORLD_DISASTERS")]
    pub mining_underworld_disasters: Option<()>,
    // endregion ===================================================================
    // region: Available Items =====================================================
    /// Specify a weapon usable by the entity.
    #[serde(alias = "WEAPON")]
    pub weapon: Vec<Weapon>,
    /// Specify an item this entity type can use as armor. You can optionally specify a rarity for
    /// this item. The default rarity is equalvalent to `COMMON` (50% chance).
    #[serde(alias = "ARMOR")]
    pub armor: Vec<(ReferenceTo<ArmorToken>, Option<RarityEnum>)>,
    /// Causes the selected weapon to fall under the "digging tools" section of the embark screen.
    /// Also forces the weapon to be made out of metal, which can cause issues if a modded entity
    /// has access to picks without access to metal - for those cases, listing the pick under the
    /// `[WEAPON]` token works just as well.
    ///
    /// Note that this tag is neither necessary nor sufficient to allow use of that item as a mining
    /// tool -- for that, the item itself needs to be a weapon with `[SKILL:MINING]`.
    #[serde(alias = "DIGGER")]
    pub digger: Vec<ReferenceTo<WeaponToken>>,
    /// Specify an item this entity type can use as gloves. You can optionally specify a rarity for
    /// this item. The default rarity is equalvalent to `COMMON` (50% chance).
    #[serde(alias = "GLOVES")]
    pub gloves: Vec<(ReferenceTo<GlovesToken>, Option<RarityEnum>)>,
    /// Specify an item this entity type can use as headwear. You can optionally specify a rarity for
    /// this item. The default rarity is equalvalent to `COMMON` (50% chance).
    #[serde(alias = "HELM")]
    pub helm: Vec<(ReferenceTo<HelmToken>, Option<RarityEnum>)>,
    /// No longer used in vanilla as of Version `0.42.01` due to the ability to generate instruments
    /// in world generation. It is still usable if pre-defined instruments are modded in, and
    /// generated musical forms are capable of selecting pre-defined instruments to use.
    ///
    /// However, reactions for making instruments, instrument parts, and/or assembling such
    /// instruments need to be added as well, as this token no longer adds such instruments to the
    /// craftdwarf's workshop menu.
    #[serde(alias = "INSTRUMENT")]
    pub instrument: Vec<ReferenceTo<InstrumentToken>>,
    /// Specify an item this entity type can use as pants. You can optionally specify a rarity for
    /// this item. The default rarity is equalvalent to `COMMON` (50% chance).
    #[serde(alias = "PANTS")]
    pub pants: Vec<(ReferenceTo<PantsToken>, Option<RarityEnum>)>,
    ///
    #[serde(alias = "SHIELD")]
    pub shield: Vec<ReferenceTo<ShieldToken>>,
    /// Specify an item this entity type can use as shoes. You can optionally specify a rarity for
    /// this item. The default rarity is equalvalent to `COMMON` (50% chance).
    #[serde(alias = "SHOES")]
    pub shoes: Vec<(ReferenceTo<ShoesToken>, Option<RarityEnum>)>,
    ///
    #[serde(alias = "SIEGEAMMO")]
    pub siegeammo: Vec<ReferenceTo<SiegeAmmoToken>>,
    ///
    #[serde(alias = "TOOL")]
    pub tool: Vec<ReferenceTo<ToolToken>>,
    ///
    #[serde(alias = "TOY")]
    pub toy: Vec<ReferenceTo<ToyToken>>,
    ///
    #[serde(alias = "TRAPCOMP")]
    pub trapcomp: Vec<ReferenceTo<TrapCompToken>>,
    // endregion ===================================================================
    // region: Available Resources =================================================
    /// Allows civilization to use products made from animals. All relevant creatures will be able
    /// to provide wool, silk, and extracts (including milk and venom) for trade, and non-sentient
    /// creatures (unless ethics state otherwise) will be able to provide eggs, caught fish, meat,
    /// leather, bone, shell, pearl, horn, and ivory.
    #[serde(alias = "USE_ANIMAL_PRODUCTS")]
    pub use_animal_products: Option<()>,
    /// Any creature in the civilization's list of usables (from the surrounding 7x7 or so of
    /// squares and map features in those squares) which has `PET` or `PET_EXOTIC` will be
    /// available as a pet, pack animal (with `PACK_ANIMAL`), wagon puller (with `WAGON_PULLER`),
    /// mount (with `MOUNT` or `MOUNT_EXOTIC`), or siege minion (with `TRAINABLE_WAR` and without
    /// `CAN_LEARN`).
    ///
    /// This notion of the initial usable creature list, which then gets pared down or otherwise
    /// considered, applies below as well. All common domestic and equipment creatures are also
    /// added to the initial list.
    #[serde(alias = "USE_ANY_PET_RACE")]
    pub use_any_pet_race: Option<()>,
    /// If they don't have it, creatures with exclusively subterranean biomes are skipped.
    /// If they have it, cave creatures with `PET` will also be available as pets, pack animals
    /// (with `PACK_ANIMAL`), wagon pullers (with `WAGON_PULLER`), mounts (with `MOUNT` or
    /// `MOUNT_EXOTIC`), and siege minions (with `TRAINABLE_WAR` and without `CAN_LEARN`).
    #[serde(alias = "USE_CAVE_ANIMALS")]
    pub use_cave_animals: Option<()>,
    /// Don't have it -> `EVIL` creatures skipped. If they have it, evil creatures with
    /// `SLOW_LEARNER` or without `CAN_LEARN` will be also available as pets (with `PET`), pack
    /// animals (with `PACK_ANIMAL`), wagon pullers (with `WAGON_PULLER`), mounts (with `MOUNT` or
    /// `MOUNT_EXOTIC`), and siege minions (with `TRAINABLE_WAR` or `SLOW_LEARNER`), even the normally
    /// untameable species.
    #[serde(alias = "USE_EVIL_ANIMALS")]
    pub use_evil_animals: Option<()>,
    /// As `EVIL` creatures for all uses of plants.
    #[serde(alias = "USE_EVIL_PLANTS")]
    pub use_evil_plants: Option<()>,
    /// As `EVIL` creatures for all uses of wood.
    #[serde(alias = "USE_EVIL_WOOD")]
    pub use_evil_wood: Option<()>,
    /// Don't have it -> `GOOD` creatures skipped. If they have it, good creatures without
    /// `CAN_LEARN` will also be available as pets (with `PET`), pack animals (with `PACK_ANIMAL`),
    /// wagon pullers (with `WAGON_PULLER`), mounts (with `MOUNT` or `MOUNT_EXOTIC`), and siege minions
    /// (with `TRAINABLE_WAR`), even the normally untameable species.
    #[serde(alias = "USE_GOOD_ANIMALS")]
    pub use_good_animals: Option<()>,
    /// As `GOOD` creatures for all uses of plants.
    #[serde(alias = "USE_GOOD_PLANTS")]
    pub use_good_plants: Option<()>,
    /// As `GOOD` creatures for all uses of wood.
    #[serde(alias = "USE_GOOD_WOOD")]
    pub use_good_wood: Option<()>,
    /// If the relevant professions are permitted, controls availability of lye (`LYE_MAKING`),
    /// charcoal (`BURN_WOOD`), and potash (`POTASH_MAKING`).
    #[serde(alias = "USE_MISC_PROCESSED_WOOD_PRODUCTS")]
    pub use_misc_processed_wood_products: Option<()>,
    /// Makes the civilization use all locally available non-exotic pets.
    #[serde(alias = "USE_NON_EXOTIC_PET_RACE")]
    pub use_non_exotic_pet_race: Option<()>,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `MOUNT`.
    /// Additionally, all available (based on `USE_ANY_PET_RACE`, `USE_CAVE_ANIMALS`,
    /// `USE_GOOD_ANIMALS`, and `USE_EVIL_ANIMALS`) creatures with `MOUNT` and `PET` will be allowed
    /// for use as mounts during combat.
    #[serde(alias = "COMMON_DOMESTIC_MOUNT")]
    pub common_domestic_mount: Option<()>,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `PACK_ANIMAL`.
    /// Additionally, all available (based on `USE_ANY_PET_RACE`, `USE_CAVE_ANIMALS`,
    /// `USE_GOOD_ANIMALS`, and `USE_EVIL_ANIMALS`) creatures with `PACK_ANIMAL` and `PET` will
    /// be allowed for use during trade as pack animals.
    #[serde(alias = "COMMON_DOMESTIC_PACK")]
    pub common_domestic_pack: Option<()>,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `PET`.
    /// Additionally, all available (based on `USE_ANY_PET_RACE`, `USE_CAVE_ANIMALS`,
    /// `USE_GOOD_ANIMALS`, and `USE_EVIL_ANIMALS`) creatures with `PET` will be allowed
    /// for use as pets.
    #[serde(alias = "COMMON_DOMESTIC_PET")]
    pub common_domestic_pet: Option<()>,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `WAGON_PULLER`.
    /// Additionally, all available (based on `USE_ANY_PET_RACE`, `USE_CAVE_ANIMALS`,
    /// `USE_GOOD_ANIMALS`, and `USE_EVIL_ANIMALS`) creatures with `WAGON_PULLER` and `PET`
    /// will be allowed for use during trade to pull wagons.
    #[serde(alias = "COMMON_DOMESTIC_PULL")]
    pub common_domestic_pull: Option<()>,
    /// Allow civ to use river products in the goods it has available for trade.
    #[serde(alias = "RIVER_PRODUCTS")]
    pub river_products: Option<()>,
    /// Allow civ to use ocean products (including amber and coral) in the goods it has available
    /// for trade.
    ///
    /// Without `OCEAN_PRODUCTS`, civilizations will not be able to trade ocean fish
    /// even if they are also available from other sources (e.g. sturgeons and stingrays).
    #[serde(alias = "OCEAN_PRODUCTS")]
    pub ocean_products: Option<()>,
    /// Allow civ to use underground plant products in the goods it has available for trade.
    /// Lack of suitable vegetation in the caverns will cause worldgen rejections.
    #[serde(alias = "INDOOR_FARMING")]
    pub indoor_farming: Option<()>,
    /// Allow civ to use outdoor plant products in the goods it has available for trade.
    /// Lack of suitable vegetation in the civ's starting area will cause worldgen rejections.
    #[serde(alias = "OUTDOOR_FARMING")]
    pub outdoor_farming: Option<()>,
    /// Allow civ to use underground plant growths (quarry bush leaves, in unmodded games)
    /// in the goods it has available for trade.
    #[serde(alias = "INDOOR_GARDENS")]
    pub indoor_gardens: Option<()>,
    /// Allow civ to use outdoor plant growths in the goods it has available for trade.
    #[serde(alias = "OUTDOOR_GARDENS")]
    pub outdoor_gardens: Option<()>,
    /// Allows civ to use indoor tree growths in the goods it has available for trade.
    /// Not used in vanilla entities, as vanilla underground trees do not grow fruit.
    #[serde(alias = "INDOOR_ORCHARDS")]
    pub indoor_orchards: Option<()>,
    /// Allows civ to use outdoor tree growths in the goods it has available for trade.
    #[serde(alias = "OUTDOOR_ORCHARDS")]
    pub outdoor_orchards: Option<()>,
    /// Civ members will attempt to wear clothing.
    #[serde(alias = "CLOTHING")]
    pub clothing: Option<()>,
    /// Will wear things made of spider silk and other subterranean materials.
    #[serde(alias = "SUBTERRANEAN_CLOTHING")]
    pub subterranean_clothing: Option<()>,
    /// Adds decorations to equipment based on the level of the generated unit. Also improves item
    /// quality.
    #[serde(alias = "EQUIPMENT_IMPROVEMENTS")]
    pub equipment_improvements: Option<()>,
    /// Adds decorations to weapons generated for bowman and master bowman. An elf hack.
    #[serde(alias = "IMPROVED_BOWS")]
    pub improved_bows: Option<()>,
    /// Allows metal materials to be used to make cages (inexpensive metals only) and crafts.
    #[serde(alias = "METAL_PREF")]
    pub metal_pref: Option<()>,
    /// Allows the civilization to make use of nearby stone types. If the `FURNACE_OPERATOR` job is
    /// permitted, also allows ore-bearing stones to be smelted into metals.
    #[serde(alias = "STONE_PREF")]
    pub stone_pref: Option<()>,
    /// The civilization can make traditionally metallic weapons such as swords and spears from
    /// wood. An elf hack.
    #[serde(alias = "WOOD_WEAPONS")]
    pub wood_weapons: Option<()>,
    /// The civilization can make traditionally metallic armor such as mail shirts and helmets from
    /// wood. An elf hack.
    #[serde(alias = "WOOD_ARMOR")]
    pub wood_armor: Option<()>,
    /// Enables creatures of this entity to bring gems in trade.
    #[serde(alias = "GEM_PREF")]
    pub gem_pref: Option<()>,
    /// Allow civ to use subterranean wood types, such as tower-cap and fungiwood logs.
    #[serde(alias = "INDOOR_WOOD")]
    pub indoor_wood: Option<()>,
    /// Allow civ to use outdoor wood types, such as mangrove and oak.
    #[serde(alias = "OUTDOOR_WOOD")]
    pub outdoor_wood: Option<()>,
    /// Precious gems cut by this civilization's jewelers can be of this shape.
    #[serde(alias = "GEM_SHAPE")]
    pub gem_shape: Vec<ReferenceTo<ShapeToken>>,
    /// Ordinary non-gem stones cut by this civilization's jewelers can be of this shape.
    #[serde(alias = "STONE_SHAPE")]
    pub stone_shape: Vec<ReferenceTo<ShapeToken>>,
    /// Allows civ to use materials with `[DIVINE]` for clothing. Used for generated divine
    /// entities.
    #[serde(alias = "DIVINE_MAT_CLOTHING")]
    pub divine_mat_clothing: Option<()>,
    /// Allows civ to use materials with `[DIVINE]` for crafts (unverified).  Used for generated divine
    /// entities.
    #[serde(alias = "DIVINE_MAT_CRAFTS")]
    pub divine_mat_crafts: Option<()>,
    /// Allows civ to use metals with `[DIVINE]` for weapons. Used for generated divine entities.
    #[serde(alias = "DIVINE_MAT_WEAPONS")]
    pub divine_mat_weapons: Option<()>,
    /// Allows civ to use metals with `[DIVINE]` for armour. Used for generated divine entities.
    #[serde(alias = "DIVINE_MAT_ARMOR")]
    pub divine_mat_armor: Option<()>,
    // endregion ===================================================================
}

/// Start an animal definition.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Animal {
    /// argument 1 of `ANIMAL`
    #[serde(alias = "ANIMAL")]
    pub reference: Option<()>,
    /// Select specific creature.
    #[serde(alias = "ANIMAL_TOKEN")]
    pub animal_token: Option<ReferenceTo<CreatureToken>>,
    /// Select specific creature caste (requires `ANIMAL_TOKEN`). Sites with animal populations will
    /// still include all castes, but only the selected ones will be used for specific roles.
    #[serde(alias = "ANIMAL_CASTE_TOKEN")]
    pub animal_caste_token: Vec<Reference>, // TODO: ref here is for creature caste
    /// Select creature castes with this creature class (multiple uses allowed).
    #[serde(alias = "ANIMAL_CLASS")]
    pub animal_class: Vec<Reference>, // TODO: ref here is for creature class
    /// Forbid creature castes with this creature class (multiple uses allowed).
    #[serde(alias = "ANIMAL_FORBIDDEN_CLASS")]
    pub animal_forbidden_class: Vec<Reference>, // TODO: ref here is for creature class
    /// Animal will be present even if it does not naturally occur in the entity's terrain. All
    /// creatures, including demons, night trolls and other generated ones will be used if no
    /// specific creature or class is selected.
    #[serde(alias = "ANIMAL_ALWAYS_PRESENT")]
    pub animal_always_present: Option<()>,
    /// Override creature usage tokens `MOUNT` and `MOUNT_EXOTIC`.
    /// Overridden by `ALWAYS` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_NEVER_MOUNT")]
    pub animal_never_mount: Option<()>,
    /// Override creature usage tokens `MOUNT` and `MOUNT_EXOTIC`.
    /// Overrides `NEVER` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_ALWAYS_MOUNT")]
    pub animal_always_mount: Option<()>,
    /// Override creature usage token `WAGON_PULLER`.
    /// Overridden by `ALWAYS` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_NEVER_WAGON_PULLER")]
    pub animal_never_wagon_puller: Option<()>,
    /// Override creature usage token `WAGON_PULLER`.
    /// Overrides `NEVER` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_ALWAYS_WAGON_PULLER")]
    pub animal_always_wagon_puller: Option<()>,
    /// Override creature usage tokens `TRAINABLE_WAR` and not `CAN_LEARN`.
    /// Overridden by `ALWAYS` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_NEVER_SIEGE")]
    pub animal_never_siege: Option<()>,
    /// Override creature usage tokens `TRAINABLE_WAR` and not `CAN_LEARN`.
    /// Overrides `NEVER` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_ALWAYS_SIEGE")]
    pub animal_always_siege: Option<()>,
    /// Override creature usage tokens `PET` and `PET_EXOTIC`.
    /// Overridden by `ALWAYS` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_NEVER_PET")]
    pub animal_never_pet: Option<()>,
    /// Override creature usage tokens `PET` and `PET_EXOTIC`.
    /// Overrides `NEVER` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_ALWAYS_PET")]
    pub animal_always_pet: Option<()>,
    /// Override creature usage token `PACK_ANIMAL`.
    /// Overridden by `ALWAYS` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_NEVER_PACK_ANIMAL")]
    pub animal_never_pack_animal: Option<()>,
    /// Override creature usage token `PACK_ANIMAL`.
    /// Overrides `NEVER` counterpart if a caste is matched by more than one animal definition.
    #[serde(alias = "ANIMAL_ALWAYS_PACK_ANIMAL")]
    pub animal_always_pack_animal: Option<()>,
}

///
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Weapon {
    /// argument 1 of `WEAPON`
    #[serde(alias = "WEAPON")]
    pub reference: Option<(ReferenceTo<WeaponToken>, Option<RarityEnum>)>,
    ///
    #[serde(alias = "AMMO")]
    pub ammo: Option<(ReferenceTo<AmmoToken>, Option<RarityEnum>)>, // TODO: semantic check, should only be nestable under an appropriate weapon
}

/// Select a tissue layer which has the `ID` attached using `TISSUE_STYLE_UNIT` token in unit raws.
/// This allows setting further cultural style parameters for the selected tissue layer.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TissueStyle {
    /// argument 1 of `TISSUE_STYLE`
    #[serde(alias = "TISSUE_STYLE")]
    pub reference: Option<ReferenceTo<TissueToken>>,
    /// Presumably sets culturally preferred tissue length for selected tissue. Needs testing.
    ///
    /// Dwarves have their beards set to `100:NONE` by default.
    #[serde(alias = "TS_MAINTAIN_LENGTH")]
    pub ts_maintain_length: Option<(Choose<u32, NoneEnum>, Choose<u32, NoneEnum>)>,
    /// Presumably sets culturally preferred tissue shapings for selected tissue. Needs testing.
    #[serde(alias = "TS_PREFERRED_SHAPING")]
    pub ts_preferred_shaping: Vec<StylingEnum>, // `TODO` this possibly depends on the given tissue above
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum StylingEnum {
    #[serde(alias = "NEATLY_COMBED")]
    NeatlyCombed,
    #[serde(alias = "BRAIDED")]
    Braided,
    #[serde(alias = "DOUBLE_BRAIDS")]
    DoubleBraids,
    #[serde(alias = "PONY_TAILS")]
    PonyTails,
    #[serde(alias = "CLEAN_SHAVEN")]
    CleanShaven,
    #[serde(alias = "STANDARD_HAIR_SHAPINGS")]
    StandardHairShapings,
    #[serde(alias = "STANDARD_BEARD_SHAPINGS")]
    StandardBeardShapings,
    #[serde(alias = "STANDARD_MOUSTACHE_SHAPINGS")]
    StandardMoustacheShapings,
    #[serde(alias = "STANDARD_SIDEBURNS_SHAPINGS")]
    StandardSideburnsShapings,
}
impl Default for StylingEnum {
    fn default() -> Self {
        Self::NeatlyCombed
    }
}
