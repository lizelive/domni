use crate::core::ReferenceTo;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternToken {
    /// Argument 1 of `[COLOR_PATTERN:...]`
    #[serde(alias = "COLOR_PATTERN")]
    pub reference: Option<ReferenceTo<Self>>,
    #[serde(alias = "PATTERN")]
    pub pattern: Option<PatternEnum>,
    #[serde(alias = "CP_COLOR")]
    pub cp_color: Vec<ReferenceTo<crate::ColorToken>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum PatternEnum {
    #[serde(alias = "SPOTS")]
    Spots,
    #[serde(alias = "STRIPES")]
    Stripes,
    #[serde(alias = "MOTTLED")]
    Mottled,
    #[serde(alias = "IRIS_EYE")]
    IrisEye,
    #[serde(alias = "PUPIL_EYE")]
    PupilEye,
}

impl Default for PatternEnum {
    fn default() -> Self {
        Self::Spots
    }
}
