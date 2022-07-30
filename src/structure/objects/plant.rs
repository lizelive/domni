use crate::core::{Choose, DFChar, Reference, ReferenceTo, Flag};
use crate::structure::{
    AllEnum, AllOrAllSolidEnum, BiomeEnum, ColorToken, DietInfoEnum, InorganicToken,
    ItemReferenceArg, LocalMaterialToken, MaterialStateEnum, MaterialTokenArg, NoneEnum,
    OverwriteSolidEnum, ReactionToken, StandardPluralEnum, SyndromeToken, UseMaterial,
    UseMaterialTemplate,
};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PlantToken {
    /// Argument 1 of `[PLANT:...]`
    #[serde(alias = "PLANT")]
    pub reference: Option<ReferenceTo<Self>>,
    // region: Nesting Tokens =====================================================================
    /// List of growth types this plant can have.
    #[serde(alias = "GROWTH")]
    pub growth: Vec<Growth>,
    /// Starts defining a new local plant material with the given name and no properties.
    #[serde(alias = "MATERIAL")]
    pub material: Vec<LocalMaterialToken>,
    /// Starts defining a new local plant material with the given name and using the properties of
    /// another local plant material.
    #[serde(alias = "USE_MATERIAL")]
    pub use_material: Vec<UseMaterial>,
    /// Starts defining a new local plant material with the given name and using the properties of
    /// the specified material template.
    #[serde(alias = "USE_MATERIAL_TEMPLATE")]
    pub use_material_template: Vec<UseMaterialTemplate>,
    /// The `BASIC_MAT` used for this plant; there can only be one of these.
    #[serde(alias = "BASIC_MAT")]
    pub basic_mat: Option<BasicMat>,
    // endregion ==================================================================================
    // region: Basic Tokens =======================================================================
    /// The singular form of the plant's name as seen in-game.
    #[serde(alias = "NAME")]
    pub name: Option<String>,
    /// The plural form of the plant's name as seen in-game.
    #[serde(alias = "NAME_PLURAL")]
    pub name_plural: Option<String>,
    /// The word or phrase used to describe items made from this plant.
    #[serde(alias = "ADJ")]
    pub adj: Option<String>,
    /// Sets the `NAME`, `NAME_PLURAL`, and `ADJ` to the specified string.
    #[serde(alias = "ALL_NAMES")]
    pub all_names: Option<String>,
    /// What dwarves can like this object for; for example, if you give plump helmets
    /// `[PREFSTRING:rounded tops]`, the game will show "Urist likes plump helmets for their
    /// rounded tops."
    #[serde(alias = "PREFSTRING")]
    pub prefstring: Vec<String>,
    // endregion ==================================================================================
    // region: Environment Tokens =================================================================
    /// Designates the lowest and highest cavern levels that the plant can appear in if its biome is
    /// subterranean. Dwarven civilizations will only export (via the embark screen or caravans)
    /// things that are available at depth 1 (the first cavern layer).
    ///
    /// Defaults to 0:0 (surface only).
    #[serde(alias = "UNDERGROUND_DEPTH")]
    pub underground_depth: Option<(u16, u16)>,
    /// Restricts the plant to growing in Good regions. Cannot be combined with `[EVIL]`.
    #[serde(alias = "GOOD")]
    pub good: Flag,
    /// Restricts the plant to growing in Evil regions. Cannot be combined with `[GOOD]`.
    #[serde(alias = "EVIL")]
    pub evil: Flag,
    /// Restricts the plant to growing in Savage regions (regardless of alignment).
    #[serde(alias = "SAVAGE")]
    pub savage: Flag,
    /// How frequently this plant is generated in a particular area. Plants with
    /// valid biome tokens and `[FREQUENCY:0]` will not grow in the wild, but will still be
    /// available for entity use and farm plots. Defaults to 50.
    #[serde(alias = "FREQUENCY")]
    pub frequency: Option<u8>,
    /// Restricts the plant to grow near natural water features. A plant with `[WET]` may be very
    /// common or very rare in an area, depending on how many water features that area has.
    ///
    /// Note that they will not grow next to dwarf-filled channels, since it explicitly checks if
    /// the tile type is "River", "River Slope", "River Source", "Waterfall" (used back in 40d for
    /// underground rivers), "Brook", "Murky Pool", or "Murky Pool Slope".
    #[serde(alias = "WET")]
    pub wet: Flag,
    /// Allows the plant to grow away from water features.
    #[serde(alias = "DRY")]
    pub dry: Flag,
    /// What biome this plant appears in.
    #[serde(alias = "BIOME")]
    pub biome: Vec<BiomeEnum>,
    // endregion ==================================================================================
    // region: Tree Tokens ========================================================================
    /// Makes the plant into a tree. Cutting down the tree will yield logs made of this material.
    /// Setting the material to `NONE` will give no wood from this tree.
    #[serde(alias = "TREE")]
    pub tree: Option<Choose<(NoneEnum, NoneEnum), MaterialTokenArg>>,
    /// What the trunk of the tree is named.
    #[serde(alias = "TRUNK_NAME")]
    pub trunk_name: Option<String>,
    /// The maximum z-level height of a mature tree's trunk, starting from about two z-levels above
    /// ground and going up.
    #[serde(alias = "MAX_TRUNK_HEIGHT")]
    pub max_trunk_height: Option<u8>,
    /// Upper limit of trunk thickness, in tiles. Counted separately for all branching trunks. Has a
    /// geometric effect on log yield.
    #[serde(alias = "MAX_TRUNK_DIAMETER")]
    pub max_trunk_diameter: Option<u8>,
    /// The number of years the trunk takes to grow one Z-level upward.
    #[serde(alias = "TRUNK_PERIOD")]
    pub trunk_period: Option<u16>,
    /// The number of years the trunk takes to grow another tile wider.
    #[serde(alias = "TRUNK_WIDTH_PERIOD")]
    pub trunk_width_period: Option<u16>,
    /// What thin branches of the tree are named.
    #[serde(alias = "LIGHT_BRANCHES_NAME")]
    pub light_branches_name: Option<String>,
    /// How dense the branches grow on this tree.
    #[serde(alias = "BRANCH_DENSITY")]
    pub branch_density: Option<u8>,
    /// How dense the branches grow on this tree.
    #[serde(alias = "LIGHT_BRANCHES_DENSITY")]
    pub light_branches_density: Option<u8>,
    /// The radius to which branches can reach. Appears to never reach further than seven tiles
    /// from the centre. Does not depend on the trunk branching amount or where trunks are. The
    /// values used in the game go from 0-3. Higher values than that can cause crashes
    /// ([Bug:10419](https://www.bay12games.com/dwarves/mantisbt/print_bug_page.php?bug_id=10419)).
    #[serde(alias = "BRANCH_RADIUS")]
    pub branch_radius: Option<u8>,
    /// The radius to which branches can reach. Appears to never reach further than seven tiles
    /// from the centre. Does not depend on the trunk branching amount or where trunks are. The
    /// values used in the game go from 0-3. Higher values than that can cause crashes
    /// ([Bug:10419](https://www.bay12games.com/dwarves/mantisbt/print_bug_page.php?bug_id=10419)).
    #[serde(alias = "LIGHT_BRANCH_RADIUS")]
    pub light_branch_radius: Option<u8>,
    /// What thick branches of the tree are named.
    #[serde(alias = "HEAVY_BRANCHES_NAME")]
    pub heavy_branches_name: Option<String>,
    /// What thick branches of the tree are named.
    #[serde(alias = "DIRECTED_BRANCHES_NAME")]
    pub directed_branches_name: Option<String>,
    /// How dense the thick branches grow on this tree. Values outside 0-3 can cause crashes
    /// ([Bug:10419](https://www.bay12games.com/dwarves/mantisbt/print_bug_page.php?bug_id=10419)).
    #[serde(alias = "HEAVY_BRANCH_DENSITY")]
    pub heavy_branch_density: Option<u8>,
    /// How dense the thick branches grow on this tree. Values outside 0-3 can cause crashes
    /// ([Bug:10419](https://www.bay12games.com/dwarves/mantisbt/print_bug_page.php?bug_id=10419)).
    #[serde(alias = "DIRECTED_BRANCH_DENSITY")]
    pub directed_branch_density: Option<u8>,
    /// Similar as `BRANCH_DENSITY` for thick branches.
    #[serde(alias = "HEAVY_BRANCH_RADIUS")]
    pub heavy_branch_radius: Option<u8>,
    /// Similar as `BRANCH_DENSITY` for thick branches.
    #[serde(alias = "DIRECTED_BRANCH_RADIUS")]
    pub directed_branch_radius: Option<u8>,
    /// How much the trunk branches out. 0 makes the trunk straight.
    #[serde(alias = "TRUNK_BRANCHING")]
    pub trunk_branching: Option<u8>,
    /// What the roots of the tree are named.
    #[serde(alias = "ROOTS_NAME", alias = "ROOT_NAME")]
    pub roots_name: Option<String>, // TODO check this; might actually be 2 tokens, 1 plural 1 singular
    /// Density of the root growth.
    #[serde(alias = "ROOT_DENSITY")]
    pub root_density: Option<u8>,
    /// How wide the roots reach out.
    #[serde(alias = "ROOT_RADIUS")]
    pub root_radius: Option<u8>,
    /// What the twigs of the tree are named.
    #[serde(alias = "TWIGS_NAME")]
    pub twigs_name: Option<String>,
    /// Twigs appear on the side of branches. Defaults to 1.
    #[serde(alias = "TWIGS_SIDE_BRANCHES")]
    pub twigs_side_branches: Option<bool>,
    /// Twigs appear above branches. Defaults to 1.
    #[serde(alias = "TWIGS_ABOVE_BRANCHES")]
    pub twigs_above_branches: Option<bool>,
    /// Twigs appear below branches. Defaults to 0.
    #[serde(alias = "TWIGS_BELOW_BRANCHES")]
    pub twigs_below_branches: Option<bool>,
    /// Twigs appear on the side of heavy branches. Defaults to 0.
    #[serde(alias = "TWIGS_SIDE_HEAVY_BRANCHES")]
    pub twigs_side_heavy_branches: Option<bool>,
    /// Twigs appear above heavy branches. Defaults to 0.
    #[serde(alias = "TWIGS_ABOVE_HEAVY_BRANCHES")]
    pub twigs_above_heavy_branches: Option<bool>,
    /// Twigs appear below heavy branches. Defaults to 0.
    #[serde(alias = "TWIGS_BELOW_HEAVY_BRANCHES")]
    pub twigs_below_heavy_branches: Option<bool>,
    /// Twigs appear on the side of the trunk. Defaults to 0.
    #[serde(alias = "TWIGS_SIDE_TRUNK")]
    pub twigs_side_trunk: Option<bool>,
    /// Twigs appear above the trunk. Defaults to 0.
    #[serde(alias = "TWIGS_ABOVE_TRUNK")]
    pub twigs_above_trunk: Option<bool>,
    /// Twigs appear below the trunk. Defaults to 0.
    #[serde(alias = "TWIGS_BELOW_TRUNK")]
    pub twigs_below_trunk: Option<bool>,
    /// The tree has a rounded cap-hood like a giant mushroom. This severely stunts a tree's maximum
    /// height - see [Bug:7313](https://www.bay12games.com/dwarves/mantisbt/view.php?id=7313).
    #[serde(alias = "TREE_HAS_MUSHROOM_CAP")]
    pub tree_has_mushroom_cap: Flag,
    /// What this mushroom-cap is called. Only makes sense with `TREE_HAS_MUSHROOM_CAP`.
    #[serde(alias = "CAP_NAME")]
    pub cap_name: Option<String>,
    /// Similar to the other `PERIOD` tags, influences the rate of the mushroom cap growth. Only
    /// makes sense with `TREE_HAS_MUSHROOM_CAP`.
    #[serde(alias = "CAP_PERIOD")]
    pub cap_period: Option<u16>,
    /// The radius of a mushroom cap. Only makes sense with `TREE_HAS_MUSHROOM_CAP`.
    #[serde(alias = "CAP_RADIUS")]
    pub cap_radius: Option<u8>,
    /// Uses the standard names for the tree components (roots, trunk, branches, etc.)
    #[serde(alias = "STANDARD_TILE_NAMES")]
    pub standard_tile_names: Flag,
    /// The tile used for trees of this type on the world map. Defaults to 24 (↑).
    #[serde(alias = "TREE_TILE")]
    pub tree_tile: Option<DFChar>,
    /// The tile used for (un)dead trees and deciduous trees (generally in winter) of this type.
    /// Defaults to 198 (╞).
    #[serde(alias = "DEAD_TREE_TILE")]
    pub dead_tree_tile: Option<DFChar>,
    /// The tile used for saplings of this tree. Defaults to 231 (τ).
    #[serde(alias = "SAPLING_TILE")]
    pub sapling_tile: Option<DFChar>,
    /// The tile used for dead saplings of this tree. Defaults to 231 (τ).
    #[serde(alias = "DEAD_SAPLING_TILE")]
    pub dead_sapling_tile: Option<DFChar>,
    /// The color of the tree on the map. Defaults to 2:0:0 (dark green).
    #[serde(alias = "TREE_COLOR")]
    pub tree_color: Option<(u8, u8, u8)>,
    /// The color of the tree on the map when (un)dead. Defaults to 0:0:1 (dark gray).
    #[serde(alias = "DEAD_TREE_COLOR")]
    pub dead_tree_color: Option<(u8, u8, u8)>,
    /// The color of saplings of this tree. Defaults to 2:0:0 (dark green).
    #[serde(alias = "SAPLING_COLOR")]
    pub sapling_color: Option<(u8, u8, u8)>,
    /// The color of dead saplings of this tree. Defaults to 0:0:1 (dark gray).
    #[serde(alias = "DEAD_SAPLING_COLOR")]
    pub dead_sapling_color: Option<(u8, u8, u8)>,
    /// The sapling of this tree will drown once the water on its tile reaches this level. Defaults
    /// to 4.
    #[serde(alias = "SAPLING_DROWN_LEVEL")]
    pub sapling_drown_level: Option<u8>,
    /// The water depth at which this tree will drown. Exact behavior is unknown. Defaults to 7.
    #[serde(alias = "TREE_DROWN_LEVEL")]
    pub tree_drown_level: Option<u8>,
    /// Makes young versions of the tree be called "[tree name] sapling"; otherwise, they are called
    /// "young [tree name]".
    #[serde(alias = "SAPLING")]
    pub sapling: Flag,
    // endregion ==================================================================================
    // region: Shrub Tokens =======================================================================
    /// Allows the plant to grow in farm plots during spring. If the plant is a surface
    /// plant, allows it to grow in the wild during spring; wild surface plants without this
    /// token will disappear at the beginning of spring. Underground plants grow wild in all
    /// seasons, regardless of their season tokens.
    #[serde(alias = "SPRING")]
    pub spring: Flag,
    /// Allows the plant to grow in farm plots during summer. If the plant is a surface
    /// plant, allows it to grow in the wild during summer; wild surface plants without this
    /// token will disappear at the beginning of summer. Underground plants grow wild in all
    /// seasons, regardless of their season tokens.
    #[serde(alias = "SUMMER")]
    pub summer: Flag,
    /// Allows the plant to grow in farm plots during autumn. If the plant is a surface
    /// plant, allows it to grow in the wild during autumn; wild surface plants without this
    /// token will disappear at the beginning of autumn. Underground plants grow wild in all
    /// seasons, regardless of their season tokens.
    #[serde(alias = "AUTUMN")]
    pub autumn: Flag,
    /// Allows the plant to grow in farm plots during winter. If the plant is a surface
    /// plant, allows it to grow in the wild during winter; wild surface plants without this
    /// token will disappear at the beginning of winter. Underground plants grow wild in all
    /// seasons, regardless of their season tokens.
    #[serde(alias = "WINTER")]
    pub winter: Flag,
    /// How long the plant takes to grow to harvest in a farm plot. The unit used is in hundreds
    /// of ticks, See [Time](https://dwarffortresswiki.org/index.php/Time). There are 1008 `GROWDUR`
    /// units in a season. Defaults to 300.
    #[serde(alias = "GROWDUR")]
    pub growdur: Option<u32>,
    /// Has no known effect. Previously set the value of the harvested plant.
    #[serde(alias = "VALUE")]
    pub value: Option<i32>,
    /// The tile used when the plant is harvested whole, or is ready to be picked from a farm plot.
    /// May either be a cp437 tile number, or a character between single quotes. See [character
    /// table](https://dwarffortresswiki.org/index.php/Character_table). Defaults to 231 (τ).
    #[serde(alias = "PICKED_TILE")]
    pub picked_tile: Option<DFChar>,
    /// The tile used when a plant harvested whole has wilted. Defaults to 169 (⌐).
    #[serde(alias = "DEAD_PICKED_TILE")]
    pub dead_picked_tile: Option<DFChar>,
    /// The tile used to represent this plant when it is wild, alive, and has no growths. Defaults
    /// to 34 (").
    #[serde(alias = "SHRUB_TILE")]
    pub shrub_tile: Option<DFChar>,
    /// The tile used to represent this plant when it is dead in the wild. Defaults to 34 (").
    #[serde(alias = "DEAD_SHRUB_TILE")]
    pub dead_shrub_tile: Option<DFChar>,
    /// The maximum stack size collected when gathered via herbalism (possibly also from farm
    /// plots?). Defaults to 5.
    #[serde(alias = "CLUSTERSIZE")]
    pub clustersize: Option<u16>,
    /// The color of the plant when it has been picked whole, or when it is ready for harvest in a
    /// farm plot. Defaults to 2:0:0 (dark green).
    #[serde(alias = "PICKED_COLOR")]
    pub picked_color: Option<(u8, u8, u8)>,
    /// The color of the plant when it has been picked whole, but has wilted. Defaults to 0:0:1
    /// (dark gray).
    #[serde(alias = "DEAD_PICKED_COLOR")]
    pub dead_picked_color: Option<(u8, u8, u8)>,
    /// The color of the plant when it is alive, wild, and has no growths. Defaults to 2:0:0 (dark
    /// green).
    #[serde(alias = "SHRUB_COLOR")]
    pub shrub_color: Option<(u8, u8, u8)>,
    /// The color of the plant when it is dead in the wild. Defaults to 6:0:0 (brown).
    #[serde(alias = "DEAD_SHRUB_COLOR")]
    pub dead_shrub_color: Option<(u8, u8, u8)>,
    /// The shrub will drown once the water on its tile reaches this level. Defaults to 4.
    #[serde(alias = "SHRUB_DROWN_LEVEL")]
    pub shrub_drown_level: Option<u8>,
    /// Names a drink made from the plant, allowing it to be used in entity resources. Previously
    /// also permitted brewing the plant into alcohol made of this material. Now, a
    /// `MATERIAL_REACTION_PRODUCT` of type `DRINK_MAT` should be used on the proper plant material.
    #[serde(alias = "DRINK")]
    pub drink: Option<MaterialTokenArg>,
    /// Permits milling the plant at a quern or millstone into a powder made of this material and
    /// allows its use in entity resources. Said material should have `[POWDER_MISC_PLANT]` to
    /// permit proper stockpiling.
    #[serde(alias = "MILL")]
    pub mill: Option<MaterialTokenArg>,
    /// Permits processing the plant at a farmer's workshop to yield threads made of this material
    /// and allows its use in entity resources. Said material should have `[THREAD_PLANT]` to permit
    /// proper stockpiling.
    #[serde(alias = "THREAD")]
    pub thread: Option<MaterialTokenArg>,
    /// Causes the plant to yield plantable seeds made of this material and having these properties.
    /// Said material should have `[SEED_MAT]` to permit proper stockpiling.
    #[serde(alias = "SEED")]
    pub seed: Option<(String, String, u8, u8, u8, MaterialTokenArg)>,
    /// Permits processing the plant into a vial at a still to yield extract made of this material.
    /// Said material should have `[EXTRACT_STORAGE:FLASK]`.
    #[serde(alias = "EXTRACT_STILL_VIAL")]
    pub extract_still_vial: Option<MaterialTokenArg>,
    /// Permits processing the plant into a vial at a farmer's workshop to yield extract made of
    /// this material. Said material should have `[EXTRACT_STORAGE:FLASK]`.
    #[serde(alias = "EXTRACT_VIAL")]
    pub extract_vial: Option<MaterialTokenArg>,
    /// Permits processing the plant into a barrel at a farmer's workshop to yield extract made of
    /// this material. Said material should have `[EXTRACT_STORAGE:BARREL]`.
    #[serde(alias = "EXTRACT_BARREL")]
    pub extract_barrel: Option<MaterialTokenArg>,
    // endregion ==================================================================================
    // region: Grass Tokens =======================================================================
    /// Makes the plant behave as a type of grass. This allows animals to graze on it, and prevents
    /// it and its growths from being picked by herbalists. (Grass growths can still be picked in
    /// adventure mode, however.)
    #[serde(alias = "GRASS")]
    pub grass: Flag,
    /// Specifies the 4 tiles used to represent grass of this type. If `VARIED_GROUND_TILES` is
    /// disabled in `d_init.txt`, these are seemingly ignored. Defaults to 46:44:96:39 (.,`').
    #[serde(alias = "GRASS_TILES")]
    pub grass_tiles: Option<(DFChar, DFChar, DFChar, DFChar)>,
    /// How often the grass switches between its main tiles and alternate tiles. The "period" value
    /// determines how quickly (in frames) the grass animates, and the "offset" value specifies how
    /// much of that time is spent displaying the alternate tiles.
    ///
    /// If the "offset" value is greater than or equal to the "period" value, the grass will only
    /// display using the alternate tiles.
    ///
    /// Defaults to 0:0.
    #[serde(alias = "ALT_PERIOD")]
    pub alt_period: Option<(u32, u32)>,
    /// When used with `ALT_PERIOD`, specifies the 4 alternate tiles used to represent grass of this
    /// type. Defaults to 46:44:96:39 (.,`'). Dead grass does not animate.
    #[serde(alias = "ALT_GRASS_TILES")]
    pub alt_grass_tiles: Option<(DFChar, DFChar, DFChar, DFChar)>,
    /// Specifies the color of this grass, using the following arguments:
    /// - color 1 (fore:back:bright)
    /// - color 2 (fore:back:bright)
    /// - dry color (fore:back:bright)
    /// - dead color (fore:back:bright)
    ///
    /// Defaults to 2:0:1:2:0:0:6:0:1:6:0:0 (light green, dark green, yellow, brown).
    #[serde(alias = "GRASS_COLORS")]
    pub grass_colors: Option<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)>,
    // endregion ==================================================================================
}

/// Defines a plant growth.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Growth {
    /// argument 1 of `GROWTH`
    #[serde(alias = "GROWTH")]
    pub reference: Option<Reference>,
    /// The name of a plant growth.
    #[serde(alias = "GROWTH_NAME")]
    pub growth_name: Option<(String, Choose<StandardPluralEnum, String>)>,
    /// Specifies what item this growth is and what it is made of. Generally, the item type should
    /// be `PLANT_GROWTH:NONE`.
    #[serde(alias = "GROWTH_ITEM")]
    pub growth_item: Option<(Reference, Reference, MaterialTokenArg)>, // ref here is an item and subitem
    /// Specifies on which part of the plant or tree the growth appears, usually for multi-tile
    /// trees.
    #[serde(alias = "GROWTH_HOST_TILE")]
    pub growth_host_tile: Vec<GrowthHostEnum>,
    ///
    #[serde(alias = "GROWTH_TRUNK_HEIGHT_PERC")]
    pub growth_trunk_height_perc: Option<(i8, i8)>, // TODO: research, can both really be negative?
    /// Currently has no effect.
    #[serde(alias = "GROWTH_DENSITY")]
    pub growth_density: Option<u32>,
    /// Specifies at which part of the year the growth appears. Default is all year round.
    ///
    /// A single growth can only have one `GROWTH_TIMING` tag. If multiple are declared, the last
    /// one will be used.
    ///
    /// To make a growth appear multiple times during the year, you need to create a different
    /// growth for every `GROWTH_TIMING` interval. By using the same material for all of the
    /// duplicate growths, all of them will be stockpiled together and be eligible for the same
    /// reactions. Edible/brewable growths will have separate entries in the kitchen menu, though.
    ///
    /// There is no known way to declare a growth timing that lasts from winter into spring.
    /// Including numbers below 0 or above 403200 in the range will make the growth available at all
    /// times, as though you hadn't defined a growth timing at all. So will including a range for
    /// which the start time is later than the end time.
    ///
    /// This has no effect on farmed growths; all eligible growths that have `[STOCKPILE_PLANT_GROWTH]`
    /// in their materials will be harvested, regardless of if they are currently within their
    /// growth timing or not.
    #[serde(alias = "GROWTH_TIMING")]
    pub growth_timing: Option<(u32, u32)>,
    /// Specifies the appearance of the growth. Can be specified more than once, for example for
    /// autumn leaves. Transitions between different timing periods will happen gradually over the
    /// course of 2000 ticks.
    ///
    /// The `GROWTH_PRINT` tile will only be displayed when the growth in question is actually
    /// present, even if its timing parameter is `ALL`.
    #[serde(alias = "GROWTH_PRINT")]
    pub growth_print: Vec<(
        DFChar,
        DFChar,
        u8,
        u8,
        u8,
        Choose<(u32, u32), Choose<AllEnum, NoneEnum>>, // TODO: something less ugly here
        Option<u32>,
    )>,
    /// The growth drops a seed if eaten raw.
    #[serde(alias = "GROWTH_HAS_SEED")]
    pub growth_has_seed: Flag,
    /// Growths drop from the plant, producing a cloud of items which fall on the ground, which
    /// herbalists can collect.
    #[serde(alias = "GROWTH_DROPS_OFF")]
    pub growth_drops_off: Flag,
    /// Growths drop collectable items from the plant without producing item clouds.
    #[serde(alias = "GROWTH_DROPS_OFF_NO_CLOUD")]
    pub growth_drops_off_no_cloud: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum GrowthHostEnum {
    /// The growth will appear on the twigs.
    #[serde(alias = "TWIGS")]
    Twigs,
    /// The growth will appear on branches.
    #[serde(alias = "BRANCHES")]
    Branches,
    /// The growth will appear on the trunk.
    #[serde(alias = "TRUNK")]
    Trunk,
    /// The growth will appear on the roots.
    #[serde(alias = "ROOTS")]
    Roots,
    /// The growth will appear on the cap.
    #[serde(alias = "CAP")]
    Cap,
    /// The growth will appear on the sapling form of the plant.
    #[serde(alias = "SAPLING")]
    Sapling,
    /// The growth will appear on branches and twigs.
    #[serde(alias = "BRANCHES_AND_TWIGS")]
    BranchesAndTwigs,
    /// The growth will appear on light branches and twigs.
    #[serde(alias = "LIGHT_BRANCHES_AND_TWIGS")]
    LightBranchesAndTwigs,
    /// The growth will appear on light branches.
    #[serde(alias = "LIGHT_BRANCHES")]
    LightBranches,
    /// The growth will appear on all branches and twigs.
    #[serde(alias = "ALL_BRANCHES_AND_TWIGS")]
    AllBranchesAndTwigs,
    /// The growth will appear on heavy branches.
    #[serde(alias = "HEAVY_BRANCHES")]
    HeavyBranches,
    /// The growth will appear on directed branches.
    #[serde(alias = "DIRECTED_BRANCHES")]
    DirectedBranches,
    /// The growth will appear on heavy branches and the trunk.
    #[serde(alias = "HEAVY_BRANCHES_AND_TRUNK")]
    HeavyBranchesAndTrunk,
    /// The growth will appear on directed branches and the trunk.
    #[serde(alias = "DIRECTED_BRANCHES_AND_TRUNK")]
    DirectedBranchesAndTrunk,
}
impl Default for GrowthHostEnum {
    fn default() -> Self {
        Self::Twigs
    }
}

/// Sets the basic material of the plant, using another defined material (local or otherwise).
///
/// According to Toady, you can use other materials (for instance, iron) but the game may hiccup
/// on plants that aren't structurally plants. For crops, said material should have
/// `[STRUCTURAL_PLANT_MAT]` to permit proper stockpiling. Generally, this should be
/// "`LOCAL_PLANT_MAT:material_name`", using a material defined using `MATERIAL`, `USE_MATERIAL`,
/// or `USE_MATERIAL_TEMPLATE`.
///
/// You can nest any material token under here to modify the new material created from the
/// original material.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BasicMat {
    /// arguments of `BASIC_MAT`
    #[serde(alias = "BASIC_MAT")]
    pub basic_mat: Option<MaterialTokenArg>,
    // region: Not Permitted in MatDef ============================================================
    /// Applies a prefix to all items made from the material. For `PLANT` and `CREATURE` materials,
    /// this defaults to the plant/creature name. Not permitted in material template definitions.
    #[serde(alias = "PREFIX")]
    pub prefix: Option<Choose<NoneEnum, String>>,
    /// Multiplies the value of the material. Not permitted in material template definitions.
    #[serde(alias = "MULTIPLY_VALUE")]
    pub multiply_value: Option<u32>,
    /// Changes a material's `HEATDAM_POINT`, but only if it was not set to `NONE`. Not permitted in
    /// material template definitions.
    #[serde(alias = "IF_EXISTS_SET_HEATDAM_POINT")]
    pub if_exists_set_heatdam_point: Option<Choose<u32, NoneEnum>>,
    /// Changes a material's `COLDDAM_POINT`, but only if it was not set to `NONE`. Not permitted in
    /// material template definitions.
    #[serde(alias = "IF_EXISTS_SET_COLDDAM_POINT")]
    pub if_exists_set_colddam_point: Option<Choose<u32, NoneEnum>>,
    /// Changes a material's `IGNITE_POINT`, but only if it was not set to `NONE`. Not permitted in
    /// material template definitions.
    #[serde(alias = "IF_EXISTS_SET_IGNITE_POINT")]
    pub if_exists_set_ignite_point: Option<Choose<u32, NoneEnum>>,
    /// Changes a material's `MELTING_POINT`, but only if it was not set to `NONE`. Not permitted in
    /// material template definitions.
    #[serde(alias = "IF_EXISTS_SET_MELTING_POINT")]
    pub if_exists_set_melting_point: Option<Choose<u32, NoneEnum>>,
    /// Changes a material's `BOILING_POINT`, but only if it was not set to `NONE`. Not permitted in
    /// material template definitions.
    #[serde(alias = "IF_EXISTS_SET_BOILING_POINT")]
    pub if_exists_set_boiling_point: Option<Choose<u32, NoneEnum>>,
    /// Changes a material's `MAT_FIXED_TEMP`, but only if it was not set to `NONE`. Not permitted
    /// in material template definitions.
    #[serde(alias = "IF_EXISTS_SET_MAT_FIXED_TEMP")]
    pub if_exists_set_mat_fixed_temp: Option<Choose<u32, NoneEnum>>,
    // endregion ==================================================================================
    // region: MatDef but not allowed in INORGANIC ================================================
    /// The material forms "wafers" instead of "bars".
    #[serde(alias = "WAFERS")]
    pub wafers: Flag,
    /// Makes `BOULDER` acceptable as a reagent in reactions that require `METAL_ORE:MATERIAL_NAME`,
    /// as well as smelting directly into metal bars.
    ///
    /// Places the material under "Metal Ores" in Stone stockpiles.
    ///
    /// The specified value determines the probability for this product
    /// (see [Tetrahedrite](https://dwarffortresswiki.org/index.php/Tetrahedrite)
    /// or [Galena](https://dwarffortresswiki.org/index.php/Galena) for details).
    #[serde(alias = "METAL_ORE")]
    pub metal_ore: Option<(ReferenceTo<InorganicToken>, u32)>,
    /// Makes `BOULDER` items made of the material acceptable for strand extraction into threads;
    /// see also `STOCKPILE_THREAD_METAL`.
    ///
    /// The value presumably determines the probability of this product extracted.
    #[serde(alias = "THREAD_METAL")]
    pub thread_metal: Option<(ReferenceTo<InorganicToken>, u32)>,
    // endregion ==================================================================================
    // region: Material definition tokens =========================================================
    /// List of syndromes tied to the material.
    #[serde(alias = "SYNDROME")]
    pub syndrome: Vec<SyndromeToken>,
    /// Overrides the name of `BOULDER` items (i.e. mined-out stones) made of the material (used for
    /// native copper/silver/gold/platinum to make them be called "nuggets" instead of "boulders").
    #[serde(alias = "STONE_NAME")]
    pub stone_name: Option<String>,
    /// Used to indicate that said material is a gemstone - when tiles are mined out, rough gems
    /// will be yielded instead of boulders. Plural can be "STP" to automatically append an "s" to
    /// the singular form, and `OVERWRITE_SOLID` will override the relevant `STATE_NAME` and
    /// `STATE_ADJ` values.
    #[serde(alias = "IS_GEM")]
    pub is_gem: Option<(
        String,
        Choose<StandardPluralEnum, String>,
        Option<OverwriteSolidEnum>,
    )>,
    /// Specifies what the material should be treated as when drinking water contaminated by it, for
    /// generating unhappy thoughts.
    #[serde(alias = "TEMP_DIET_INFO")]
    pub temp_diet_info: Option<DietInfoEnum>,
    /// Allows the material to be used as dye, and defines color of dyed items.
    #[serde(alias = "POWDER_DYE")]
    pub powder_dye: Option<ReferenceTo<ColorToken>>,
    /// Specifies the tile that will be used to represent unmined tiles made of this material.
    /// Generally only used with stones. Defaults to 219 ('█').
    #[serde(alias = "TILE")]
    pub tile: Option<DFChar>,
    /// Specifies the tile that will be used to represent `BOULDER` items made of this material.
    /// Generally only used with stones. Defaults to 7 ('•').
    #[serde(alias = "ITEM_SYMBOL")]
    pub item_symbol: Option<DFChar>,
    /// The on-screen color of the material. Uses a standard 3-digit color token. Equivalent to
    /// `[TILE_COLOR:a:b:c]`, `[BUILD_COLOR:b:a:X]` (X = 1 if 'a' equals 'b', 0 otherwise), and
    /// `[BASIC_COLOR:a:c]`.
    #[serde(alias = "DISPLAY_COLOR")]
    pub display_color: Option<(u8, u8, u8)>,
    /// The color of objects made of this material which use both the foreground and background
    /// color: doors, floodgates, hatch covers, bins, barrels, and cages. Defaults to 7:7:1 (white).
    #[serde(alias = "BUILD_COLOR")]
    pub build_color: Option<(u8, u8, u8)>,
    /// The color of unmined tiles containing this material (for stone and soil), as well as
    /// engravings in this material. Defaults to 7:7:1 (white).
    #[serde(alias = "TILE_COLOR")]
    pub tile_color: Option<(u8, u8, u8)>,
    /// The color of objects made of this material which use only the foreground color, including
    /// workshops, floors and boulders, and smoothed walls. Defaults to 7:1 (white).
    #[serde(alias = "BASIC_COLOR")]
    pub basic_color: Option<(u8, u8)>,
    /// Determines the color of the material at the specified state. Colors come from
    /// `descriptor_color_standard.txt`. The nearest color value is used to display contaminants
    /// and body parts made of this material.
    #[serde(alias = "STATE_COLOR")]
    pub state_color: Vec<(
        Choose<MaterialStateEnum, AllOrAllSolidEnum>,
        ReferenceTo<ColorToken>,
    )>,
    /// Determines the name of the material at the specified state, as displayed in-game.
    #[serde(alias = "STATE_NAME")]
    pub state_name: Vec<(Choose<MaterialStateEnum, AllOrAllSolidEnum>, String)>,
    /// Like `STATE_NAME`, but used in different situations. Equipment made from the material uses
    /// the state adjective and not the state name.
    #[serde(alias = "STATE_ADJ")]
    pub state_adj: Vec<(Choose<MaterialStateEnum, AllOrAllSolidEnum>, String)>,
    /// Sets both `STATE_NAME` and `STATE_ADJ` at the same time.
    #[serde(alias = "STATE_NAME_ADJ")]
    pub state_name_adj: Vec<(Choose<MaterialStateEnum, AllOrAllSolidEnum>, String)>,
    /// The material's tendency to absorb liquids. Containers made of materials with nonzero
    /// absorption cannot hold liquids unless they have been glazed. Defaults to 0.
    #[serde(alias = "ABSORPTION")]
    pub absorption: Option<u32>,
    /// Specifies how hard of an impact (in kilopascals) the material can withstand before it will
    /// start deforming permanently. Used for blunt-force combat. Defaults to 10000.
    #[serde(alias = "IMPACT_YIELD")]
    pub impact_yield: Option<u32>,
    /// Specifies how hard of an impact the material can withstand before it will fail entirely.
    /// Used for blunt-force combat. Defaults to 10000.
    #[serde(alias = "IMPACT_FRACTURE")]
    pub impact_fracture: Option<u32>,
    /// Specifies how much the material will have given (in parts-per-100000) when the yield point
    /// is reached. Used for blunt-force combat. Defaults to 0.
    ///
    /// Apparently affects in combat whether the corresponding tissue is bruised (value >= 50000),
    /// torn (value between 25000 and 49999), or fractured (value <= 24999).
    #[serde(alias = "IMPACT_STRAIN_AT_YIELD", alias = "IMPACT_ELASTICITY")]
    pub impact_strain_at_yield: Option<u32>,
    /// Specifies how hard the material can be compressed before it will start deforming
    /// permanently. Determines a tissue's resistance to pinching and response to strangulation.
    /// Defaults to 10000.
    #[serde(alias = "COMPRESSIVE_YIELD")]
    pub compressive_yield: Option<u32>,
    /// Specifies how hard the material can be compressed before it will fail entirely. Determines a
    /// tissue's resistance to pinching and response to strangulation. Defaults to 10000.
    #[serde(alias = "COMPRESSIVE_FRACTURE")]
    pub compressive_fracture: Option<u32>,
    /// Specifies how much the material will have given when it has been compressed to its yield
    /// point. Determines a tissue's resistance to pinching and response to strangulation. Defaults
    /// to 0.
    #[serde(
        alias = "COMPRESSIVE_STRAIN_AT_YIELD",
        alias = "COMPRESSIVE_ELASTICITY"
    )]
    pub compressive_strain_at_yield: Option<u32>,
    /// Specifies how hard the material can be stretched before it will start deforming permanently.
    /// Determines a tissue's resistance to a latching and tearing bite. Defaults to 10000.
    #[serde(alias = "TENSILE_YIELD")]
    pub tensile_yield: Option<u32>,
    /// Specifies how hard the material can be stretched before it will fail entirely. Determines a
    /// tissue's resistance to a latching and tearing bite. Defaults to 10000.
    #[serde(alias = "TENSILE_FRACTURE")]
    pub tensile_fracture: Option<u32>,
    /// Specifies how much the material will have given when it is stretched to its yield point.
    /// Determines a tissue's resistance to a latching and tearing bite. Defaults to 0.
    #[serde(alias = "TENSILE_STRAIN_AT_YIELD", alias = "TENSILE_ELASTICITY")]
    pub tensile_strain_at_yield: Option<u32>,
    /// Specifies how hard the material can be twisted before it will start deforming permanently.
    /// Used for latching and shaking with a blunt attack (no default creature has such an attack,
    /// but they can be modded in). Defaults to 10000.
    #[serde(alias = "TORSION_YIELD")]
    pub torsion_yield: Option<u32>,
    /// Specifies how hard the material can be twisted before it will fail entirely. Used for
    /// latching and shaking with a blunt attack (no default creature has such an attack, but they
    /// can be modded in). Defaults to 10000.
    #[serde(alias = "TORSION_FRACTURE")]
    pub torsion_fracture: Option<u32>,
    /// Specifies how much the material will have given when it is twisted to its yield point. Used
    /// for latching and shaking with a blunt attack (no default creature has such an attack, but
    /// they can be modded in). Defaults to 0.
    #[serde(alias = "TORSION_STRAIN_AT_YIELD", alias = "TORSION_ELASTICITY")]
    pub torsion_strain_at_yield: Option<u32>,
    /// Specifies how hard the material can be sheared before it will start deforming permanently.
    /// Used for cutting calculations. Defaults to 10000.
    #[serde(alias = "SHEAR_YIELD")]
    pub shear_yield: Option<u32>,
    /// Specifies how hard the material can be sheared before it will fail entirely. Used for
    /// cutting calculations. Defaults to 10000.
    #[serde(alias = "SHEAR_FRACTURE")]
    pub shear_fracture: Option<u32>,
    /// Specifies how much the material will have given when sheared to its yield point. Used for
    /// cutting calculations. Defaults to 0.
    #[serde(alias = "SHEAR_STRAIN_AT_YIELD", alias = "SHEAR_ELASTICITY")]
    pub shear_strain_at_yield: Option<u32>,
    /// Specifies how hard the material can be bent before it will start deforming permanently.
    /// Determines a tissue's resistance to being mangled with a joint lock. Defaults to 10000.
    #[serde(alias = "BENDING_YIELD")]
    pub bending_yield: Option<u32>,
    /// Specifies how hard the material can be bent before it will fail entirely. Determines a
    /// tissue's resistance to being mangled with a joint lock. Defaults to 10000.
    #[serde(alias = "BENDING_FRACTURE")]
    pub bending_fracture: Option<u32>,
    /// Specifies how much the material will have given when bent to its yield point. Determines a
    /// tissue's resistance to being mangled with a joint lock. Defaults to 0.
    #[serde(alias = "BENDING_STRAIN_AT_YIELD", alias = "BENDING_ELASTICITY")]
    pub bending_strain_at_yield: Option<u32>,
    /// How sharp the material is. Used in cutting calculations. Does not allow an inferior metal to
    /// penetrate superior armor. Applying a value of at least 10000 to a stone will allow weapons
    /// to be made from that stone. Defaults to 10000.
    #[serde(alias = "MAX_EDGE")]
    pub max_edge: Option<u32>,
    /// Value modifier for the material. Defaults to 1. This number can be made negative by placing
    /// a "-" in front, resulting in things that you are paid to buy and must pay to sell.
    #[serde(alias = "MATERIAL_VALUE")]
    pub material_value: Option<i32>,
    /// Rate at which the material heats up or cools down (in joules/kilokelvin). If set to `NONE`,
    /// the temperature will be fixed at its initial value.
    /// See [Temperature](https://dwarffortresswiki.org/index.php/Temperature) for more information.
    /// Defaults to `NONE`.
    #[serde(alias = "SPEC_HEAT")]
    pub spec_heat: Option<Choose<u32, NoneEnum>>,
    /// Temperature above which the material takes damage from heat. May be set to `NONE`. If the
    /// material has an ignite point but no heatdam point, it will burn for a very long time (9
    /// months and 16.8 days). Defaults to `NONE`.
    #[serde(alias = "HEATDAM_POINT")]
    pub heatdam_point: Option<Choose<u32, NoneEnum>>,
    /// Temperature below which the material takes damage from cold. Defaults to `NONE`.
    #[serde(alias = "COLDDAM_POINT")]
    pub colddam_point: Option<Choose<u32, NoneEnum>>,
    /// Temperature at which the material will catch fire. Defaults to `NONE`.
    #[serde(alias = "IGNITE_POINT")]
    pub ignite_point: Option<Choose<u32, NoneEnum>>,
    /// Temperature at which the material melts. Defaults to `NONE`.
    #[serde(alias = "MELTING_POINT")]
    pub melting_point: Option<Choose<u32, NoneEnum>>,
    /// Temperature at which the material boils. Defaults to `NONE`.
    #[serde(alias = "BOILING_POINT")]
    pub boiling_point: Option<Choose<u32, NoneEnum>>,
    /// Items composed of this material will initially have this temperature. Used in conjunction
    /// with `[SPEC_HEAT:NONE]` to make material's temperature fixed at the specified value.
    /// Defaults to `NONE`.
    #[serde(alias = "MAT_FIXED_TEMP")]
    pub mat_fixed_temp: Option<Choose<u32, NoneEnum>>,
    /// Specifies the density (in kilograms per cubic meter) of the material when in solid form.
    /// Also affects combat calculations; affects blunt-force damage and ability of edged weapons to
    /// pierce tissue layers. Defaults to `NONE`.
    #[serde(alias = "SOLID_DENSITY")]
    pub solid_density: Option<Choose<u32, NoneEnum>>,
    /// Specifies the density of the material when in liquid form. Defaults to `NONE`.
    #[serde(alias = "LIQUID_DENSITY")]
    pub liquid_density: Option<Choose<u32, NoneEnum>>,
    /// Supposedly not used. Theoretically, should determine density (at given pressure) in
    /// gas state, on which in turn would depend (together with weight of vaporized material) on the
    /// volume covered by spreading vapors. Defaults to `NONE`.
    #[serde(alias = "MOLAR_MASS")]
    pub molar_mass: Option<Choose<u32, NoneEnum>>,
    /// Specifies the type of container used to store the material. Used in conjunction with the
    /// `[EXTRACT_BARREL]`, `[EXTRACT_VIAL]`, or `[EXTRACT_STILL_VIAL]` plant tokens. Defaults to
    /// `BARREL`.
    #[serde(alias = "EXTRACT_STORAGE")]
    pub extract_storage: Option<Reference>, // TODO: ref is container type
    /// Specifies the item type used for butchering results made of this material. Stock raws use
    /// `GLOB:NONE` for fat and `MEAT:NONE` for other meat materials.
    #[serde(alias = "BUTCHER_SPECIAL")]
    pub butcher_special: Option<ItemReferenceArg>,
    /// When a creature is butchered, meat yielded from organs made from this material will be named
    /// via this token.
    #[serde(alias = "MEAT_NAME")]
    pub meat_name: Option<(Choose<NoneEnum, String>, String, String)>,
    /// Specifies the name of blocks made from this material.
    #[serde(alias = "BLOCK_NAME")]
    pub block_name: Option<(String, Choose<StandardPluralEnum, String>)>,
    /// Used with reaction raws to associate a reagent material with a product material. The first
    /// argument is used by `HAS_MATERIAL_REACTION_PRODUCT` and `GET_MATERIAL_FROM_REAGENT` in
    /// reaction raws. The remainder is a material reference, generally `LOCAL_CREATURE_MAT:SUBTYPE`
    /// or `LOCAL_PLANT_MAT:SUBTYPE` or `INORGANIC:STONETYPE`.
    #[serde(alias = "MATERIAL_REACTION_PRODUCT")]
    pub material_reaction_product: Vec<(ReferenceTo<ReactionToken>, MaterialTokenArg)>,
    /// Used with reaction raws to associate a reagent material with a complete item. The first
    /// argument is used by `HAS_ITEM_REACTION_PRODUCT` and `GET_ITEM_DATA_FROM_REAGENT` in reaction
    /// raws. The rest refers to the type of item, then its material.
    #[serde(alias = "ITEM_REACTION_PRODUCT")]
    pub item_reaction_product: Vec<(Reference, ItemReferenceArg, MaterialTokenArg)>,
    /// Used to classify all items made of the material, so that reactions can use them as generic
    /// reagents.
    ///
    /// In default raws, the following classes are used:
    /// - `FAT`, `TALLOW`, `SOAP`, `PARCHMENT`, `PAPER_PLANT`, `PAPER_SLURRY`, `MILK`, `CHEESE`, `WAX`
    /// - `CAN_GLAZE` - items made from this material can be glazed.
    /// - `FLUX` - can be used as flux in pig iron and steel making.
    /// - `GYPSUM` - can be processed into gypsum plaster.
    /// - `CALCIUM_CARBONATE` - can be used in production of quicklime.
    #[serde(alias = "REACTION_CLASS")]
    pub reaction_class: Vec<Reference>,
    /// Allows the material to be used to make casts.
    #[serde(alias = "HARDENS_WITH_WATER")]
    pub hardens_with_water: Option<MaterialTokenArg>,
    /// Soap has `[SOAP_LEVEL:2]`. Effects unknown. Defaults to 0.
    #[serde(alias = "SOAP_LEVEL")]
    pub soap_level: Option<u32>,
    // region: Material usage tokens (no args) ====================================================
    /// Lets the game know that an animal was likely killed in the production of this item. Entities
    /// opposed to killing animals (which currently does not include Elves) will refuse to accept
    /// these items in trade.
    #[serde(alias = "IMPLIES_ANIMAL_KILL")]
    pub implies_animal_kill: Flag,
    /// Classifies the material as plant-based alcohol, allowing its storage in food stockpiles
    /// under "Drink (Plant)".
    #[serde(alias = "ALCOHOL_PLANT")]
    pub alcohol_plant: Flag,
    /// Classifies the material as animal-based alcohol, allowing its storage in food stockpiles
    /// under "Drink (Animal)".
    #[serde(alias = "ALCOHOL_CREATURE")]
    pub alcohol_creature: Flag,
    /// Classifies the material as generic alcohol. Implied by both `ALCOHOL_PLANT` and
    /// `ALCOHOL_CREATURE`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "ALCOHOL")]
    pub alcohol: Flag,
    /// Classifies the material as plant-based cheese, allowing its storage in food stockpiles
    /// under "Cheese (Plant)".
    #[serde(alias = "CHEESE_PLANT")]
    pub cheese_plant: Flag,
    /// Classifies the material as animal-based cheese, allowing its storage in food stockpiles
    /// under "Cheese (Animal)".
    #[serde(alias = "CHEESE_CREATURE")]
    pub cheese_creature: Flag,
    /// Classifies the material as generic cheese. Implied by both `CHEESE_PLANT` and
    /// `CHEESE_CREATURE`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "CHEESE")]
    pub cheese: Flag,
    /// Classifies the material as plant powder, allowing its storage in food stockpiles under
    /// "Milled Plant".
    #[serde(alias = "POWDER_MISC_PLANT")]
    pub powder_misc_plant: Flag,
    /// Classifies the material as creature powder, allowing its storage in food stockpiles under
    /// "Bone Meal".
    #[serde(alias = "POWDER_MISC_CREATURE")]
    pub powder_misc_creature: Flag,
    /// Classifies the material as generic powder. Implied by both `POWDER_MISC_PLANT` and
    /// `POWDER_MISC_CREATURE`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "POWDER_MISC")]
    pub powder_misc: Flag,
    /// Permits globs of the material in solid form to be stored in food stockpiles under "Fat" -
    /// without it, dwarves will come by and "clean" the items, destroying them (unless
    /// `[DO_NOT_CLEAN_GLOB]` is also included).
    #[serde(alias = "STOCKPILE_GLOB", alias = "STOCKPILE_GLOB_SOLID")]
    pub stockpile_glob: Flag,
    /// Classifies the material as milled paste, allowing its storage in food stockpiles under
    /// "Paste".
    #[serde(alias = "STOCKPILE_GLOB_PASTE")]
    pub stockpile_glob_paste: Flag,
    /// Classifies the material as pressed goods, allowing its storage in food stockpiles under
    /// "Pressed Material".
    #[serde(alias = "STOCKPILE_GLOB_PRESSED")]
    pub stockpile_glob_pressed: Flag,
    /// Classifies the material as a plant growth (e.g. fruits, leaves), allowing its storage in
    /// food stockpiles under Plant Growth/Fruit.
    #[serde(alias = "STOCKPILE_PLANT_GROWTH")]
    pub stockpile_plant_growth: Flag,
    /// Classifies the material as a plant extract, allowing its storage in food stockpiles under
    /// "Extract (Plant)".
    #[serde(alias = "LIQUID_MISC_PLANT")]
    pub liquid_misc_plant: Flag,
    /// Classifies the material as a creature extract, allowing its storage in food stockpiles under
    /// "Extract (Animal)".
    #[serde(alias = "LIQUID_MISC_CREATURE")]
    pub liquid_misc_creature: Flag,
    /// Classifies the material as a miscellaneous liquid, allowing its storage in food stockpiles
    /// under "Misc. Liquid" along with lye.
    #[serde(alias = "LIQUID_MISC_OTHER")]
    pub liquid_misc_other: Flag,
    /// Classifies the material as a generic liquid. Implied by `LIQUID_MISC_PLANT`,
    /// `LIQUID_MISC_CREATURE`, and `LIQUID_MISC_OTHER`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "LIQUID_MISC")]
    pub liquid_misc: Flag,
    /// Classifies the material as a plant, allowing its storage in food stockpiles under "Plants".
    #[serde(alias = "STRUCTURAL_PLANT_MAT")]
    pub structural_plant_mat: Flag,
    /// Classifies the material as a plant seed, allowing its storage in food stockpiles under
    /// "Seeds".
    #[serde(alias = "SEED_MAT")]
    pub seed_mat: Flag,
    /// Classifies the material as bone, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "BONE")]
    pub bone: Flag,
    /// Classifies the material as wood, allowing its use for carpenters and storage in wood
    /// stockpiles. Entities opposed to killing plants (i.e. Elves) will refuse to accept these
    /// items in trade.
    #[serde(alias = "WOOD")]
    pub wood: Flag,
    /// Classifies the material as plant fiber, allowing its use for clothiers and storage in cloth
    /// stockpiles under "Thread (Plant)" and "Cloth (Plant)".
    #[serde(alias = "THREAD_PLANT")]
    pub thread_plant: Flag,
    /// Classifies the material as tooth, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "TOOTH")]
    pub tooth: Flag,
    /// Classifies the material as horn, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "HORN")]
    pub horn: Flag,
    /// Classifies the material as pearl, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "PEARL")]
    pub pearl: Flag,
    /// Classifies the material as shell, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "SHELL")]
    pub shell: Flag,
    /// Classifies the material as leather, allowing its use for leatherworkers and storage in
    /// leather stockpiles.
    #[serde(alias = "LEATHER")]
    pub leather: Flag,
    /// Classifies the material as silk, allowing its use for clothiers and storage in cloth
    /// stockpiles under "Thread (Silk)" and "Cloth (Silk)".
    #[serde(alias = "SILK")]
    pub silk: Flag,
    /// Classifies the material as soap, allowing it to be used as a bath detergent and stored in
    /// bar/block stockpiles under "Bars: Other Materials".
    #[serde(alias = "SOAP")]
    pub soap: Flag,
    /// Material generates miasma when it rots.
    #[serde(alias = "GENERATES_MIASMA")]
    pub generates_miasma: Flag,
    /// Classifies the material as edible meat.
    #[serde(alias = "MEAT")]
    pub meat: Flag,
    /// Material will rot if not stockpiled appropriately. Currently only affects food and refuse,
    /// other items made of this material will not rot.
    #[serde(alias = "ROTS")]
    pub rots: Flag,
    /// Tells the game to classify contaminants of this material as being "blood" in Adventurer mode
    /// tile descriptions ("Here we have a Dwarf in a slurry of blood.").
    #[serde(alias = "BLOOD_MAP_DESCRIPTOR")]
    pub blood_map_descriptor: Flag,
    /// Tells the game to classify contaminants of this material as being "ichor".
    #[serde(alias = "ICHOR_MAP_DESCRIPTOR")]
    pub ichor_map_descriptor: Flag,
    /// Tells the game to classify contaminants of this material as being "goo".
    #[serde(alias = "GOO_MAP_DESCRIPTOR")]
    pub goo_map_descriptor: Flag,
    /// Tells the game to classify contaminants of this material as being "slime".
    #[serde(alias = "SLIME_MAP_DESCRIPTOR")]
    pub slime_map_descriptor: Flag,
    /// Tells the game to classify contaminants of this material as being "pus".
    #[serde(alias = "PUS_MAP_DESCRIPTOR")]
    pub pus_map_descriptor: Flag,
    /// Tells the game to classify contaminants of this material as being "sweat".
    #[serde(alias = "SWEAT_MAP_DESCRIPTOR")]
    pub sweat_map_descriptor: Flag,
    /// Tells the game to classify contaminants of this material as being "tears".
    #[serde(alias = "TEARS_MAP_DESCRIPTOR")]
    pub tears_map_descriptor: Flag,
    /// Tells the game to classify contaminants of this material as being "spit".
    #[serde(alias = "SPIT_MAP_DESCRIPTOR")]
    pub spit_map_descriptor: Flag,
    /// Contaminants composed of this material evaporate over time, slowly disappearing from the
    /// map. Used internally by water.
    #[serde(alias = "EVAPORATES")]
    pub evaporates: Flag,
    /// Used for materials which cause syndromes, causes it to enter the creature's blood instead of
    /// simply spattering on the surface.
    #[serde(alias = "ENTERS_BLOOD")]
    pub enters_blood: Flag,
    /// Can be eaten by vermin.
    #[serde(alias = "EDIBLE_VERMIN")]
    pub edible_vermin: Flag,
    /// Can be eaten raw.
    #[serde(alias = "EDIBLE_RAW")]
    pub edible_raw: Flag,
    /// Can be cooked and then eaten.
    #[serde(alias = "EDIBLE_COOKED")]
    pub edible_cooked: Flag,
    /// Prevents globs made of this material from being cleaned up and destroyed.
    #[serde(alias = "DO_NOT_CLEAN_GLOB")]
    pub do_not_clean_glob: Flag,
    /// Prevents the material from showing up in Stone stockpile settings.
    #[serde(alias = "NO_STONE_STOCKPILE")]
    pub no_stone_stockpile: Flag,
    /// Allows the creation of metal furniture at the metalsmith's forge.
    #[serde(alias = "ITEMS_METAL")]
    pub items_metal: Flag,
    /// Equivalent to `ITEMS_HARD`. Given to bone.
    #[serde(alias = "ITEMS_BARRED")]
    pub items_barred: Flag,
    /// Equivalent to `ITEMS_HARD`. Given to shell.
    #[serde(alias = "ITEMS_SCALED")]
    pub items_scaled: Flag,
    /// Equivalent to `ITEMS_SOFT`. Given to leather.
    #[serde(alias = "ITEMS_LEATHER")]
    pub items_leather: Flag,
    /// Random crafts made from this material cannot be made into rings, crowns, scepters or
    /// figurines. Given to plant fiber, silk and wool.
    #[serde(alias = "ITEMS_SOFT")]
    pub items_soft: Flag,
    /// Random crafts made from this material include all seven items. Given to stone, wood, bone,
    /// shell, chitin, claws, teeth, horns, hooves and beeswax. Hair, pearls and eggshells also have
    /// the tag.
    #[serde(alias = "ITEMS_HARD")]
    pub items_hard: Flag,
    /// Used to define that the material is a stone. Allows its usage in masonry and stonecrafting
    /// and storage in stone stockpiles, among other effects.
    #[serde(alias = "IS_STONE")]
    pub is_stone: Flag,
    /// Used for a stone that cannot be dug into.
    #[serde(alias = "UNDIGGABLE")]
    pub undiggable: Flag,
    /// Causes containers made of this material to be prefixed with "unglazed" if they have not yet
    /// been glazed.
    #[serde(alias = "DISPLAY_UNGLAZED")]
    pub display_unglazed: Flag,
    /// Classifies the material as yarn, allowing its use for clothiers and its storage in cloth
    /// stockpiles under "Thread (Yarn)" and "Cloth (Yarn)".
    #[serde(alias = "YARN")]
    pub yarn: Flag,
    /// Classifies the material as metal thread, permitting thread and cloth to be stored in cloth
    /// stockpiles under "Thread (Metal)" and "Cloth (Metal)".
    #[serde(alias = "STOCKPILE_THREAD_METAL")]
    pub stockpile_thread_metal: Flag,
    /// Defines the material as being metal, allowing it to be used at forges.
    #[serde(alias = "IS_METAL")]
    pub is_metal: Flag,
    /// Used internally by green glass, clear glass, and crystal glass.
    #[serde(alias = "IS_GLASS")]
    pub is_glass: Flag,
    /// Can be used in the production of crystal glass.
    #[serde(alias = "CRYSTAL_GLASSABLE")]
    pub crystal_glassable: Flag,
    /// Melee weapons can be made out of this material.
    #[serde(alias = "ITEMS_WEAPON")]
    pub items_weapon: Flag,
    /// Ranged weapons can be made out of this material.
    #[serde(alias = "ITEMS_WEAPON_RANGED")]
    pub items_weapon_ranged: Flag,
    /// Anvils can be made out of this material.
    #[serde(alias = "ITEMS_ANVIL")]
    pub items_anvil: Flag,
    /// Ammunition can be made out of this material.
    #[serde(alias = "ITEMS_AMMO")]
    pub items_ammo: Flag,
    /// Picks can be made out of this material.
    #[serde(alias = "ITEMS_DIGGER")]
    pub items_digger: Flag,
    /// Armor can be made out of this material.
    #[serde(alias = "ITEMS_ARMOR")]
    pub items_armor: Flag,
    /// Used internally by amber and coral. Functionally equivalent to `ITEMS_HARD`.
    #[serde(alias = "ITEMS_DELICATE")]
    pub items_delicate: Flag,
    /// Siege engine parts can be made out of this material. Does not appear to work.
    #[serde(alias = "ITEMS_SIEGE_ENGINE")]
    pub items_siege_engine: Flag,
    /// Querns and millstones can be made out of this material.
    #[serde(alias = "ITEMS_QUERN")]
    pub items_quern: Flag,
    // endregion ==================================================================================
    // endregion ==================================================================================
}
