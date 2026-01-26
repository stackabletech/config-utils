use std::collections::HashMap;

use clap::ValueEnum;
use lazy_static::lazy_static;
use properties::PropertiesEscaper;
use xml::XmlEscaper;

mod properties;
mod xml;

lazy_static! {
    // Yes, we could use `strum` for that, but we try to keep the dependencies minimal.
    pub static ref KNOWN_FILE_TYPES: HashMap<String, ReplaceTargetType> = {
        let mut types = HashMap::new();
        types.insert("properties".to_owned(), ReplaceTargetType::Properties);
        types.insert("xml".to_owned(), ReplaceTargetType::Xml);
        types
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
            ReplaceTargetType::EnvVar => line,
        }
    }
}
