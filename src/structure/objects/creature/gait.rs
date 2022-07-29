use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum GaitTypeEnum {
    /// Used for moving normally over flat ground tiles, and up and down ramps and stairs.
    ///
    /// Walking gaits are land-based gaits which require the creature to be standing up,
    /// and have more than half of their `[STANCE]` body parts, e.g. legs, intact and working.
    #[serde(alias = "WALK")]
    Walk,
    /// Used for moving over ground tiles whilst
    /// [prone](https://dwarffortresswiki.org/index.php/Status_icon#Non-flashing).
    ///
    /// Unlike walking gaits, crawling gaits do not require either standing up,
    /// or `[STANCE]` body parts. They are much slower than walking gaits in general.
    ///
    /// Please note that an uninjured, slithering snake is considered to be using a walking gait,
    /// not a crawling gait: its body is its `[STANCE]` body part. If a snake is injured in the body,
    /// it will revert to a crawling gait.
    #[serde(alias = "CRAWL")]
    Crawl,
    /// Used for moving through tiles containing water or magma at a depth of at least 4/7.
    ///
    /// In order to swim, a creature needs either the `[SWIMS_INNATE]` tag, or `[SWIMS_LEARNED]`
    /// along with `[CAN_LEARN]` and skill in swimming.
    #[serde(alias = "SWIM")]
    Swim,
    /// Used for moving through open space. An "Open space" is a map tile state that indicates there
    /// is nothing there. No floor, no walls, no creatures, absolutely nothing.
    ///
    /// In both Dwarf mode and Adventure mode, you can look at the world around you. A description
    /// of the tile being examined will be displayed on the right hand side of the screen.
    ///
    /// In order to fly, a creature needs the `[FLIER]` tag, and for enough of its body parts tagged
    /// `[FLIER]` (e.g. wings) to be intact, if applicable. Flying does not require a minimum speed
    /// to stay airborne, and turning while flying is no more difficult than turning while walking.
    #[serde(alias = "FLY")]
    Fly,
    /// Used for moving whilst [climbing](https://dwarffortresswiki.org/index.php/Climber).
    ///
    /// Climbing gaits are used for moving up and down vertical surfaces, such as trees or walls,
    /// as well as for moving horizontally while supporting oneself against a vertical surface.
    /// In order to climb, a creature needs intact body parts to climb with: `[GRASP]` body parts
    /// by default, or `[STANCE]` body parts if the creature has the `[STANCE_CLIMBER]` token.
    /// Stance climbers include cats and giant cave spiders.
    #[serde(alias = "CLIMB")]
    Climb,
}
impl Default for GaitTypeEnum {
    fn default() -> Self {
        Self::Walk
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NoBuildUpEnum {
    /// Can be specified instead of a `<start speed>` value to make the `<max speed>` instantly
    /// achievable upon initiating movement (this is equivalent to a `<build up time>` of 0).
    ///
    /// Note that `<build up time>` and `<max turning speed>` are both ignored if specified
    /// alongside this (as `NO_BUILD_UP` trumps `<build up time>` and preserves `<max speed>` whilst
    /// turning, and `<max turning speed>` cannot exceed `<max speed>`) so they should both be
    /// emitted when using `NO_BUILD_UP`.
    #[serde(alias = "NO_BUILD_UP")]
    NoBuildUp,
}
impl Default for NoBuildUpEnum {
    fn default() -> Self {
        Self::NoBuildUp
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GaitFlagTokenArg {
    /// Makes `THICKENS_ON_ENERGY_STORAGE` and `THICKENS_ON_STRENGTH` tissue layers slow movement
    /// depending on how thick they are. Adding the `STRENGTH` gait flag counteracts the impact of
    /// the latter layer.
    pub layers_slow: Option<()>,
    /// Speeds/slows movement depending on the creature's
    /// [Strength](https://dwarffortresswiki.org/index.php/Attribute#Strength) stat.
    pub strength: Option<()>,
    /// Speeds/slows movement depending on the creature's
    /// [Agility](https://dwarffortresswiki.org/index.php/Attribute#Agility) stat.
    pub agility: Option<()>,
    /// Slows movement by the specified percentage when the creature is
    /// [sneaking](https://dwarffortresswiki.org/index.php/Ambusher).
    pub stealth_slows: Option<i32>,
}
