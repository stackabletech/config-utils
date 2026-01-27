use std::collections::HashMap;

use clap::ValueEnum;
use lazy_static::lazy_static;
use properties::PropertiesEscaper;
use xml::XmlEscaper;

use crate::{
    file_types::env_var::EnvVarEscaper,
    template::{replacement_action_for_env_var, replacement_action_for_file, Error},
    ENV_VAR_END_PATTERN, ENV_VAR_START_PATTERNS, FILE_END_PATTERN, FILE_START_PATTERNS,
};

mod env_var;
mod properties;
mod xml;

type Replacement = (
    &'static str,
    &'static str,
    fn(&str) -> Result<String, Error>,
);

lazy_static! {
    // Yes, we could use `strum` for that, but we try to keep the dependencies minimal.
    pub static ref KNOWN_FILE_TYPES: HashMap<String, ReplaceTargetType> = {
        let mut types = HashMap::new();
        types.insert("properties".to_owned(), ReplaceTargetType::Properties);
        types.insert("xml".to_owned(), ReplaceTargetType::Xml);
        types
    };
    pub static ref REPLACEMENTS: Vec<Replacement> = {
    let mut replacements: Vec<Replacement> = Vec::new();

        for start_pattern in ENV_VAR_START_PATTERNS {
            replacements.push((
                start_pattern,
                ENV_VAR_END_PATTERN,
                replacement_action_for_env_var,
            ));
        }
        for start_pattern in FILE_START_PATTERNS {
            replacements.push((start_pattern, FILE_END_PATTERN, replacement_action_for_file));
        }
        replacements
    };
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ReplaceTargetType {
    Properties,
    Xml,
    EnvVar,
}

pub trait Escape {
    fn escape(line: String) -> String;
}

impl ReplaceTargetType {
    pub fn escape(&self, line: String) -> String {
        match self {
            ReplaceTargetType::Properties => PropertiesEscaper::escape(line),
            ReplaceTargetType::Xml => XmlEscaper::escape(line),
            ReplaceTargetType::EnvVar => EnvVarEscaper::escape(line),
        }
    }
}
