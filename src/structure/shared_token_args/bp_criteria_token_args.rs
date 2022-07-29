use crate::core::{Reference, ReferenceTo};

use serde::{Deserialize, Serialize};

use crate::structure::BodyToken;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BodyPartTypeEnum {
    /// Marks the body part as an opening in the body. If it is `[EMBEDDED]`, it cannot be gouged.
    #[serde(alias = "APERTURE")]
    Aperture,
    /// Body part is used to breathe. If all body parts with `[BREATHE]` are damaged or destroyed,
    /// the creature will suffocate unless it has the `[NOBREATHE]` tag. Note that bruising counts
    /// as (fast-healing) damage.
    #[serde(alias = "BREATHE")]
    Breathe,
    /// Body part is responsible for blood circulation. Exact effects not known.
    #[serde(alias = "CIRCULATION")]
    Circulation,
    /// Body part is used to connect other body parts together. Used for the neck and lower spine.
    #[serde(alias = "CONNECTOR")]
    Connector,
    /// Defines part as a digit. Body parts that are digits, or have them as direct sub-parts, can
    /// perform gouging attacks within a wrestling hold.
    #[serde(alias = "DIGIT")]
    Digit,
    /// Body part with this tag is embedded on the surface of parent body part. i.e.: eyes and mouth
    /// on head. It cannot be chopped off, can't be used to wrestle enemies and can't be grabbed by
    /// them.
    #[serde(alias = "EMBEDDED")]
    Embedded,
    /// Flags the body part as being needed for flight. Damage to a certain number of `FLIER` body
    /// parts will prevent the creature from flying.
    ///
    /// Note that a creature can only fly if the creature has the `[FLIER]` tag in its creature
    /// definition, and that a flying creature does not actually need any `FLIER` body parts.
    /// This tag's only purpose is to identify body parts which will cause a creature to lose the
    /// ability to fly when damaged.
    #[serde(alias = "FLIER")]
    Flier,
    /// Creature can wield a picked-up weapon with the body part, and can use the part to initiate
    /// almost all wrestling moves.
    ///
    /// When creatures are spawned with a weapon and shield, one `GRASP` part will hold a weapon
    /// while all others will hold shields.
    ///
    /// A grasp-able bodypart is needed for Grasp-attacks, which are in turn needed to start a fist
    /// fight. Creatures throwing a tantrum, but missing a bodypart with the grasp-property, will
    /// be cancelling their fist fight, due to being 'too injured'.
    #[serde(alias = "GRASP")]
    Grasp,
    /// Body part is susceptible to low blows. Used for guts. Damage to this body part causes nausea
    /// and may make the creature lose turns, vomiting uncontrollably.
    #[serde(alias = "GUTS")]
    Guts,
    /// Flags the body part as being able to wear head clothing like hats, helms, etc. If all heads
    /// are chopped off, the creature dies. Multiple heads are redundant - for example, hydras can
    /// survive with several missing heads.
    #[serde(alias = "HEAD")]
    Head,
    /// Body part is used to hear. May be a requirement for the body part to wear earrings.
    #[serde(alias = "HEAR")]
    Hear,
    /// Marks the body part as being inside the body. It is behind all the other tissues of the body
    /// part, cannot be severed, nor used for wrestling. It cannot be targeted directly in combat,
    /// but can be damaged by attacks to the parent body part.
    #[serde(alias = "INTERNAL")]
    Internal,
    /// Body part is a joint. If the limb it's in is grabbed in a wrestling hold, it can be broken
    /// with bending force, disabling the parent limb. If the joint is modded to sit outside the
    /// body, grabbing and breaking it snaps the entire limb right off.
    #[serde(alias = "JOINT")]
    Joint,
    /// Marks body part as on the left side of the body and vulnerable to attacks from the left.
    /// Used in conjunction with tags in the `b_detail_plan_default` raw.
    #[serde(alias = "LEFT")]
    Left,
    /// Body part is a limb. It can be used to initiate most wrestling moves.
    ///
    /// If it is located between an `[UPPERBODY]` part and a `[GRASP]` body part, it is eligible to
    /// be covered by certain types of armor (body armors and gauntlets).
    ///
    /// If it is located between a `[LOWERBODY]` part and a `[STANCE]` body part, it is eligible to
    /// be covered by other types of armor (Leg armors like pants, etc.; trailing body armors like
    /// mail shirts and robes; and high boots).
    #[serde(alias = "LIMB")]
    Limb,
    /// Flags the body part as being able to wear lower body clothing like skirts, pants, etc.
    ///
    /// If all parts with this token are chopped off or pulped, the creature dies. If the creature
    /// has multiple parts with this token, they will not die until all parts with this token have
    /// been pulped or severed. No such creature exists in the base game, however.
    #[serde(alias = "LOWERBODY")]
    Lowerbody,
    /// Body part is a mouth. Implication unknown.
    #[serde(alias = "MOUTH")]
    Mouth,
    /// Body part is the hub of nervous function. Used for the parts of the spine. Damage disables
    /// everything in the parent bodypart and what's below it, causing death by suffocation in most
    /// cases.
    #[serde(alias = "NERVOUS")]
    Nervous,
    /// Marks body part as on the right side of the body and vulnerable to attacks from the right.
    /// Used in conjunction with tags in the `b_detail_plan_default` raw.
    #[serde(alias = "RIGHT")]
    Right,
    /// Body part is used to see with. If the creature has no `SIGHT` body parts, or if all its
    /// sight body parts are damaged or destroyed, it can't see unless it has the `[EXTRAVISION]`
    /// tag in its creature definition.
    #[serde(alias = "SIGHT")]
    Sight,
    /// Body part is part of the creature's skeleton.
    #[serde(alias = "SKELETON")]
    Skeleton,
    /// "`SMALL` means that the part isn't displayed as part of the overall displayed body part lists.
    /// They can't be splinted. They are more often targeted for torture (although those situations
    /// might not occur anymore). They are removed in skeletons if they aren't specifically
    /// skeletons/joints/digits/apertures. They are more easily lost in world gen duels. They are
    /// the only gougable/pinchable parts (note: at least this is no longer the case.). `SMALL` is
    /// an old tag, so it has accumulated some weird functions which'll get split off over time. "
    ///
    /// --Toady
    #[serde(alias = "SMALL")]
    Small,
    /// Body part is used to smell. No known function. (could possibly control reactions to miasma
    /// in fortress mode?)
    #[serde(alias = "SMELL")]
    Smell,
    /// Body part breaks off and goes flying if broken, even with blunt force. Used on teeth to make
    /// them easy to knock out. Rendered invalid by `[INTERNAL]`.
    #[serde(alias = "SOCKET")]
    Socket,
    /// Allows the creature to stand. Damage or loss of these body parts will cause creature to fall
    /// over. Loss of one `STANCE` part can be substituted with a crutch. Does not give the body
    /// part an ability to initiate wrestling moves, unlike `[GRASP]` or `[LIMB]`.
    #[serde(alias = "STANCE")]
    Stance,
    /// The central core of the body. Used with the brain. Damage causes instant death unless the
    /// creature has `[NO_THOUGHT_CENTER_FOR_MOVEMENT]`/`[NOTHOUGHT]`.
    #[serde(alias = "THOUGHT")]
    Thought,
    /// Body part can be strangled. Latching bites that hit the head have a chance to target this
    /// instead. Note: this tag doesn't control any bleeding behavior.
    #[serde(alias = "THROAT")]
    Throat,
    /// This bodypart can be turned into a totem by craftsmen. Always drops from slaughtered
    /// creatures, no matter how small.
    #[serde(alias = "TOTEMABLE")]
    Totemable,
    /// Makes the body part pop out of the body when cut through. Used on guts. Body part shows up
    /// as "~" and drags behind the victim when spilled.
    #[serde(alias = "UNDER_PRESSURE")]
    UnderPressure,
    /// Flags the body part as being able to wear upper body clothing like coats, breastplates etc.
    ///
    /// If all parts with this token are pulped or chopped off, the creature dies. Multiple
    /// `UPPERBODY` parts are redundant, but no such creatures exist in the base game.
    /// All default creatures with bodies have the upper body as the root of the body tree, making
    /// it impossible to chop off.
    #[serde(alias = "UPPERBODY")]
    Upperbody,
    /// Allows the item to be obtained from butchered or rotted vermin. Used with shells.
    #[serde(alias = "VERMIN_BUTCHER_ITEM")]
    VerminButcherItem,
    /// Creatures with a body part containing this token may be gelded, which prevents them
    /// reproducing. Gelding may also occur during combat if this body part is damaged sufficiently.
    #[serde(alias = "GELDABLE")]
    Geldable,
}
impl Default for BodyPartTypeEnum {
    fn default() -> Self {
        Self::Aperture
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BpCriteriaTokenArg {
    /// Select body parts by their `CATEGORY`, as defined in a `[CATEGORY:...]` body token.
    ByCategory(Reference), // TODO: ref is a CATEGORY from inside BodyToken
    /// Select body parts by their type (for example, `GRASP`, or `BREATHE`).
    ///
    /// Most tokens applied to a given body part count as "types".
    ByType(BodyPartTypeEnum),
    /// Select specific body parts by their token ID/reference.
    ByToken(ReferenceTo<BodyToken>),
}
impl Default for BpCriteriaTokenArg {
    fn default() -> Self {
        Self::ByCategory(Reference::default())
    }
}
