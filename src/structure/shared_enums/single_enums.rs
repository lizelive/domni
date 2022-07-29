use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum AllEnum {
    #[serde(alias = "ALL")]
    All,
}

impl Default for AllEnum {
    fn default() -> Self {
        Self::All
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NoneEnum {
    #[serde(alias = "NONE")]
    None,
}

impl Default for NoneEnum {
    fn default() -> Self {
        Self::None
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum StandardPluralEnum {
    #[serde(alias = "STP")]
    Stp,
}
impl Default for StandardPluralEnum {
    fn default() -> Self {
        Self::Stp
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NoMatGlossEnum {
    #[serde(alias = "NO_MATGLOSS")]
    NoMatgloss,
}

impl Default for NoMatGlossEnum {
    fn default() -> Self {
        Self::NoMatgloss
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum NotApplicableEnum {
    #[serde(alias = "NA")]
    NotApplicable,
}
impl Default for NotApplicableEnum {
    fn default() -> Self {
        Self::NotApplicable
    }
}
