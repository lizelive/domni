use crate::core::{AllowEmpty, ReferenceTo, Flag};

use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LanguageToken {
    #[serde(alias = "WORD")]
    WordToken(WordToken),
    #[serde(alias = "SYMBOL")]
    SymbolToken(SymbolToken),
    #[serde(alias = "TRANSLATION")]
    TranslationToken(TranslationToken),
}

impl Default for LanguageToken {
    fn default() -> Self {
        Self::WordToken(WordToken::default())
    }
}
#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TranslationToken {
    /// Argument 1 of `TRANSLATION`
    /// The reference for a Translation in other RAW files and tokens
    #[serde(alias = "TRANSLATION")]
    pub reference: Option<ReferenceTo<Self>>,
    ///
    #[serde(alias = "T_WORD")]
    pub t_word: Vec<(ReferenceTo<WordToken>, String)>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SymbolToken {
    /// Argument 1 of `SYMBOL`
    /// The reference for a Symbol in other RAW files and tokens
    #[serde(alias = "SYMBOL")]
    pub reference: Option<ReferenceTo<Self>>,
    ///
    #[serde(alias = "S_WORD")]
    pub s_word: Vec<ReferenceTo<WordToken>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct WordToken {
    /// Argument 1 of `WORD`
    /// The reference for a Word in other RAW files and tokens
    #[serde(alias = "WORD")]
    pub reference: Option<ReferenceTo<Self>>,
    ///
    #[serde(alias = "NOUN")]
    pub nouns: Vec<NounToken>,
    ///
    #[serde(alias = "ADJ")]
    pub adj: Vec<AdjToken>,
    ///
    #[serde(alias = "VERB")]
    pub verb: Vec<VerbToken>,
    ///
    #[serde(alias = "PREFIX")]
    pub prefix: Vec<PrefixToken>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NounToken {
    /// Argument 1 of `NOUN`
    ///
    #[serde(alias = "NOUN")]
    pub words: Option<(String, AllowEmpty<String>)>,
    ///
    #[serde(alias = "FRONT_COMPOUND_NOUN_SING")]
    pub front_compound_noun_sing: Flag,
    ///
    #[serde(alias = "REAR_COMPOUND_NOUN_SING")]
    pub read_compound_noun_sing: Flag,
    ///
    #[serde(alias = "THE_COMPOUND_NOUN_SING")]
    pub the_compound_noun_sing: Flag,
    ///
    #[serde(alias = "THE_NOUN_SING")]
    pub the_noun_sing: Flag,
    ///
    #[serde(alias = "OF_NOUN_SING")]
    pub of_noun_sing: Flag,
    ///
    #[serde(alias = "FRONT_COMPOUND_NOUN_PLUR")]
    pub front_compound_noun_plur: Flag,
    ///
    #[serde(alias = "REAR_COMPOUND_NOUN_PLUR")]
    pub read_compound_noun_plur: Flag,
    ///
    #[serde(alias = "THE_COMPOUND_NOUN_PLUR")]
    pub the_compound_noun_plur: Flag,
    ///
    #[serde(alias = "THE_NOUN_PLUR")]
    pub the_noun_plur: Flag,
    ///
    #[serde(alias = "OF_NOUN_PLUR")]
    pub of_noun_plur: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct VerbToken {
    /// Argument 1 of `VERB`
    #[serde(alias = "VERB")]
    pub words: Option<(String, String, String, String, String)>,
    ///
    #[serde(alias = "STANDARD_VERB")]
    pub standard_verb: Flag,
    ///
    #[serde(alias = "FRONT_COMPOUND_ADJ")]
    pub front_compound_adj: Flag,
    ///
    #[serde(alias = "THE_COMPOUND_ADJ")]
    pub the_compound_adj: Flag,
    ///
    #[serde(alias = "REAR_COMPOUND_ADJ")]
    pub rear_compound_adj: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AdjToken {
    /// Argument 1 of `ADJ`
    #[serde(alias = "ADJ")]
    pub words: Option<String>,
    ///
    #[serde(alias = "ADJ_DIST")]
    pub adj_dist: Option<i32>,
    ///
    #[serde(alias = "FRONT_COMPOUND_ADJ")]
    pub front_compound_adj: Flag,
    ///
    #[serde(alias = "THE_COMPOUND_ADJ")]
    pub the_compound_adj: Flag,
    ///
    #[serde(alias = "REAR_COMPOUND_ADJ")]
    pub rear_compound_adj: Flag,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PrefixToken {
    /// Argument 1 of `PREFIX`
    #[serde(alias = "PREFIX")]
    pub words: Option<String>,
    ///
    #[serde(alias = "FRONT_COMPOUND_PREFIX")]
    pub front_compound_prefix: Flag,
    ///
    #[serde(alias = "THE_COMPOUND_PREFIX")]
    pub the_compound_prefix: Flag,
}
