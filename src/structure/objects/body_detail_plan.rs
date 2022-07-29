use crate::core::{Choose, Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

use crate::structure::{BpCriteriaTokenArg, MaterialToken, TissueToken};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BodyDetailPlanToken {
    /// Argument 1 of `[BODY_DETAIL_PLAN:...]`
    #[serde(alias = "BODY_DETAIL_PLAN")]
    pub reference: Option<ReferenceTo<Self>>,
    /// Adds a new material to the creature based on the specified template and assigned to the
    /// specified identifier.
    #[serde(alias = "ADD_MATERIAL")]
    pub add_material: Vec<(Reference, ReferenceTo<MaterialToken>)>,
    /// Adds a new tissue to the creature based on the specified template and assigned to the
    /// specified identifier.
    #[serde(alias = "ADD_TISSUE")]
    pub add_tissue: Vec<(Reference, ReferenceTo<TissueToken>)>,
    /// Defines a series of tissue layers composing the specified body parts. Alternatively to
    /// specifying a tissue, variable arguments can be entered (numbered arbitrarily to a max of 5)
    /// to be filled with tissues when the plan is called in the creature entry. The `SELECT_TISSUE`
    /// creature token with `TL_RELATIVE_THICKNESS` can change tissue thickness, but tissue layering
    /// is hard to do without a new detail plan.
    #[serde(alias = "BP_LAYERS")]
    pub bp_layers: Vec<(BpCriteriaTokenArg, Vec<BpLayerTokenArg>)>,
    /// Defines a series of tissue layers over the specified body parts. Alternatively to specifying
    /// a tissue, variable arguments can be entered (numbered arbitrarily to a max of 5) to be
    /// filled with tissues when the plan is called in the creature entry. The `SELECT_TISSUE`
    /// creature token with `TL_RELATIVE_THICKNESS` can change tissue thickness, but tissue layering
    /// is hard to do without a new detail plan.
    #[serde(alias = "BP_LAYERS_OVER")]
    pub bp_layers_over: Vec<(BpCriteriaTokenArg, Vec<BpLayerTokenArg>)>,
    /// Defines a series of tissue layers under the specified body parts. Alternatively to
    /// specifying a tissue, variable arguments can be entered (numbered arbitrarily to a max of 5)
    /// to be filled with tissues when the plan is called in the creature entry. The `SELECT_TISSUE`
    /// creature token with `TL_RELATIVE_THICKNESS` can change tissue thickness, but tissue layering
    /// is hard to do without a new detail plan.
    #[serde(alias = "BP_LAYERS_UNDER")]
    pub bp_layers_under: Vec<(BpCriteriaTokenArg, Vec<BpLayerTokenArg>)>,
    /// Defines a position for the specified body part (the nose is assigned the position `FRONT`,
    /// as it's on the front of the face). This has some effects on combat, attacks and the like.
    ///
    /// The position token `SIDES` is of unverified validity.
    #[serde(alias = "BP_POSITION")]
    pub bp_position: Vec<(BpCriteriaTokenArg, PositionEnum)>,
    /// Defines a positional relationship between one body part and another (for example, the right
    /// eyelid is `AROUND` the right eye with coverage 50, as it only partially covers the eye).
    /// This has some effects on combat, attacks and the like.
    #[serde(alias = "BP_RELATION")]
    pub bp_relation: Vec<(
        BpCriteriaTokenArg,
        BpRelationEnum,
        BpCriteriaTokenArg,
        Option<u8>,
    )>,
    /// Defines a relsize for the selected body part for the current body detail plan.
    #[serde(alias = "BP_RELSIZE")]
    pub bp_relsize: Vec<(BpCriteriaTokenArg, u32)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum PositionEnum {
    #[serde(alias = "FRONT")]
    Front,
    #[serde(alias = "BACK")]
    Back,
    #[serde(alias = "LEFT")]
    Left,
    #[serde(alias = "RIGHT")]
    Right,
    #[serde(alias = "TOP")]
    Top,
    #[serde(alias = "BOTTOM")]
    Bottom,
}
impl Default for PositionEnum {
    fn default() -> Self {
        Self::Front
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BpRelationEnum {
    /// Used to specify that the previously defined body part surrounds the following body part.
    #[serde(alias = "AROUND")]
    Around,
    /// Used to specify that the previously defined body part is surrounded by the following body part.
    #[serde(alias = "SURROUNDED_BY")]
    SurroundedBy,
    /// Used to specify that the previously defined body part is above the following body part.
    #[serde(alias = "ABOVE")]
    Above,
    /// Used to specify that the previously defined body part is below the following body part.
    #[serde(alias = "BELOW")]
    Below,
    /// Used to specify that the previously defined body part is in front of the following body part.
    #[serde(alias = "IN_FRONT")]
    InFront,
    /// Used to specify that the previously defined body part is behind the following body part.
    #[serde(alias = "BEHIND")]
    Behind,
    /// Used to specify a part that is cleaned by the previously defined part
    /// (eg, an eyelid cleans an eye).
    #[serde(alias = "CLEANS")]
    Cleans,
    /// Used to specify a part that cleans the previously defined part
    /// (eg, an eye is cleaned by an eyelid).
    #[serde(alias = "CLEANED_BY")]
    CleanedBy,
}
impl Default for BpRelationEnum {
    fn default() -> Self {
        Self::Around
    }
}

// region: BP_LAYERS args =========================================================================
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum ArgEnum {
    /// Placeholder for the first tissue type fed into this body detail plan.
    /// For example, dwarves send into the `VERTEBRATE_TISSUE_LAYERS` body detail plan,
    /// the tissue `SKIN` for `ARG1`.
    #[serde(alias = "ARG1")]
    Arg1,
    /// Placeholder for the second tissue type fed into this body detail plan.
    /// For example, dwarves send into the `VERTEBRATE_TISSUE_LAYERS` body detail plan,
    /// the tissue `FAT` for `ARG2`.
    #[serde(alias = "ARG2")]
    Arg2,
    /// Placeholder for the third tissue type fed into this body detail plan.
    /// For example, dwarves send into the `VERTEBRATE_TISSUE_LAYERS` body detail plan,
    /// the tissue `MUSCLE` for `ARG3`.
    #[serde(alias = "ARG3")]
    Arg3,
    /// Placeholder for the fourth tissue type fed into this body detail plan.
    /// For example, dwarves send into the `VERTEBRATE_TISSUE_LAYERS` body detail plan,
    /// the tissue `BONE` for `ARG4`.
    #[serde(alias = "ARG4")]
    Arg4,
    /// Placeholder for the fifth tissue type fed into this body detail plan.
    /// For example, dwarves send into the `VERTEBRATE_TISSUE_LAYERS` body detail plan,
    /// the tissue `CARTILAGE` for `ARG5`.
    #[serde(alias = "ARG5")]
    Arg5,
}
impl Default for ArgEnum {
    fn default() -> Self {
        Self::Arg1
    }
}

// TODO: research; can you have more than 5 of these total in one BP_LAYERS?
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BpLayerTokenArg {
    pub tissue: (
        // TODO: research; are multiple uses of the same arg allowed?
        Choose<ArgEnum, Reference>,
        // TODO: research; find out if this is actually a percentage
        u32,
    ),
    pub position_or_relation:
        Option<Choose<PositionEnum, (BpRelationEnum, BpCriteriaTokenArg, Option<u8>)>>,
}
