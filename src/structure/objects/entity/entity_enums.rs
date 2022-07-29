use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SiteTypeEnum {
    // TODO: update site type doc comment for all tokens using this enum
    #[serde(alias = "CITY")]
    City,
    #[serde(alias = "TREE_CITY")]
    TreeCity,
    #[serde(alias = "DARK_FORTRESS")]
    Rare,
    #[serde(alias = "CAVE")]
    Cave,
    #[serde(alias = "CAVE_DETAILED")]
    CaveDetailed,
    #[serde(alias = "PLAYER_FORTRESS")]
    PlayerFortress,
    #[serde(alias = "MONUMENT")]
    Monument,
}
impl Default for SiteTypeEnum {
    fn default() -> Self {
        Self::City
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ConstructionEnum {
    #[serde(alias = "ROAD")]
    Road,
    #[serde(alias = "TUNNEL")]
    Tunnel,
    #[serde(alias = "BRIDGE")]
    Bridge,
    #[serde(alias = "WALL")]
    Wall,
}
impl Default for ConstructionEnum {
    fn default() -> Self {
        Self::Road
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ArtFacetEnum {
    #[serde(alias = "OWN_RACE")]
    OwnRace,
    #[serde(alias = "FANCIFUL")]
    Fanciful,
    #[serde(alias = "EVIL")]
    Evil,
    #[serde(alias = "GOOD")]
    Good,
}
impl Default for ArtFacetEnum {
    fn default() -> Self {
        Self::OwnRace
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ArtImageModifierEnum {
    #[serde(alias = "CREATURE")]
    Creature,
    #[serde(alias = "PLANT")]
    Plant,
    #[serde(alias = "TREE")]
    Tree,
    #[serde(alias = "SHAPE")]
    Shape,
    #[serde(alias = "ITEM")]
    Item,
}
impl Default for ArtImageModifierEnum {
    fn default() -> Self {
        Self::Creature
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ItemImprovementModifierEnum {
    #[serde(alias = "ART_IMAGE")]
    ArtImage,
    #[serde(alias = "COVERED")]
    Covered,
    #[serde(alias = "GLAZED")]
    Glazed,
    #[serde(alias = "RINGS_HANGING")]
    RingsHanging,
    #[serde(alias = "BANDS")]
    Bands,
    #[serde(alias = "SPIKES")]
    Spikes,
    #[serde(alias = "ITEMSPECIFIC")]
    Itemspecific,
    #[serde(alias = "THREAD")]
    Thread,
    #[serde(alias = "CLOTH")]
    Cloth,
    #[serde(alias = "SEWN_IMAGE")]
    SewnImage,
}
impl Default for ItemImprovementModifierEnum {
    fn default() -> Self {
        Self::ArtImage
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SymbolNounEnum {
    #[serde(alias = "ALL")]
    All,
    #[serde(alias = "REMAINING")]
    Remaining,
    #[serde(alias = "BATTLE")]
    Battle,
    #[serde(alias = "BRIDGE")]
    Bridge,
    #[serde(alias = "CIV")]
    Civ,
    #[serde(alias = "CRAFT_GUILD")]
    CraftGuild,
    #[serde(alias = "LIBRARY")]
    Library,
    #[serde(alias = "MERCHANT_COMPANY")]
    MerchantCompany,
    #[serde(alias = "MILITARY_UNIT")]
    MilitaryUnit,
    #[serde(alias = "RELIGION")]
    Religion,
    #[serde(alias = "ROAD")]
    Road,
    #[serde(alias = "SIEGE")]
    Siege,
    #[serde(alias = "SITE")]
    Site,
    #[serde(alias = "TEMPLE")]
    Temple,
    #[serde(alias = "TUNNEL")]
    Tunnel,
    #[serde(alias = "VESSEL")]
    Vessel,
    #[serde(alias = "WALL")]
    Wall,
    #[serde(alias = "WAR")]
    War,
}
impl Default for SymbolNounEnum {
    fn default() -> Self {
        Self::All
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ReligionTypeEnum {
    /// The creatures will worship a group of gods, each aligned with their spheres
    /// and other appropriate ones as well.
    #[serde(alias = "PANTHEON")]
    Pantheon,
    /// The creatures will worship a single force associated with the terrain of
    /// their initial biome.
    #[serde(alias = "REGIONAL_FORCE")]
    RegionalForce,
}
impl Default for ReligionTypeEnum {
    fn default() -> Self {
        Self::Pantheon
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum RarityEnum {
    /// The item has a 1/2 chance of being added to any given civilization of this entity type.
    #[serde(alias = "COMMON")]
    Common,
    /// The item has a 1/10 chance of being added to any given civilization of this entity type.
    /// If the "item adding" algorithm fails to add any items the first time around, it will do
    /// a second pass where `UNCOMMON` items have a 1/2 chance of being added.
    #[serde(alias = "UNCOMMON")]
    Uncommon,
    /// The item has a 1/100 chance of being added to any given civilization of this entity type.
    #[serde(alias = "RARE")]
    Rare,
    /// The item is always used by civilizations of this entity type.
    #[serde(alias = "FORCED")]
    Forced,
}
impl Default for RarityEnum {
    fn default() -> Self {
        Self::Common
    }
}

// `TODO` need to implement this still
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum EthicTypeEnum {
    /// The result of a tantrumming citizen attacking another in fortress mode. Other effects
    /// unknown.
    #[serde(alias = "ASSAULT")]
    Assault,
    /// This determines if the race will sometimes devour defeated enemy combatants.
    #[serde(alias = "EAT_SAPIENT_KILL")]
    EatSapientKill,
    /// This includes whether or not a race is willing to butcher other sapients.
    ///
    /// Note that due to a bug player forts will never butcher intelligent creatures in fortress
    /// mode (they are still willing to eat their meat though, should they have access to it).
    /// However this works as intended in adventure mode, worldgen, and offsite (caravans will
    /// deliver products made from sentients, etc.)
    #[serde(alias = "EAT_SAPIENT_OTHER")]
    EatSapientOther,
    /// A response between `MISGUIDED` and `UNTHINKABLE` causes the entity to refuse
    /// animal products in trade — namely, materials with `[IMPLIES_ANIMAL_KILL]`.
    ///
    /// If the civ sells those animal products (because of a civ token), they will be marked as
    /// "grown"; for example, "grown leather".
    #[serde(alias = "KILL_ANIMAL")]
    KillAnimal,
    /// If `REQUIRED`, all lethal combat with an enemy who is an enemy of the whole entity will put
    /// the creature in no quarter mode.
    #[serde(alias = "KILL_ENEMY")]
    KillEnemy,
    /// If `REQUIRED`, all lethal combat with an enemy in the same entity will put the creature in
    /// no quarter mode. Determines whether and how often entity members will be murdered.
    #[serde(alias = "KILL_ENTITY_MEMBER")]
    KillEntityMember,
    /// If `REQUIRED`, all lethal combat with an enemy who is neutral with the entity will put the
    /// creature in no quarter mode, and the creature will also demand that strangers identify
    /// themselves.
    #[serde(alias = "KILL_NEUTRAL")]
    KillNeutral,
    /// This includes a race's position towards wood — a response between `MISGUIDED` and
    /// `UNTHINKABLE` causes the entity to refuse wooden objects (except for "grown"
    /// wooden objects) in trade, and also prohibits them from bringing caravan wagons.
    ///
    /// Caravans will sell grown wood objects (if the civ has `WOOD_PREF`) and even grown non-wood
    /// objects but that elves refuse to buy (if the civ uses misc processed wood products).
    #[serde(alias = "KILL_PLANT")]
    KillPlant,
    /// Unknown, possibly giving false witness reports.
    #[serde(alias = "LYING")]
    Lying,
    /// This determines whether animal kills in world-gen will lead to characters with trophies.
    #[serde(alias = "MAKE_TROPHY_ANIMAL")]
    MakeTrophyAnimal,
    /// This determines whether kills of one's own race in world-gen will lead to characters with
    /// trophies.
    ///
    /// Historical figures can go to your fortress with jewellery of leather, tooth, hair, bone or
    /// nail from their race, even `INTELLIGENT`; for example: goblin with -goblin tooth ring-.
    #[serde(alias = "MAKE_TROPHY_SAME_RACE")]
    MakeTrophySameRace,
    /// This determines whether kills of other sapients in world-gen will lead to characters with
    /// trophies. Like `MAKE_TROPHY_SAME_RACE`, but about other races, including `INTELLIGENT`.
    #[serde(alias = "MAKE_TROPHY_SAPIENT")]
    MakeTrophySapient,
    /// The result of a citizen violating noble mandates in fortress mode. Other effects unknown.
    #[serde(alias = "OATH_BREAKING")]
    OathBreaking,
    /// Civilization will enslave defeated enemies and bring them back to their site. Also affects
    /// whether you may trade caged sapient beings to merchants.
    ///
    /// Aside from diplomacy, higher/lower values don't seem to affect anything beyond if a
    /// civilization is willing to take slaves at all.
    #[serde(alias = "SLAVERY")]
    Slavery,
    /// This determines whether the civ will try to steal goods.
    #[serde(alias = "THEFT")]
    Theft,
    ///
    #[serde(alias = "TORTURE_ANIMALS")]
    TortureAnimals,
    /// Civilization will sometimes execute non-combatants after defeating enemy defenders.
    #[serde(alias = "TORTURE_AS_EXAMPLE")]
    TortureAsExample,
    ///
    #[serde(alias = "TORTURE_FOR_FUN")]
    TortureForFun,
    ///
    #[serde(alias = "TORTURE_FOR_INFORMATION")]
    TortureForInformation,
    /// Protects position-holders from being murdered like everyone else – the reason that demon
    /// overlords of goblins manage to live for centuries, despite goblins' regard of killing each
    /// other as being a personal matter.
    #[serde(alias = "TREASON")]
    Treason,
    /// Unconfirmed, possibly ignoring burrow restrictions.
    #[serde(alias = "TRESPASSING")]
    Trespassing,
    /// The result of a tantruming citizen breaking furniture in fortress mode. Other effects
    /// unknown.
    #[serde(alias = "VANDALISM")]
    Vandalism,
}
impl Default for EthicTypeEnum {
    fn default() -> Self {
        Self::Assault
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum EthicReactionEnum {
    #[serde(alias = "NOT_APPLICABLE")]
    NotApplicable,
    #[serde(alias = "ACCEPTABLE")]
    Acceptable,
    #[serde(alias = "PERSONAL_MATTER")]
    PersonalMatter,
    #[serde(alias = "JUSTIFIED_IF_NO_REPERCUSSIONS")]
    JustifiedIfNoRepercussions,
    #[serde(alias = "JUSTIFIED_IF_GOOD_REASON")]
    JustifiedIfGoodReason,
    #[serde(alias = "JUSTIFIED_IF_EXTREME_REASON")]
    JustifiedIfExtremeReason,
    #[serde(alias = "JUSTIFIED_IF_SELF_DEFENSE")]
    JustifiedIfSelfDefense,
    #[serde(alias = "ONLY_IF_SANCTIONED")]
    OnlyIfSanctioned,
    #[serde(alias = "MISGUIDED")]
    Misguided,
    #[serde(alias = "SHUN")]
    Shun,
    #[serde(alias = "APPALLING")]
    Appalling,
    #[serde(alias = "PUNISH_REPRIMAND")]
    PunishReprimand,
    #[serde(alias = "PUNISH_SERIOUS")]
    PunishSerious,
    #[serde(alias = "PUNISH_EXILE")]
    PunishExile,
    #[serde(alias = "PUNISH_CAPITAL")]
    PunishCapital,
    #[serde(alias = "UNTHINKABLE")]
    Unthinkable,
    #[serde(alias = "REQUIRED")]
    Required,
}
impl Default for EthicReactionEnum {
    fn default() -> Self {
        Self::NotApplicable
    }
}

// `TODO` implement this
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum CulturalValueEnum {
    /// - +41 to +50: is an absolute believer in the rule of law
    /// - +26 to +40: has a great deal of respect for the law
    /// - +11 to +25: respects the law
    /// - −10 to +10: doesn't feel strongly about the law
    /// - −25 to −11: does not respect the law
    /// - −40 to −26: disdains the law
    /// - −50 to −41: finds the idea of laws abhorrent
    #[serde(alias = "LAW")]
    Law,
    /// - +41 to +50: has the highest regard for loyalty
    /// - +26 to +40: greatly prizes loyalty
    /// - +11 to +25: values loyalty
    /// - −10 to +10: doesn't particularly value loyalty
    /// - −25 to −11: views loyalty unfavorably
    /// - −40 to −26: disdains loyalty
    /// - −50 to −41: is disgusted by the idea of loyalty
    #[serde(alias = "LOYALTY")]
    Loyalty,
    /// - +41 to +50: sees family as one of the most important things in life
    /// - +26 to +40: values family greatly
    /// - +11 to +25: values family
    /// - −10 to +10: does not care about family one way or the other
    /// - −25 to −11: is put off by family
    /// - −40 to −26: lacks any respect for family
    /// - −50 to −41: finds the idea of family loathsome
    #[serde(alias = "FAMILY")]
    Family,
    /// - +41 to +50: believes friendship is a key to the ideal life
    /// - +26 to +40: sees friendship as one of the finer things in life
    /// - +11 to +25: thinks friendship is important
    /// - −10 to +10: does not care about friendship
    /// - −25 to −11: finds friendship burdensome
    /// - −40 to −26: is completely put off by the idea of friends
    /// - −50 to −41: finds the whole idea of friendship disgusting
    #[serde(alias = "FRIENDSHIP")]
    Friendship,
    /// - +41 to +50: believes that the acquisition of power over others is the ideal goal in life
    /// and worthy of the highest respect
    /// - +26 to +40: sees power over others as something to strive for
    /// - +11 to +25: respects power
    /// - −10 to +10: doesn't find power particularly praiseworthy
    /// - −25 to −11: has a negative view of those who exercise power over others
    /// - −40 to −26: hates those who wield power over others
    /// - −50 to −41: finds the acquisition and use of power abhorrent and would have all masters
    /// toppled
    #[serde(alias = "POWER")]
    Power,
    /// - +41 to +50: believes the truth is inviolable regardless of the cost
    /// - +26 to +40: believes that honesty is a high ideal
    /// - +11 to +25: values honesty
    /// - −10 to +10: does not particularly value the truth
    /// - −25 to −11: finds blind honesty foolish
    /// - −40 to −26: sees lying as an important means to an end
    /// - −50 to −41: is repelled by the idea of honesty and lies without compunction
    #[serde(alias = "TRUTH")]
    Truth,
    /// - +41 to +50: holds well-laid plans and shrewd deceptions in the highest regard
    /// - +26 to +40: greatly respects the shrewd and guileful
    /// - +11 to +25: values cunning
    /// - −10 to +10: does not really value cunning and guile
    /// - −25 to −11: sees guile and cunning as indirect and somewhat worthless
    /// - −40 to −26: holds shrewd and crafty individuals in the lowest esteem
    /// - −50 to −41: is utterly disgusted by guile and cunning
    #[serde(alias = "CUNNING")]
    Cunning,
    /// - +41 to +50: believes that artful speech and eloquent expression are some of the highest
    /// ideals
    /// - +26 to +40: deeply respects eloquent speakers
    /// - +11 to +25: values eloquence
    /// - −10 to +10: doesn't value eloquence so much
    /// - −25 to −11: finds eloquence and artful speech off-putting
    /// - −40 to −26: finds [him/her]self somewhat disgusted with eloquent speakers
    /// - −50 to −41: sees artful speech and eloquence as a wasteful form of deliberate deception
    /// and treats it as such
    #[serde(alias = "ELOQUENCE")]
    Eloquence,
    /// - +41 to +50: holds fairness as one of the highest ideals and despises cheating of any kind
    /// - +26 to +40: has great respect for fairness
    /// - +11 to +25: respects fair-dealing and fair-play
    /// - −10 to +10: does not care about fairness
    /// - −25 to −11: sees life as unfair and doesn't mind it that way
    /// - −40 to −26: finds the idea of fair-dealing foolish and cheats when [he/she] finds it
    /// profitable
    /// - −50 to −41: is disgusted by the idea of fairness and will freely cheat anybody at any time
    #[serde(alias = "FAIRNESS")]
    Fairness,
    /// - +41 to +50: views decorum as a high ideal and is deeply offended by those that fail to
    /// maintain it
    /// - +26 to +40: greatly respects those that observe decorum and maintain their dignity
    /// - +11 to +25: values decorum, dignity and proper behavior
    /// - −10 to +10: doesn't care very much about decorum
    /// - −25 to −11: finds maintaining decorum a silly, fumbling waste of time
    /// - −40 to −26: sees those that attempt to maintain dignified and proper behavior as vain and
    /// offensive
    /// - −50 to −41: is affronted by the whole notion of maintaining decorum and finds so-called
    /// dignified people disgusting
    #[serde(alias = "DECORUM")]
    Decorum,
    /// - +41 to +50: holds the maintenance of tradition as one of the highest ideals
    /// - +26 to +40: is a firm believer in the value of tradition
    /// - +11 to +25: values tradition
    /// - −10 to +10: doesn't have any strong feelings about tradition
    /// - −25 to −11: disregards tradition
    /// - −40 to −26: finds the following of tradition foolish and limiting
    /// - −50 to −41: is disgusted by tradition and would flout any [he/she] encounters if given a
    /// chance
    #[serde(alias = "TRADITION")]
    Tradition,
    /// - +41 to +50: believes that the creation and appreciation of artwork is one of the highest
    /// ideals
    /// - +26 to +40: greatly respects artists and their works
    /// - +11 to +25: values artwork
    /// - −10 to +10: doesn't care about art one way or another
    /// - −25 to −11: finds artwork boring
    /// - −40 to −26: sees the whole pursuit of art as silly
    /// - −50 to −41: finds art offensive and would have it destroyed whenever possible
    #[serde(alias = "ARTWORK")]
    Artwork,
    /// - +41 to +50: places cooperation as one of the highest ideals
    /// - +26 to +40: sees cooperation as very important in life
    /// - +11 to +25: values cooperation
    /// - −10 to +10: doesn't see cooperation as valuable
    /// - −25 to −11: dislikes cooperation
    /// - −40 to −26: views cooperation as a low ideal not worthy of any respect
    /// - −50 to −41: is thoroughly disgusted by cooperation
    #[serde(alias = "COOPERATION")]
    Cooperation,
    /// - +41 to +50: believes that freedom and independence are completely non-negotiable and would
    /// fight to defend them
    /// - +26 to +40: treasures independence
    /// - +11 to +25: values independence
    /// - −10 to +10: doesn't really value independence one way or another
    /// - −25 to −11: finds the ideas of independence and freedom somewhat foolish
    /// - −40 to −26: sees freedom and independence as completely worthless
    /// - −50 to −41: hates freedom and would crush the independent spirit wherever it is found
    #[serde(alias = "INDEPENDENCE")]
    Independence,
    /// - +41 to +50: views any show of emotion as offensive
    /// - +26 to +40: thinks it is of the utmost importance to present a bold face and never grouse,
    /// complain or even show emotion
    /// - +11 to +25: believes it is important to conceal emotions and refrain from complaining
    /// - −10 to +10: doesn't see much value in being stoic
    /// - −25 to −11: sees no value in holding back complaints and concealing emotions
    /// - −40 to −26: feels that those who attempt to conceal their emotions are vain and foolish
    /// - −50 to −41: sees concealment of emotions as a betrayal and tries [his/her] best never to
    /// associate with such secretive fools
    #[serde(alias = "STOICISM")]
    Stoicism,
    /// - +41 to +50: feels that introspection and all forms of self-examination are the keys to a
    /// good life and worthy of respect
    /// - +26 to +40: deeply values introspection
    /// - +11 to +25: sees introspection as important
    /// - −10 to +10: doesn't really see the value in self-examination
    /// - −25 to −11: finds introspection to be a waste of time
    /// - −40 to −26: thinks that introspection is valueless and those that waste time in self-
    /// examination are deluded fools
    /// - −50 to −41: finds the whole idea of introspection completely offensive and contrary to the
    /// ideals of a life well-lived
    #[serde(alias = "INTROSPECTION")]
    Introspection,
    /// - +41 to +50: believes that self-mastery and the denial of impulses are of the highest
    /// ideals
    /// - +26 to +40: finds moderation and self-control to be very important
    /// - +11 to +25: values self-control
    /// - −10 to +10: doesn't particularly value self-control
    /// - −25 to −11: finds those that deny their impulses somewhat stiff
    /// - −40 to −26: sees the denial of impulses as a vain and foolish pursuit
    /// - −50 to −41: has abandoned any attempt at self-control and finds the whole concept deeply
    /// offensive
    #[serde(alias = "SELF_CONTROL")]
    SelfControl,
    /// - +41 to +50: views tranquility as one of the highest ideals
    /// - +26 to +40: strongly values tranquility and quiet
    /// - +11 to +25: values tranquility and a peaceful day
    /// - −10 to +10: doesn't have a preference between tranquility and tumult
    /// - −25 to −11: prefers a noisy, bustling life to boring days without activity
    /// - −40 to −26: is greatly disturbed by quiet and a peaceful existence
    /// - −50 to −41: is disgusted by tranquility and would that the world would constantly churn
    /// with noise and activity
    #[serde(alias = "TRANQUILITY")]
    Tranquility,
    /// - +41 to +50: would have the world operate in complete harmony without the least bit of
    /// strife or disorder
    /// - +26 to +40: strongly believes that a peaceful and ordered society without dissent is best
    /// - +11 to +25: values a harmonious existence
    /// - −10 to +10: sees equal parts of harmony and discord as part of life
    /// - −25 to −11: doesn't respect a society that has settled into harmony without debate and
    /// strife
    /// - −40 to −26: can't fathom why anyone would want to live in an orderly and harmonious
    /// society
    /// - −50 to −41: believes deeply that chaos and disorder are the truest expressions of life and
    /// would disrupt harmony wherever it is found
    #[serde(alias = "HARMONY")]
    Harmony,
    /// - +41 to +50: believes that little is better in life than a good party
    /// - +26 to +40: truly values merrymaking and parties
    /// - +11 to +25: finds merrymaking and partying worthwhile activities
    /// - −10 to +10: doesn't really value merrymaking
    /// - −25 to −11: sees merrymaking as a waste
    /// - −40 to −26: is disgusted by merrymakers
    /// - −50 to −41: is appalled by merrymaking, parties and other such worthless activities
    #[serde(alias = "MERRIMENT")]
    Merriment,
    /// - +41 to +50: holds crafts[man]ship to be of the highest ideals and celebrates talented
    /// artisans and their masterworks
    /// - +26 to +40: has a great deal of respect for worthy crafts[man]ship
    /// - +11 to +25: values good crafts[man]ship
    /// - −10 to +10: doesn't particularly care about crafts[man]ship
    /// - −25 to −11: considers crafts[man]ship to be relatively worthless
    /// - −40 to −26: sees the pursuit of good crafts[man]ship as a total waste
    /// - −50 to −41: views crafts[man]ship with disgust and would desecrate a so-called masterwork
    /// or two if [he/she] could get away with it
    #[serde(alias = "CRAFTSMANSHIP")]
    Craftsmanship,
    /// - +41 to +50: believes that martial prowess defines the good character of an individual
    /// - +26 to +40: deeply respects skill at arms
    /// - +11 to +25: values martial prowess
    /// - −10 to +10: does not really value skills related to fighting
    /// - −25 to −11: finds those that develop skill with weapons and fighting distasteful
    /// - −40 to −26: thinks that the pursuit of the skills of warfare and fighting is a low pursuit
    /// indeed
    /// - −50 to −41: abhors those that pursue the mastery of weapons and skill with fighting
    #[serde(alias = "MARTIAL_PROWESS")]
    MartialProwess,
    /// - +41 to +50: believes that the mastery of a skill is one of the highest pursuits
    /// - +26 to +40: really respects those that take the time to master a skill
    /// - +11 to +25: respects the development of skill
    /// - −10 to +10: doesn't care if others take the time to master skills
    /// - −25 to −11: finds the pursuit of skill mastery off-putting
    /// - −40 to −26: believes that the time taken to master a skill is a horrible waste
    /// - −50 to −41: sees the whole idea of taking time to master a skill as appalling
    #[serde(alias = "SKILL")]
    Skill,
    /// - +41 to +50: believes that hard work is one of the highest ideals and a key to the good
    /// life
    /// - +26 to +40: deeply respects those that work hard at their labors
    /// - +11 to +25: values hard work
    /// - −10 to +10: doesn't really see the point of working hard
    /// - −25 to −11: sees working hard as a foolish waste of time
    /// - −40 to −26: thinks working hard is an abject idiocy
    /// - −50 to −41: finds the proposition that one should work hard in life utterly abhorrent
    #[serde(alias = "HARD_WORK")]
    HardWork,
    /// - +41 to +50: finds sacrifice to be one of the highest ideals
    /// - +26 to +40: believes that those who sacrifice for others should be deeply respected
    /// - +11 to +25: values sacrifice
    /// - −10 to +10: doesn't particularly respect sacrifice as a virtue
    /// - −25 to −11: sees sacrifice as wasteful and foolish
    /// - −40 to −26: finds sacrifice to be the height of folly
    /// - −50 to −41: thinks that the entire concept of sacrifice for others is truly disgusting
    #[serde(alias = "SACRIFICE")]
    Sacrifice,
    /// - +41 to +50: holds the idea of competition among the most important values and would
    /// encourage it wherever possible
    /// - +26 to +40: views competition as a crucial driving force in the world
    /// - +11 to +25: sees competition as reasonably important
    /// - −10 to +10: doesn't have strong views on competition
    /// - −25 to −11: sees competition as wasteful and silly
    /// - −40 to −26: deeply dislikes competition
    /// - −50 to −41: finds the very idea of competition obscene
    #[serde(alias = "COMPETITION")]
    Competition,
    /// - +41 to +50: believes that perseverance is one of the greatest qualities somebody can have
    /// - +26 to +40: greatly respects individuals that persevere through their trials and labors
    /// - +11 to +25: respects perseverance
    /// - −10 to +10: doesn't think much about the idea of perseverance
    /// - −25 to −11: sees perseverance in the face of adversity as bull-headed and foolish
    /// - −40 to −26: thinks there is something deeply wrong with people that persevere through
    /// adversity
    /// - −50 to −41: finds the notion that one would persevere through adversity completely
    /// abhorrent
    #[serde(alias = "PERSEVERANCE")]
    Perseverance,
    /// - +41 to +50: believes that it would be a fine thing if all time were leisure time
    /// - +26 to +40: treasures leisure time and thinks it is very important in life
    /// - +11 to +25: values leisure time
    /// - −10 to +10: doesn't think one way or the other about leisure time
    /// - −25 to −11: finds leisure time wasteful
    /// - −40 to −26: is offended by leisure time and leisurely living
    /// - −50 to −41: believes that those that take leisure time are evil and finds the whole idea
    /// disgusting
    #[serde(alias = "LEISURE_TIME")]
    LeisureTime,
    /// - +41 to +50: sees engaging in commerce as a high ideal in life
    /// - +26 to +40: really respects commerce and those that engage in trade
    /// - +11 to +25: respects commerce
    /// - −10 to +10: doesn't particularly respect commerce
    /// - −25 to −11: is somewhat put off by trade and commerce
    /// - −40 to −26: finds those that engage in trade and commerce to be fairly disgusting
    /// - −50 to −41: holds the view that commerce is a vile obscenity
    #[serde(alias = "COMMERCE")]
    Commerce,
    /// - +41 to +50: sees romance as one of the highest ideals
    /// - +26 to +40: thinks romance is very important in life
    /// - +11 to +25: values romance
    /// - −10 to +10: doesn't care one way or the other about romance
    /// - −25 to −11: finds romance distasteful
    /// - −40 to −26: is somewhat disgusted by romance
    /// - −50 to −41: finds even the abstract idea of romance repellent
    #[serde(alias = "ROMANCE")]
    Romance,
    /// - +41 to +50: holds nature to be of greater value than most aspects of civilization
    /// - +26 to +40: has a deep respect for animals, plants and the natural world
    /// - +11 to +25: values nature
    /// - −10 to +10: doesn't care about nature one way or another
    /// - −25 to −11: finds nature somewhat disturbing
    /// - −40 to −26: has a deep dislike of the natural world
    /// - −50 to −41: would just as soon have nature and the great outdoors burned to ashes and
    /// converted into a great mining pit
    #[serde(alias = "NATURE")]
    Nature,
    /// - +41 to +50: believes the idea of war is utterly repellent and would have peace at all
    /// costs
    /// - +26 to +40: believes that peace is always preferable to war
    /// - +11 to +25: values peace over war
    /// - −10 to +10: doesn't particularly care between war and peace
    /// - −25 to −11: sees war as a useful means to an end
    /// - −40 to −26: believes war is preferable to peace in general
    /// - −50 to −41: thinks that the world should be engaged in perpetual warfare
    #[serde(alias = "PEACE")]
    Peace,
    /// - +41 to +50: finds the quest for knowledge to be of the very highest value
    /// - +26 to +40: views the pursuit of knowledge as deeply important
    /// - +11 to +25: values knowledge
    /// - −10 to +10: doesn't see the attainment of knowledge as important
    /// - −25 to −11: finds the pursuit of knowledge to be a waste of effort
    /// - −40 to −26: thinks the quest for knowledge is a delusional fantasy
    /// - −50 to −41: sees the attainment and preservation of knowledge as an offensive enterprise
    /// engaged in by arrogant fools
    #[serde(alias = "KNOWLEDGE")]
    Knowledge,
}
impl Default for CulturalValueEnum {
    fn default() -> Self {
        Self::Law
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SeasonEnum {
    #[serde(alias = "SPRING")]
    Spring,
    #[serde(alias = "SUMMER")]
    Summer,
    #[serde(alias = "AUTUMN")]
    Autumn,
    #[serde(alias = "WINTER")]
    Winter,
}
impl Default for SeasonEnum {
    fn default() -> Self {
        Self::Spring
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ScholarTypeEnum {
    #[serde(alias = "ALL")]
    All,
    #[serde(alias = "ASTRONOMER")]
    Astronomer,
    #[serde(alias = "CHEMIST")]
    Chemist,
    #[serde(alias = "DOCTOR")]
    Doctor,
    #[serde(alias = "ENGINEER")]
    Engineer,
    #[serde(alias = "GEOGRAPHER")]
    Geographer,
    #[serde(alias = "HISTORIAN")]
    Historian,
    #[serde(alias = "MATHEMATICIAN")]
    Mathematician,
    #[serde(alias = "NATURALIST")]
    Naturalist,
    #[serde(alias = "PHILOSOPHER")]
    Philosopher,
}
impl Default for ScholarTypeEnum {
    fn default() -> Self {
        Self::All
    }
}
