use crate::core::{Choose, DFChar, Reference, ReferenceTo};
use crate::structure::{
    AllEnum, AllOrAllSolidEnum, ColorToken, DietInfoEnum, InorganicToken, ItemReferenceArg,
    MaterialStateEnum, MaterialTokenArg, NoneEnum, OverwriteSolidEnum, PluralEnum, ReactionToken,
    StandardPluralEnum, SyndromeToken, TissueShapeEnum, TissueToken,
};

use serde::{Deserialize, Serialize};

/// Begins defining a new local tissue in the creature.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalTissueToken {
    /// Argument 1 of `[TISSUE:...]`
    #[serde(alias = "TISSUE")]
    pub reference: Option<Reference>,
    // region: Tissue Tokens ======================================================================
    /// Name of the tissue.
    ///
    /// Arguments are: `<name>:<plural>`
    ///
    /// Plural can alternatively be `NP` (No Plural) or `STP (Standard Plural,
    /// adds an 's' on the end).
    #[serde(alias = "TISSUE_NAME")]
    pub name: Option<(String, Choose<PluralEnum, String>)>,
    /// Defines the tissue material.
    #[serde(alias = "TISSUE_MATERIAL")]
    pub material: Option<MaterialTokenArg>,
    /// The relative thickness of the tissue.
    /// A higher thickness is harder to penetrate, but raising a tissue's relative thickness
    /// decreases the thickness of all other tissues.
    #[serde(alias = "RELATIVE_THICKNESS")]
    pub relative_thickness: Option<u32>,
    /// Speed at which the tissue heals itself; lower is faster. Common values are `100` and `1000`
    ///
    /// Omitting the token will result in a tissue that never heals.
    #[serde(alias = "HEALING_RATE")]
    pub healing_rate: Option<u32>,
    /// How many arteries/veins are in the tissue.
    /// Related to how much said tissue bleeds.
    ///
    /// Higher = More bleeding (Which is why the heart has the highest value.)
    /// - Default heart = `10`
    /// - Default skin = `1`
    ///
    /// Also see: `MAJOR_ARTERIES` and `ARTERIES`
    #[serde(alias = "VASCULAR")]
    pub vascular: Option<i32>,
    /// Related to how much pain your character will suffer when said tissue is damaged.
    /// Higher = More pain when damaged (which is why the bone tissue has a much higher value
    /// than other tissues; a broken bone hurts a lot more than a flesh cut).
    /// - Default bones = `50`
    /// - Default skin = `5`
    #[serde(alias = "PAIN_RECEPTORS")]
    pub pain_receptors: Option<i32>,
    /// The thickness of the tissue increases when character strength increases.
    /// Used for muscles in vanilla.
    #[serde(alias = "THICKENS_ON_STRENGTH")]
    pub thickness_on_strength: Option<()>,
    /// Thickness of said tissue increases when the
    /// character eats and doesn't exercise sufficiently.
    /// Used for fat in vanilla.
    #[serde(alias = "THICKENS_ON_ENERGY_STORAGE")]
    pub thickness_on_energy_storage: Option<()>,
    /// The tissue contains arteries.
    /// Edged attacks have the chance to break an artery, increasing blood loss.
    /// Used for muscles in vanilla.
    ///
    /// Also see: `MAJOR_ARTERIES` and `VASCULAR`
    #[serde(alias = "ARTERIES")]
    pub arteries: Option<()>,
    /// Denotes whether or not the tissue will be scarred once healed.
    #[serde(alias = "SCARS")]
    pub scars: Option<()>,
    /// Holds the body part together.
    /// A cut or a fracture will disable the body part it's in.
    #[serde(alias = "STRUCTURAL")]
    pub structural: Option<()>,
    /// Any ligaments or tendons are part of this tissue.
    /// Vulnerable to edged attacks, damage disables the limb.
    ///
    /// Used for bones and chitin in vanilla.
    #[serde(alias = "CONNECTIVE_TISSUE_ANCHOR")]
    pub connective_tissue_anchor: Option<()>,
    /// The tissue will not heal, or heals slower, until it is set by a bone doctor.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SETTABLE")]
    pub settable: Option<()>,
    /// The broken tissue can be fixed with a cast or a splint to restore function while it heals.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SPLINTABLE")]
    pub splintable: Option<()>,
    /// The tissue performs some sort of special function (e.g. sight, hearing, breathing, etc.)
    /// An organ with such a function will stop working if a sufficient amount of damage is
    /// sustained by its `FUNCTIONAL` tissues. If an organ has no `FUNCTIONAL` tissues,
    /// it will stop working only if it is severed or destroyed entirely by heat or cold.
    #[serde(alias = "FUNCTIONAL")]
    pub functional: Option<()>,
    /// Nervous function - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "NERVOUS")]
    pub nervous: Option<()>,
    /// If a creature has no functioning parts with the `THOUGHT` token, it will be unable to move
    /// or breathe; `NO_THOUGHT_CENTER_FOR_MOVEMENT` bypasses this limitation.
    /// Mostly used in `[OBJECT:BODY]`.
    #[serde(alias = "THOUGHT")]
    pub though: Option<()>,
    /// Seems to affect where sensory or motor nerves are located,
    /// and whether damage to this tissue will render a limb useless.
    #[serde(alias = "MUSCULAR")]
    pub muscular: Option<()>,
    /// Used to smell - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SMELL")]
    pub smell: Option<()>,
    /// Used to hearing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "HEAR")]
    pub hear: Option<()>,
    /// Unknown - not used.
    /// Most likely related to flying.
    #[serde(alias = "FLIGHT")]
    pub flight: Option<()>,
    /// Used to breathing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "BREATHE")]
    pub breathe: Option<()>,
    /// Used to seeing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SIGHT")]
    pub sight: Option<()>,
    /// Holds body parts together.
    /// A body part will not be severed unless all of its component tissues with the
    /// `CONNECTS` tag are severed.
    #[serde(alias = "CONNECTS")]
    pub connects: Option<()>,
    /// Causes tissue to sometimes severely bleed when damaged.
    /// This is independent of its `VASCULAR` value.
    ///
    /// Also see: `ARTERIES`
    #[serde(alias = "MAJOR_ARTERIES")]
    pub major_arteries: Option<()>,
    /// Tissue supplies the creature with heat insulation.
    /// Higher values result in more insulation.
    #[serde(alias = "INSULATION")]
    pub insulation: Option<u32>,
    /// Unknown - not used.
    /// Maybe makes the tissue have no direct purpose?
    ///
    /// Also see: `STYLEABLE`
    #[serde(alias = "COSMETIC")]
    pub cosmetic: Option<()>,
    /// The tissue can be styled as per a tissue style (defined in an entity entry)
    ///
    /// Also see: `COSMETIC`
    #[serde(alias = "STYLEABLE")]
    pub styleable: Option<()>,
    /// The shape of the tissue, like if it is a layer or feathers.
    #[serde(alias = "TISSUE_SHAPE")]
    pub tissue_shape: Option<TissueShapeEnum>,
    /// Tissue is implicitly attached to another tissue and will fall off if that tissue
    /// layer is destroyed. Used for hair and feathers in vanilla, which are subordinate to skin.
    #[serde(alias = "SUBORDINATE_TO_TISSUE")]
    pub subordinate_to_tissue: Option<ReferenceTo<TissueToken>>,
    /// Sets/forces a default material state for the selected tissue.
    #[serde(alias = "TISSUE_MAT_STATE")]
    pub tissue_mat_state: Option<MaterialStateEnum>,
    /// The selected tissue leaks out of the creature when the layers above it are pierced.
    #[serde(alias = "TISSUE_LEAKS")]
    pub tissue_leaks: Option<()>,
    // endregion ==================================================================================
}

/// Defines a new local creature tissue and populates it with all properties defined in the
/// local tissue specified in the second argument.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UseTissue {
    /// Argument 1 of `[USE_TISSUE:...]`
    #[serde(alias = "USE_TISSUE")]
    pub reference: Option<(Reference, Reference)>,
    // region: Tissue Tokens ======================================================================
    /// Name of the tissue.
    ///
    /// Arguments are: `<name>:<plural>`
    ///
    /// Plural can alternatively be `NP` (No Plural) or `STP (Standard Plural,
    /// adds an 's' on the end).
    #[serde(alias = "TISSUE_NAME")]
    pub name: Option<(String, Choose<PluralEnum, String>)>,
    /// Defines the tissue material.
    #[serde(alias = "TISSUE_MATERIAL")]
    pub material: Option<MaterialTokenArg>,
    /// The relative thickness of the tissue.
    /// A higher thickness is harder to penetrate, but raising a tissue's relative thickness
    /// decreases the thickness of all other tissues.
    #[serde(alias = "RELATIVE_THICKNESS")]
    pub relative_thickness: Option<u32>,
    /// Speed at which the tissue heals itself; lower is faster. Common values are `100` and `1000`
    ///
    /// Omitting the token will result in a tissue that never heals.
    #[serde(alias = "HEALING_RATE")]
    pub healing_rate: Option<u32>,
    /// How many arteries/veins are in the tissue.
    /// Related to how much said tissue bleeds.
    ///
    /// Higher = More bleeding (Which is why the heart has the highest value.)
    /// - Default heart = `10`
    /// - Default skin = `1`
    ///
    /// Also see: `MAJOR_ARTERIES` and `ARTERIES`
    #[serde(alias = "VASCULAR")]
    pub vascular: Option<i32>,
    /// Related to how much pain your character will suffer when said tissue is damaged.
    /// Higher = More pain when damaged (which is why the bone tissue has a much higher value
    /// than other tissues; a broken bone hurts a lot more than a flesh cut).
    /// - Default bones = `50`
    /// - Default skin = `5`
    #[serde(alias = "PAIN_RECEPTORS")]
    pub pain_receptors: Option<i32>,
    /// The thickness of the tissue increases when character strength increases.
    /// Used for muscles in vanilla.
    #[serde(alias = "THICKENS_ON_STRENGTH")]
    pub thickness_on_strength: Option<()>,
    /// Thickness of said tissue increases when the
    /// character eats and doesn't exercise sufficiently.
    /// Used for fat in vanilla.
    #[serde(alias = "THICKENS_ON_ENERGY_STORAGE")]
    pub thickness_on_energy_storage: Option<()>,
    /// The tissue contains arteries.
    /// Edged attacks have the chance to break an artery, increasing blood loss.
    /// Used for muscles in vanilla.
    ///
    /// Also see: `MAJOR_ARTERIES` and `VASCULAR`
    #[serde(alias = "ARTERIES")]
    pub arteries: Option<()>,
    /// Denotes whether or not the tissue will be scarred once healed.
    #[serde(alias = "SCARS")]
    pub scars: Option<()>,
    /// Holds the body part together.
    /// A cut or a fracture will disable the body part it's in.
    #[serde(alias = "STRUCTURAL")]
    pub structural: Option<()>,
    /// Any ligaments or tendons are part of this tissue.
    /// Vulnerable to edged attacks, damage disables the limb.
    ///
    /// Used for bones and chitin in vanilla.
    #[serde(alias = "CONNECTIVE_TISSUE_ANCHOR")]
    pub connective_tissue_anchor: Option<()>,
    /// The tissue will not heal, or heals slower, until it is set by a bone doctor.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SETTABLE")]
    pub settable: Option<()>,
    /// The broken tissue can be fixed with a cast or a splint to restore function while it heals.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SPLINTABLE")]
    pub splintable: Option<()>,
    /// The tissue performs some sort of special function (e.g. sight, hearing, breathing, etc.)
    /// An organ with such a function will stop working if a sufficient amount of damage is
    /// sustained by its `FUNCTIONAL` tissues. If an organ has no `FUNCTIONAL` tissues,
    /// it will stop working only if it is severed or destroyed entirely by heat or cold.
    #[serde(alias = "FUNCTIONAL")]
    pub functional: Option<()>,
    /// Nervous function - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "NERVOUS")]
    pub nervous: Option<()>,
    /// If a creature has no functioning parts with the `THOUGHT` token, it will be unable to move
    /// or breathe; `NO_THOUGHT_CENTER_FOR_MOVEMENT` bypasses this limitation.
    /// Mostly used in `[OBJECT:BODY]`.
    #[serde(alias = "THOUGHT")]
    pub though: Option<()>,
    /// Seems to affect where sensory or motor nerves are located,
    /// and whether damage to this tissue will render a limb useless.
    #[serde(alias = "MUSCULAR")]
    pub muscular: Option<()>,
    /// Used to smell - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SMELL")]
    pub smell: Option<()>,
    /// Used to hearing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "HEAR")]
    pub hear: Option<()>,
    /// Unknown - not used.
    /// Most likely related to flying.
    #[serde(alias = "FLIGHT")]
    pub flight: Option<()>,
    /// Used to breathing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "BREATHE")]
    pub breathe: Option<()>,
    /// Used to seeing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SIGHT")]
    pub sight: Option<()>,
    /// Holds body parts together.
    /// A body part will not be severed unless all of its component tissues with the
    /// `CONNECTS` tag are severed.
    #[serde(alias = "CONNECTS")]
    pub connects: Option<()>,
    /// Causes tissue to sometimes severely bleed when damaged.
    /// This is independent of its `VASCULAR` value.
    ///
    /// Also see: `ARTERIES`
    #[serde(alias = "MAJOR_ARTERIES")]
    pub major_arteries: Option<()>,
    /// Tissue supplies the creature with heat insulation.
    /// Higher values result in more insulation.
    #[serde(alias = "INSULATION")]
    pub insulation: Option<u32>,
    /// Unknown - not used.
    /// Maybe makes the tissue have no direct purpose?
    ///
    /// Also see: `STYLEABLE`
    #[serde(alias = "COSMETIC")]
    pub cosmetic: Option<()>,
    /// The tissue can be styled as per a tissue style (defined in an entity entry)
    ///
    /// Also see: `COSMETIC`
    #[serde(alias = "STYLEABLE")]
    pub styleable: Option<()>,
    /// The shape of the tissue, like if it is a layer or feathers.
    #[serde(alias = "TISSUE_SHAPE")]
    pub tissue_shape: Option<TissueShapeEnum>,
    /// Tissue is implicitly attached to another tissue and will fall off if that tissue
    /// layer is destroyed. Used for hair and feathers in vanilla, which are subordinate to skin.
    #[serde(alias = "SUBORDINATE_TO_TISSUE")]
    pub subordinate_to_tissue: Option<ReferenceTo<TissueToken>>,
    /// Sets/forces a default material state for the selected tissue.
    #[serde(alias = "TISSUE_MAT_STATE")]
    pub tissue_mat_state: Option<MaterialStateEnum>,
    /// The selected tissue leaks out of the creature when the layers above it are pierced.
    #[serde(alias = "TISSUE_LEAKS")]
    pub tissue_leaks: Option<()>,
    // endregion ==================================================================================
}

/// Loads a tissue template listed in `OBJECT:TISSUE_TEMPLATE` files, such as
/// `tissue_template_default.txt`.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UseTissueTemplate {
    /// Argument 1 of `[USE_TISSUE_TEMPLATE:...]`
    #[serde(alias = "USE_TISSUE_TEMPLATE")]
    pub reference: Option<(Reference, ReferenceTo<TissueToken>)>,
    // region: Tissue Tokens ======================================================================
    /// Name of the tissue.
    ///
    /// Arguments are: `<name>:<plural>`
    ///
    /// Plural can alternatively be `NP` (No Plural) or `STP (Standard Plural,
    /// adds an 's' on the end).
    #[serde(alias = "TISSUE_NAME")]
    pub name: Option<(String, Choose<PluralEnum, String>)>,
    /// Defines the tissue material.
    #[serde(alias = "TISSUE_MATERIAL")]
    pub material: Option<MaterialTokenArg>,
    /// The relative thickness of the tissue.
    /// A higher thickness is harder to penetrate, but raising a tissue's relative thickness
    /// decreases the thickness of all other tissues.
    #[serde(alias = "RELATIVE_THICKNESS")]
    pub relative_thickness: Option<u32>,
    /// Speed at which the tissue heals itself; lower is faster. Common values are `100` and `1000`
    ///
    /// Omitting the token will result in a tissue that never heals.
    #[serde(alias = "HEALING_RATE")]
    pub healing_rate: Option<u32>,
    /// How many arteries/veins are in the tissue.
    /// Related to how much said tissue bleeds.
    ///
    /// Higher = More bleeding (Which is why the heart has the highest value.)
    /// - Default heart = `10`
    /// - Default skin = `1`
    ///
    /// Also see: `MAJOR_ARTERIES` and `ARTERIES`
    #[serde(alias = "VASCULAR")]
    pub vascular: Option<i32>,
    /// Related to how much pain your character will suffer when said tissue is damaged.
    /// Higher = More pain when damaged (which is why the bone tissue has a much higher value
    /// than other tissues; a broken bone hurts a lot more than a flesh cut).
    /// - Default bones = `50`
    /// - Default skin = `5`
    #[serde(alias = "PAIN_RECEPTORS")]
    pub pain_receptors: Option<i32>,
    /// The thickness of the tissue increases when character strength increases.
    /// Used for muscles in vanilla.
    #[serde(alias = "THICKENS_ON_STRENGTH")]
    pub thickness_on_strength: Option<()>,
    /// Thickness of said tissue increases when the
    /// character eats and doesn't exercise sufficiently.
    /// Used for fat in vanilla.
    #[serde(alias = "THICKENS_ON_ENERGY_STORAGE")]
    pub thickness_on_energy_storage: Option<()>,
    /// The tissue contains arteries.
    /// Edged attacks have the chance to break an artery, increasing blood loss.
    /// Used for muscles in vanilla.
    ///
    /// Also see: `MAJOR_ARTERIES` and `VASCULAR`
    #[serde(alias = "ARTERIES")]
    pub arteries: Option<()>,
    /// Denotes whether or not the tissue will be scarred once healed.
    #[serde(alias = "SCARS")]
    pub scars: Option<()>,
    /// Holds the body part together.
    /// A cut or a fracture will disable the body part it's in.
    #[serde(alias = "STRUCTURAL")]
    pub structural: Option<()>,
    /// Any ligaments or tendons are part of this tissue.
    /// Vulnerable to edged attacks, damage disables the limb.
    ///
    /// Used for bones and chitin in vanilla.
    #[serde(alias = "CONNECTIVE_TISSUE_ANCHOR")]
    pub connective_tissue_anchor: Option<()>,
    /// The tissue will not heal, or heals slower, until it is set by a bone doctor.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SETTABLE")]
    pub settable: Option<()>,
    /// The broken tissue can be fixed with a cast or a splint to restore function while it heals.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SPLINTABLE")]
    pub splintable: Option<()>,
    /// The tissue performs some sort of special function (e.g. sight, hearing, breathing, etc.)
    /// An organ with such a function will stop working if a sufficient amount of damage is
    /// sustained by its `FUNCTIONAL` tissues. If an organ has no `FUNCTIONAL` tissues,
    /// it will stop working only if it is severed or destroyed entirely by heat or cold.
    #[serde(alias = "FUNCTIONAL")]
    pub functional: Option<()>,
    /// Nervous function - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "NERVOUS")]
    pub nervous: Option<()>,
    /// If a creature has no functioning parts with the `THOUGHT` token, it will be unable to move
    /// or breathe; `NO_THOUGHT_CENTER_FOR_MOVEMENT` bypasses this limitation.
    /// Mostly used in `[OBJECT:BODY]`.
    #[serde(alias = "THOUGHT")]
    pub though: Option<()>,
    /// Seems to affect where sensory or motor nerves are located,
    /// and whether damage to this tissue will render a limb useless.
    #[serde(alias = "MUSCULAR")]
    pub muscular: Option<()>,
    /// Used to smell - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SMELL")]
    pub smell: Option<()>,
    /// Used to hearing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "HEAR")]
    pub hear: Option<()>,
    /// Unknown - not used.
    /// Most likely related to flying.
    #[serde(alias = "FLIGHT")]
    pub flight: Option<()>,
    /// Used to breathing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "BREATHE")]
    pub breathe: Option<()>,
    /// Used to seeing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SIGHT")]
    pub sight: Option<()>,
    /// Holds body parts together.
    /// A body part will not be severed unless all of its component tissues with the
    /// `CONNECTS` tag are severed.
    #[serde(alias = "CONNECTS")]
    pub connects: Option<()>,
    /// Causes tissue to sometimes severely bleed when damaged.
    /// This is independent of its `VASCULAR` value.
    ///
    /// Also see: `ARTERIES`
    #[serde(alias = "MAJOR_ARTERIES")]
    pub major_arteries: Option<()>,
    /// Tissue supplies the creature with heat insulation.
    /// Higher values result in more insulation.
    #[serde(alias = "INSULATION")]
    pub insulation: Option<u32>,
    /// Unknown - not used.
    /// Maybe makes the tissue have no direct purpose?
    ///
    /// Also see: `STYLEABLE`
    #[serde(alias = "COSMETIC")]
    pub cosmetic: Option<()>,
    /// The tissue can be styled as per a tissue style (defined in an entity entry)
    ///
    /// Also see: `COSMETIC`
    #[serde(alias = "STYLEABLE")]
    pub styleable: Option<()>,
    /// The shape of the tissue, like if it is a layer or feathers.
    #[serde(alias = "TISSUE_SHAPE")]
    pub tissue_shape: Option<TissueShapeEnum>,
    /// Tissue is implicitly attached to another tissue and will fall off if that tissue
    /// layer is destroyed. Used for hair and feathers in vanilla, which are subordinate to skin.
    #[serde(alias = "SUBORDINATE_TO_TISSUE")]
    pub subordinate_to_tissue: Option<ReferenceTo<TissueToken>>,
    /// Sets/forces a default material state for the selected tissue.
    #[serde(alias = "TISSUE_MAT_STATE")]
    pub tissue_mat_state: Option<MaterialStateEnum>,
    /// The selected tissue leaks out of the creature when the layers above it are pierced.
    #[serde(alias = "TISSUE_LEAKS")]
    pub tissue_leaks: Option<()>,
    // endregion ==================================================================================
}

/// Selects a tissue for editing.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SelectTissue {
    /// Argument 1 of `[SELECT_TISSUE:...]`
    #[serde(alias = "SELECT_TISSUE")]
    pub reference: Option<Reference>,
    // region: Tissue Tokens ======================================================================
    /// Name of the tissue.
    ///
    /// Arguments are: `<name>:<plural>`
    ///
    /// Plural can alternatively be `NP` (No Plural) or `STP (Standard Plural,
    /// adds an 's' on the end).
    #[serde(alias = "TISSUE_NAME")]
    pub name: Option<(String, Choose<PluralEnum, String>)>,
    /// Defines the tissue material.
    #[serde(alias = "TISSUE_MATERIAL")]
    pub material: Option<MaterialTokenArg>,
    /// The relative thickness of the tissue.
    /// A higher thickness is harder to penetrate, but raising a tissue's relative thickness
    /// decreases the thickness of all other tissues.
    #[serde(alias = "RELATIVE_THICKNESS")]
    pub relative_thickness: Option<u32>,
    /// Speed at which the tissue heals itself; lower is faster. Common values are `100` and `1000`
    ///
    /// Omitting the token will result in a tissue that never heals.
    #[serde(alias = "HEALING_RATE")]
    pub healing_rate: Option<u32>,
    /// How many arteries/veins are in the tissue.
    /// Related to how much said tissue bleeds.
    ///
    /// Higher = More bleeding (Which is why the heart has the highest value.)
    /// - Default heart = `10`
    /// - Default skin = `1`
    ///
    /// Also see: `MAJOR_ARTERIES` and `ARTERIES`
    #[serde(alias = "VASCULAR")]
    pub vascular: Option<i32>,
    /// Related to how much pain your character will suffer when said tissue is damaged.
    /// Higher = More pain when damaged (which is why the bone tissue has a much higher value
    /// than other tissues; a broken bone hurts a lot more than a flesh cut).
    /// - Default bones = `50`
    /// - Default skin = `5`
    #[serde(alias = "PAIN_RECEPTORS")]
    pub pain_receptors: Option<i32>,
    /// The thickness of the tissue increases when character strength increases.
    /// Used for muscles in vanilla.
    #[serde(alias = "THICKENS_ON_STRENGTH")]
    pub thickness_on_strength: Option<()>,
    /// Thickness of said tissue increases when the
    /// character eats and doesn't exercise sufficiently.
    /// Used for fat in vanilla.
    #[serde(alias = "THICKENS_ON_ENERGY_STORAGE")]
    pub thickness_on_energy_storage: Option<()>,
    /// The tissue contains arteries.
    /// Edged attacks have the chance to break an artery, increasing blood loss.
    /// Used for muscles in vanilla.
    ///
    /// Also see: `MAJOR_ARTERIES` and `VASCULAR`
    #[serde(alias = "ARTERIES")]
    pub arteries: Option<()>,
    /// Denotes whether or not the tissue will be scarred once healed.
    #[serde(alias = "SCARS")]
    pub scars: Option<()>,
    /// Holds the body part together.
    /// A cut or a fracture will disable the body part it's in.
    #[serde(alias = "STRUCTURAL")]
    pub structural: Option<()>,
    /// Any ligaments or tendons are part of this tissue.
    /// Vulnerable to edged attacks, damage disables the limb.
    ///
    /// Used for bones and chitin in vanilla.
    #[serde(alias = "CONNECTIVE_TISSUE_ANCHOR")]
    pub connective_tissue_anchor: Option<()>,
    /// The tissue will not heal, or heals slower, until it is set by a bone doctor.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SETTABLE")]
    pub settable: Option<()>,
    /// The broken tissue can be fixed with a cast or a splint to restore function while it heals.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SPLINTABLE")]
    pub splintable: Option<()>,
    /// The tissue performs some sort of special function (e.g. sight, hearing, breathing, etc.)
    /// An organ with such a function will stop working if a sufficient amount of damage is
    /// sustained by its `FUNCTIONAL` tissues. If an organ has no `FUNCTIONAL` tissues,
    /// it will stop working only if it is severed or destroyed entirely by heat or cold.
    #[serde(alias = "FUNCTIONAL")]
    pub functional: Option<()>,
    /// Nervous function - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "NERVOUS")]
    pub nervous: Option<()>,
    /// If a creature has no functioning parts with the `THOUGHT` token, it will be unable to move
    /// or breathe; `NO_THOUGHT_CENTER_FOR_MOVEMENT` bypasses this limitation.
    /// Mostly used in `[OBJECT:BODY]`.
    #[serde(alias = "THOUGHT")]
    pub though: Option<()>,
    /// Seems to affect where sensory or motor nerves are located,
    /// and whether damage to this tissue will render a limb useless.
    #[serde(alias = "MUSCULAR")]
    pub muscular: Option<()>,
    /// Used to smell - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SMELL")]
    pub smell: Option<()>,
    /// Used to hearing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "HEAR")]
    pub hear: Option<()>,
    /// Unknown - not used.
    /// Most likely related to flying.
    #[serde(alias = "FLIGHT")]
    pub flight: Option<()>,
    /// Used to breathing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "BREATHE")]
    pub breathe: Option<()>,
    /// Used to seeing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SIGHT")]
    pub sight: Option<()>,
    /// Holds body parts together.
    /// A body part will not be severed unless all of its component tissues with the
    /// `CONNECTS` tag are severed.
    #[serde(alias = "CONNECTS")]
    pub connects: Option<()>,
    /// Causes tissue to sometimes severely bleed when damaged.
    /// This is independent of its `VASCULAR` value.
    ///
    /// Also see: `ARTERIES`
    #[serde(alias = "MAJOR_ARTERIES")]
    pub major_arteries: Option<()>,
    /// Tissue supplies the creature with heat insulation.
    /// Higher values result in more insulation.
    #[serde(alias = "INSULATION")]
    pub insulation: Option<u32>,
    /// Unknown - not used.
    /// Maybe makes the tissue have no direct purpose?
    ///
    /// Also see: `STYLEABLE`
    #[serde(alias = "COSMETIC")]
    pub cosmetic: Option<()>,
    /// The tissue can be styled as per a tissue style (defined in an entity entry)
    ///
    /// Also see: `COSMETIC`
    #[serde(alias = "STYLEABLE")]
    pub styleable: Option<()>,
    /// The shape of the tissue, like if it is a layer or feathers.
    #[serde(alias = "TISSUE_SHAPE")]
    pub tissue_shape: Option<TissueShapeEnum>,
    /// Tissue is implicitly attached to another tissue and will fall off if that tissue
    /// layer is destroyed. Used for hair and feathers in vanilla, which are subordinate to skin.
    #[serde(alias = "SUBORDINATE_TO_TISSUE")]
    pub subordinate_to_tissue: Option<ReferenceTo<TissueToken>>,
    /// Sets/forces a default material state for the selected tissue.
    #[serde(alias = "TISSUE_MAT_STATE")]
    pub tissue_mat_state: Option<MaterialStateEnum>,
    /// The selected tissue leaks out of the creature when the layers above it are pierced.
    #[serde(alias = "TISSUE_LEAKS")]
    pub tissue_leaks: Option<()>,
    // endregion ==================================================================================
}

/// Selects a locally defined material for editing. You can select `ALL` to modify all materials
/// in the creature.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SelectMaterial {
    /// Arguments of `[SELECT_MATERIAL:...]`
    #[serde(alias = "SELECT_MATERIAL")]
    //
    pub reference: Option<Choose<AllEnum, Reference>>,
    /// Adds a material to selected materials. Used immediately after `[SELECT_MATERIAL]`.
    #[serde(alias = "PLUS_MATERIAL")] // TODO nest
    pub plus_material: Vec<Reference>,
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
    pub wafers: Option<()>,
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
    pub implies_animal_kill: Option<()>,
    /// Classifies the material as plant-based alcohol, allowing its storage in food stockpiles
    /// under "Drink (Plant)".
    #[serde(alias = "ALCOHOL_PLANT")]
    pub alcohol_plant: Option<()>,
    /// Classifies the material as animal-based alcohol, allowing its storage in food stockpiles
    /// under "Drink (Animal)".
    #[serde(alias = "ALCOHOL_CREATURE")]
    pub alcohol_creature: Option<()>,
    /// Classifies the material as generic alcohol. Implied by both `ALCOHOL_PLANT` and
    /// `ALCOHOL_CREATURE`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "ALCOHOL")]
    pub alcohol: Option<()>,
    /// Classifies the material as plant-based cheese, allowing its storage in food stockpiles
    /// under "Cheese (Plant)".
    #[serde(alias = "CHEESE_PLANT")]
    pub cheese_plant: Option<()>,
    /// Classifies the material as animal-based cheese, allowing its storage in food stockpiles
    /// under "Cheese (Animal)".
    #[serde(alias = "CHEESE_CREATURE")]
    pub cheese_creature: Option<()>,
    /// Classifies the material as generic cheese. Implied by both `CHEESE_PLANT` and
    /// `CHEESE_CREATURE`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "CHEESE")]
    pub cheese: Option<()>,
    /// Classifies the material as plant powder, allowing its storage in food stockpiles under
    /// "Milled Plant".
    #[serde(alias = "POWDER_MISC_PLANT")]
    pub powder_misc_plant: Option<()>,
    /// Classifies the material as creature powder, allowing its storage in food stockpiles under
    /// "Bone Meal".
    #[serde(alias = "POWDER_MISC_CREATURE")]
    pub powder_misc_creature: Option<()>,
    /// Classifies the material as generic powder. Implied by both `POWDER_MISC_PLANT` and
    /// `POWDER_MISC_CREATURE`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "POWDER_MISC")]
    pub powder_misc: Option<()>,
    /// Permits globs of the material in solid form to be stored in food stockpiles under "Fat" -
    /// without it, dwarves will come by and "clean" the items, destroying them (unless
    /// `[DO_NOT_CLEAN_GLOB]` is also included).
    #[serde(alias = "STOCKPILE_GLOB", alias = "STOCKPILE_GLOB_SOLID")]
    pub stockpile_glob: Option<()>,
    /// Classifies the material as milled paste, allowing its storage in food stockpiles under
    /// "Paste".
    #[serde(alias = "STOCKPILE_GLOB_PASTE")]
    pub stockpile_glob_paste: Option<()>,
    /// Classifies the material as pressed goods, allowing its storage in food stockpiles under
    /// "Pressed Material".
    #[serde(alias = "STOCKPILE_GLOB_PRESSED")]
    pub stockpile_glob_pressed: Option<()>,
    /// Classifies the material as a plant growth (e.g. fruits, leaves), allowing its storage in
    /// food stockpiles under Plant Growth/Fruit.
    #[serde(alias = "STOCKPILE_PLANT_GROWTH")]
    pub stockpile_plant_growth: Option<()>,
    /// Classifies the material as a plant extract, allowing its storage in food stockpiles under
    /// "Extract (Plant)".
    #[serde(alias = "LIQUID_MISC_PLANT")]
    pub liquid_misc_plant: Option<()>,
    /// Classifies the material as a creature extract, allowing its storage in food stockpiles under
    /// "Extract (Animal)".
    #[serde(alias = "LIQUID_MISC_CREATURE")]
    pub liquid_misc_creature: Option<()>,
    /// Classifies the material as a miscellaneous liquid, allowing its storage in food stockpiles
    /// under "Misc. Liquid" along with lye.
    #[serde(alias = "LIQUID_MISC_OTHER")]
    pub liquid_misc_other: Option<()>,
    /// Classifies the material as a generic liquid. Implied by `LIQUID_MISC_PLANT`,
    /// `LIQUID_MISC_CREATURE`, and `LIQUID_MISC_OTHER`. Exact behavior unknown, possibly vestigial.
    #[serde(alias = "LIQUID_MISC")]
    pub liquid_misc: Option<()>,
    /// Classifies the material as a plant, allowing its storage in food stockpiles under "Plants".
    #[serde(alias = "STRUCTURAL_PLANT_MAT")]
    pub structural_plant_mat: Option<()>,
    /// Classifies the material as a plant seed, allowing its storage in food stockpiles under
    /// "Seeds".
    #[serde(alias = "SEED_MAT")]
    pub seed_mat: Option<()>,
    /// Classifies the material as bone, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "BONE")]
    pub bone: Option<()>,
    /// Classifies the material as wood, allowing its use for carpenters and storage in wood
    /// stockpiles. Entities opposed to killing plants (i.e. Elves) will refuse to accept these
    /// items in trade.
    #[serde(alias = "WOOD")]
    pub wood: Option<()>,
    /// Classifies the material as plant fiber, allowing its use for clothiers and storage in cloth
    /// stockpiles under "Thread (Plant)" and "Cloth (Plant)".
    #[serde(alias = "THREAD_PLANT")]
    pub thread_plant: Option<()>,
    /// Classifies the material as tooth, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "TOOTH")]
    pub tooth: Option<()>,
    /// Classifies the material as horn, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "HORN")]
    pub horn: Option<()>,
    /// Classifies the material as pearl, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "PEARL")]
    pub pearl: Option<()>,
    /// Classifies the material as shell, allowing its use for bone carvers and restriction from
    /// stockpiles by material.
    #[serde(alias = "SHELL")]
    pub shell: Option<()>,
    /// Classifies the material as leather, allowing its use for leatherworkers and storage in
    /// leather stockpiles.
    #[serde(alias = "LEATHER")]
    pub leather: Option<()>,
    /// Classifies the material as silk, allowing its use for clothiers and storage in cloth
    /// stockpiles under "Thread (Silk)" and "Cloth (Silk)".
    #[serde(alias = "SILK")]
    pub silk: Option<()>,
    /// Classifies the material as soap, allowing it to be used as a bath detergent and stored in
    /// bar/block stockpiles under "Bars: Other Materials".
    #[serde(alias = "SOAP")]
    pub soap: Option<()>,
    /// Material generates miasma when it rots.
    #[serde(alias = "GENERATES_MIASMA")]
    pub generates_miasma: Option<()>,
    /// Classifies the material as edible meat.
    #[serde(alias = "MEAT")]
    pub meat: Option<()>,
    /// Material will rot if not stockpiled appropriately. Currently only affects food and refuse,
    /// other items made of this material will not rot.
    #[serde(alias = "ROTS")]
    pub rots: Option<()>,
    /// Tells the game to classify contaminants of this material as being "blood" in Adventurer mode
    /// tile descriptions ("Here we have a Dwarf in a slurry of blood.").
    #[serde(alias = "BLOOD_MAP_DESCRIPTOR")]
    pub blood_map_descriptor: Option<()>,
    /// Tells the game to classify contaminants of this material as being "ichor".
    #[serde(alias = "ICHOR_MAP_DESCRIPTOR")]
    pub ichor_map_descriptor: Option<()>,
    /// Tells the game to classify contaminants of this material as being "goo".
    #[serde(alias = "GOO_MAP_DESCRIPTOR")]
    pub goo_map_descriptor: Option<()>,
    /// Tells the game to classify contaminants of this material as being "slime".
    #[serde(alias = "SLIME_MAP_DESCRIPTOR")]
    pub slime_map_descriptor: Option<()>,
    /// Tells the game to classify contaminants of this material as being "pus".
    #[serde(alias = "PUS_MAP_DESCRIPTOR")]
    pub pus_map_descriptor: Option<()>,
    /// Tells the game to classify contaminants of this material as being "sweat".
    #[serde(alias = "SWEAT_MAP_DESCRIPTOR")]
    pub sweat_map_descriptor: Option<()>,
    /// Tells the game to classify contaminants of this material as being "tears".
    #[serde(alias = "TEARS_MAP_DESCRIPTOR")]
    pub tears_map_descriptor: Option<()>,
    /// Tells the game to classify contaminants of this material as being "spit".
    #[serde(alias = "SPIT_MAP_DESCRIPTOR")]
    pub spit_map_descriptor: Option<()>,
    /// Contaminants composed of this material evaporate over time, slowly disappearing from the
    /// map. Used internally by water.
    #[serde(alias = "EVAPORATES")]
    pub evaporates: Option<()>,
    /// Used for materials which cause syndromes, causes it to enter the creature's blood instead of
    /// simply spattering on the surface.
    #[serde(alias = "ENTERS_BLOOD")]
    pub enters_blood: Option<()>,
    /// Can be eaten by vermin.
    #[serde(alias = "EDIBLE_VERMIN")]
    pub edible_vermin: Option<()>,
    /// Can be eaten raw.
    #[serde(alias = "EDIBLE_RAW")]
    pub edible_raw: Option<()>,
    /// Can be cooked and then eaten.
    #[serde(alias = "EDIBLE_COOKED")]
    pub edible_cooked: Option<()>,
    /// Prevents globs made of this material from being cleaned up and destroyed.
    #[serde(alias = "DO_NOT_CLEAN_GLOB")]
    pub do_not_clean_glob: Option<()>,
    /// Prevents the material from showing up in Stone stockpile settings.
    #[serde(alias = "NO_STONE_STOCKPILE")]
    pub no_stone_stockpile: Option<()>,
    /// Allows the creation of metal furniture at the metalsmith's forge.
    #[serde(alias = "ITEMS_METAL")]
    pub items_metal: Option<()>,
    /// Equivalent to `ITEMS_HARD`. Given to bone.
    #[serde(alias = "ITEMS_BARRED")]
    pub items_barred: Option<()>,
    /// Equivalent to `ITEMS_HARD`. Given to shell.
    #[serde(alias = "ITEMS_SCALED")]
    pub items_scaled: Option<()>,
    /// Equivalent to `ITEMS_SOFT`. Given to leather.
    #[serde(alias = "ITEMS_LEATHER")]
    pub items_leather: Option<()>,
    /// Random crafts made from this material cannot be made into rings, crowns, scepters or
    /// figurines. Given to plant fiber, silk and wool.
    #[serde(alias = "ITEMS_SOFT")]
    pub items_soft: Option<()>,
    /// Random crafts made from this material include all seven items. Given to stone, wood, bone,
    /// shell, chitin, claws, teeth, horns, hooves and beeswax. Hair, pearls and eggshells also have
    /// the tag.
    #[serde(alias = "ITEMS_HARD")]
    pub items_hard: Option<()>,
    /// Used to define that the material is a stone. Allows its usage in masonry and stonecrafting
    /// and storage in stone stockpiles, among other effects.
    #[serde(alias = "IS_STONE")]
    pub is_stone: Option<()>,
    /// Used for a stone that cannot be dug into.
    #[serde(alias = "UNDIGGABLE")]
    pub undiggable: Option<()>,
    /// Causes containers made of this material to be prefixed with "unglazed" if they have not yet
    /// been glazed.
    #[serde(alias = "DISPLAY_UNGLAZED")]
    pub display_unglazed: Option<()>,
    /// Classifies the material as yarn, allowing its use for clothiers and its storage in cloth
    /// stockpiles under "Thread (Yarn)" and "Cloth (Yarn)".
    #[serde(alias = "YARN")]
    pub yarn: Option<()>,
    /// Classifies the material as metal thread, permitting thread and cloth to be stored in cloth
    /// stockpiles under "Thread (Metal)" and "Cloth (Metal)".
    #[serde(alias = "STOCKPILE_THREAD_METAL")]
    pub stockpile_thread_metal: Option<()>,
    /// Defines the material as being metal, allowing it to be used at forges.
    #[serde(alias = "IS_METAL")]
    pub is_metal: Option<()>,
    /// Used internally by green glass, clear glass, and crystal glass.
    #[serde(alias = "IS_GLASS")]
    pub is_glass: Option<()>,
    /// Can be used in the production of crystal glass.
    #[serde(alias = "CRYSTAL_GLASSABLE")]
    pub crystal_glassable: Option<()>,
    /// Melee weapons can be made out of this material.
    #[serde(alias = "ITEMS_WEAPON")]
    pub items_weapon: Option<()>,
    /// Ranged weapons can be made out of this material.
    #[serde(alias = "ITEMS_WEAPON_RANGED")]
    pub items_weapon_ranged: Option<()>,
    /// Anvils can be made out of this material.
    #[serde(alias = "ITEMS_ANVIL")]
    pub items_anvil: Option<()>,
    /// Ammunition can be made out of this material.
    #[serde(alias = "ITEMS_AMMO")]
    pub items_ammo: Option<()>,
    /// Picks can be made out of this material.
    #[serde(alias = "ITEMS_DIGGER")]
    pub items_digger: Option<()>,
    /// Armor can be made out of this material.
    #[serde(alias = "ITEMS_ARMOR")]
    pub items_armor: Option<()>,
    /// Used internally by amber and coral. Functionally equivalent to `ITEMS_HARD`.
    #[serde(alias = "ITEMS_DELICATE")]
    pub items_delicate: Option<()>,
    /// Siege engine parts can be made out of this material. Does not appear to work.
    #[serde(alias = "ITEMS_SIEGE_ENGINE")]
    pub items_siege_engine: Option<()>,
    /// Querns and millstones can be made out of this material.
    #[serde(alias = "ITEMS_QUERN")]
    pub items_quern: Option<()>,
    // endregion ==================================================================================
    // endregion ==================================================================================
}
