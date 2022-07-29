use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

/// Plural shorthand alternatives
pub enum PluralEnum {
    /// No Plural
    #[serde(alias = "NP")]
    Np,
    /// Standard Plural, adds an 's' on the end
    #[serde(alias = "STP")]
    Stp,
}
impl Default for PluralEnum {
    fn default() -> Self {
        Self::Np
    }
}
