use serde::{Deserialize, Serialize};

// TODO: investigate if these are hardcoded to only be applicable to cerain body part CATEGORY's;
// the wiki groups these according to body part category, but categories are arbitrary and user definable!
// TODO: Fill this out; this likely matches to specific description string ingame; maybe use that here.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum BpAppModifersEnum {
    /// The height of the body part.
    #[serde(alias = "HEIGHT")]
    Height,
    /// The length of the body part.
    #[serde(alias = "LENGTH")]
    Length,
    /// The broadness of the body part.
    #[serde(alias = "BROADNESS")]
    Broadness,
    #[serde(alias = "CLOSE_SET")]
    CloseSet,
    #[serde(alias = "DEEP_SET")]
    DeepSet,
    #[serde(alias = "ROUND_VS_NARROW")]
    RoundVsNarrow,
    #[serde(alias = "LARGE_IRIS")]
    LargeIris,
    #[serde(alias = "THICKNESS")]
    Thickness,
    #[serde(alias = "UPTURNED")]
    Upturned,
    #[serde(alias = "CONVEX")]
    Convex,
    #[serde(alias = "SPLAYED_OUT")]
    SplayedOut,
    #[serde(alias = "HANGING_LOBES")]
    HangingLobes,
    #[serde(alias = "GAPS")]
    Gaps,
    #[serde(alias = "HIGH_CHEEKBONES")]
    HighCheekbones,
    #[serde(alias = "BROAD_CHIN")]
    BroadChin,
    #[serde(alias = "JUTTING_CHIN")]
    JuttingChin,
    #[serde(alias = "SQUARE_CHIN")]
    SquareChin,
    #[serde(alias = "DEEP_VOICE")]
    DeepVoice,
    #[serde(alias = "RASPY_VOICE")]
    RaspyVoice,
}
impl Default for BpAppModifersEnum {
    fn default() -> Self {
        Self::Height
    }
}
