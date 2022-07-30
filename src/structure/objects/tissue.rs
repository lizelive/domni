use crate::core::{Choose, ReferenceTo, Flag};
use crate::structure::{MaterialStateEnum, MaterialTokenArg, PluralEnum};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TissueToken {
    /// Argument 1 of `[TISSUE_TEMPLATE:...]`
    #[serde(alias = "TISSUE_TEMPLATE")]
    pub reference: Option<ReferenceTo<Self>>,
    // region: Tissue Tokens ======================================================================
    /// Name of the tissue.
    ///
    /// Arguments are: `<name>:<plural>`
    ///
    /// Plural can alternatively be `NP` (No Plural) or `STP` (Standard Plural,
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
    pub thickness_on_strength: Flag,
    /// Thickness of said tissue increases when the
    /// character eats and doesn't exercise sufficiently.
    /// Used for fat in vanilla.
    #[serde(alias = "THICKENS_ON_ENERGY_STORAGE")]
    pub thickness_on_energy_storage: Flag,
    /// The tissue contains arteries.
    /// Edged attacks have the chance to break an artery, increasing blood loss.
    /// Used for muscles in vanilla.
    ///
    /// Also see: `MAJOR_ARTERIES` and `VASCULAR`
    #[serde(alias = "ARTERIES")]
    pub arteries: Flag,
    /// Denotes whether or not the tissue will be scarred once healed.
    #[serde(alias = "SCARS")]
    pub scars: Flag,
    /// Holds the body part together.
    /// A cut or a fracture will disable the body part it's in.
    #[serde(alias = "STRUCTURAL")]
    pub structural: Flag,
    /// Any ligaments or tendons are part of this tissue.
    /// Vulnerable to edged attacks, damage disables the limb.
    ///
    /// Used for bones and chitin in vanilla.
    #[serde(alias = "CONNECTIVE_TISSUE_ANCHOR")]
    pub connective_tissue_anchor: Flag,
    /// The tissue will not heal, or heals slower, until it is set by a bone doctor.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SETTABLE")]
    pub settable: Flag,
    /// The broken tissue can be fixed with a cast or a splint to restore function while it heals.
    /// Used for bones, shell and chitin in vanilla.
    #[serde(alias = "SPLINTABLE")]
    pub splintable: Flag,
    /// The tissue performs some sort of special function (e.g. sight, hearing, breathing, etc.)
    /// An organ with such a function will stop working if a sufficient amount of damage is
    /// sustained by its `FUNCTIONAL` tissues. If an organ has no `FUNCTIONAL` tissues,
    /// it will stop working only if it is severed or destroyed entirely by heat or cold.
    #[serde(alias = "FUNCTIONAL")]
    pub functional: Flag,
    /// Nervous function - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "NERVOUS")]
    pub nervous: Flag,
    /// If a creature has no functioning parts with the `THOUGHT` token, it will be unable to move
    /// or breathe; `NO_THOUGHT_CENTER_FOR_MOVEMENT` bypasses this limitation.
    /// Mostly used in `[OBJECT:BODY]`.
    #[serde(alias = "THOUGHT")]
    pub though: Flag,
    /// Seems to affect where sensory or motor nerves are located,
    /// and whether damage to this tissue will render a limb useless.
    #[serde(alias = "MUSCULAR")]
    pub muscular: Flag,
    /// Used to smell - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SMELL")]
    pub smell: Flag,
    /// Used to hearing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "HEAR")]
    pub hear: Flag,
    /// Unknown - not used.
    /// Most likely related to flying.
    #[serde(alias = "FLIGHT")]
    pub flight: Flag,
    /// Used to breathing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "BREATHE")]
    pub breathe: Flag,
    /// Used to seeing - not used.
    /// This token is used in `[OBJECT:BODY]` tokens.
    #[serde(alias = "SIGHT")]
    pub sight: Flag,
    /// Holds body parts together.
    /// A body part will not be severed unless all of its component tissues with the
    /// `CONNECTS` tag are severed.
    #[serde(alias = "CONNECTS")]
    pub connects: Flag,
    /// Causes tissue to sometimes severely bleed when damaged.
    /// This is independent of its `VASCULAR` value.
    ///
    /// Also see: `ARTERIES`
    #[serde(alias = "MAJOR_ARTERIES")]
    pub major_arteries: Flag,
    /// Tissue supplies the creature with heat insulation.
    /// Higher values result in more insulation.
    #[serde(alias = "INSULATION")]
    pub insulation: Option<u32>,
    /// Unknown - not used.
    /// Maybe makes the tissue have no direct purpose?
    ///
    /// Also see: `STYLEABLE`
    #[serde(alias = "COSMETIC")]
    pub cosmetic: Flag,
    /// The tissue can be styled as per a tissue style (defined in an entity entry)
    ///
    /// Also see: `COSMETIC`
    #[serde(alias = "STYLEABLE")]
    pub styleable: Flag,
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
    pub tissue_leaks: Flag,
    // endregion ==================================================================================
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

/// The shape of the tissue
pub enum TissueShapeEnum {
    /// Regular layer tissue.
    #[serde(alias = "LAYER")]
    Layer,
    /// Can be spun into thread at a farmer's workshop.
    /// Edge attacks will pass right through the tissue.
    #[serde(alias = "STRANDS")]
    Strands,
    /// Edge attacks will pass right through the tissue.
    #[serde(alias = "FEATHERS")]
    Feathers,
    /// Unknown effect.
    #[serde(alias = "SCALES")]
    Scales,
    /// Custom shape. Unknown effect.
    #[serde(alias = "CUSTOM")]
    Custom,
}
impl Default for TissueShapeEnum {
    fn default() -> Self {
        Self::Layer
    }
}
