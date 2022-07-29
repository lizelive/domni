use crate::core::{Choose, Clamp, DFChar, Reference, ReferenceTo, Flag};

use serde::{Deserialize, Serialize};

use crate::structure::{MusicSkillEnum, SkillEnum};

#[allow(clippy::large_enum_variant)]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ItemToken {
    #[serde(alias = "ITEM_AMMO")]
    AmmoToken(AmmoToken),
    #[serde(alias = "ITEM_ARMOR")]
    ArmorToken(ArmorToken),
    #[serde(alias = "ITEM_FOOD")]
    FoodToken(FoodToken),
    #[serde(alias = "ITEM_GLOVES")]
    GlovesToken(GlovesToken),
    #[serde(alias = "ITEM_HELM")]
    HelmToken(HelmToken),
    #[serde(alias = "ITEM_INSTRUMENT")]
    InstrumentToken(InstrumentToken),
    #[serde(alias = "ITEM_PANTS")]
    PantsToken(PantsToken),
    #[serde(alias = "ITEM_SHIELD")]
    ShieldToken(ShieldToken),
    #[serde(alias = "ITEM_SHOES")]
    ShoesToken(ShoesToken),
    #[serde(alias = "ITEM_SIEGEAMMO")]
    SiegeAmmoToken(SiegeAmmoToken),
    #[serde(alias = "ITEM_TOOL")]
    ToolToken(ToolToken),
    #[serde(alias = "ITEM_TOY")]
    ToyToken(ToyToken),
    #[serde(alias = "ITEM_TRAPCOMP")]
    TrapCompToken(TrapCompToken),
    #[serde(alias = "ITEM_WEAPON")]
    WeaponToken(WeaponToken),
}
impl Default for ItemToken {
    fn default() -> Self {
        Self::AmmoToken(AmmoToken::default())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AmmoToken {
    /// Argument 1 of `[ITEM_AMMO:...]`
    #[serde(alias = "ITEM_AMMO")]
    pub reference: Option<ReferenceTo<Self>>,
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// This ammo can be fired from a weapon that is set to fire the same ammo type.
    /// Defaults to `BOLT`.
    #[serde(alias = "CLASS")]
    pub class: Option<Reference>,
    /// How large the ammunition is.
    #[serde(alias = "SIZE")] // Required token
    pub size: Option<u32>,
    /// The attack used by this ammo when used as a melee weapon.
    #[serde(alias = "ATTACK")]
    pub attack: Option<ItemAttack>, // TODO: test ingame if WeaponAttack could be used instead here
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ArmorToken {
    /// Argument 1 of `[ITEM_ARMOR:...]`
    #[serde(alias = "ITEM_ARMOR")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Length of the sleeves, counted in `[LIMB]` body parts towards the hands. A value of 0 only
    /// protects both halves of the torso, 1 extends over the upper arms and so on. Regardless of
    /// the value, body armor can never extend to cover the hands or head.
    ///
    /// Currently bugged, high values of `UBSTEP` will result in the item protecting
    /// facial features, fingers, and toes, while leaving those parts that it cannot protect
    /// unprotected (but still counting them as steps).
    /// [Bug:1821](http://www.bay12games.com/dwarves/mantisbt/view.php?id=1821)
    #[serde(alias = "UBSTEP")]
    pub ubstep: Option<Choose<u8, MaxEnum>>,
    // region: Shared by garments/shields/trapcomp/weapons; are all required ======================
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// How much material is needed to make the item. Most important with bars. The number of bars
    /// required to make the item is the value divided by three.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    // endregion ==================================================================================
    // region: Shared by all garments =============================================================
    /// Adjective preceding the material name (e.g. "large copper dagger").
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// Metal versions of this item count as one `ARMORLEVEL` higher and thus won't be worn by
    /// random peasants. This tag will not work unless `ARMORLEVEL` is explicitly declared: if you
    /// leave out `ARMORLEVEL`, even metal armor will default to level 0.
    #[serde(alias = "METAL_ARMOR_LEVELS")]
    pub metal_armor_levels: Flag,
    /// Metal versions of this item will have "chain" added between the material and item name.
    #[serde(alias = "CHAIN_METAL_TEXT")]
    pub chain_metal_text: Flag,
    /// Clothiers can make this item from all kinds of cloth. If paired with `[LEATHER]`, the item
    /// has an equal chance of being either in randomly generated outfits. Further uses of this tag
    /// are unknown.
    #[serde(alias = "SOFT")]
    pub soft: Flag,
    /// Default state in the absence of a `[SOFT]` token. Actual effects unknown.
    #[serde(alias = "HARD")]
    pub hard: Flag,
    /// Item can be made from metal. Overrides `[SOFT]` and `[LEATHER]` in randomly generated
    /// outfits, if the `ARMORLEVEL` permits. Civilizations with `[WOOD_ARMOR]` will make this
    /// item out of wood instead.
    #[serde(alias = "METAL")]
    pub metal: Flag,
    /// Craftsmen can make this item from bones. Randomly generated outfits don't include bone
    /// armor.
    #[serde(alias = "BARRED")]
    pub barred: Flag,
    /// Craftsmen can make this item from shells. Randomly generated outfits don't include shell
    /// armor.
    #[serde(alias = "SCALED")]
    pub scaled: Flag,
    /// Leatherworkers can make this item from leather. If paired with `[SOFT]`, this item has an
    /// equal chance of being either in randomly generated outfits.
    #[serde(alias = "LEATHER")]
    pub leather: Flag,
    /// Only one shaped piece of clothing can be worn on a single body slot at a time.
    #[serde(alias = "SHAPED")]
    pub shaped: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, if lower.
    /// This makes the garment flex and give way instead of shattering under force. Strong materials
    /// that resist cutting will blunt edged attacks into bone-crushing hits instead.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_ALL")]
    pub structural_elasticity_chain_all: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, but only if
    /// the garment is made from metal.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_METAL")]
    pub structural_elasticity_chain_metal: Flag,
    /// Reduces the armor material's `SHEAR_YIELD` to 20000, `SHEAR_FRACTURE` to 30000 and increases
    /// the `*_STRAIN_AT_YIELD` properties to 50000, but only if the garment is made from cloth.
    /// This makes the item very weak against edged attacks, even if the thread material is
    /// normally very strong.
    #[serde(alias = "STRUCTURAL_ELASTICITY_WOVEN_THREAD")]
    pub structural_elasticity_woven_thread: Flag,
    /// The item's bulkiness when worn. Aside from the layer limitations, it's a big contributor to
    /// the thickness and weight (and therefore price) of the garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_SIZE")]
    pub layer_size: Option<u32>,
    /// The maximum amount of garments that can fit underneath this garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_PERMIT")]
    pub layer_permit: Option<u32>,
    /// Where the item goes in relation to other clothes. Socks cannot be worn on top of boots!
    ///
    /// The `LAYER_PERMIT` of the highest layer is used on a given section of the body - you can fit
    /// a lot of shirts and other undergarments underneath a robe, but not if you wear a leather
    /// jerkin on top of it, and you can still wear a cloak over the whole ensemble. Defaults to
    /// `UNDER`.
    #[serde(alias = "LAYER")]
    pub layer: Option<LayerEnum>,
    /// How often the garment gets in the way of a contaminant or an attack. Armor with a 5%
    /// coverage value, for example, will be near useless because 95% of attacks will bypass it
    /// completely. Temperature effects and armor thickness are also influenced. Defaults to 100.
    #[serde(alias = "COVERAGE")]
    pub coverage: Option<u8>,
    /// The garment's general purpose. Defaults to 1 for shields, 0 for everything else. Class 0
    /// items are claimed and used by civilians as ordinary clothing and are subject to wear.
    #[serde(alias = "ARMORLEVEL")]
    pub armorlevel: Option<u8>, // shared by all garments, and shields
    // endregion ==================================================================================
    // region: Shared by ARMOR and PANTS ==========================================================
    /// Changes the plural form of this item to "`phrase of` item". Primarily pertains to the stock
    /// screens.
    ///
    /// Example, "suits of" platemail, "pairs of" trousers, etc.
    #[serde(alias = "PREPLURAL")]
    pub preplural: Option<String>,
    /// If the item has no material associated with it (e.g. stockpile menus and trade
    /// negotiations), this will be displayed in its place. Used for leather armor in vanilla.
    #[serde(alias = "MATERIAL_PLACEHOLDER")]
    pub material_placeholder: Option<String>,
    /// Length of the legs/hem, counted in `[LIMB]` body parts towards the feet. A value of 0 only
    /// covers the lower body, 1 extends over the upper legs and so on. Regardless of the value,
    /// body armor or pants can never extend to cover the feet.
    #[serde(alias = "LBSTEP")]
    pub lbstep: Option<Choose<u8, MaxEnum>>,
    // endregion ==================================================================================
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FoodToken {
    /// Argument 1 of `[ITEM_FOOD:...]`
    #[serde(alias = "ITEM_FOOD")]
    pub reference: Option<ReferenceTo<Self>>,
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<String>,
    /// Specifies the number of ingredients that are used in this type of prepared meal:
    /// - 2 for Easy. (default)
    /// - 3 for Fine.
    /// - 4 for Lavish.
    #[serde(alias = "LEVEL")]
    pub level: Option<Clamp<u8, 2, 4>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GlovesToken {
    /// Argument 1 of `[ITEM_GLOVES:...]`
    #[serde(alias = "ITEM_GLOVES")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Length of gloves or footwear, counted in `[LIMB]` body parts towards the torso. A value of 1
    /// lets gloves cover the lower arms, a value of 2 stretches a boot all the way over the upper
    /// leg and so on.
    ///
    /// Regardless of the value, none of these items can ever extend to cover the upper or lower
    /// body. Shields also have this token, but it only seems to affect weight.
    #[serde(alias = "UPSTEP")]
    pub upstep: Option<Choose<u8, MaxEnum>>, // shared by glove, shield, shoes
    // region: Shared by garments/shields/trapcomp/weapons; are all required ======================
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// How much material is needed to make the item. Most important with bars. The number of bars
    /// required to make the item is the value divided by three.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    // endregion ==================================================================================
    // region: Shared by all garments =============================================================
    /// Adjective preceding the material name (e.g. "large copper dagger").
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// Metal versions of this item count as one `ARMORLEVEL` higher and thus won't be worn by
    /// random peasants. This tag will not work unless `ARMORLEVEL` is explicitly declared: if you
    /// leave out `ARMORLEVEL`, even metal armor will default to level 0.
    #[serde(alias = "METAL_ARMOR_LEVELS")]
    pub metal_armor_levels: Flag,
    /// Metal versions of this item will have "chain" added between the material and item name.
    #[serde(alias = "CHAIN_METAL_TEXT")]
    pub chain_metal_text: Flag,
    /// Clothiers can make this item from all kinds of cloth. If paired with `[LEATHER]`, the item
    /// has an equal chance of being either in randomly generated outfits. Further uses of this tag
    /// are unknown.
    #[serde(alias = "SOFT")]
    pub soft: Flag,
    /// Default state in the absence of a `[SOFT]` token. Actual effects unknown.
    #[serde(alias = "HARD")]
    pub hard: Flag,
    /// Item can be made from metal. Overrides `[SOFT]` and `[LEATHER]` in randomly generated
    /// outfits, if the `ARMORLEVEL` permits. Civilizations with `[WOOD_ARMOR]` will make this
    /// item out of wood instead.
    #[serde(alias = "METAL")]
    pub metal: Flag,
    /// Craftsmen can make this item from bones. Randomly generated outfits don't include bone
    /// armor.
    #[serde(alias = "BARRED")]
    pub barred: Flag,
    /// Craftsmen can make this item from shells. Randomly generated outfits don't include shell
    /// armor.
    #[serde(alias = "SCALED")]
    pub scaled: Flag,
    /// Leatherworkers can make this item from leather. If paired with `[SOFT]`, this item has an
    /// equal chance of being either in randomly generated outfits.
    #[serde(alias = "LEATHER")]
    pub leather: Flag,
    /// Only one shaped piece of clothing can be worn on a single body slot at a time.
    #[serde(alias = "SHAPED")]
    pub shaped: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, if lower.
    /// This makes the garment flex and give way instead of shattering under force. Strong materials
    /// that resist cutting will blunt edged attacks into bone-crushing hits instead.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_ALL")]
    pub structural_elasticity_chain_all: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, but only if
    /// the garment is made from metal.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_METAL")]
    pub structural_elasticity_chain_metal: Flag,
    /// Reduces the armor material's `SHEAR_YIELD` to 20000, `SHEAR_FRACTURE` to 30000 and increases
    /// the `*_STRAIN_AT_YIELD` properties to 50000, but only if the garment is made from cloth.
    /// This makes the item very weak against edged attacks, even if the thread material is
    /// normally very strong.
    #[serde(alias = "STRUCTURAL_ELASTICITY_WOVEN_THREAD")]
    pub structural_elasticity_woven_thread: Flag,
    /// The item's bulkiness when worn. Aside from the layer limitations, it's a big contributor to
    /// the thickness and weight (and therefore price) of the garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_SIZE")]
    pub layer_size: Option<u32>,
    /// The maximum amount of garments that can fit underneath this garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_PERMIT")]
    pub layer_permit: Option<u32>,
    /// Where the item goes in relation to other clothes. Socks cannot be worn on top of boots!
    ///
    /// The `LAYER_PERMIT` of the highest layer is used on a given section of the body - you can fit
    /// a lot of shirts and other undergarments underneath a robe, but not if you wear a leather
    /// jerkin on top of it, and you can still wear a cloak over the whole ensemble. Defaults to
    /// `UNDER`.
    #[serde(alias = "LAYER")]
    pub layer: Option<LayerEnum>,
    /// How often the garment gets in the way of a contaminant or an attack. Armor with a 5%
    /// coverage value, for example, will be near useless because 95% of attacks will bypass it
    /// completely. Temperature effects and armor thickness are also influenced. Defaults to 100.
    #[serde(alias = "COVERAGE")]
    pub coverage: Option<u8>,
    /// The garment's general purpose. Defaults to 1 for shields, 0 for everything else. Class 0
    /// items are claimed and used by civilians as ordinary clothing and are subject to wear.
    #[serde(alias = "ARMORLEVEL")] // shared by all garments, and shields
    pub armorlevel: Option<u8>,
    // endregion ==================================================================================
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmToken {
    /// Argument 1 of `[ITEM_HELM:...]`
    #[serde(alias = "ITEM_HELM")]
    pub reference: Option<ReferenceTo<Self>>,
    // region: Shared by garments/shields/trapcomp/weapons; are all required ======================
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// How much material is needed to make the item. Most important with bars. The number of bars
    /// required to make the item is the value divided by three.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    // endregion ==================================================================================
    // region: Shared by all garments =============================================================
    /// Adjective preceding the material name (e.g. "large copper dagger").
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// Metal versions of this item count as one `ARMORLEVEL` higher and thus won't be worn by
    /// random peasants. This tag will not work unless `ARMORLEVEL` is explicitly declared: if you
    /// leave out `ARMORLEVEL`, even metal armor will default to level 0.
    #[serde(alias = "METAL_ARMOR_LEVELS")]
    pub metal_armor_levels: Flag,
    /// Metal versions of this item will have "chain" added between the material and item name.
    #[serde(alias = "CHAIN_METAL_TEXT")]
    pub chain_metal_text: Flag,
    /// Clothiers can make this item from all kinds of cloth. If paired with `[LEATHER]`, the item
    /// has an equal chance of being either in randomly generated outfits. Further uses of this tag
    /// are unknown.
    #[serde(alias = "SOFT")]
    pub soft: Flag,
    /// Default state in the absence of a `[SOFT]` token. Actual effects unknown.
    #[serde(alias = "HARD")]
    pub hard: Flag,
    /// Item can be made from metal. Overrides `[SOFT]` and `[LEATHER]` in randomly generated
    /// outfits, if the `ARMORLEVEL` permits. Civilizations with `[WOOD_ARMOR]` will make this
    /// item out of wood instead.
    #[serde(alias = "METAL")]
    pub metal: Flag,
    /// Craftsmen can make this item from bones. Randomly generated outfits don't include bone
    /// armor.
    #[serde(alias = "BARRED")]
    pub barred: Flag,
    /// Craftsmen can make this item from shells. Randomly generated outfits don't include shell
    /// armor.
    #[serde(alias = "SCALED")]
    pub scaled: Flag,
    /// Leatherworkers can make this item from leather. If paired with `[SOFT]`, this item has an
    /// equal chance of being either in randomly generated outfits.
    #[serde(alias = "LEATHER")]
    pub leather: Flag,
    /// Only one shaped piece of clothing can be worn on a single body slot at a time.
    #[serde(alias = "SHAPED")]
    pub shaped: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, if lower.
    /// This makes the garment flex and give way instead of shattering under force. Strong materials
    /// that resist cutting will blunt edged attacks into bone-crushing hits instead.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_ALL")]
    pub structural_elasticity_chain_all: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, but only if
    /// the garment is made from metal.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_METAL")]
    pub structural_elasticity_chain_metal: Flag,
    /// Reduces the armor material's `SHEAR_YIELD` to 20000, `SHEAR_FRACTURE` to 30000 and increases
    /// the `*_STRAIN_AT_YIELD` properties to 50000, but only if the garment is made from cloth.
    /// This makes the item very weak against edged attacks, even if the thread material is
    /// normally very strong.
    #[serde(alias = "STRUCTURAL_ELASTICITY_WOVEN_THREAD")]
    pub structural_elasticity_woven_thread: Flag,
    /// The item's bulkiness when worn. Aside from the layer limitations, it's a big contributor to
    /// the thickness and weight (and therefore price) of the garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_SIZE")]
    pub layer_size: Option<u32>,
    /// The maximum amount of garments that can fit underneath this garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_PERMIT")]
    pub layer_permit: Option<u32>,
    /// Where the item goes in relation to other clothes. Socks cannot be worn on top of boots!
    ///
    /// The `LAYER_PERMIT` of the highest layer is used on a given section of the body - you can fit
    /// a lot of shirts and other undergarments underneath a robe, but not if you wear a leather
    /// jerkin on top of it, and you can still wear a cloak over the whole ensemble. Defaults to
    /// `UNDER`.
    #[serde(alias = "LAYER")]
    pub layer: Option<LayerEnum>,
    /// How often the garment gets in the way of a contaminant or an attack. Armor with a 5%
    /// coverage value, for example, will be near useless because 95% of attacks will bypass it
    /// completely. Temperature effects and armor thickness are also influenced. Defaults to 100.
    #[serde(alias = "COVERAGE")]
    pub coverage: Option<u8>,
    /// The garment's general purpose. Defaults to 1 for shields, 0 for everything else. Class 0
    /// items are claimed and used by civilians as ordinary clothing and are subject to wear.
    #[serde(alias = "ARMORLEVEL")] // shared by all garments, and shields
    pub armorlevel: Option<u8>,
    // endregion ==================================================================================
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstrumentToken {
    /// Argument 1 of `[ITEM_INSTRUMENT:...]`
    #[serde(alias = "ITEM_INSTRUMENT")]
    pub reference: Option<ReferenceTo<Self>>,
    // region: Unique to instruments ==============================================================
    /// Makes the instrument stationary.
    #[serde(alias = "PLACED_AS_BUILDING")]
    pub placed_as_building: Flag,
    /// Sets a piece as the central part of the instrument.
    #[serde(alias = "DOMINANT_MATERIAL_PIECE")]
    pub dominant_material_piece: Option<Reference>,
    /// Defines an instrument piece. The 1st argument is the identifier that can be used in other
    /// raw tags to refer to this instrument piece. The 2nd is the tool which is required
    /// (and consumed) during the construction process to create this instrument piece.
    ///
    /// If an instrument does not have any pieces, `SELF` can be used for any argument which needs
    /// to be an instrument piece.
    #[serde(alias = "INSTRUMENT_PIECE")]
    pub instrument_piece: Vec<(
        Reference,
        ReferenceTo<ToolToken>,
        String,
        String,
        NameTypeEnum,
    )>,
    /// The instrument's volume range, in millibels (100 mB = 1 dB).
    #[serde(alias = "VOLUME_mB")]
    pub volume_mb: Option<(u32, u32)>,
    /// Defines how a musician can produce sound when using this instrument. Can be used multiple
    /// times.
    #[serde(alias = "SOUND_PRODUCTION")]
    pub sound_production: Vec<(SoundProductionEnum, Reference, Option<Reference>)>,
    /// Defines how the pitch can be varied by the musician. Can be used multiple times.
    #[serde(alias = "PITCH_CHOICE")]
    pub pitch_choice: Vec<(PitchMethodEnum, Reference, Option<Reference>)>,
    /// Defines how the instrument may be tuned. Can be used multiple times.
    #[serde(alias = "TUNING")]
    pub tuning: Vec<(TuningMethodEnum, Reference)>,
    /// Pitch is `min`:`max` in cents with middle C at zero. There are 1200 cents in an octave.
    ///
    /// The game verbally differentiates values from -4200 to 4200, but you can go outside
    /// that if you like.  The in-game generated instruments will range from roughly C0 to C8
    /// (-4800 to 4800), sometimes beyond for really unusual ones.
    ///
    /// You can also use `[INDEFINITE_PITCH]` instead.
    #[serde(alias = "PITCH_RANGE")]
    pub pitch_range: Option<(i32, i32)>,
    /// You can add as many timbre words as you want. The generated timbres have a series of
    /// checks for conflicts, but they don't apply to the raws, so how you use them is up to you.
    #[serde(alias = "TIMBRE")]
    pub timbre: Option<(TimbreEnum, Vec<TimbreEnum>)>,
    /// The pitch range overrides the global pitch for a register, but the register timbres are
    /// added to the global ones. You can add as many timbre words as you want.
    ///
    /// Pitch is `min`:`max` in cents with middle C at zero. There are 1200 cents in an octave.
    ///
    /// The game verbally differentiates values from -4200 to 4200, but you can go outside
    /// that if you like.  The in-game generated instruments will range from roughly C0 to C8
    /// (-4800 to 4800), sometimes beyond for really unusual ones.
    ///
    /// You can also use `[INDEFINITE_PITCH]` instead.
    #[serde(alias = "REGISTER")]
    pub register: Vec<(i32, i32, TimbreEnum, Vec<TimbreEnum>)>,
    /// The skill used for playing this instrument.
    #[serde(alias = "MUSIC_SKILL")]
    pub music_skill: Option<MusicSkillEnum>,
    /// Can be used instead of either `REGISTER` or `PITCH_RANGE`.
    #[serde(alias = "INDEFINITE_PITCH")]
    pub indefinite_pitch: Flag,
    // endregion ==================================================================================

    // TODO: prune the following list/region of shared tokens, these aren't all actually shared.
    // The following 3 links may be useful for figuring out which are truly shared:
    // - https://github.com/DFHack/df-structures/blob/master/df.items.xml
    // - https://github.com/DFHack/df-structures/blob/master/df.item-raws.xml
    // - https://github.com/DFHack/df-structures/blob/master/df.item-vectors.xml
    // Also, the descriptions for most of them are almost definitely inapplicable for instruments.
    // region: Shared by tools and instruments ====================================================
    /// Volume of tool in mL or cubic centimeters. Required.
    #[serde(alias = "SIZE")]
    pub size: Option<u32>,
    /// Name of the tool. Required.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// Defines the item value of the tool. Required.
    #[serde(alias = "VALUE")]
    pub value: Option<u32>,
    /// Defines the tile used to represent the tool. Required.
    #[serde(alias = "TILE")]
    pub tile: Option<DFChar>,
    /// Permits the tool to be made from any bone.
    #[serde(alias = "BONE_MAT")]
    pub bone_mat: Flag,
    /// Permits the tool to be made from any ceramic material.
    #[serde(alias = "CERAMIC_MAT")]
    pub ceramic_mat: Flag,
    /// Allows a string to describe the tool when viewed. The text box can accommodate up to 325
    /// characters until it cuts off, but the spacing of actual sentences puts the realistic limit
    /// closer to 300.
    #[serde(alias = "DESCRIPTION")]
    pub description: Option<String>,
    /// Permits the tool to be made from any glass.
    #[serde(alias = "GLASS_MAT")]
    pub glass_mat: Flag,
    /// Permits the tool to be made from anything with the `[ITEMS_HARD]` token, such as wood, stone
    /// or metal.
    #[serde(alias = "HARD_MAT")]
    pub hard_mat: Flag,
    /// Permits the tool to be made from any leather.
    #[serde(alias = "LEATHER_MAT")]
    pub leather_mat: Flag,
    /// Permits the tool to be made from anything with the `[IS_METAL]` token.
    #[serde(alias = "METAL_MAT")]
    pub metal_mat: Flag,
    /// Permits the tool to be made from any metal with the `[ITEMS_WEAPON]` token.
    #[serde(alias = "METAL_WEAPON_MAT")]
    pub metal_weapon_mat: Flag,
    /// Permits the tool to be made from any "sheet" material, such as papyrus, paper, and
    /// parchment. May be connected to the `PAPER_SLURRY`/`PAPER_PLANT` reaction classes,
    /// but this is not verified.
    #[serde(alias = "SHEET_MAT")]
    pub sheet_mat: Flag,
    /// Permits the tool to be made from any shell.
    #[serde(alias = "SHELL_MAT")]
    pub shell_mat: Flag,
    /// Permits the tool to be made from any silk.
    #[serde(alias = "SILK_MAT")]
    pub silk_mat: Flag,
    /// Permits the tool to be made from any material with the `[ITEMS_SOFT]` token, such as leather
    /// or textiles.
    #[serde(alias = "SOFT_MAT")]
    pub soft_mat: Flag,
    /// Permits the tool to be made from any stone. Presumably connected to the `[IS_STONE]` token.
    #[serde(alias = "STONE_MAT")]
    pub stone_mat: Flag,
    /// Permits the tool to be made from any plant fiber, such as pig tails.
    #[serde(alias = "THREAD_PLANT_MAT")]
    pub thread_plant_mat: Flag,
    /// Permits the tool to be made from any wood.
    #[serde(alias = "WOOD_MAT")]
    pub wood_mat: Flag,
    /// According to Toady, "Won't be used in world gen libraries (to differentiate scrolls from
    /// quires). Also put it on bindings, rollers, instr. pieces for completeness/future use".
    /// Used on scroll rollers, book bindings, and quires.
    #[serde(alias = "INCOMPLETE_ITEM")]
    pub incomplete_item: Flag,
    /// Items that appear in the wild come standard with this kind of improvement. Used on scrolls:
    /// `[DEFAULT_IMPROVEMENT:SPECIFIC:ROLLERS:HARD_MAT]`
    ///
    /// Currently bugged, the effect is also applied to everything made in-game. This causes
    /// scrolls to have two sets of rollers, for example.
    #[serde(alias = "DEFAULT_IMPROVEMENT")]
    pub default_improvement: Option<ImprovementTypeTokenArg>,
    /// Prevents the tool from being improved. Used on honeycombs, scroll rollers, book bindings,
    /// and quires.
    #[serde(alias = "UNIMPROVABLE")]
    pub unimprovable: Flag,
    /// **This token's purpose is unknown, and it may be an alias of another token; if you know
    /// what it does, please open an issue on the issue tracker.**
    #[serde(alias = "NO_DEFAULT_IMPROVEMENTS")]
    pub no_default_improvements: Flag,
    /// The background of the tile will be colored, instead of the foreground.
    #[serde(alias = "INVERTED_TILE")]
    pub inverted_tile: Flag,
    /// According to Toady, "only custom reactions are used to make this item". Found on scrolls and
    /// quires.
    #[serde(alias = "NO_DEFAULT_JOB")]
    pub no_default_job: Flag,
    /// Defines the task performed using the tool.
    #[serde(alias = "TOOL_USE")]
    pub tool_use: Vec<ToolUseEnum>,
    /// Allows item to be stored in a furniture stockpile.
    #[serde(alias = "FURNITURE")]
    pub furniture: Flag,
    // TODO: ref is shape category
    #[serde(alias = "SHAPE_CATEGORY")]
    pub shape_category: Option<Reference>,
    /// Used on dice.
    #[serde(alias = "USES_FACE_IMAGE_SET")]
    pub uses_face_image_set: Flag,
    /// Adjective preceding the material name (e.g. "large copper dagger")
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// How much the item can contain. Defaults to 0.
    #[serde(alias = "CONTAINER_CAPACITY")]
    pub container_capacity: Option<u32>,
    /// Required for weapons.
    #[serde(alias = "SHOOT_FORCE")]
    pub shoot_force: Option<u32>,
    /// Required for weapons.
    #[serde(alias = "SHOOT_MAXVEL")]
    pub shoot_maxvel: Option<u32>,
    /// The skill to determine effectiveness in melee with this tool. Required for weapons.
    #[serde(alias = "SKILL")]
    pub skill: Option<SkillEnum>,
    /// Makes this tool a ranged weapon that uses the specified ammo. The specified skill
    /// determines accuracy in ranged combat.
    #[serde(alias = "RANGED")]
    pub ranged: Option<(SkillEnum, ReferenceTo<AmmoToken>)>,
    /// Creatures under this size (in cm^3) must use the tool two-handed. Required for weapons.
    #[serde(alias = "TWO_HANDED")]
    pub two_handed: Option<u32>,
    /// Minimum body size (in cm^3) to use the tool at all (multigrasp required until `TWO_HANDED`
    /// value). Required for weapons.
    #[serde(alias = "MINIMUM_SIZE")]
    pub minimum_size: Option<u32>,
    /// Number of bar units needed for forging, as well as the amount gained from melting. Required
    /// for weapons.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    /// You can have many `ATTACK` tags and one will be randomly selected for each attack, with
    /// `EDGE` attacks 100 times more common than `BLUNT` attacks. Required for weapons.
    #[serde(alias = "ATTACK")]
    pub attack: Vec<WeaponAttack>,
    // endregion ==================================================================================
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NameTypeEnum {
    #[serde(alias = "STANDARD")]
    Standard,
    #[serde(alias = "ALWAYS_PLURAL")]
    AlwaysPlural,
    #[serde(alias = "ALWAYS_SINGULAR")]
    AlwaysSingular,
}
impl Default for NameTypeEnum {
    fn default() -> Self {
        Self::Standard
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum PitchMethodEnum {
    #[serde(alias = "MEMBRANE_POSITION")]
    MembranePosition,
    #[serde(alias = "SUBPART_CHOICE")]
    SubpartChoice,
    #[serde(alias = "KEYBOARD")]
    Keyboard,
    /// Requires two `INSTRUMENT_PIECE` tokens, first for "string" second for "neck"
    /// -- or whatever is being pressed against what.
    #[serde(alias = "STOPPING_FRET")]
    StoppingFret,
    /// Requires two `INSTRUMENT_PIECE` tokens.
    #[serde(alias = "STOPPING_AGAINST_BODY")]
    StoppingAgainstBody,
    #[serde(alias = "STOPPING_HOLE")]
    StoppingHole,
    #[serde(alias = "STOPPING_HOLE_KEY")]
    StoppingHoleKey,
    #[serde(alias = "SLIDE")]
    Slide,
    #[serde(alias = "HARMONIC_SERIES")]
    HarmonicSeries,
    #[serde(alias = "VALVE_ROUTES_AIR")]
    ValveRoutesAir,
    #[serde(alias = "BP_IN_BELL")]
    BpInBell,
    /// Requires two `INSTRUMENT_PIECE` tokens, first is what is being changed e.g. "strings",
    /// second is "body" which has the pedalboard -- or whatever piece is being stepped on.
    #[serde(alias = "FOOT_PEDALS")]
    FootPedals,
}
impl Default for PitchMethodEnum {
    fn default() -> Self {
        Self::MembranePosition
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TuningMethodEnum {
    #[serde(alias = "PEGS")]
    Pegs,
    #[serde(alias = "ADJUSTABLE_BRIDGES")]
    AdjustableBridhes,
    #[serde(alias = "CROOKS")]
    Crooks,
    #[serde(alias = "TIGHTENING")]
    Tightening,
    #[serde(alias = "LEVERS")]
    Levers,
}
impl Default for TuningMethodEnum {
    fn default() -> Self {
        Self::Pegs
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SoundProductionEnum {
    #[serde(alias = "PLUCKED_BY_BP")]
    PluckedByBp,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "PLUCKED")]
    Plucked,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "BOWED")]
    Bowed,
    #[serde(alias = "STRUCK_BY_BP")]
    StruckByBp,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "STRUCK")]
    Struck,
    #[serde(alias = "VIBRATE_BP_AGAINST_OPENING")]
    VibrateBpAgainstOpening,
    #[serde(alias = "BLOW_AGAINST_FIPPLE")]
    BlowAgainstFipple,
    #[serde(alias = "BLOW_OVER_OPENING_SIDE")]
    BlowOverOpeningSide,
    #[serde(alias = "BLOW_OVER_OPENING_END")]
    BlowOverOpeningEnd,
    #[serde(alias = "BLOW_OVER_SINGLE_REED")]
    BlowOverSingleReed,
    #[serde(alias = "BLOW_OVER_DOUBLE_REED")]
    BlowOverDoubleReed,
    #[serde(alias = "BLOW_OVER_FREE_REED")]
    BlowOverFreeReed,
    #[serde(alias = "STRUCK_TOGETHER")]
    StruckTogether,
    #[serde(alias = "SHAKEN")]
    Shaken,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "SCRAPED")]
    Scraped,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "FRICTION")]
    Friction,
    #[serde(alias = "RESONATOR")]
    Resonator,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "BAG_OVER_REED")]
    BagOverReed,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "AIR_OVER_REED")]
    AirOverReed,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "AIR_OVER_FREE_REED")]
    AirOverFreeReed,
    /// Requires two `INSTRUMENT_PIECE` tokens: actor, then target.
    #[serde(alias = "AIR_AGAINST_FIPPLE")]
    AirAgainstFipple,
}
impl Default for SoundProductionEnum {
    fn default() -> Self {
        Self::PluckedByBp
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum TimbreEnum {
    #[serde(alias = "CLEAR")]
    Clear,
    #[serde(alias = "NOISY")]
    Noisy,
    #[serde(alias = "FULL")]
    Full,
    #[serde(alias = "THIN")]
    Thin,
    #[serde(alias = "ROUND")]
    Round,
    #[serde(alias = "SHARP")]
    Sharp,
    #[serde(alias = "SMOOTH")]
    Smooth,
    #[serde(alias = "CHOPPY")]
    Choppy,
    #[serde(alias = "STEADY")]
    Steady,
    #[serde(alias = "EVOLVING")]
    Evolving,
    #[serde(alias = "STRONG")]
    Strong,
    #[serde(alias = "DELICATE")]
    Delicate,
    #[serde(alias = "BRIGHT")]
    Bright,
    #[serde(alias = "GRACEFUL")]
    Graceful,
    #[serde(alias = "SPARSE")]
    Sparse,
    #[serde(alias = "BREATHY")]
    Breathy,
    #[serde(alias = "STRAINED")]
    Strained,
    #[serde(alias = "BROAD")]
    Broad,
    #[serde(alias = "LIGHT")]
    Light,
    #[serde(alias = "MELLOW")]
    Mellow,
    #[serde(alias = "WOBBLING")]
    Wobbling,
    #[serde(alias = "FOCUSED")]
    Focused,
    #[serde(alias = "EVEN")]
    Even,
    #[serde(alias = "FLUID")]
    Fluid,
    #[serde(alias = "VIBRATING")]
    Vibrating,
    #[serde(alias = "QUAVERING")]
    Quavering,
    #[serde(alias = "EERIE")]
    Eerie,
    #[serde(alias = "FRAGILE")]
    Fragile,
    #[serde(alias = "BRITTLE")]
    Brittle,
    #[serde(alias = "PURE")]
    Pure,
    #[serde(alias = "PIERCING")]
    Piercing,
    #[serde(alias = "STRIDENT")]
    Strident,
    #[serde(alias = "WAVERING")]
    Wavering,
    #[serde(alias = "HARSH")]
    Harsh,
    #[serde(alias = "REEDY")]
    Reedy,
    #[serde(alias = "NASAL")]
    Nasal,
    #[serde(alias = "BUZZY")]
    Buzzy,
    #[serde(alias = "ROUGH")]
    Rough,
    #[serde(alias = "WARM")]
    Warm,
    #[serde(alias = "RUGGED")]
    Rugged,
    #[serde(alias = "HEAVY")]
    Heavy,
    #[serde(alias = "FLAT")]
    Flat,
    #[serde(alias = "DARK")]
    Dark,
    #[serde(alias = "CRISP")]
    Crisp,
    #[serde(alias = "SONOROUS")]
    Sonorous,
    #[serde(alias = "WATERY")]
    Watery,
    #[serde(alias = "GENTLE")]
    Gentle,
    #[serde(alias = "SLICING")]
    Slicing,
    #[serde(alias = "LIQUID")]
    Liquid,
    #[serde(alias = "RAUCOUS")]
    Raucous,
    #[serde(alias = "BREEZY")]
    Breezy,
    #[serde(alias = "RASPY")]
    Raspy,
    #[serde(alias = "WISPY")]
    Wispy,
    #[serde(alias = "SHRILL")]
    Shrill,
    #[serde(alias = "MUDDY")]
    Muddy,
    #[serde(alias = "RICH")]
    Rich,
    #[serde(alias = "DULL")]
    Dull,
    #[serde(alias = "FLOATING")]
    Floating,
    #[serde(alias = "RINGING")]
    Ringing,
    #[serde(alias = "RESONANT")]
    Resonant,
    #[serde(alias = "SWEET")]
    Sweet,
    #[serde(alias = "RIPPLING")]
    Rippling,
    #[serde(alias = "SPARKLING")]
    Sparkling,
}
impl Default for TimbreEnum {
    fn default() -> Self {
        Self::Clear
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PantsToken {
    /// Argument 1 of `[ITEM_PANTS:...]`
    #[serde(alias = "ITEM_PANTS")]
    pub reference: Option<ReferenceTo<Self>>,
    // region: Shared by garments/shields/trapcomp/weapons; are all required ======================
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// How much material is needed to make the item. Most important with bars. The number of bars
    /// required to make the item is the value divided by three.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    // endregion ==================================================================================
    // region: Shared by all garments =============================================================
    /// Adjective preceding the material name (e.g. "large copper dagger").
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// Metal versions of this item count as one `ARMORLEVEL` higher and thus won't be worn by
    /// random peasants. This tag will not work unless `ARMORLEVEL` is explicitly declared: if you
    /// leave out `ARMORLEVEL`, even metal armor will default to level 0.
    #[serde(alias = "METAL_ARMOR_LEVELS")]
    pub metal_armor_levels: Flag,
    /// Metal versions of this item will have "chain" added between the material and item name.
    #[serde(alias = "CHAIN_METAL_TEXT")]
    pub chain_metal_text: Flag,
    /// Clothiers can make this item from all kinds of cloth. If paired with `[LEATHER]`, the item
    /// has an equal chance of being either in randomly generated outfits. Further uses of this tag
    /// are unknown.
    #[serde(alias = "SOFT")]
    pub soft: Flag,
    /// Default state in the absence of a `[SOFT]` token. Actual effects unknown.
    #[serde(alias = "HARD")]
    pub hard: Flag,
    /// Item can be made from metal. Overrides `[SOFT]` and `[LEATHER]` in randomly generated
    /// outfits, if the `ARMORLEVEL` permits. Civilizations with `[WOOD_ARMOR]` will make this
    /// item out of wood instead.
    #[serde(alias = "METAL")]
    pub metal: Flag,
    /// Craftsmen can make this item from bones. Randomly generated outfits don't include bone
    /// armor.
    #[serde(alias = "BARRED")]
    pub barred: Flag,
    /// Craftsmen can make this item from shells. Randomly generated outfits don't include shell
    /// armor.
    #[serde(alias = "SCALED")]
    pub scaled: Flag,
    /// Leatherworkers can make this item from leather. If paired with `[SOFT]`, this item has an
    /// equal chance of being either in randomly generated outfits.
    #[serde(alias = "LEATHER")]
    pub leather: Flag,
    /// Only one shaped piece of clothing can be worn on a single body slot at a time.
    #[serde(alias = "SHAPED")]
    pub shaped: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, if lower.
    /// This makes the garment flex and give way instead of shattering under force. Strong materials
    /// that resist cutting will blunt edged attacks into bone-crushing hits instead.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_ALL")]
    pub structural_elasticity_chain_all: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, but only if
    /// the garment is made from metal.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_METAL")]
    pub structural_elasticity_chain_metal: Flag,
    /// Reduces the armor material's `SHEAR_YIELD` to 20000, `SHEAR_FRACTURE` to 30000 and increases
    /// the `*_STRAIN_AT_YIELD` properties to 50000, but only if the garment is made from cloth.
    /// This makes the item very weak against edged attacks, even if the thread material is
    /// normally very strong.
    #[serde(alias = "STRUCTURAL_ELASTICITY_WOVEN_THREAD")]
    pub structural_elasticity_woven_thread: Flag,
    /// The item's bulkiness when worn. Aside from the layer limitations, it's a big contributor to
    /// the thickness and weight (and therefore price) of the garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_SIZE")]
    pub layer_size: Option<u32>,
    /// The maximum amount of garments that can fit underneath this garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_PERMIT")]
    pub layer_permit: Option<u32>,
    /// Where the item goes in relation to other clothes. Socks cannot be worn on top of boots!
    ///
    /// The `LAYER_PERMIT` of the highest layer is used on a given section of the body - you can fit
    /// a lot of shirts and other undergarments underneath a robe, but not if you wear a leather
    /// jerkin on top of it, and you can still wear a cloak over the whole ensemble. Defaults to
    /// `UNDER`.
    #[serde(alias = "LAYER")]
    pub layer: Option<LayerEnum>,
    /// How often the garment gets in the way of a contaminant or an attack. Armor with a 5%
    /// coverage value, for example, will be near useless because 95% of attacks will bypass it
    /// completely. Temperature effects and armor thickness are also influenced. Defaults to 100.
    #[serde(alias = "COVERAGE")]
    pub coverage: Option<u8>,
    /// The garment's general purpose. Defaults to 1 for shields, 0 for everything else. Class 0
    /// items are claimed and used by civilians as ordinary clothing and are subject to wear.
    #[serde(alias = "ARMORLEVEL")] // shared by all garments, and shields
    pub armorlevel: Option<u8>,
    // endregion ==================================================================================
    // region: Shared by ARMOR and PANTS ==========================================================
    /// Changes the plural form of this item to "`phrase of` item". Primarily pertains to the stock
    /// screens.
    ///
    /// Example, "suits of" platemail, "pairs of" trousers, etc.
    #[serde(alias = "PREPLURAL")]
    pub preplural: Option<String>,
    /// If the item has no material associated with it (e.g. stockpile menus and trade
    /// negotiations), this will be displayed in its place. Used for leather armor in vanilla.
    #[serde(alias = "MATERIAL_PLACEHOLDER")]
    pub material_placeholder: Option<String>,
    /// Length of the legs/hem, counted in `[LIMB]` body parts towards the feet. A value of 0 only
    /// covers the lower body, 1 extends over the upper legs and so on. Regardless of the value,
    /// body armor or pants can never extend to cover the feet.
    #[serde(alias = "LBSTEP")]
    pub lbstep: Option<Choose<u8, MaxEnum>>,
    // endregion ==================================================================================
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShieldToken {
    /// Argument 1 of `[ITEM_SHIELD:...]`
    #[serde(alias = "ITEM_SHIELD")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Affects the block chance of the shield. Defaults to 10.
    #[serde(alias = "BLOCKCHANCE")]
    pub blockchance: Option<Clamp<u8, 0, 100>>,
    /// Length of gloves or footwear, counted in `[LIMB]` body parts towards the torso. A value of 1
    /// lets gloves cover the lower arms, a value of 2 stretches a boot all the way over the upper
    /// leg and so on.
    ///
    /// Regardless of the value, none of these items can ever extend to cover the upper or lower
    /// body. Shields also have this token, but it only seems to affect weight.
    #[serde(alias = "UPSTEP")]
    pub upstep: Option<Choose<u8, MaxEnum>>, // shared by glove, shield, shoes
    /// The garment's general purpose. Defaults to 1 for shields, 0 for everything else. Class 0
    /// items are claimed and used by civilians as ordinary clothing and are subject to wear.
    #[serde(alias = "ARMORLEVEL")]
    pub armorlevel: Option<u8>, // shared by all garments, and shields
    // region: Shared by garments/shields/trapcomp/weapons; are all required ======================
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// How much material is needed to make the item. Most important with bars. The number of bars
    /// required to make the item is the value divided by three.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    // endregion ==================================================================================
    /// Adjective preceding the material name (e.g. "large copper dagger").
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ShoesToken {
    /// Argument 1 of `[ITEM_SHOES:...]`
    #[serde(alias = "ITEM_SHOES")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Length of gloves or footwear, counted in `[LIMB]` body parts towards the torso. A value of 1
    /// lets gloves cover the lower arms, a value of 2 stretches a boot all the way over the upper
    /// leg and so on.
    ///
    /// Regardless of the value, none of these items can ever extend to cover the upper or lower
    /// body. Shields also have this token, but it only seems to affect weight.
    #[serde(alias = "UPSTEP")]
    pub upstep: Option<Choose<u8, MaxEnum>>, // shared by glove, shield, shoes
    // region: Shared by garments/shields/trapcomp/weapons; are all required ======================
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// How much material is needed to make the item. Most important with bars. The number of bars
    /// required to make the item is the value divided by three.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    // endregion ==================================================================================
    // region: Shared by all garments =============================================================
    /// Adjective preceding the material name (e.g. "large copper dagger").
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// Metal versions of this item count as one `ARMORLEVEL` higher and thus won't be worn by
    /// random peasants. This tag will not work unless `ARMORLEVEL` is explicitly declared: if you
    /// leave out `ARMORLEVEL`, even metal armor will default to level 0.
    #[serde(alias = "METAL_ARMOR_LEVELS")]
    pub metal_armor_levels: Flag,
    /// Metal versions of this item will have "chain" added between the material and item name.
    #[serde(alias = "CHAIN_METAL_TEXT")]
    pub chain_metal_text: Flag,
    /// Clothiers can make this item from all kinds of cloth. If paired with `[LEATHER]`, the item
    /// has an equal chance of being either in randomly generated outfits. Further uses of this tag
    /// are unknown.
    #[serde(alias = "SOFT")]
    pub soft: Flag,
    /// Default state in the absence of a `[SOFT]` token. Actual effects unknown.
    #[serde(alias = "HARD")]
    pub hard: Flag,
    /// Item can be made from metal. Overrides `[SOFT]` and `[LEATHER]` in randomly generated
    /// outfits, if the `ARMORLEVEL` permits. Civilizations with `[WOOD_ARMOR]` will make this
    /// item out of wood instead.
    #[serde(alias = "METAL")]
    pub metal: Flag,
    /// Craftsmen can make this item from bones. Randomly generated outfits don't include bone
    /// armor.
    #[serde(alias = "BARRED")]
    pub barred: Flag,
    /// Craftsmen can make this item from shells. Randomly generated outfits don't include shell
    /// armor.
    #[serde(alias = "SCALED")]
    pub scaled: Flag,
    /// Leatherworkers can make this item from leather. If paired with `[SOFT]`, this item has an
    /// equal chance of being either in randomly generated outfits.
    #[serde(alias = "LEATHER")]
    pub leather: Flag,
    /// Only one shaped piece of clothing can be worn on a single body slot at a time.
    #[serde(alias = "SHAPED")]
    pub shaped: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, if lower.
    /// This makes the garment flex and give way instead of shattering under force. Strong materials
    /// that resist cutting will blunt edged attacks into bone-crushing hits instead.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_ALL")]
    pub structural_elasticity_chain_all: Flag,
    /// Increases the `*_STRAIN_AT_YIELD` properties of the armor's material to 50000, but only if
    /// the garment is made from metal.
    #[serde(alias = "STRUCTURAL_ELASTICITY_CHAIN_METAL")]
    pub structural_elasticity_chain_metal: Flag,
    /// Reduces the armor material's `SHEAR_YIELD` to 20000, `SHEAR_FRACTURE` to 30000 and increases
    /// the `*_STRAIN_AT_YIELD` properties to 50000, but only if the garment is made from cloth.
    /// This makes the item very weak against edged attacks, even if the thread material is
    /// normally very strong.
    #[serde(alias = "STRUCTURAL_ELASTICITY_WOVEN_THREAD")]
    pub structural_elasticity_woven_thread: Flag,
    /// The item's bulkiness when worn. Aside from the layer limitations, it's a big contributor to
    /// the thickness and weight (and therefore price) of the garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_SIZE")]
    pub layer_size: Option<u32>,
    /// The maximum amount of garments that can fit underneath this garment. See
    /// [Armor](https://dwarffortresswiki.org/index.php/Armor) for more on item sizes and
    /// layering. Defaults to 10.
    #[serde(alias = "LAYER_PERMIT")]
    pub layer_permit: Option<u32>,
    /// Where the item goes in relation to other clothes. Socks cannot be worn on top of boots!
    ///
    /// The `LAYER_PERMIT` of the highest layer is used on a given section of the body - you can fit
    /// a lot of shirts and other undergarments underneath a robe, but not if you wear a leather
    /// jerkin on top of it, and you can still wear a cloak over the whole ensemble. Defaults to
    /// `UNDER`.
    #[serde(alias = "LAYER")]
    pub layer: Option<LayerEnum>,
    /// How often the garment gets in the way of a contaminant or an attack. Armor with a 5%
    /// coverage value, for example, will be near useless because 95% of attacks will bypass it
    /// completely. Temperature effects and armor thickness are also influenced. Defaults to 100.
    #[serde(alias = "COVERAGE")]
    pub coverage: Option<u8>,
    /// The garment's general purpose. Defaults to 1 for shields, 0 for everything else. Class 0
    /// items are claimed and used by civilians as ordinary clothing and are subject to wear.
    #[serde(alias = "ARMORLEVEL")] // shared by all garments, and shields
    pub armorlevel: Option<u8>,
    // endregion ==================================================================================
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum LayerEnum {
    #[serde(alias = "UNDER")]
    Under,
    #[serde(alias = "OVER")]
    Over,
    #[serde(alias = "ARMOR")]
    Armor,
    #[serde(alias = "COVER")]
    Cover,
}
impl Default for LayerEnum {
    fn default() -> Self {
        Self::Under
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MaxEnum {
    #[serde(alias = "MAX")]
    Max,
}
impl Default for MaxEnum {
    fn default() -> Self {
        Self::Max
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SiegeAmmoToken {
    /// Argument 1 of `[ITEM_SIEGEAMMO:...]`
    #[serde(alias = "ITEM_SIEGEAMMO")]
    pub reference: Option<ReferenceTo<Self>>,
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// Specifies what type of siege engine uses this ammunition. Currently, only `BALLISTA` is
    /// permitted.
    #[serde(alias = "CLASS")]
    pub class: Option<SiegeAmmoClassEnum>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum SiegeAmmoClassEnum {
    #[serde(alias = "BALLISTA")]
    Ballista,
}
impl Default for SiegeAmmoClassEnum {
    fn default() -> Self {
        Self::Ballista
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ToolToken {
    /// Argument 1 of `[ITEM_TOOL:...]`
    #[serde(alias = "ITEM_TOOL")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Volume of tool in mL or cubic centimeters. Required.
    #[serde(alias = "SIZE")]
    pub size: Option<u32>,
    /// Name of the tool. Required.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// Defines the item value of the tool. Required.
    #[serde(alias = "VALUE")]
    pub value: Option<u32>,
    /// Defines the tile used to represent the tool. Required.
    #[serde(alias = "TILE")]
    pub tile: Option<DFChar>,
    /// Permits the tool to be made from any bone.
    #[serde(alias = "BONE_MAT")]
    pub bone_mat: Flag,
    /// Permits the tool to be made from any ceramic material.
    #[serde(alias = "CERAMIC_MAT")]
    pub ceramic_mat: Flag,
    /// Allows a string to describe the tool when viewed. The text box can accommodate up to 325
    /// characters until it cuts off, but the spacing of actual sentences puts the realistic limit
    /// closer to 300.
    #[serde(alias = "DESCRIPTION")]
    pub description: Option<String>,
    /// Permits the tool to be made from any glass.
    #[serde(alias = "GLASS_MAT")]
    pub glass_mat: Flag,
    /// Permits the tool to be made from anything with the `[ITEMS_HARD]` token, such as wood, stone
    /// or metal.
    #[serde(alias = "HARD_MAT")]
    pub hard_mat: Flag,
    /// Permits the tool to be made from any leather.
    #[serde(alias = "LEATHER_MAT")]
    pub leather_mat: Flag,
    /// Permits the tool to be made from anything with the `[IS_METAL]` token.
    #[serde(alias = "METAL_MAT")]
    pub metal_mat: Flag,
    /// Permits the tool to be made from any metal with the `[ITEMS_WEAPON]` token.
    #[serde(alias = "METAL_WEAPON_MAT")]
    pub metal_weapon_mat: Flag,
    /// Permits the tool to be made from any "sheet" material, such as papyrus, paper, and
    /// parchment. May be connected to the `PAPER_SLURRY`/`PAPER_PLANT` reaction classes,
    /// but this is not verified.
    #[serde(alias = "SHEET_MAT")]
    pub sheet_mat: Flag,
    /// Permits the tool to be made from any shell.
    #[serde(alias = "SHELL_MAT")]
    pub shell_mat: Flag,
    /// Permits the tool to be made from any silk.
    #[serde(alias = "SILK_MAT")]
    pub silk_mat: Flag,
    /// Permits the tool to be made from any material with the `[ITEMS_SOFT]` token, such as leather
    /// or textiles.
    #[serde(alias = "SOFT_MAT")]
    pub soft_mat: Flag,
    /// Permits the tool to be made from any stone. Presumably connected to the `[IS_STONE]` token.
    #[serde(alias = "STONE_MAT")]
    pub stone_mat: Flag,
    /// Permits the tool to be made from any plant fiber, such as pig tails.
    #[serde(alias = "THREAD_PLANT_MAT")]
    pub thread_plant_mat: Flag,
    /// Permits the tool to be made from any wood.
    #[serde(alias = "WOOD_MAT")]
    pub wood_mat: Flag,
    /// According to Toady, "Won't be used in world gen libraries (to differentiate scrolls from
    /// quires). Also put it on bindings, rollers, instr. pieces for completeness/future use".
    /// Used on scroll rollers, book bindings, and quires.
    #[serde(alias = "INCOMPLETE_ITEM")]
    pub incomplete_item: Flag,
    /// Items that appear in the wild come standard with this kind of improvement. Used on scrolls:
    /// `[DEFAULT_IMPROVEMENT:SPECIFIC:ROLLERS:HARD_MAT]`
    ///
    /// Currently bugged, the effect is also applied to everything made in-game. This causes
    /// scrolls to have two sets of rollers, for example.
    #[serde(alias = "DEFAULT_IMPROVEMENT")]
    pub default_improvement: Option<ImprovementTypeTokenArg>,
    /// Prevents the tool from being improved. Used on honeycombs, scroll rollers, book bindings,
    /// and quires.
    #[serde(alias = "UNIMPROVABLE")]
    pub unimprovable: Flag,
    /// **This token's purpose is unknown, and it may be an alias of another token; if you know
    /// what it does, please open an issue on the issue tracker.**
    #[serde(alias = "NO_DEFAULT_IMPROVEMENTS")]
    pub no_default_improvements: Flag,
    /// The background of the tile will be colored, instead of the foreground.
    #[serde(alias = "INVERTED_TILE")]
    pub inverted_tile: Flag,
    /// According to Toady, "only custom reactions are used to make this item". Found on scrolls and
    /// quires.
    #[serde(alias = "NO_DEFAULT_JOB")]
    pub no_default_job: Flag,
    /// Defines the task performed using the tool.
    #[serde(alias = "TOOL_USE")]
    pub tool_use: Vec<ToolUseEnum>,
    /// Allows item to be stored in a furniture stockpile.
    #[serde(alias = "FURNITURE")]
    pub furniture: Flag,
    // TODO: ref is shape category
    #[serde(alias = "SHAPE_CATEGORY")]
    pub shape_category: Option<Reference>,
    /// Used on dice.
    #[serde(alias = "USES_FACE_IMAGE_SET")]
    pub uses_face_image_set: Flag,
    /// Adjective preceding the material name (e.g. "large copper dagger")
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// How much the item can contain. Defaults to 0.
    #[serde(alias = "CONTAINER_CAPACITY")]
    pub container_capacity: Option<u32>,
    /// Required for weapons.
    #[serde(alias = "SHOOT_FORCE")]
    pub shoot_force: Option<u32>,
    /// Required for weapons.
    #[serde(alias = "SHOOT_MAXVEL")]
    pub shoot_maxvel: Option<u32>,
    /// The skill to determine effectiveness in melee with this tool. Required for weapons.
    #[serde(alias = "SKILL")]
    pub skill: Option<SkillEnum>,
    /// Makes this tool a ranged weapon that uses the specified ammo. The specified skill
    /// determines accuracy in ranged combat.
    #[serde(alias = "RANGED")]
    pub ranged: Option<(SkillEnum, ReferenceTo<AmmoToken>)>,
    /// Creatures under this size (in cm^3) must use the tool two-handed. Required for weapons.
    #[serde(alias = "TWO_HANDED")]
    pub two_handed: Option<u32>,
    /// Minimum body size (in cm^3) to use the tool at all (multigrasp required until `TWO_HANDED`
    /// value). Required for weapons.
    #[serde(alias = "MINIMUM_SIZE")]
    pub minimum_size: Option<u32>,
    /// Number of bar units needed for forging, as well as the amount gained from melting. Required
    /// for weapons.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>,
    /// You can have many `ATTACK` tags and one will be randomly selected for each attack, with
    /// `EDGE` attacks 100 times more common than `BLUNT` attacks. Required for weapons.
    #[serde(alias = "ATTACK")]
    pub attack: Vec<WeaponAttack>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ToolUseEnum {
    /// Cauldron adventure mode decoration / weapon
    #[serde(alias = "LIQUID_COOKING")]
    LiquidCooking,
    /// Ladle, an adventure mode decoration/weapon.
    #[serde(alias = "LIQUID_SCOOP")]
    LiquidScoop,
    /// Mortar, an adventure mode decoration/weapon.
    #[serde(alias = "GRIND_POWDER_RECEPTACLE")]
    GrindPowderReceptacle,
    /// Pestle, an adventure mode decoration/weapon.
    #[serde(alias = "GRIND_POWDER_GRINDER")]
    GrindPowderGrinder,
    /// Carving knife, an adventure mode decoration/weapon.
    #[serde(alias = "MEAT_CARVING")]
    MeatCarving,
    /// Boning knife, an adventure mode decoration/weapon.
    #[serde(alias = "MEAT_BONING")]
    MeatBoning,
    /// Slicing knife, an adventure mode decoration/weapon.
    #[serde(alias = "MEAT_SLICING")]
    MeatSlicing,
    /// Meat cleaver, an adventure mode decoration/weapon.
    #[serde(alias = "MEAT_CLEAVING")]
    MeatCleaving,
    /// Carving fork, an adventure mode decoration/weapon.
    #[serde(alias = "HOLD_MEAT_FOR_CARVING")]
    HoldMeatForCarving,
    /// Bowl, an adventure mode decoration/weapon.
    #[serde(alias = "MEAL_CONTAINER")]
    MealContainer,
    /// Nest box for your birds to lay eggs.
    #[serde(alias = "NEST_BOX")]
    NestBox,
    /// Jug; can store honey or oil.
    #[serde(alias = "LIQUID_CONTAINER")]
    LiquidContainer,
    /// Large pot; can store beer.
    #[serde(alias = "FOOD_STORAGE")]
    FoodStorage,
    /// Hive; can make honey.
    #[serde(alias = "HIVE")]
    Hive,
    /// Pouch, an adventure mode coin purse.
    #[serde(alias = "SMALL_OBJECT_STORAGE")]
    SmallObjectStorage,
    /// Minecart; item hauling/weapon.
    #[serde(alias = "TRACK_CART")]
    TrackCart,
    /// Wheelbarrow; allows hauling items faster.
    #[serde(alias = "HEAVY_OBJECT_HAULING")]
    HeavyObjectHauling,
    /// Stepladder, allows gathering fruit from trees.
    #[serde(alias = "STAND_AND_WORK_ABOVE")]
    StandAndWorkAbove,
    /// Scroll rollers.
    #[serde(alias = "ROLL_UP_SHEET")]
    RollUpSheet,
    /// Book binding.
    #[serde(alias = "PROTECT_FOLDED_SHEETS")]
    ProtectFoldedSheets,
    /// Scroll & quire.
    #[serde(alias = "CONTAIN_WRITING")]
    ContainWriting,
    /// Bookcase.
    #[serde(alias = "BOOKCASE")]
    Bookcase,
    /// Pedestal & display case for museums.
    #[serde(alias = "DISPLAY_OBJECT")]
    DisplayObject,
    /// Altar for (eventually) offering sacrifices.
    #[serde(alias = "PLACE_OFFERING")]
    PlaceOffering,
    /// Dice.
    #[serde(alias = "DIVINATION")]
    Divination,
    /// Dice.
    #[serde(alias = "GAMES_OF_CHANCE")]
    GamesOfChance,
}
impl Default for ToolUseEnum {
    fn default() -> Self {
        Self::LiquidCooking
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ImprovementTypeTokenArg {
    // TODO: token should be marked as deprecated/unused/ignored by the game, see #83
    // #[serde(alias = "ART_IMAGE")]
    // ArtImage,
    // #[serde(alias = "COVERED")]
    Covered(ImprovementMaterialFlagEnum),
    // #[serde(alias = "RINGS_HANGING")]
    RingsHanging(ImprovementMaterialFlagEnum),
    // #[serde(alias = "BANDS")]
    Bands(ImprovementMaterialFlagEnum),
    // #[serde(alias = "SPIKES")]
    Spikes(ImprovementMaterialFlagEnum),
    // #[serde(alias = "SPECIFIC", alias = "ITEMSPECIFIC")]
    Specific((ItemSpecificEnum, ImprovementMaterialFlagEnum)),
    // TODO: token should be marked as deprecated/unused/ignored by the game, see #83
    // #[serde(alias = "THREAD")]
    // Thread,
    // TODO: token should be marked as deprecated/unused/ignored by the game, see #83
    // #[serde(alias = "CLOTH")]
    // Cloth,
    // TODO: token should be marked as deprecated/unused/ignored by the game, see #83
    // #[serde(alias = "SEWN_IMAGE")]
    // SewnImage,
    // #[serde(alias = "PAGES")]
    Pages(ImprovementMaterialFlagEnum),
    // TODO: token should be marked as deprecated/unused/ignored by the game, see #83
    // #[serde(alias = "ILLUSTRATION")]
    // Illustration,
    // #[serde(alias = "INSTRUMENT_PIECE")]
    InstrumentPiece((Reference, ImprovementMaterialFlagEnum)), // Ref to INSTRUMENT PIECE, distinct from ITEM_INSTRUMENT
    // #[serde(alias = "WRITING")]
    Writing(ImprovementMaterialFlagEnum),
    // TODO: token should be marked as deprecated/unused/ignored by the game, see #83
    // #[serde(alias = "IMAGE_SET")]
    // ImageSet,
}
impl Default for ImprovementTypeTokenArg {
    fn default() -> Self {
        Self::Covered(ImprovementMaterialFlagEnum::default())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ItemSpecificEnum {
    #[serde(alias = "ROLLERS")]
    Rollers,
    #[serde(alias = "HANDLE")]
    Handle,
}
impl Default for ItemSpecificEnum {
    fn default() -> Self {
        Self::Rollers
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ImprovementMaterialFlagEnum {
    /// Permits the improvement to be made from any bone.
    #[serde(alias = "BONE_MAT")]
    BoneMat,
    /// Permits the improvement to be made from any ceramic material.
    #[serde(alias = "CERAMIC_MAT")]
    CeramicMat,
    /// Permits the improvement to be made from any glass.
    #[serde(alias = "GLASS_MAT")]
    GlassMat,
    /// Permits the improvement to be made from anything with the `[ITEMS_HARD]` token, such as wood, stone
    /// or metal.
    #[serde(alias = "HARD_MAT")]
    HardMat,
    /// Permits the improvement to be made from any leather.
    #[serde(alias = "LEATHER_MAT")]
    LeatherMat,
    /// Permits the improvement to be made from anything with the `[IS_METAL]` token.
    #[serde(alias = "METAL_MAT")]
    MetalMat,
    /// Permits the improvement to be made from any metal with the `[ITEMS_WEAPON]` token.
    #[serde(alias = "METAL_WEAPON_MAT")]
    MetalWeaponMat,
    /// Permits the improvement to be made from any "sheet" material, such as papyrus, paper, and
    /// parchment. May be connected to the `PAPER_SLURRY`/`PAPER_PLANT` reaction classes,
    /// but this is not verified.
    #[serde(alias = "SHEET_MAT")]
    SheetMat,
    /// Permits the improvement to be made from any shell.
    #[serde(alias = "SHELL_MAT")]
    ShellMat,
    /// Permits the improvement to be made from any silk.
    #[serde(alias = "SILK_MAT")]
    SilkMat,
    /// Permits the improvement to be made from any material with the `[ITEMS_SOFT]` token, such as leather
    /// or textiles.
    #[serde(alias = "SOFT_MAT")]
    SoftMat,
    /// Permits the improvement to be made from any stone. Presumably connected to the `[IS_STONE]` token.
    #[serde(alias = "STONE_MAT")]
    StoneMat,
    /// Permits the improvement to be made from any plant fiber, such as pig tails.
    #[serde(alias = "THREAD_PLANT_MAT")]
    ThreadPlantMat,
    /// Permits the improvement to be made from any wood.
    #[serde(alias = "WOOD_MAT")]
    WoodMat,
}
impl Default for ImprovementMaterialFlagEnum {
    fn default() -> Self {
        Self::BoneMat
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ToyToken {
    /// Argument 1 of `[ITEM_TOY:...]`
    #[serde(alias = "ITEM_TOY")]
    pub reference: Option<ReferenceTo<Self>>,
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// Presumably prevents the item from being made from cloth, silk, or leather, present on
    /// everything but puzzleboxes and drums. Appears to work backwards for strange moods.
    #[serde(alias = "HARD_MAT")]
    pub hard_mat: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TrapCompToken {
    /// Argument 1 of `[ITEM_TRAPCOMP:...]`
    #[serde(alias = "ITEM_TRAPCOMP")]
    pub reference: Option<ReferenceTo<Self>>,
    /// What this item will be called in-game.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>,
    /// How much material is needed to make the item. Is most important with bars.
    /// The number of bars needed is the value divided by three.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>, // TODO: required
    /// Appears before the name of the weapon's material. For example: "menacing steel spike".
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// How large the item is. Defaults to 100.
    #[serde(alias = "SIZE")]
    pub size: Option<u32>,
    /// Number of times it hits. Defaults to 1
    #[serde(alias = "HITS")]
    pub hits: Option<u32>,
    /// Weapon may be installed in a screw pump.
    #[serde(alias = "IS_SCREW")]
    pub is_screw: Flag,
    /// Weapon may be installed in a spike trap.
    #[serde(alias = "IS_SPIKE")]
    pub is_spike: Flag,
    /// Weapon may be made out of wood.
    #[serde(alias = "WOOD")]
    pub wood: Flag,
    /// Weapon may be made out of metal.
    #[serde(alias = "METAL")]
    pub metal: Flag,
    /// Sets the attack characteristics of the weapon
    #[serde(alias = "ATTACK")]
    pub attack: Option<ItemAttack>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WeaponToken {
    /// Argument 1 of `[ITEM_WEAPON:...]`
    #[serde(alias = "ITEM_WEAPON")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Name of the weapon.
    #[serde(alias = "NAME")]
    pub name: Option<(String, String)>, // TODO: required
    /// Number of bar units needed for forging, as well as the amount gained from melting.
    #[serde(alias = "MATERIAL_SIZE")]
    pub material_size: Option<u32>, // TODO: Required
    /// Adjective of the weapon, e.g. the "large" in "large copper dagger".
    #[serde(alias = "ADJECTIVE")]
    pub adjective: Option<String>,
    /// Volume of weapon in mL or cubic cm. Defaults to 100.
    #[serde(alias = "SIZE")]
    pub size: Option<u32>,
    /// The amount of force used when firing projectiles - velocity is presumably determined by the
    /// projectile's mass. Defaults to 0.
    #[serde(alias = "SHOOT_FORCE")]
    pub shoot_force: Option<u32>,
    /// The maximum speed a fired projectile can have.
    #[serde(alias = "SHOOT_MAXVEL")]
    pub shoot_maxvel: Option<u32>,
    /// The skill to determine effectiveness in melee with this weapon. Defaults to `MACE`.
    ///
    /// Skill of `AXE` will allow it to be used to chop down trees. Skill of `MINER` will allow
    /// it to be used for mining.
    ///
    /// Outsider adventurers (or regular ones with no points in offensive skills) will receive a
    /// weapon with the `SPEAR` skill, selected at random if multiple ones have been modded in.
    /// All adventurers will also start with a weapon using the `DAGGER` skill, again selected at
    /// random if multiple such weapons exist.
    #[serde(alias = "SKILL")]
    pub skill: Option<SkillEnum>,
    /// Makes this a ranged weapon that uses the specified ammo. The specified skill determines
    /// accuracy in ranged combat.
    #[serde(alias = "RANGED")]
    pub ranged: Option<(SkillEnum, Reference)>, // `TODO` reference is ammo class
    /// Creatures under this size (in cm^3) must use the weapon two-handed. Defaults to 50000.
    #[serde(alias = "TWO_HANDED")]
    pub two_handed: Option<u32>,
    /// Minimum body size (in cm^3) to use the weapon at all (multigrasp required until `TWO_HANDED`
    /// value). Defaults to 40000.
    #[serde(alias = "MINIMUM_SIZE")]
    pub minimum_size: Option<u32>,
    /// Allows the weapon to be made at a craftsdwarf's workshop from a sharp (`[MAX_EDGE:10000]` or
    /// higher) stone (i.e. obsidian) plus a wood log.
    #[serde(alias = "CAN_STONE")]
    pub can_stone: Flag,
    /// Restricts this weapon to being made of wood.
    #[serde(alias = "TRAINING")]
    pub training: Flag,
    /// List of attacks this weapon can have.
    #[serde(alias = "ATTACK")]
    pub attack: Vec<WeaponAttack>,
}

// region: Attack definitions

/// Specifies the attack characteristics of this item.
/// - attack type: `BLUNT` or `EDGE`
/// - contact_area: value
/// - penetration_size: value
/// - verb2nd: string
/// - verb3rd: string
/// - noun: string or `NO_SUB`
/// - velocity_multiplier: value
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ItemAttack {
    /// Arguments of the `ATTACK` token
    #[serde(alias = "ATTACK")]
    pub attack: Option<(
        AttackTypeEnum,
        u32,
        u32,
        String,
        String,
        Choose<NoSubEnum, String>,
        u32,
    )>,
    /// Determines the length of time to prepare this attack and until one can perform this attack
    /// again. Values appear to be calculated in adventure mode ticks.
    #[serde(alias = "ATTACK_PREPARE_AND_RECOVER")]
    pub attack_prepare_and_recover: Option<(u32, u32)>,
}

/// Defines an attack on this weapon.
/// You can have many `ATTACK` tags and one will be randomly selected for each attack, with `EDGE`
/// attacks used 100 times more often than `BLUNT` attacks.
///
/// Contact area is usually high for slashing and low for bludgeoning, piercing, and poking.
/// Penetration value tends to be low for slashing and high for stabbing. Penetration is ignored if
/// attack is `BLUNT`.
/// - attack type: `BLUNT` or `EDGE`
/// - contact_area: value
/// - penetration_size: value
/// - verb2nd: string
/// - verb3rd: string
/// - noun: string or `NO_SUB`
/// - velocity_multiplier: value
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WeaponAttack {
    /// Arguments of the `ATTACK` token
    #[serde(alias = "ATTACK")]
    pub attack: Option<(
        AttackTypeEnum,
        u32,
        u32,
        String,
        String,
        Choose<NoSubEnum, String>,
        u32,
    )>,
    /// Determines the length of time to prepare this attack and until one can perform this attack
    /// again. Values appear to be calculated in adventure mode ticks.
    #[serde(alias = "ATTACK_PREPARE_AND_RECOVER")]
    pub attack_prepare_and_recover: Option<(u32, u32)>,
    /// Multiple strikes with this attack cannot be performed effectively.
    #[serde(alias = "ATTACK_FLAG_BAD_MULTIATTACK")]
    pub attack_flag_bad_multiattack: Flag,
    /// Multiple strikes with this attack can be performed with no penalty.
    #[serde(alias = "ATTACK_FLAG_INDEPENDENT_MULTIATTACK")]
    pub attack_flag_independent_multiattack: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AttackTypeEnum {
    #[serde(alias = "EDGE")]
    Edge,
    #[serde(alias = "BLUNT")]
    Blunt,
}
impl Default for AttackTypeEnum {
    fn default() -> Self {
        Self::Edge
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NoSubEnum {
    #[serde(alias = "NO_SUB")]
    NoSub,
}
impl Default for NoSubEnum {
    fn default() -> Self {
        Self::NoSub
    }
}
// endregion
