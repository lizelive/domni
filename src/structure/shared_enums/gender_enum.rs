use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub enum MaleOrFemaleEnum {
    #[serde(alias = "MALE")]
    Male,
    #[serde(alias = "FEMALE")]
    Female,
}

impl Default for MaleOrFemaleEnum {
    fn default() -> Self {
        Self::Male
    }
}
