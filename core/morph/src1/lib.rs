//! Analisi morfologica da file Hunspell (.aff + .dic)

pub mod aff_parser;
pub mod dic_parser;
pub mod verb_extractor;
pub mod noun_extractor;
pub mod adj_extractor;
pub mod sson_export;

pub use aff_parser::{AffData, SuffixRule, PrefixRule, AffixRule};
pub use dic_parser::{DicEntry, WordFlags};
pub use verb_extractor::{VerbInfo, ConjugationType, Tense, Person, Number};
pub use noun_extractor::NounInfo;
pub use adj_extractor::AdjectiveInfo;
