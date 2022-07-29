use crate::core::{AllowEmpty, Any, Choose, Clamp, DFChar, Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

use crate::structure::{
    AlertOrPeacefulIntermittentEnum, Attack, BodyAppearanceModifier, BodyAttributeEnum,
    BodyDetailPlanToken, BodyGlossToken, BodyToken, BpCriteriaTokenArg, CanDoInteraction,
    ColorToken, CreatureToken, CreatureVariationToken, ExtraButcherObject, GaitFlagTokenArg,
    GaitTypeEnum, HabitTypeEnum, ItemReferenceArg, LairCharacteristicEnum, LairTypeEnum,
    MaleOrFemaleEnum, MaterialStateEnum, MaterialTokenArg, NoBuildUpEnum, NoneEnum,
    PersonalityTraitEnum, PlantOrCreatureTokenArg, SecretionTriggerEnum, SelectTissueLayer,
    SetBpGroup, SetTlGroup, SkillEnum, SoulAttributeEnum, TestAllEnum, TissueLayer, UnitTypeEnum,
    VocalizationEnum,
};

// TODO: don't exit back up to parent when a creature-only token is used; instead give an error
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Caste {
    /// argument 1 of `CASTE`
    #[serde(alias = "CASTE")]
    pub reference: Option<Reference>,
    // region: Caste tokens, forbidden in Creature ================================================
    /// Defines a new caste derived directly from a previous caste. The new caste inherits all
    /// properties of the old one.
    ///
    /// The effect of this tag is automatic if one has not yet defined any castes: "Any caste-level
    /// tag that occurs before castes are explicitly declared is saved up and placed on any caste
    /// that is declared later, unless the caste is explicitly derived from another caste."
    ///
    /// This is because before you have defined a `[CASTE:..]`, you are technically already using
    /// the unique `ALL` caste, whose tokens are applied to all specific castes.
    #[serde(alias = "USE_CASTE")]
    pub use_caste: Vec<(Reference, Reference)>,
    // endregion ==================================================================================
    // region: Caste Tokens =======================================================================
    /// A list of `[ATTACK:...]` tokens this creature caste has.
    #[serde(alias = "ATTACK")]
    pub attacks: Vec<Attack>,
    /// List of interactions the creature can perform.
    #[serde(alias = "CAN_DO_INTERACTION")]
    pub can_do_interaction: Vec<CanDoInteraction>,
    /// A list of `SET_BP_GROUP` tokens on this creature.
    #[serde(alias = "SET_BP_GROUP")]
    pub set_bp_group: Vec<SetBpGroup>,
    /// A list of `SET_TL_GROUP` tokens on this creature.
    #[serde(alias = "SET_TL_GROUP")]
    pub set_tl_group: Vec<SetTlGroup>,
    /// A list of `SElECT_TISSUE_LAYER` tokens on this creature.
    #[serde(alias = "SELECT_TISSUE_LAYER")]
    pub select_tissue_layer: Vec<SelectTissueLayer>,
    /// A list of `TISSUE_LAYER` tokens on this creature.
    #[serde(alias = "TISSUE_LAYER")]
    pub tissue_layer: Vec<TissueLayer>,
    /// Presumably a counterpart to `TISSUE_LAYER_UNDER`, which adds a tissue layer over a given part.
    #[serde(alias = "TISSUE_LAYER_OVER")]
    // TODO: the optional reference here is weird and needs some research;
    // look for this token in `creature_masterwork_fish.txt` to see what I mean.
    pub tissue_layer_over: Vec<(BpCriteriaTokenArg, Reference, Option<Reference>)>,
    /// Adds the tissue layer under a given part.
    ///
    /// For example an Iron Man has a gaseous poison within and this tissue (`GAS` is its name) has
    /// the token `[TISSUE_LEAKS]` and its state is `GAS`, so when you puncture the iron outside and
    /// damage this tissue it leaks gas (can have a syndrome by using a previous one in the creature
    /// sample).
    #[serde(alias = "TISSUE_LAYER_UNDER")]
    pub tissue_layer_under: Vec<(BpCriteriaTokenArg, Reference)>,
    /// Prevents tamed creature from being made available for adoption, instead allowing it to
    /// automatically adopt whoever it wants. The basic requirements for adoption are intact, and
    /// the creature will only adopt individuals who have a preference for their species. Used by
    /// cats in the vanilla game.
    #[serde(alias = "ADOPTS_OWNER")]
    pub adopts_owner: Option<()>,
    /// Makes the creature need alcohol to get through the working day; it will choose to drink
    /// booze instead of water if possible. Going sober for too long reduces speed.
    #[serde(alias = "ALCOHOL_DEPENDENT")]
    pub alcohol_dependent: Option<()>,
    /// Sets the creature to be active during the day, night, and twilight in Adventurer Mode. Seems
    /// to be a separate value from `[DIURNAL]`/`[NOCTURNAL]`/`[CREPUSCULAR]`, rather than implying them.
    #[serde(alias = "ALL_ACTIVE")]
    pub all_active: Option<()>,
    /// Found on `[LARGE_PREDATOR]`s who ambush their prey. Instead of charging relentlessly at prey,
    /// the predator will wait till the prey is within a few squares before charging. May or may not
    /// work on other creatures (unverified).
    #[serde(alias = "AMBUSHPREDATOR")]
    pub ambushpredator: Option<()>,
    /// Allows a creature to breathe both inside and outside of water (unlike `[AQUATIC]`) - does
    /// not prevent drowning in magma.
    #[serde(alias = "AMPHIBIOUS")]
    pub amphibious: Option<()>,
    /// Applies the specified creature variation.
    ///
    /// In addition to the required `CREATURE_VARIATION` object ID, you may give extra arguments;
    /// These extra arguments will replace instances of `!ARGn` in the creature variation, where
    /// `n` is the index of the argument given to `APPLY_CREATURE_VARIATION`; the first argument
    /// will replace any "`!ARG1`", the second any "`!ARG2`" and so on. You may use any number of arguments.
    ///
    /// If you have an `!ARGn` of a higher number than the arguments given, the replacements will
    /// act oddly, depending on if `n` has more digits than the number of arguments given.
    /// For example, `[APPLY_CREATURE_VARIATION:EXAMPLE_CV:one:two:three]`, which has three
    /// arguments given, will leave all instances of `!ARG5` in `EXAMPLE_CV` intact as `!ARG5`,
    /// but `!ARG10` will become `one0`.
    ///
    /// The pipe character `|` is turned into `:` when inserted, so single arguments in
    /// `APPLY_CREATURE_VARIATION` may be turned into multi-argument segments in the output.
    ///  For example, with this creature variation:
    ///
    /// ```df_raw
    /// [CREATURE_VARIATION:DIMORPHIC_FEATHER_COLORS]
    ///     [CV_NEW_TAG:SELECT_CASTE:FEMALE]
    ///     [CV_NEW_TAG:SET_TL_GROUP:BY_CATEGORY:ALL:FEATHER]
    ///         [CV_NEW_TAG:TL_COLOR_MODIFIER:!ARG1]
    ///             [CV_NEW_TAG:TLCM_NOUN:feathers:PLURAL]
    /// [CV_NEW_TAG:SELECT_CASTE:MALE]
    ///     [CV_NEW_TAG:SET_TL_GROUP:BY_CATEGORY:ALL:FEATHER]
    ///         [CV_NEW_TAG:TL_COLOR_MODIFIER:!ARG2]
    ///             [CV_NEW_TAG:TLCM_NOUN:feathers:PLURAL]
    /// ```
    /// If you use `[APPLY_CREATURE_VARIATION:DIMORPHIC_FEATHER_COLORS:BROWN|1:BLUE|10|GREEN|1|BLACK|1]`,
    /// the following is what will be added to the creature:
    ///
    /// ```df_raw
    /// [SELECT_CASTE:FEMALE]
    ///     [SET_TL_GROUP:BY_CATEGORY:ALL:FEATHER]
    ///     [TL_COLOR_MODIFIER:BROWN:1]
    ///         [TLCM_NOUN:feathers:PLURAL]
    /// [SELECT_CASTE:MALE]
    ///     [SET_TL_GROUP:BY_CATEGORY:ALL:FEATHER]
    ///     [TL_COLOR_MODIFIER:BLUE:10:GREEN:1:BLACK:1]
    ///         [TLCM_NOUN:feathers:PLURAL]
    /// ```
    #[serde(alias = "APPLY_CREATURE_VARIATION")]
    pub apply_creature_variation: Vec<(
        ReferenceTo<CreatureVariationToken>,
        Option<(Vec<AllowEmpty<Any>>,)>,
    )>,
    /// Enables the creature to breathe in water, but causes it to air-drown on dry land.
    #[serde(alias = "AQUATIC")]
    pub aquatic: Option<()>,
    /// Causes the creature to be excluded from the object testing arena's creature spawning list.
    ///
    /// Typically applied to spoileriffic creatures.
    #[serde(alias = "ARENA_RESTRICTED")]
    pub arena_restricted: Option<()>,
    /// Prevents the creature from attacking or frighten creatures with the `[NATURAL]` tag.
    #[serde(alias = "AT_PEACE_WITH_WILDLIFE")]
    pub at_peace_with_wildlife: Option<()>,
    /// Specifies when a megabeast or semi-megabeast will attack the fortress. The attacks will
    /// start occuring when at least one of the requirements is met. Setting a value to 0 disables
    /// the trigger.
    ///
    /// Arguments: population, exported wealth, created wealth.
    #[serde(alias = "ATTACK_TRIGGER")]
    pub attack_trigger: Option<(u32, u32, u32)>,
    /// Age at which creature is considered a child, the default is zero. One can think of this as
    /// the duration of the baby stage.
    #[serde(alias = "BABY")]
    pub baby: Option<u32>,
    /// Defines a new name for a creature in baby state at the caste level. For non-caste-specific
    /// baby names, see `[GENERAL_BABY_NAME]`.
    ///
    /// Arguments: singular, plural
    #[serde(alias = "BABYNAME")]
    pub babyname: Option<(String, String)>,
    /// Creature may be subject to beaching, becoming stranded on shores, where they will eventually
    /// air-drown. The number indicates the frequency of the occurrence. Presumably requires the
    /// creature to be `[AQUATIC]`. Used by orcas, sperm whales and sea nettle jellyfish in the
    /// vanilla game.
    #[serde(alias = "BEACH_FREQUENCY")]
    pub beach_frequency: Option<u32>,
    /// The creature is non-aggressive by default, and will never automatically be engaged by
    /// companions or soldiers. It will run away from any creatures that are not friendly to it, and
    /// will only defend itself if it becomes enraged. Can be thought of as the counterpoint of the
    /// `[LARGE_PREDATOR]` tag. When tamed, animals with this tag will be useless for fortress
    /// defense.
    #[serde(alias = "BENIGN")]
    pub benign: Option<()>,
    /// Specifies what the creature's blood is made of.
    #[serde(alias = "BLOOD")]
    pub blood: Option<(MaterialTokenArg, MaterialStateEnum)>,
    /// Causes vampire-like behaviour; the creature will suck the blood of unconscious victims when
    /// its thirst for blood grows sufficiently large. When controlling the creature in adventure
    /// mode, this can be done at will. Seems to be required to make the creature denouncable as a
    /// creature of the night.
    #[serde(alias = "BLOODSUCKER")]
    pub bloodsucker: Option<()>,
    /// Draws body parts from `OBJECT:BODY` files (such as `body_default.txt`)
    ///
    /// For example:
    ///
    /// `[BODY:BODY_WITH_HEAD_FLAG:HEART:GUTS:BRAIN:MOUTH]`
    ///
    /// This is the body from a purring maggot. It creates a body with head, a heart, some guts, a
    /// brain, and a mouth. That's all a maggot needs.
    ///
    /// **If the body is left undefined, the creature will cause a crash whenever it spawns.**
    #[serde(alias = "BODY")]
    pub body: Option<(Vec<ReferenceTo<BodyToken>>,)>,
    /// A list of `BODY_APPEARANCE_MODIFIER` tokens on this creature.
    #[serde(alias = "BODY_APPEARANCE_MODIFIER")]
    pub body_appearance_modifier: Vec<BodyAppearanceModifier>,
    /// Loads a plan listed in `OBJECT:BODY_DETAIL_PLAN` files, such as `b_detail_plan_default.txt`. Mass
    /// applies `USE_MATERIAL_TEMPLATE`, mass alters `RELSIZE`, alters body part positions, and will
    /// allow tissue layers to be defined. Tissue layers are defined in order of skin to bone here.
    ///
    /// For example:
    ///
    /// `[BODY_DETAIL_PLAN:VERTEBRATE_TISSUE_LAYERS:SKIN:FAT:MUSCLE:BONE:CARTILAGE]`
    ///
    /// This creates the detailed body of a fox, the skin, fat, muscle, bones and cartilage out of
    /// the vertebrate tissues.
    ///
    /// A maggot would only need:
    ///
    /// `[BODY_DETAIL_PLAN:EXOSKELETON_TISSUE_LAYERS:SKIN:FAT:MUSCLE]`
    #[serde(alias = "BODY_DETAIL_PLAN")]
    pub body_detail_plan: Vec<(
        ReferenceTo<BodyDetailPlanToken>,
        Option<Reference>,
        Option<Reference>,
        Option<Reference>,
        Option<Reference>,
        Option<Reference>,
    )>,
    /// Sets size at a given age. Size is in cubic centimeters, and for normal body materials, is
    /// roughly equal to the creature's average weight in grams.
    ///
    /// For example, this is the size of a minotaur:
    ///
    /// `[BODY_SIZE:0:0:10000]`
    ///
    /// `[BODY_SIZE:1:168:50000]`
    ///
    /// `[BODY_SIZE:12:0:220000]`
    ///
    /// Its birth size would be 10,000 cm3 (~10 kg). At 1 year and 168 days old it would be
    /// 50,000 cm3 (~50 kg). And as an adult (at 12 years old) it would be 220,000 cm3 and weigh
    /// roughly 220 kg.
    #[serde(alias = "BODY_SIZE")]
    pub body_size: Vec<(u32, u32, u32)>,
    /// Substitutes body part text with replacement text. Draws gloss information from `OBJECT:BODY`
    /// files (such as `body_default.txt`)
    #[serde(alias = "BODYGLOSS")]
    pub bodygloss: Vec<(Vec<ReferenceTo<BodyGlossToken>>,)>,
    /// Creature eats bones. Implies `[CARNIVORE]`. Adventurers with this token are currently unable
    /// to eat bones. [Bug:11069](https://www.bay12games.com/dwarves/mantisbt/view.php?id=11069)
    #[serde(alias = "BONECARN")]
    pub bonecarn: Option<()>,
    /// Allows a creature to destroy furniture and buildings. Value `[1]` targets mostly doors,
    /// hatches, furniture and the like. Value `[2]` targets any building/structure other than
    /// floors, walls, and stairs.
    #[serde(alias = "BUILDINGDESTROYER")]
    pub buildingdestroyer: Option<u8>,
    /// The creature gains skills and can have professions. If a member of a civilization (even a
    /// pet) has this token, it'll need to eat, drink and sleep. Note that this token makes the
    /// creature unable to be eaten by an adventurer, so it is not recommended for uncivilized
    /// monsters. Adventurers lacking this token can allocate but not increase attributes and
    /// skills. Skills allocated will disappear on start.
    #[serde(alias = "CAN_LEARN")]
    pub can_learn: Option<()>,
    /// Can talk. Note that this is not necessary for a creature to gain social skills.
    #[serde(alias = "CAN_SPEAK")]
    pub can_speak: Option<()>,
    /// Creature cannot climb, even if it has free grasp parts.
    #[serde(alias = "CANNOT_CLIMB")]
    pub cannot_climb: Option<()>,
    /// Creature cannot jump.
    #[serde(alias = "CANNOT_JUMP")]
    pub cannot_jump: Option<()>,
    /// Acts like `[NOT_LIVING]`, except that `[OPPOSED_TO_LIFE]` creatures will attack them.
    #[serde(alias = "CANNOT_UNDEAD")]
    pub cannot_undead: Option<()>,
    /// Allows the creature to open doors that are set to be unpassable for pets. In adventure mode,
    /// creatures lacking this token will be unable to pass through door tiles except whilst these
    /// are occupied by other creatures.
    #[serde(alias = "CANOPENDOORS")]
    pub canopendoors: Option<()>,
    /// Creature only eats meat. If the creature goes on rampages in worldgen, it will often devour
    /// the people/animals it kills.
    #[serde(alias = "CARNIVORE")]
    pub carnivore: Option<()>,
    /// Caste-specific `[ALTTILE]`; if set, the creature of this caste will blink between its
    /// `[CASTE_TILE]` and its `CASTE_ALTTILE`.
    #[serde(alias = "CASTE_ALTTILE")]
    pub caste_alttile: Option<DFChar>,
    /// Caste-specific `[COLOR]`. Color of the creature's tile.
    /// See [Color](https://dwarffortresswiki.org/index.php/Color) for usage.
    #[serde(alias = "CASTE_COLOR")]
    pub caste_color: Option<(u8, u8, u8)>,
    /// Caste-specific `[GLOWCOLOR]`; The colour of the creature/caste's `[CASTE_GLOWTILE]`.
    #[serde(alias = "CASTE_GLOWCOLOR")]
    pub caste_glowcolor: Option<(u8, u8, u8)>,
    /// Caste-specific `[GLOWTILE]`; if present, the being glows in the dark (generally used for
    /// Adventurer Mode). The tile is what replaces the being's current tile when it is obscured
    /// from your sight by darkness.
    #[serde(alias = "CASTE_GLOWTILE")]
    pub caste_glowtile: Option<DFChar>,
    /// Caste-specific `[NAME]`; the generic name for any creature of this type and caste.
    #[serde(alias = "CASTE_NAME")]
    pub caste_name: Option<(String, String, String)>,
    /// Caste-specific `[PROFESSION_NAME]`; the generic name for members of this profession,
    /// of this caste.
    #[serde(alias = "CASTE_PROFESSION_NAME")]
    pub caste_profession_name: Vec<(UnitTypeEnum, String, String)>,
    /// Caste-specific `[SOLDIER_ALTTILE]`; if this creature of this caste is active in its
    /// civilization's military, it will blink between its default tile and this one.
    /// Requires `[CASTE_SOLDIER_TILE]`.
    #[serde(alias = "CASTE_SOLDIER_ALTTILE")]
    pub caste_soldier_alttile: Option<DFChar>,
    /// Caste-specific `[CREATURE_SOLDIER_TILE]`; creatures of this caste active in their
    /// civilization's military will use this tile instead.
    #[serde(alias = "CASTE_SOLDIER_TILE")]
    pub caste_soldier_tile: Option<DFChar>,
    /// Caste-specific version of `[SPEECH]`; boasting speeches relating to killing this creature
    /// of this caste. Examples of `SPEECH` include `dwarf.txt` and `elf.txt` in `data\speech`.
    /// Must end in `.txt`.
    #[serde(alias = "CASTE_SPEECH")]
    pub caste_speech: Option<String>, // TODO: string is a txt path
    /// Caste-specific `[CREATURE_TILE]`; the symbol of the creature of this caste in ASCII mode.
    #[serde(alias = "CASTE_TILE")]
    pub caste_tile: Option<DFChar>,
    /// Gives the creature a bonus in caves. Also causes cave adaptation.
    #[serde(alias = "CAVE_ADAPT")]
    pub cave_adapt: Option<()>,
    /// Multiplies body size by a factor of (integer)%. 50 halves size, 200 doubles.
    #[serde(alias = "CHANGE_BODY_SIZE_PERC")]
    pub change_body_size_perc: Option<u32>,
    /// Age at which creature is considered an adult. One can think of this as the duration of the
    /// child stage. Allows the creature's offspring to be rendered fully tame if trained during
    /// their childhood.
    #[serde(alias = "CHILD")]
    pub child: Option<u32>,
    /// Defines a new name for a creature in child state at the caste level. For non-caste-specific
    /// child names, see `[GENERAL_CHILD_NAME]`.
    #[serde(alias = "CHILDNAME")]
    pub childname: Option<(String, String)>,
    /// Number of eggs laid in one sitting.
    #[serde(alias = "CLUTCH_SIZE")]
    pub clutch_size: Option<(u32, u32)>,
    /// Caste hovers around colony.
    #[serde(alias = "COLONY_EXTERNAL")]
    pub colony_external: Option<()>,
    /// When combined with any of `[PET]`, `[PACK_ANIMAL]`, `[WAGON_PULLER]` and/or `[MOUNT]`, the
    /// creature is guaranteed to be domesticated by any civilization with `[COMMON_DOMESTIC_PET]`,
    /// `[COMMON_DOMESTIC_PACK]`.
    ///
    /// `[COMMON_DOMESTIC_PULL]` and/or `[COMMON_DOMESTIC_MOUNT]` respectively. Such civilizations
    /// will always have access to the creature, even in the absence of wild populations. This token
    /// is invalid on `[FANCIFUL]` creatures.
    #[serde(alias = "COMMON_DOMESTIC")]
    pub common_domestic: Option<()>,
    /// Creatures of this caste's species with the `[SPOUSE_CONVERTER]` and
    /// `[NIGHT_CREATURE_HUNTER]` tokens will kidnap `[SPOUSE_CONVERSION_TARGET]`s of an appropriate
    /// sex and convert them into castes with `CONVERTED_SPOUSE`.
    #[serde(alias = "CONVERTED_SPOUSE")]
    pub converted_spouse: Option<()>,
    /// Set this to allow the creature to be cooked in meals without first being butchered/cleaned.
    /// Used by some water-dwelling vermin such as mussels, nautiluses and oysters.
    #[serde(alias = "COOKABLE_LIVE")]
    pub cookable_live: Option<()>,
    /// Creature is 'berserk' and will attack all other creatures, except members of its own species
    /// that also have the `CRAZED` tag. It will show "Berserk" in the unit list. Berserk creatures go
    /// on rampages during worldgen much more frequently than non-berserk ones.
    #[serde(alias = "CRAZED")]
    pub crazed: Option<()>,
    /// An arbitrary creature classification. Can be set to anything, but only existing uses are
    /// `GENERAL_POISON` (used in syndromes), `EDIBLE_GROUND_BUG` (valid targets for `GOBBLE_VERMIN_x`
    /// tokens), and `MAMMAL` (self-explanatory). A single creature can have multiple classes.
    /// Eligibility for certain entity positions can also be permitted or restricted by this tag.
    #[serde(alias = "CREATURE_CLASS")]
    pub creature_class: Vec<Reference>,
    /// Sets the creature to be active at twilight in adventurer mode.
    #[serde(alias = "CREPUSCULAR")]
    pub crepuscular: Option<()>,
    /// Allows a creature to steal and eat edible items from a site. It will attempt to grab a food
    /// item and immediately make its way to the map's edge, where it will disappear with it. If the
    /// creature goes on rampages during worldgen, it will often steal food instead of attacking.
    /// Trained and tame instances of the creature will no longer display this behavior.
    #[serde(alias = "CURIOUSBEAST_EATER")]
    pub curiousbeast_eater: Option<()>,
    /// Allows a creature to (very quickly) drink your alcohol. Or spill the barrel to the ground.
    /// Also affects undead versions of the creature. Unlike food or item thieves, drink thieves
    /// will consume your alcohol on the spot rather than run away with one piece of it. Trained and
    /// tame instances of the creature will no longer display this behavior.
    #[serde(alias = "CURIOUSBEAST_GUZZLER")]
    pub curiousbeast_guzzler: Option<()>,
    /// Allows a creature to steal things (apparently, of the highest value it can find). It will
    /// attempt to grab an item of value and immediately make its way to the map's edge, where it
    /// will disappear with it. If the creature goes on rampages in worldgen, it will often steal
    /// items instead of attacking - kea birds are infamous for this. Trained and tame instances of
    /// the creature will no longer display this behavior. Also, makes the creature unable to drop
    /// hauled items until it enters combat.
    #[serde(alias = "CURIOUSBEAST_ITEM")]
    pub curiousbeast_item: Option<()>,
    /// Found on generated demons. Marks the caste to be used in the initial wave after breaking
    /// into the underworld, and by the demon civilizations created during world-gen breachings.
    /// Could not be specified in user-defined raws until version `0.47.01`.
    #[serde(alias = "DEMON")]
    pub demon: Option<()>,
    /// A brief description of the creature type, as displayed when viewing the creature's
    /// description/thoughts & preferences screen.
    #[serde(alias = "DESCRIPTION")]
    pub description: Option<String>,
    /// Dies upon attacking. Used by honey bees to simulate them dying after using their stingers.
    #[serde(alias = "DIE_WHEN_VERMIN_BITE")]
    pub die_when_vermin_bite: Option<()>,
    /// Increases experience gain during adventure mode. Creatures with 11 or higher are not
    /// assigned for quests in adventure mode.
    #[serde(alias = "DIFFICULTY")]
    pub difficulty: Option<u32>,
    /// Sets the creature to be active during the day in Adventurer Mode.
    #[serde(alias = "DIURNAL")]
    pub diurnal: Option<()>,
    /// The creature hunts vermin by diving from the air. On tame creatures it works the same as
    /// normal `[HUNTS_VERMIN]`. Found on peregrine falcons.
    #[serde(alias = "DIVE_HUNTS_VERMIN")]
    pub dive_hunts_vermin: Option<()>,
    /// Defines the material composition of eggs laid by the creature. Egg-laying creatures in the
    /// default game define this 3 times, using `LOCAL_CREATURE_MAT:EGGSHELL`,
    /// `LOCAL_CREATURE_MAT:EGG_WHITE`, and then `LOCAL_CREATURE_MAT:EGG_YOLK`. Eggs will be made out of
    /// eggshell. Edibility is determined by tags on whites or yolk, but they otherwise do not
    /// exist.
    #[serde(alias = "EGG_MATERIAL")]
    pub egg_material: Vec<(MaterialTokenArg, MaterialStateEnum)>,
    /// Determines the size of laid eggs. Doesn't affect hatching or cooking, but bigger eggs will
    /// be heavier, and may take longer to be hauled depending on the hauler's strength.
    #[serde(alias = "EGG_SIZE")]
    pub egg_size: Option<u32>,
    /// Allows the creature to wear or wield items.
    #[serde(alias = "EQUIPS")]
    pub equips: Option<()>,
    /// A list of `EXTRA_BUTCHER_OBJECT` tokens on this creature.
    #[serde(alias = "EXTRA_BUTCHER_OBJECT")]
    pub extra_butcher_object: Vec<ExtraButcherObject>,
    /// Defines a creature extract which can be obtained via small animal dissection.
    #[serde(alias = "EXTRACT")]
    pub extract: Option<MaterialTokenArg>,
    /// Creature can see regardless of whether it has working eyes and has full 360 degree vision,
    /// making it impossible to strike the creature from a blind spot in combat.
    #[serde(alias = "EXTRAVISION")]
    pub extravision: Option<()>,
    /// Found on subterranean animal-man tribals. Currently defunct. In previous versions, it caused
    /// these creatures to crawl out of chasms and underground rivers.
    #[serde(alias = "FEATURE_ATTACK_GROUP")]
    pub feature_attack_group: Option<()>,
    /// Found on forgotten beasts. Presumably makes it act as such, initiating underground attacks on
    /// fortresses, or leads to the pop-up message upon encountering one.
    ///
    /// Hides the creature from displaying in a world_sites_and_pops file, and does not create historical
    /// figures like generated forgotten beasts do.
    ///
    /// Requires specifying a `[BIOME]` in which the creature will live, and both surface and subterranean
    /// biomes are allowed. Does not stack with `[LARGE_ROAMING]` and if both are used the creature will
    /// not spawn. Appears to be incompatible with `[DEMON]` even if used in separate castes.
    ///
    /// Could not be specified in user-defined raws until version `0.47.01`.
    #[serde(alias = "FEATURE_BEAST")]
    pub feature_beast: Option<()>,
    /// Makes the creature biologically female, enabling her to bear young. Usually specified inside
    /// a caste.
    #[serde(alias = "FEMALE")]
    pub female: Option<()>,
    /// Makes the creature immune to `FIREBALL` and `FIREJET` attacks, and allows it to path through
    /// high temperature zones, like lava or fires. Does not, by itself, make the creature immune to
    /// the damaging effects of burning in fire, and does not prevent general heat damage or melting
    /// on its own (this would require adjustments to be made to the creature's body materials - see
    /// the dragon raws for an example).
    #[serde(alias = "FIREIMMUNE")]
    pub fireimmune: Option<()>,
    /// Like `[FIREIMMUNE]`, but also renders the creature immune to `DRAGONFIRE` attacks.
    #[serde(alias = "FIREIMMUNE_SUPER")]
    pub fireimmune_super: Option<()>,
    /// The creature's corpse is a single `FISH_RAW` food item that needs to be cleaned (into a
    /// `FISH` item) at a fishery to become edible. Before being cleaned the item is referred to as
    /// "raw". The food item is categorized under "fish" on the food and stocks screens, and when
    /// uncleaned it is sorted under "raw fish" in the stocks (but does not show up on the food
    /// screen).
    ///
    /// Without this or `[COOKABLE_LIVE]`, fished vermin will turn into food the same way as non-
    /// vermin creatures, resulting in multiple units of food (meat, brain, lungs, eyes, spleen
    /// etc.) from a single fished vermin. These units of food are categorized as meat by the game.
    #[serde(alias = "FISHITEM")]
    pub fishitem: Option<()>,
    /// The creature's body is constantly at this temperature, heating up or cooling the surrounding
    /// area. Alters the temperature of the creature's inventory and all adjacent tiles, with all
    /// the effects that this implies. May trigger wildfires at high enough values. Also makes the
    /// creature immune to extreme heat or cold as long as the temperature set is not harmful to the
    /// materials that the creature is made from.
    ///
    /// Note that temperatures of 12000 and higher may cause pathfinding issues in fortress mode.
    #[serde(alias = "FIXED_TEMP")]
    pub fixed_temp: Option<u32>,
    /// If engaged in combat, the creature will flee at the first sign of resistance. Used by
    /// kobolds in the vanilla game.
    #[serde(alias = "FLEEQUICK")]
    pub fleequick: Option<()>,
    /// Allows a creature to fly, independent of it having wings or not. Fortress Mode pathfinding
    /// only partially incorporates flying - flying creatures need a land path to exist between them
    /// and an area in order to access it, but as long as one such path exists, they do not need to
    /// use it, instead being able to fly over intervening obstacles. Winged creatures with this
    /// token can lose their ability to fly if their wings are crippled or severed. Winged creatures
    /// without this token will be unable to fly. (A 'wing' in this context refers to any body part
    /// with its own `FLIER` token).
    #[serde(alias = "FLIER")]
    pub flier: Option<()>,
    /// Defines a gait by which the creature can move.
    /// See [Gait](https://dwarffortresswiki.org/index.php/Gait) for more information.
    ///
    /// - `<max speed>` indicates the maximum speed achievable by a creature using this gait.
    ///
    /// - `<build up time>` indicates how long it will take for a creature using this gait to go from
    /// `<start speed>` to `<max speed>`. For example, a value of 10 means that it should be able to
    /// reach the maximum speed by moving 10 tiles in a straight line over even terrain.
    ///
    /// - `<max turning speed>` indicates the maximum speed permissible when the creature suddenly
    /// changes its direction of motion. The creature's speed will be reduced to `<max turning speed>`
    /// if travelling at a higher speed than this before turning.
    ///
    /// - `<start speed>` indicates the creature's speed when it starts moving using this gait.
    ///
    /// - `<energy expenditure>` indicates how energy-consuming the gait is. Higher values cause the
    /// creature to tire out faster. Persistent usage of a high-intensity gait will eventually lead
    /// to exhaustion and collapse.
    ///
    /// You may use `NO_BUILD_UP` instead of `<build up time>`, `<max turning speed>` and `<start speed>`.
    ///
    /// It's possible to specify a `<start speed>` greater than the `<max speed>`; the moving creature
    /// will decelerate towards its `<max speed>` in this case.
    #[serde(alias = "GAIT")]
    pub gait: Vec<(
        GaitTypeEnum,
        String,
        u32,
        Choose<NoBuildUpEnum, (u32, u32, u32)>,
        u32,
        Option<GaitFlagTokenArg>,
    )>,
    /// Has the same function as `[MATERIAL_FORCE_MULTIPLIER]`, but applies to all attacks instead
    /// of just those involving a specific material. Appears to be overridden by
    /// `MATERIAL_FORCE_MULTIPLIER` (werebeasts, for example, use both tokens to provide resistance
    /// to all materials, with one exception to which they are especially vulnerable).
    #[serde(alias = "GENERAL_MATERIAL_FORCE_MULTIPLIER")]
    pub general_material_force_multiplier: Option<(u32, u32)>,
    /// Makes the creature get infections from necrotic tissue.
    #[serde(alias = "GETS_INFECTIONS_FROM_ROT")]
    pub gets_infections_from_rot: Option<()>,
    /// Makes the creature's wounds become infected if left untreated for too long.
    #[serde(alias = "GETS_WOUND_INFECTIONS")]
    pub gets_wound_infections: Option<()>,
    /// The creature can and will gnaw its way out of animal traps and cages using the specified
    /// verb, depending on the material from which it is made (normally wood).
    #[serde(alias = "GNAWER")]
    pub gnawer: Option<String>,
    /// The creature eats vermin of the specified class.
    #[serde(alias = "GOBBLE_VERMIN_CLASS")]
    pub gobble_vermin_class: Vec<Reference>, // TODO: ref is creature class
    /// The creature eats a specified vermin.
    #[serde(alias = "GOBBLE_VERMIN_CREATURE")]
    pub gobble_vermin_creature: Vec<(ReferenceTo<CreatureToken>, Reference)>,
    /// The value determines how rapidly grass is trampled when a creature steps on it - a value of
    /// 0 causes the creature to never damage grass, while a value of 100 causes grass to be
    /// trampled as rapidly as possible. Defaults to 5.
    #[serde(alias = "GRASSTRAMPLE")]
    pub grasstrample: Option<u32>,
    /// Used in Creature Variants. This token changes the adult body size to the average of the old
    /// adult body size and the target value and scales all intermediate growth stages by the same
    /// factor.
    #[serde(alias = "GRAVITATE_BODY_SIZE")]
    pub gravitate_body_size: Option<u32>,
    /// The creature is a grazer. If tamed in Fortress mode, it needs a pasture to survive. The
    /// higher the number, the less frequently it needs to eat in order to live. See
    /// [Pasture](https://dwarffortresswiki.org/index.php/Pasture) for details on its issues.
    #[serde(alias = "GRAZER")]
    pub grazer: Option<u32>,
    /// Defines certain behaviors for the creature.
    ///
    /// These habits require the creature to have a `[LAIR]` to work properly, and also don't seem
    /// to work on creatures who are not a `[SEMIMEGABEAST]`, `[MEGABEAST]` or
    /// `[NIGHT_CREATURE_HUNTER]`.
    #[serde(alias = "HABIT")]
    pub habit: Vec<(HabitTypeEnum, u32)>,
    /// "If you set `HABIT_NUM` to a number, it should give you that exact number of habits
    /// according to the weights" All lists of `HABIT`s are preceded by `[HABIT_NUM:TEST_ALL]`.
    #[serde(alias = "HABIT_NUM")]
    pub habit_num: Option<Choose<u32, TestAllEnum>>,
    /// The creature has nerves in its muscles. Cutting the muscle tissue can sever motor and
    /// sensory nerves.
    #[serde(alias = "HAS_NERVES")]
    pub has_nerves: Option<()>,
    /// The creature has a shell. Seemingly no longer used - holdover from previous versions.
    #[serde(alias = "HASSHELL")]
    pub hasshell: Option<()>,
    /// Default 'NONE'. The creature's normal body temperature. Creature ceases maintaining
    /// temperature on death unlike fixed material temperatures. Provides minor protection from
    /// environmental temperature to the creature.
    #[serde(alias = "HOMEOTHERM")]
    pub homeotherm: Option<Choose<u32, NoneEnum>>,
    /// Creature hunts and kills nearby vermin.
    #[serde(alias = "HUNTS_VERMIN")]
    pub hunts_vermin: Option<()>,
    /// The creature cannot move. Found on sponges. Will also stop a creature from breeding in
    /// fortress mode (MALE and `FEMALE` are affected, if one is `IMMOBILE` no breeding will
    /// happen).
    #[serde(alias = "IMMOBILE")]
    pub immobile: Option<()>,
    /// The creature is immobile while on land. Only works on `[AQUATIC]` creatures which can't
    /// breathe on land.
    #[serde(alias = "IMMOBILE_LAND")]
    pub immobile_land: Option<()>,
    /// The creature radiates fire. It will ignite, and potentially completely destroy, items the
    /// creature is standing on. Keep booze away from critters with this tag. Also gives the vermin
    /// a high chance of escaping from animal traps and cages made of certain materials.
    #[serde(alias = "IMMOLATE")]
    pub immolate: Option<()>,
    /// Alias for `[CAN_SPEAK]` + `[CAN_LEARN]` but additionally keeps creatures from being
    /// butchered by the AI during worldgen and post-gen. In fortress mode, `[CAN_LEARN]` is
    /// enough.
    #[serde(alias = "INTELLIGENT")]
    pub intelligent: Option<()>,
    /// Determines if the creature leaves behind a non-standard corpse (i.e. wood, statue, bars,
    /// stone, pool of liquid, etc.).
    #[serde(alias = "ITEMCORPSE")]
    pub itemcorpse: Option<(ItemReferenceArg, MaterialTokenArg)>,
    /// The quality of an item-type corpse left behind. Valid values are:
    /// - 0: ordinary
    /// - 1: well-crafted
    /// - 2: finely-crafted
    /// - 3: superior
    /// - 4: exceptional
    /// - 5: masterpiece.
    #[serde(alias = "ITEMCORPSE_QUALITY")]
    pub itemcorpse_quality: Option<u8>, // TODO maybe nest under itemcorpse?
    /// Found on megabeasts, semimegabeasts, and night creatures. The creature will seek out sites
    /// of this type and take them as lairs.
    #[serde(alias = "LAIR")]
    pub lair: Vec<(LairTypeEnum, Clamp<u8, 0, 100>)>,
    /// Defines certain features of the creature's lair. The only valid characteristic is
    /// `HAS_DOORS`.
    #[serde(alias = "LAIR_CHARACTERISTIC")]
    pub lair_characteristic: Option<(LairCharacteristicEnum, Clamp<u8, 0, 100>)>,
    /// This creature will actively hunt adventurers in its lair.
    #[serde(alias = "LAIR_HUNTER")]
    pub lair_hunter: Option<()>,
    /// What this creature says while hunting adventurers in its lair. Requires a `.txt` on the end.
    #[serde(alias = "LAIR_HUNTER_SPEECH")]
    pub lair_hunter_speech: Option<String>, // TODO: string is a txt path
    /// Will attack things that are smaller than it (like dwarves). Only one group of "large
    /// predators" (possibly two groups on "savage" maps) will appear on any given map. In adventure
    /// mode, large predators will try to ambush and attack you (and your party will attack them
    /// back).
    ///
    /// When tamed, large predators tend to be much more aggressive to enemies than non-large
    /// predators, making them a good choice for an animal army.
    ///
    /// They may go on rampages in worldgen, and adventurers may receive quests to kill them.
    /// Also, they can be mentioned in the intro paragraph when starting a fortress e.g.
    /// "ere the wolves get hungry."
    #[serde(alias = "LARGE_PREDATOR")]
    pub large_predator: Option<()>,
    /// Creature lays eggs instead of giving birth to live young.
    #[serde(alias = "LAYS_EGGS")]
    pub lays_eggs: Option<()>,
    /// Creature lays the specified item instead of regular eggs.
    #[serde(alias = "LAYS_UNUSUAL_EGGS")]
    pub lays_unusual_eggs: Option<(ItemReferenceArg, MaterialTokenArg)>,
    /// The creature has ligaments in its `[CONNECTIVE_TISSUE_ANCHOR]` tissues (bone or chitin by
    /// default). Cutting the bone/chitin tissue severs the ligaments, disabling motor function if
    /// the target is a limb.
    #[serde(alias = "LIGAMENTS")]
    pub ligaments: Option<(MaterialTokenArg, u32)>,
    /// The creature will generate light, such as in adventurer mode at night.
    #[serde(alias = "LIGHT_GEN")]
    pub light_gen: Option<()>,
    /// The creature will attack enemies rather than flee from them. This tag has the same effect on
    /// player-controlled creatures - including modded dwarves. Retired as of version `0.40.14`
    /// in favor of `[LARGE_PREDATOR]`.
    #[serde(alias = "LIKES_FIGHTING")]
    pub likes_fighting: Option<()>,
    /// Creature uses "sssssnake talk" (multiplies 'S' when talking - "My name isss Recisssiz.").
    /// Used by serpent men and reptile men in the vanilla game. C's with the same pronunciation
    /// (depending on the word) are not affected by this token.
    #[serde(alias = "LISP")]
    pub lisp: Option<()>,
    /// Determines the number of offspring per one birth.
    #[serde(alias = "LITTERSIZE")]
    pub littersize: Option<(u32, u32)>,
    /// Wild animals of this species may occasionally join a civilization. Prevents trading of
    /// (tame) instances of this creature in caravans.
    #[serde(alias = "LOCAL_POPS_PRODUCE_HEROES")]
    pub local_pops_produce_heroes: Option<()>,
    /// Lets a creature open doors that are set to forbidden in Fortress Mode.
    #[serde(alias = "LOCKPICKER")]
    pub lockpicker: Option<()>,
    /// Determines how well a creature can see in the dark - higher is better. Dwarves have 10,000,
    /// which amounts to perfect nightvision.
    #[serde(alias = "LOW_LIGHT_VISION")]
    pub low_light_vision: Option<Clamp<u16, 0, 10_000>>, // TODO: find out if it can be negative
    /// No function, presumably a placeholder.
    #[serde(alias = "MAGICAL")]
    pub magical: Option<()>,
    /// The creature is able to see while submerged in magma.
    #[serde(alias = "MAGMA_VISION")]
    pub magma_vision: Option<()>,
    /// Makes the creature biologically male. Usually declared inside a caste.
    #[serde(alias = "MALE")]
    pub male: Option<()>,
    // region: Mannerisms =========================================================================
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_ARMS")]
    pub mannerism_arms: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_BREATH")]
    pub mannerism_breath: Option<()>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_CHEEK")]
    pub mannerism_cheek: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_EAR")]
    pub mannerism_ear: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_EYELIDS")]
    pub mannerism_eyelids: Option<()>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_EYES")]
    pub mannerism_eyes: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_FEET")]
    pub mannerism_feet: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_FINGERS")]
    pub mannerism_fingers: Option<(String, String)>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_HAIR")]
    pub mannerism_hair: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_HANDS")]
    pub mannerism_hands: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_HEAD")]
    pub mannerism_head: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_KNUCKLES")]
    pub mannerism_knuckles: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_LAUGH")]
    pub mannerism_laugh: Option<()>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_LEG")]
    pub mannerism_leg: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_LIPS")]
    pub mannerism_lips: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_MOUTH")]
    pub mannerism_mouth: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_NAILS")]
    pub mannerism_nails: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_NOSE")]
    pub mannerism_nose: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_POSTURE")]
    pub mannerism_posture: Option<()>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_SIT")]
    pub mannerism_sit: Option<()>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_SMILE")]
    pub mannerism_smile: Option<()>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_STRETCH")]
    pub mannerism_stretch: Option<()>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_TONGUE")]
    pub mannerism_tongue: Option<String>,
    /// Adds a possible mannerism to the creature's profile.
    ///
    /// Mannerisms give a possibility of a personality quirk being included in each member of a
    /// caste.
    ///
    /// Mannerisms are hardcoded and not otherwise placed in the raws.
    ///
    /// Words placed with the mannerism token are the strings used in the mannerisms for that
    /// particular part (e.g. replacing "finger" with "toe" will give you "when she gets
    /// exasperated, she often points and shakes her toe").
    #[serde(alias = "MANNERISM_WALK")]
    pub mannerism_walk: Option<()>,
    // endregion ==================================================================================
    /// When struck with a weapon made of the specified material, the force exerted will be
    /// multiplied by A/B, thus making the creature more or less susceptible to this material.
    /// For example, if A is 2 and B is 1, the force exerted by the defined material will be
    /// doubled. If A is 1 and B is 2, it will be halved instead.
    ///
    /// See also `[GENERAL_MATERIAL_FORCE_MULTIPLIER]`, which can be used to make this sort of
    /// effect applicable to all materials.
    #[serde(alias = "MATERIAL_FORCE_MULTIPLIER")]
    pub material_force_multiplier: Vec<(MaterialTokenArg, u32, u32)>,
    /// Sets the creature to be active at dawn in adventurer mode.
    #[serde(alias = "MATUTINAL")]
    pub matutinal: Option<()>,
    /// Determines the creature's natural lifespan, using the specified minimum and maximum age
    /// values (in years). Each individual creature with this token is generated with a
    /// predetermined date (calculated down to the exact tick!) between these values, at which it is
    /// destined to die of old age, should it live long enough.
    ///
    /// Note that the probability of death at any given age does not increase as the creature gets
    /// older ([source](https://i.imgur.com/A1A4aA9.png)).
    ///
    /// Creatures which lack this token are naturally immortal. The `NO_AGING` syndrome tag will
    /// prevent death by old age from occurring. Also note that, among civilized creatures, castes
    /// which lack this token will refuse to marry others with it, and vice versa.
    #[serde(alias = "MAXAGE")]
    pub maxage: Option<(u32, u32)>,
    /// Makes the creature slowly stroll around, unless it's in combat or performing a job. If
    /// combined with `[CAN_LEARN]`, will severely impact their pathfinding and lead the creature to
    /// move extremely slowly when not performing any task.
    #[serde(alias = "MEANDERER")]
    pub meanderer: Option<()>,
    /// A 'boss' creature. A small number of the creatures are created during worldgen, their
    /// histories and descendants (if any) will be tracked in worldgen (as opposed to simply
    /// 'spawning'), and they will occasionally go on rampages, potentially leading to worship if
    /// they attack the same place multiple times. Their presence and number will also influence age
    /// names. When appearing in fortress mode, they will have a pop-up message announcing their
    /// arrival. They will remain hostile to military even after being tamed.
    /// [Bug:10731](https://www.bay12games.com/dwarves/mantisbt/view.php?id=10731)
    ///
    /// See the wiki [megabeast](https://dwarffortresswiki.org/index.php/Megabeast) page for more details.
    ///
    /// Requires specifying a `[BIOME]` in which the creature will live. Subterranean biomes appear
    /// to not be allowed.
    #[serde(alias = "MEGABEAST")]
    pub megabeast: Option<()>,
    /// This means you can increase your attribute to a given percentage of its starting value
    /// (or the average value + your starting value if that is higher).
    ///
    /// Default is 200, which would increase the attribute by either 200%, or by a flat +200
    /// if that is higher.
    #[serde(alias = "MENT_ATT_CAP_PERC")]
    pub ment_att_cap_perc: Vec<(SoulAttributeEnum, u32)>,
    /// Sets up a mental attribute's range of values (0-5000). All mental attribute ranges default
    /// to `200:800:900:1000:1100:1300:2000`.
    #[serde(alias = "MENT_ATT_RANGE")]
    pub ment_att_range: Vec<(SoulAttributeEnum, u32, u32, u32, u32, u32, u32, u32)>,
    /// Mental attribute gain/decay rates. Lower numbers in the last three slots make decay occur
    /// faster. Defaults are `500:4:5:4`.
    #[serde(alias = "MENT_ATT_RATES")]
    pub ment_att_rates: Vec<(
        SoulAttributeEnum,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
    )>,
    /// Allows the creature to be milked in the farmer's workshop. The frequency is the amount of
    /// ticks the creature needs to "recharge" (i.e. how much time needs to pass before it can be
    /// milked again). Does not work on sentient creatures, regardless of ethics.
    #[serde(alias = "MILKABLE")]
    pub milkable: Option<(MaterialTokenArg, u32)>,
    /// The creature spawns stealthed and will attempt to path into the fortress, pulling any levers
    /// it comes across. It will be invisible on the map and unit list until spotted by a citizen,
    /// at which point the game will pause and recenter on the creature. Used by gremlins in the
    /// vanilla game. "They go on little missions to mess with various fortress buildings, not just
    /// levers."
    #[serde(alias = "MISCHIEVOUS", alias = "MISCHIEVIOUS")]
    pub mischievous: Option<()>,
    /// Seemingly no longer used.
    #[serde(alias = "MODVALUE")]
    pub modvalue: Option<()>,
    /// Creature may be used as a mount. No use for the player in fortress mode, but enemy sieging
    /// forces may arrive with cavalry. Mounts are usable in adventure mode.
    #[serde(alias = "MOUNT")]
    pub mount: Option<()>,
    /// Creature may be used as a mount, but civilizations cannot domesticate it in worldgen without
    /// certain exceptions.
    #[serde(alias = "MOUNT_EXOTIC")]
    pub mount_exotic: Option<()>,
    /// Allows the creature to have all-around vision as long as it has multiple heads that can see.
    #[serde(alias = "MULTIPART_FULL_VISION")]
    pub multipart_full_vision: Option<()>,
    /// Makes the species usually produce a single offspring per birth, occasionally producing twins
    /// or triplets using typical real-world human probabilities. Requires `[FEMALE]`.
    #[serde(alias = "MULTIPLE_LITTER_RARE")]
    pub multiple_litter_rare: Option<()>,
    /// Animal is considered to be natural. `NATURAL` animals will not engage creatures tagged with
    /// `[AT_PEACE_WITH_WILDLIFE]` in combat unless they are members of a hostile entity and vice-
    /// versa.
    #[serde(alias = "NATURAL", alias = "NATURAL_ANIMAL")]
    pub natural: Option<()>,
    /// The creature possesses the specified skill at this level inherently - that is, it begins
    /// with the skill at this level, and the skill may never rust below that. A value of 15 is
    /// legendary.
    #[serde(alias = "NATURAL_SKILL")]
    pub natural_skill: Vec<(SkillEnum, u32)>,
    /// Creatures with this token can appear in bogeyman ambushes in adventure mode, where they
    /// adopt classical bogeyman traits such as stalking the adventurer and vaporising when dawn
    /// breaks. Such traits do not manifest if the creature is encountered outside of a bogeyman
    /// ambush (for instance, as a megabeast or a civilised being). In addition, their corpses and
    /// severed body parts turn into smoke after a short while. Note that setting the "Number of
    /// Bogeyman Types" in advanced world generation to 0 will only remove randomly-generated
    /// bogeymen.
    #[serde(alias = "NIGHT_CREATURE_BOGEYMAN")]
    pub night_creature_bogeyman: Option<()>,
    /// Found on some necromancers. Creatures with this tag will experiment and create strange
    /// hybrid creatures. This tag appears to be all that is needed to allow necromancers or modded
    /// secrets to create every category of procedural experimental creature
    /// ([Though more testing is required to fully confirm this](http://www.bay12forums.com/smf/index.php?topic=175437.msg8085255#msg8085255)).
    #[serde(alias = "NIGHT_CREATURE_EXPERIMENTER")]
    pub night_creature_experimenter: Option<()>,
    /// Found on night trolls and werebeasts. Implies that the creature is a night creature, and
    /// shows its description in legends mode entry. The creature is always hostile and will start
    /// no quarter combat with any nearby creatures, except for members of its own race. Note that
    /// this tag does not override the creature's normal behavior in fortress mode except for the
    /// aforementioned aggression, and doesn't prevent the creature from fleeing the battles it
    /// started. It also removes the creature's materials from stockpile settings list, making them
    /// be stored there regardless of settings.
    ///
    /// This tag causes the usual behaviour of werebeasts in worldgen, that is, fleeing towns upon
    /// being cursed and conducting raids from a lair. If this tag is absent from a deity curse, the
    /// accursed will simply be driven out of towns in a similar manner to vampires. When paired
    /// with `SPOUSE_CONVERTER`, a very small population of the creature will be created during
    /// worldgen (sometimes only a single individual will be created), and their histories will be
    /// tracked (that is, they will not spawn spontaneously later, they must either have children or
    /// convert other creatures to increase their numbers). The creature will settle in a lair and
    /// go on rampages during worldgen. It will actively attempt to seek out potential conversion
    /// targets to abduct, convert, and have children with (if possible).
    #[serde(alias = "NIGHT_CREATURE_HUNTER")]
    pub night_creature_hunter: Option<()>,
    /// Found on nightmares. Corpses and severed body parts derived from creatures with this token
    /// turn into smoke after a short while.
    #[serde(alias = "NIGHT_CREATURE_NIGHTMARE")]
    pub night_creature_nightmare: Option<()>,
    /// The creature caste does not appear in autumn.
    #[serde(alias = "NO_AUTUMN")]
    pub no_autumn: Option<()>,
    /// Creature doesn't require connected body parts to move (presumably); generally used on undead
    /// creatures with connections that have rotted away.
    #[serde(alias = "NO_CONNECTIONS_FOR_MOVEMENT")]
    pub no_connections_for_movement: Option<()>,
    /// Creature cannot become dizzy.
    #[serde(alias = "NO_DIZZINESS")]
    pub no_dizziness: Option<()>,
    /// Creature does not need to drink.
    #[serde(alias = "NO_DRINK")]
    pub no_drink: Option<()>,
    /// Creature does not need to eat.
    #[serde(alias = "NO_EAT")]
    pub no_eat: Option<()>,
    /// Creature cannot suffer fevers.
    #[serde(alias = "NO_FEVERS")]
    pub no_fevers: Option<()>,
    /// The creature is biologically sexless. Makes the creature unable to breed.
    #[serde(alias = "NO_GENDER")]
    pub no_gender: Option<()>,
    /// The creature cannot raise any physical attributes.
    #[serde(alias = "NO_PHYS_ATT_GAIN")]
    pub no_phys_att_gain: Option<()>,
    /// The creature cannot lose any physical attributes.
    #[serde(alias = "NO_PHYS_ATT_RUST")]
    pub no_phys_att_rust: Option<()>,
    /// Creature does not need to sleep. Can still be rendered unconscious by other means.
    #[serde(alias = "NO_SLEEP")]
    pub no_sleep: Option<()>,
    /// The creature caste does not appear in spring.
    #[serde(alias = "NO_SPRING")]
    pub no_spring: Option<()>,
    /// The creature caste does not appear in summer.
    #[serde(alias = "NO_SUMMER")]
    pub no_summer: Option<()>,
    /// Creature doesn't require an organ with the `[THOUGHT]` tag to survive or attack; generally
    /// used on creatures that don't have brains.
    #[serde(alias = "NO_THOUGHT_CENTER_FOR_MOVEMENT")]
    pub no_thought_center_for_movement: Option<()>,
    /// Prevents creature from selecting its color based on its profession (e.g. Miner, Hunter,
    /// Wrestler).
    #[serde(alias = "NO_UNIT_TYPE_COLOR")]
    pub no_unit_type_color: Option<()>,
    /// Likely prevents the creature from leaving broken vegetation tracks (unverified).
    #[serde(alias = "NO_VEGETATION_PERTURB")]
    pub no_vegetation_perturb: Option<()>,
    /// The creature caste does not appear in winter.
    #[serde(alias = "NO_WINTER")]
    pub no_winter: Option<()>,
    /// Creature has no bones.
    #[serde(alias = "NOBONES")]
    pub nobones: Option<()>,
    /// Creature doesn't need to breathe or have `[BREATHE]` parts in body, nor can it drown or be
    /// strangled. Creatures living in magma must have this tag, otherwise they will drown.
    #[serde(alias = "NOBREATHE")]
    pub nobreathe: Option<()>,
    /// Sets the creature to be active at night in adventure mode.
    #[serde(alias = "NOCTURNAL")]
    pub nocturnal: Option<()>,
    /// Creature has no emotions. It is immune to the effects of stress and unable to rage, and its
    /// needs cannot be fulfilled in any way. Used on undead in the vanilla game.
    #[serde(alias = "NOEMOTION")]
    pub noemotion: Option<()>,
    /// Creature can't become tired or over-exerted from taking too many combat actions or moving at
    /// full speed for extended periods of time.
    #[serde(alias = "NOEXERT")]
    pub noexert: Option<()>,
    /// Creature doesn't feel fear and will never flee from battle. Additionally, it causes bogeymen
    /// and nightmares to become friendly towards the creature.
    #[serde(alias = "NOFEAR")]
    pub nofear: Option<()>,
    /// Creature will not drop meat when butchered.
    #[serde(alias = "NOMEAT")]
    pub nomeat: Option<()>,
    /// Creature isn't nauseated by gut hits and cannot vomit.
    #[serde(alias = "NONAUSEA")]
    pub nonausea: Option<()>,
    /// Creature doesn't feel pain.
    #[serde(alias = "NOPAIN")]
    pub nopain: Option<()>,
    /// Creature will not drop a hide when butchered.
    #[serde(alias = "NOSKIN")]
    pub noskin: Option<()>,
    /// Creature will not drop a skull on butchering, rot, or decay of severed head.
    #[serde(alias = "NOSKULL")]
    pub noskull: Option<()>,
    /// Does not produce miasma when rotting.
    #[serde(alias = "NOSMELLYROT")]
    pub nosmellyrot: Option<()>,
    /// Weapons can't get stuck in the creature.
    #[serde(alias = "NOSTUCKINS")]
    pub nostuckins: Option<()>,
    /// Creature can't be stunned and knocked unconscious by pain or head injuries. Creatures with
    /// this tag never wake up from sleep in Fortress Mode. If this creature needs to sleep while
    /// playing, it will die.
    #[serde(alias = "NOSTUN")]
    pub nostun: Option<()>,
    /// Cannot be butchered.
    #[serde(alias = "NOT_BUTCHERABLE")]
    pub not_butcherable: Option<()>,
    /// Cannot be raised from the dead by necromancers or evil clouds. Implies the creature is not a
    /// normal living being. Used by vampires, mummies and inorganic creatures like the amethyst man
    /// and bronze colossus. Creatures who are `[OPPOSED_TO_LIFE]` (undead) will be docile towards
    /// creatures with this token.
    #[serde(alias = "NOT_LIVING")]
    pub not_living: Option<()>,
    /// Creature doesn't require a `[THOUGHT]` body part to survive. Has the added effect of
    /// preventing speech, though directly controlling creatures that would otherwise be capable of
    /// speaking allows them to engage in conversation.
    #[serde(alias = "NOTHOUGHT")]
    pub nothought: Option<()>,
    /// How easy the creature is to smell. The higher the number, the easier the creature can be
    /// smelt. Zero is odorless. Default is 50.
    #[serde(alias = "ODOR_LEVEL")]
    pub odor_level: Option<u32>,
    /// What the creature smells like. If no odor string is defined, the creature name (not the
    /// caste name) is used.
    #[serde(alias = "ODOR_STRING")]
    pub odor_string: Option<String>,
    /// Is hostile to all creatures except undead and other non-living ones and will show Opposed to
    /// life in the unit list. Used by undead in the vanilla game. Functions without the
    /// `[NOT_LIVING]` token, and seems to imply said token as well. Undead will not be hostile to
    /// otherwise-living creatures given this token. Living creatures given this token will attack
    /// living creatures that lack it, while ignoring other living creatures that also have this
    /// token.
    #[serde(alias = "OPPOSED_TO_LIFE")]
    pub opposed_to_life: Option<()>,
    /// Determines caste's likelihood of having sexual attraction to certain sexes. Values default
    /// to `75:20:5` for the same sex and `5:20:75` for the opposite sex. The first value indicates how
    /// likely to be entirely uninterested in the sex, the second decides if the creature will
    /// become lovers but not marry, the third decides if the creature will marry this sex. The
    /// values themselves are simply ratios and do not need to add up to any particular value.
    #[serde(alias = "ORIENTATION")]
    pub orientation: Vec<(MaleOrFemaleEnum, u32, u32, u32)>,
    /// Lets you play as an outsider of this species in adventure mode.
    #[serde(alias = "OUTSIDER_CONTROLLABLE")]
    pub outsider_controllable: Option<()>,
    /// Allows the creature to be used as a pack animal. Currently only used by merchants without
    /// wagons. Also prevents creature from dropping hauled items on its own -- do not use for
    /// player-controllable creatures!
    #[serde(alias = "PACK_ANIMAL")]
    pub pack_animal: Option<()>,
    /// The creature is immune to all paralyzing special attacks.
    #[serde(alias = "PARALYZEIMMUNE")]
    pub paralyzeimmune: Option<()>,
    /// Used to control the bat riders with paralyze-dart blowguns that flew through the `2D` chasm.
    /// Doesn't do anything now.
    #[serde(alias = "PATTERNFLIER")]
    pub patternflier: Option<()>,
    /// In earlier versions, creature would generate pearls. Apparently does nothing in the current
    /// version.
    #[serde(alias = "PEARL")]
    pub pearl: Option<()>,
    /// Controls the ability of vermin to find a way into containers when they are eating food from
    /// your stockpiles. The value for vermin in the game's current version ranges from 1-3. A
    /// higher value is better at penetrating containers.
    #[serde(alias = "PENETRATEPOWER")]
    pub penetratepower: Option<u32>,
    /// Determines the range and chance of personality traits. Standard is `0:50:100`.
    ///
    /// See [Personality trait](https://dwarffortresswiki.org/index.php/Personality_trait)
    /// for more info.
    #[serde(alias = "PERSONALITY")]
    pub personality: Vec<(PersonalityTraitEnum, u8, u8, u8)>,
    /// Allows the creature to be tamed in Fortress mode. Prerequisite for all other working animal
    /// roles. Civilizations that encounter it in worldgen will tame and domesticate it for their
    /// own use. Adding this to civilization members will classify them as pets instead of citizens,
    /// with all the problems that entails. However, you can solve these problems using the popular
    /// plugin Dwarf Therapist, which is completely unaffected by the tag.
    #[serde(alias = "PET")]
    pub pet: Option<()>,
    /// Allows the creature to be tamed in Fortress mode. Prequisite for all other working animal
    /// roles. Civilizations cannot domesticate it in worldgen, with certain exceptions.
    /// Adding this to civilization members will classify them as pets instead of citizens, with
    /// all the problems that entails. ([Example](https://dwarffortresswiki.org/index.php/Gremlin)).
    ///
    /// May make them more difficult to tame, but this needs verification.
    #[serde(alias = "PET_EXOTIC")]
    pub pet_exotic: Option<()>,
    /// How valuable a tamed animal is. Actual cost in points in the embarking screen is
    /// `1+(PETVALUE/2)` for an untrained animal, `1+PETVALUE` for a war/hunting one.
    #[serde(alias = "PETVALUE")]
    pub petvalue: Option<u32>,
    /// Divides the creature's `[PETVALUE]` by the specified number. Used by honey bees to prevent a
    /// single hive from being worth a fortune.
    #[serde(alias = "PETVALUE_DIVISOR")]
    pub petvalue_divisor: Option<u32>,
    /// This means you can increase your attribute to a given percentage of its starting value
    /// (or the average value + your starting value if that is higher).
    ///
    /// Default is 200, which would increase the attribute by either 200%, or by a flat +200
    /// if that is higher.
    #[serde(alias = "PHYS_ATT_CAP_PERC")]
    pub phys_att_cap_perc: Vec<(BodyAttributeEnum, u32)>,
    /// Sets up a physical attribute's range of values (0-5000). All physical attribute ranges
    /// default to `200:700:900:1000:1100:1300:2000`.
    #[serde(alias = "PHYS_ATT_RANGE")]
    pub phys_att_range: Vec<(BodyAttributeEnum, u32, u32, u32, u32, u32, u32, u32)>,
    /// Physical attribute gain/decay rates. Lower numbers in the last three slots make decay occur
    /// faster. Defaults for `STRENGTH`, `AGILITY`, `TOUGHNESS`, and `ENDURANCE` are `500:3:4:3`,
    /// while `RECUPERATION` and `DISEASE_RESISTANCE` default to `500:NONE:NONE:NONE`.
    #[serde(alias = "PHYS_ATT_RATES")]
    pub phys_att_rates: Vec<(
        BodyAttributeEnum,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
    )>,

    /// Weighted population of caste; Lower is rarer. Not to be confused with `[FREQUENCY]`.
    #[serde(alias = "POP_RATIO")]
    pub pop_ratio: Option<u32>,
    /// The minimum/maximum numbers of how many of these creatures are present in each world map
    /// tile of the appropriate region. Defaults to 1:1 if not specified.
    #[serde(alias = "POPULATION_NUMBER")]
    pub population_number: Option<(u32, u32)>,
    /// Allows the being to represent itself as a deity, allowing it to become the leader of a
    /// civilized group. Used by unique demons in the vanilla game. Requires `[CAN_SPEAK]` to
    /// actually do anything more than settle at a location (e.g. write books, lead armies, profane
    /// temples). Doesn't appear to do anything for creatures that are already civilized. Once the
    /// creature ascends to a position of leadership, it will proceed to act as a standard ruler for
    /// their entity and fulfill the same functions (hold tournaments, tame creatures, etc).
    #[serde(alias = "POWER")]
    pub power: Option<()>,
    /// Creature has a percentage chance to flip out at visible non-friendly creatures. Enraged
    /// creatures attack anything regardless of timidity and get a strength bonus to their hits.
    /// This is what makes badgers so hardcore.
    #[serde(alias = "PRONE_TO_RAGE")]
    pub prone_to_rage: Option<Clamp<u8, 0, 100>>,
    /// The creature has pus. Specifies the stuff secreted by infected wounds.
    #[serde(alias = "PUS")]
    pub pus: Option<(MaterialTokenArg, MaterialStateEnum)>,
    /// Specifies a new relative size for a part than what is stated in the body plan. For example,
    /// dwarves have larger livers.
    #[serde(alias = "RELSIZE")]
    pub relsize: Vec<(BpCriteriaTokenArg, u32)>,
    /// What creature's remains are called.
    #[serde(alias = "REMAINS")]
    pub remains: Option<(String, String)>,
    /// What color the creature's remains are.
    #[serde(alias = "REMAINS_COLOR")]
    pub remains_color: Option<ColorToken>,
    /// Goes with `[VERMIN_BITE]` and `[DIE_WHEN_VERMIN_BITE]`, the vermin creature will leave
    /// remains on death when biting. Leaving this tag out will cause the creature to disappear
    /// entirely after it bites.
    #[serde(alias = "REMAINS_ON_VERMIN_BITE_DEATH")]
    pub remains_on_vermin_bite_death: Option<()>,
    /// Unknown.
    #[serde(alias = "REMAINS_UNDETERMINED")]
    pub remains_undetermined: Option<()>,
    /// The creature will retract into a body part when threatened. It will be unable to move or
    /// attack, but enemies will only be able to attack the specified body part. (eg. Turtle,
    /// Hedgehog)
    ///
    /// Second-person descriptions are used for adventurer mode natural ability. "<pro_pos>" can be
    /// used in the descriptions, being replaced with the proper pronoun (or lack thereof) in-game.
    ///
    /// Undead curled up creatures are buggy.
    /// See [Bug:11463](https://www.bay12games.com/dwarves/mantisbt/view.php?id=11463)
    /// and [Bug:10519](https://www.bay12games.com/dwarves/mantisbt/view.php?id=10519).
    #[serde(alias = "RETRACT_INTO_BP")]
    pub retract_into_bp: Option<(BpCriteriaTokenArg, String, String, String, String)>,
    /// Cat behavior. If it kills a vermin creature and has an owner, it carries the remains in its
    /// mouth and drops them at their feet. Requires `[HUNTS_VERMIN]`, obviously.
    #[serde(alias = "RETURNS_VERMIN_KILLS_TO_OWNER")]
    pub returns_vermin_kills_to_owner: Option<()>,
    /// Creature will occasionally root around in the grass, looking for insects.
    ///
    /// Used for flavor in Adventurer Mode, spawns vermin edible for this creature in Fortress Mode.
    /// Creatures missing the specified body part will be unable to perform this action. The action
    /// produces a message (visible in adventure mode) in the form:
    ///
    /// `[creature]` `[verb text]` the `[description of creature's location]`
    ///
    /// In adventure mode, the "rooting around" ability will be included in the "natural abilities"
    /// menu, represented by its second person verb text.
    #[serde(alias = "ROOT_AROUND")]
    pub root_around: Option<(BpCriteriaTokenArg, String, String)>,
    /// Causes the specified tissue layer(s) of the indicated body part(s) to secrete the designated
    /// material. A size 100 ('covering') contaminant is created over the affected body part(s) in
    /// its specified material state (and at the temperature appropriate to this state) when the
    /// trigger condition is met, as long as one of the secretory tissue layers is still intact.
    #[serde(alias = "SECRETION")]
    pub secretion: Vec<(
        MaterialTokenArg,
        MaterialStateEnum,
        BpCriteriaTokenArg,
        Reference,
        // only an Option<> because of `creature_next_underground.txt` in vanilla:
        // TODO: find out why this is allowed to be an Option<>
        Option<SecretionTriggerEnum>,
    )>,
    /// Essentially the same as `[MEGABEAST]`, but more of them are created during worldgen.
    ///
    /// See the [semi-megabeast page](https://dwarffortresswiki.org/index.php/Semi-megabeast)
    /// for more info.
    #[serde(alias = "SEMIMEGABEAST")]
    pub semimegabeast: Option<()>,
    /// Gives the creature the ability to sense creatures belonging to the specified creature class
    /// even when they lie far beyond line of sight, including through walls and floors. It also
    /// appears to reduce or negate the combat penalty of blind units when fighting creatures they
    /// can sense. In adventure mode, the specified tile will be used to represent sensed creatures
    /// when they cannot be seen directly.
    #[serde(alias = "SENSE_CREATURE_CLASS")]
    pub sense_creature_class: Vec<(Reference, DFChar, u32, u32, u32)>, // TODO: ref is creature class
    /// The rate at which this creature learns this skill. Requires `[CAN_LEARN]` or `[INTELLIGENT]`
    /// to function.
    #[serde(alias = "SKILL_LEARN_RATE")]
    pub skill_learn_rate: Vec<(SkillEnum, u32)>,
    /// The rate at which this creature learns all skills. Requires `[CAN_LEARN]` or `[INTELLIGENT]`
    /// to function.
    #[serde(alias = "SKILL_LEARN_RATES")]
    pub skill_learn_rates: Option<u32>,
    /// Like `[SKILL_RATES]`, but applies to individual skills instead. Requires `[CAN_LEARN]` or
    /// `[INTELLIGENT]` to function.
    #[serde(alias = "SKILL_RATE")]
    pub skill_rate: Vec<(SkillEnum, u32, u32, u32, u32)>,
    /// Affects skill gain and decay. Lower numbers in the last three slots make decay occur faster
    /// (`[SKILL_RATES:100:1:1:1]` would cause rapid decay). The counter rates may also be replaced
    /// with `NONE`.
    ///
    /// Default is `[SKILL_RATES:100:8:16:16]`. Requires `[CAN_LEARN]` or `[INTELLIGENT]` to
    /// function.
    #[serde(alias = "SKILL_RATES")]
    pub skill_rates: Option<(
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
        Choose<u32, NoneEnum>,
    )>,
    /// The rate at which this skill decays. Lower values cause the skill to decay faster. Requires
    /// `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    #[serde(alias = "SKILL_RUST_RATE")]
    pub skill_rust_rate: Vec<(SkillEnum, u32, u32, u32)>,
    /// The rate at which all skills decay. Lower values cause the skills to decay faster. Requires
    /// `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    #[serde(alias = "SKILL_RUST_RATES")]
    pub skill_rust_rates: Option<(u32, u32, u32)>,
    /// Shorthand for `[CAN_LEARN]` + `[SKILL_LEARN_RATES:50]` (unverified). Used by a number of
    /// 'primitive' creatures (like ogres, giants and troglodytes) in the vanilla game. Applicable
    /// to player races. Prevents a player from recruiting nobility, even basic ones. Subterranean
    /// creatures with this token combined with `[EVIL]` will become servants of goblins in their
    /// civilizations, in the style of trolls.
    #[serde(alias = "SLOW_LEARNER")]
    pub slow_learner: Option<()>,
    /// Creature leaves "remains" instead of a corpse. Used by vermin.
    #[serde(alias = "SMALL_REMAINS")]
    pub small_remains: Option<()>,
    /// Creature makes sounds periodically, which can be heard in Adventure mode.
    ///
    /// First-person reads "You bark"
    ///
    /// Third-person reads "The capybara barks"
    ///
    /// Out of sight reads "You hear a loud bark"
    ///
    /// with the text in bold being the description arguments of the token.
    #[serde(alias = "SOUND")]
    pub sound: Vec<(
        AlertOrPeacefulIntermittentEnum,
        u32,
        u32,
        Choose<VocalizationEnum, NoneEnum>,
        String,
        String,
        String,
    )>,
    /// Creature will only appear in biomes with this plant or creature available. Grazers given a
    /// specific type of grass (such as pandas and bamboo) will only eat that grass and nothing
    /// else, risking starvation if there's none available.
    #[serde(alias = "SPECIFIC_FOOD")]
    pub specific_food: Vec<PlantOrCreatureTokenArg>,
    /// This creature can be converted by a night creature with `[SPOUSE_CONVERTER]`.
    #[serde(alias = "SPOUSE_CONVERSION_TARGET")]
    pub spouse_conversion_target: Option<()>,
    /// If the creature has the `[NIGHT_CREATURE_HUNTER]` tag, it will kidnap
    /// `[SPOUSE_CONVERSION_TARGET]`s and transform them into the caste of its species with the
    /// `[CONVERTED_SPOUSE]` tag during worldgen. It may also start families this way.
    #[serde(alias = "SPOUSE_CONVERTER")]
    pub spouse_converter: Option<()>,
    /// If the creature rules over a site, it will cause the local landscape to be corrupted into
    /// evil surroundings associated with the creature's spheres. The creature must have at least
    /// one of the following spheres for this to take effect: `BLIGHT`, `DEATH`, `DISEASE`,
    /// `DEFORMITY`, `NIGHTMARES`. The first three kill vegetation, while the others sometimes do.
    ///
    /// The last two get evil plants and evil animals sometimes. `NIGHTMARES` gets bogeymen
    /// ([source](http://www.bay12forums.com/smf/index.php?topic=169696.msg8162224#msg8162224)).
    /// Used by demons in the vanilla game.
    #[serde(alias = "SPREAD_EVIL_SPHERES_IF_RULER")]
    pub spread_evil_spheres_if_ruler: Option<()>,
    /// Caste does not require `[GRASP]` body parts to climb -- it can climb with `[STANCE]` parts
    /// instead.
    #[serde(alias = "STANCE_CLIMBER")]
    pub stance_climber: Option<()>,
    /// Acts as `[GRAZER]` but set to `20000*G*(max size)^(-3/4)`, where `G` defaults to 100 but can
    /// be set in d_init, and the whole thing is trapped between 150 and 3 million. Used for all
    /// grazers in the default creature raws.
    #[serde(alias = "STANDARD_GRAZER")]
    pub standard_grazer: Option<()>,
    /// The creature will get strange moods in fortress mode and can produce artifacts.
    #[serde(alias = "STRANGE_MOODS")]
    pub strange_moods: Option<()>,
    /// Gives the creature knowledge of any secrets with `[SUPERNATURAL_LEARNING_POSSIBLE]` that
    /// match its spheres. Also prevents it from becoming a vampire or a werebeast. Other effects
    /// are unknown.
    #[serde(alias = "SUPERNATURAL")]
    pub supernatural: Option<()>,
    /// The creature naturally knows how to swim perfectly and does not use the swimmer skill, as
    /// opposed to `[SWIMS_LEARNED]`. However, Fortress mode AI never paths into water anyway,
    /// so it's less useful there.
    #[serde(alias = "SWIMS_INNATE")]
    pub swims_innate: Option<()>,
    /// The creature swims only as well as their present swimming skill allows them to.
    #[serde(alias = "SWIMS_LEARNED")]
    pub swims_learned: Option<()>,
    /// Dilutes the effects of syndromes which have the specified identifier. A percentage of 100
    /// is equal to the regular syndrome effect severity, higher percentages reduce severity.
    #[serde(alias = "SYNDROME_DILUTION_FACTOR")]
    pub syndrome_dilution_factor: Vec<(Reference, u32)>, // ref is a syndrome identifier
    /// The creature has tendons in its `[CONNECTIVE_TISSUE_ANCHOR]` tissues (bone or chitin by
    /// default). Cutting the bone/chitin tissue severs the tendons, disabling motor function if the
    /// target is a limb.
    #[serde(alias = "TENDONS")]
    pub tendons: Option<(MaterialTokenArg, u32)>,
    /// The creature's webs can catch larger creatures.
    #[serde(alias = "THICKWEB")]
    pub thickweb: Option<()>,
    /// Found on titans. Cannot be specified in user-defined raws.
    #[serde(alias = "TITAN")] // TODO mark generated, see #84
    pub titan: Option<()>,
    /// How much the creature can carry when used by merchants.
    #[serde(alias = "TRADE_CAPACITY")]
    pub trade_capacity: Option<u32>,
    /// Shortcut for `[TRAINABLE_HUNTING]` + `[TRAINABLE_WAR]`.
    #[serde(alias = "TRAINABLE")]
    pub trainable: Option<()>,
    /// Can be trained as a hunting beast, increasing speed.
    #[serde(alias = "TRAINABLE_HUNTING")]
    pub trainable_hunting: Option<()>,
    /// Can be trained as a war beast, increasing strength and endurance.
    #[serde(alias = "TRAINABLE_WAR")]
    pub trainable_war: Option<()>,
    /// Allows the creature to go into [martial trances](https://dwarffortresswiki.org/index.php/Martial_trance).
    /// Used by dwarves in the vanilla game.
    #[serde(alias = "TRANCES")]
    pub trances: Option<()>,
    /// The creature will never trigger traps it steps on. Used by a number of creatures. Doesn't
    /// make the creature immune to remotely activated traps (like retractable spikes being
    /// triggered while the creature is standing over them). `TRAPAVOID` creatures lose this power
    /// if they're immobilized while standing in a trap, be it by stepping on thick web, being
    /// paralyzed or being knocked unconscious.
    #[serde(alias = "TRAPAVOID")]
    pub trapavoid: Option<()>,
    /// The creature is displayed as blue when in 7/7 water. Used on fish and amphibious creatures
    /// which swim under the water.
    #[serde(alias = "UNDERSWIM")]
    pub underswim: Option<()>,
    /// Found on generated demons; causes the game to create a single named instance of the demon
    /// which will emerge from the underworld and take over civilizations during worldgen. Could not
    /// be specified in user-defined raws prior to version `0.47.01`.
    #[serde(alias = "UNIQUE_DEMON")]
    pub unique_demon: Option<()>,
    /// Changes the language of the creature into unintelligible 'kobold-speak', which creatures of
    /// other species will be unable to understand. If a civilized creature has this and is not part
    /// of a `[SKULKING]` civ, it will tend to start wars with all nearby civilizations and will be
    /// unable to make peace treaties due to 'inability to communicate'.
    #[serde(alias = "UTTERANCES")]
    pub utterances: Option<()>,
    /// The creature is made of swampstuff. Doesn't appear to do anything in particular. Used by
    /// grimelings in the vanilla game.
    #[serde(alias = "VEGETATION")]
    pub vegetation: Option<()>,
    /// Enables vermin to bite other creatures, injecting the specified material. See
    /// `[SPECIALATTACK_INJECT_EXTRACT]` for details about injection - this token presumably works
    /// in a similar manner (unverified).
    #[serde(alias = "VERMIN_BITE")] // TODO research; can this be a Vec?
    pub vermin_bite: Option<(u32, String, MaterialTokenArg, MaterialStateEnum)>,
    /// Some dwarves will hate the creature and get unhappy thoughts when around it, and show a negative
    /// preference towards them. Being hated by some dwarves does not prevent the vermin from
    /// appearing as a positive preference for other dwarves.
    #[serde(alias = "VERMIN_HATEABLE")]
    pub vermin_hateable: Option<()>,
    /// This makes the creature move in a swarm of creatures of the same race as it (e.g. swarm of
    /// flies, swarm of ants).
    #[serde(alias = "VERMIN_MICRO")]
    pub vermin_micro: Option<()>,
    /// The creature cannot be caught by fishing.
    #[serde(alias = "VERMIN_NOFISH")]
    pub vermin_nofish: Option<()>,
    /// The creature will not be observed randomly roaming about the map.
    #[serde(alias = "VERMIN_NOROAM")]
    pub vermin_noroam: Option<()>,
    /// The creature cannot be caught in baited animal traps; however, a "catch live land animal"
    /// task may still be able to capture one if a dwarf finds one roaming around.
    #[serde(alias = "VERMIN_NOTRAP")]
    pub vermin_notrap: Option<()>,
    /// Old shorthand for "does cat stuff". Contains `[AT_PEACE_WITH_WILDLIFE]` +
    /// `[RETURNS_VERMIN_KILLS_TO_OWNER]` + `[HUNTS_VERMIN]` + `[ADOPTS_OWNER]`.
    #[serde(alias = "VERMINHUNTER")]
    pub verminhunter: Option<()>,
    /// Sets the creature to be active during the evening in adventurer mode.
    #[serde(alias = "VESPERTINE")]
    pub vespertine: Option<()>,
    /// Value should determine how close you have to get to a critter before it attacks (or prevents
    /// adv mode travel etc.) Default is 20.
    #[serde(alias = "VIEWRANGE")]
    pub viewrange: Option<u32>,
    /// The width of the creature's vision arcs, in degrees (i.e. 0 to 360). The first number is
    /// binocular vision, the second is non-binocular vision.
    ///
    /// Binocular vision has a minimum of about 10 degrees, monocular, a maximum of about 350
    /// degrees. Values past these limits will be accepted, but will default to ~10 degrees and ~350
    /// degrees respectively.
    #[serde(alias = "VISION_ARC")]
    pub vision_arc: Option<(Clamp<u16, 10, 360>, Clamp<u16, 0, 350>)>,
    /// Allows the creature to pull caravan wagons. If a civilization doesn't have access to any, it
    /// is restricted to trading with pack animals.
    #[serde(alias = "WAGON_PULLER")]
    pub wagon_puller: Option<()>,
    /// Allows the creature to create webs, and defines what the webs are made of.
    #[serde(alias = "WEBBER")]
    pub webber: Option<MaterialTokenArg>,
    /// The creature will not get caught in thick webs. Used by creatures who can shoot thick webs
    /// (such as giant cave spiders) in order to make them immune to their own attacks.
    #[serde(alias = "WEBIMMUNE")]
    pub webimmune: Option<()>,
    // endregion ==================================================================================
}
