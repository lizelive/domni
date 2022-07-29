use crate::core::{Clamp, Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

use super::{BpCriteriaTokenArg, MaterialStateEnum, MaterialTokenArg, SkillEnum};

use crate::structure::InteractionToken;

/// Begin defining a new attack this creature/caste can use, including its name, and the body
/// part(s) used to perform the attack.
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Attack {
    /// Arguments of the `ATTACK` token
    #[serde(alias = "ATTACK")]
    pub reference_and_bp: Option<(Reference, AttackPerformerTokenArg)>, // TODO ref is the attack name
    /// The contact area of the attack, measured in % of the body part's volume. Note that all
    /// attack percentages can be more than 100%.
    #[serde(alias = "ATTACK_CONTACT_PERC")]
    pub attack_contact_perc: Option<u32>,
    /// Multiple strikes with this attack cannot be performed effectively.
    #[serde(alias = "ATTACK_FLAG_BAD_MULTIATTACK")]
    pub attack_flag_bad_multiattack: Option<()>,
    /// Attacks that damage tissue have the chance to latch on in a wrestling hold. The grabbing
    /// bodypart can then use the "shake around" wrestling move, causing severe, armor-bypassing
    /// tensile damage according to the attacker's body volume.
    #[serde(alias = "ATTACK_FLAG_CANLATCH")]
    pub attack_flag_canlatch: Option<()>,
    /// The attack is edged, with all the effects on physical resistance and contact area that it
    /// entails.
    #[serde(alias = "ATTACK_FLAG_EDGE")]
    pub attack_flag_edge: Option<()>,
    /// Multiple strikes with this attack can be performed with no penalty. The creature will use
    /// all attacks with this token at once.
    #[serde(alias = "ATTACK_FLAG_INDEPENDENT_MULTIATTACK")]
    pub attack_flag_independent_multiattack: Option<()>,
    /// Displays the name of the body part used to perform an attack while announcing it, e.g. "The
    /// weaver punches the bugbat with his right hand".
    #[serde(alias = "ATTACK_FLAG_WITH")]
    pub attack_flag_with: Option<()>,
    /// The penetration value of the attack, measured in % of the body part's volume. Requires
    /// `ATTACK_FLAG_EDGE`.
    #[serde(alias = "ATTACK_PENETRATION_PERC")]
    pub attack_penetration_perc: Option<Clamp<u16, 0, 15_000>>,
    /// Determines the length of time to prepare this attack and until one can perform this attack
    /// again. Values appear to be calculated in adventure mode ticks.
    #[serde(alias = "ATTACK_PREPARE_AND_RECOVER")]
    pub attack_prepare_and_recover: Option<(u32, u32)>,
    /// Usage frequency. `MAIN` attacks are 100 times more frequently chosen than `SECOND`.
    /// Opportunity attacks ignore this preference.
    #[serde(alias = "ATTACK_PRIORITY")]
    pub attack_priority: Option<AttackPriorityEnum>,
    /// Defines the skill used by the attack.
    #[serde(alias = "ATTACK_SKILL")]
    pub attack_skill: Option<SkillEnum>,
    /// The velocity multiplier of the attack, multiplied by 1000.
    #[serde(alias = "ATTACK_VELOCITY_MODIFIER")]
    pub attack_velocity_modifier: Option<u32>,
    /// Descriptive text for the attack.
    #[serde(alias = "ATTACK_VERB")]
    pub attack_verb: Option<(String, String)>,
    /// When added to an attack, causes the attack to inject the specified material into the
    /// victim's bloodstream.
    ///
    /// Once injected, the material will participate in thermal exchange within the creature - injecting
    /// something like molten iron (`INORGANIC:IRON:LIQUID`) would cause most unmodded creatures to
    /// melt (note that some of the injected material also splatters over the bodypart used to carry
    /// out the attack, so it should be protected appropriately).
    ///
    /// If the injected material has an associated syndrome with the `[SYN_INJECTED]` token, it will
    /// be transmitted to the victim. If the attack is blunt, the injected material lacks the
    /// `[ENTERS_BLOOD]` token, the attacked bodypart has no `[VASCULAR]` tissues, or the victim is
    /// bloodless, the material will splatter over the attacked body part instead.
    #[serde(alias = "SPECIALATTACK_INJECT_EXTRACT")]
    pub specialattack_inject_extract: Vec<(MaterialTokenArg, MaterialStateEnum, u32, u32)>,
    /// When this attack lands successfully, a specified interaction will take effect on the target
    /// creature. The attack must break the target creature's skin in order to work. This will take
    /// effect in worldgen as well. If the attack would break skin, the interaction will occur
    /// before the attack actually lands.
    #[serde(alias = "SPECIALATTACK_INTERACTION")]
    pub specialattack_interaction: Vec<ReferenceTo<InteractionToken>>,
    /// Successful attack draws out an amount of blood randomized between the min and max value.
    /// Beware that this will trigger any ingestion syndromes attached to the target creature's
    /// blood - for example, using this attack on a vampire will turn you into one too.
    #[serde(alias = "SPECIALATTACK_SUCK_BLOOD")]
    pub specialattack_suck_blood: Option<(u32, u32)>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AttackPriorityEnum {
    #[serde(alias = "MAIN")]
    Main,
    #[serde(alias = "SECOND")]
    Second,
}
impl Default for AttackPriorityEnum {
    fn default() -> Self {
        Self::Main
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AttackPerformerTokenArg {
    /// Specifies the type of body part used to perform the attack; for example,
    /// `BODYPART:BY_CATEGORY:HORN` would mean 1 body part categorized as a horn is used to perform
    /// this attack (presuming the creature has such a body part).
    Bodypart(BpCriteriaTokenArg),
    /// Uses a group of many body parts attached to a "parent" bodypart to perform the attack, rather than
    /// just one; for example, `CHILD_BODYPART_GROUP:BY_CATEGORY:HEAD:BY_CATEGORY:TOOTH` will use all
    /// the teeth on 1 head the creature has.
    ChildBodypartGroup((BpCriteriaTokenArg, BpCriteriaTokenArg)),
    /// Uses all specific "sub-tissues" of a specific kind on a body part; for example,
    /// `CHILD_TISSUE_LAYER_GROUP:BY_TYPE:GRASP:BY_CATEGORY:FINGER:NAIL` means this attack will use
    /// all the nails, on all the fingers, of a specific "grasp" body part (ie, a hand).
    ChildTissueLayerGroup((BpCriteriaTokenArg, BpCriteriaTokenArg, Reference)),
}
impl Default for AttackPerformerTokenArg {
    fn default() -> Self {
        Self::Bodypart(BpCriteriaTokenArg::default())
    }
}
