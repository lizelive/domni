use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AllOrAllSolidEnum {
    /// All material states.
    #[serde(alias = "ALL")]
    All,
    /// All solid material states.
    #[serde(alias = "ALL_SOLID")]
    AllSolid,
}
impl Default for AllOrAllSolidEnum {
    fn default() -> Self {
        Self::All
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MaterialStateEnum {
    /// Material is in a solid state.
    /// Like:
    /// - Iron at room temperature
    /// - Water blow freezing (ice)
    #[serde(alias = "SOLID")]
    Solid,
    /// Material is in a liquid state.
    /// Like:
    /// - Iron at very high temperatures (molten iron)
    /// - Water at room temperature
    #[serde(alias = "LIQUID")]
    Liquid,
    /// Material is in a gas state.
    /// Like:
    /// - Oxygen at room temperature (air)
    /// - Water above its boiling point (steam)
    #[serde(alias = "GAS")]
    Gas,
    /// Material is ground to a powder.
    #[serde(alias = "POWDER")]
    Powder,
    /// Unknown
    #[serde(alias = "SOLID_POWDER")]
    SolidPowder,
    /// Unknown
    #[serde(alias = "PASTE")]
    Paste,
    /// Unknown
    #[serde(alias = "SOLID_PASTE")]
    SolidPaste,
    /// Unknown
    #[serde(alias = "PRESSED")]
    Pressed,
    /// Unknown
    #[serde(alias = "SOLID_PRESSED")]
    SolidPressed,
}
impl Default for MaterialStateEnum {
    fn default() -> Self {
        Self::Solid
    }
}
