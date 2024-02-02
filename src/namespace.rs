use serde::Deserialize;

use crate::{
    alias::Alias, class::Class, enums::Enumeration, function::Function,
    function_macro::FunctionMacro, record::Record, union::Union, version::Version,
};

#[derive(Debug, Deserialize)]
pub struct Namespace {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: Version,
    #[serde(rename = "@identifier-prefixes")]
    identifier_prefix: String,
    #[serde(rename = "@symbol-prefixes")]
    symbol_prefix: String,
    #[serde(default, rename = "@shared-library")]
    shared_library: Option<String>,
    #[serde(default, rename = "record")]
    records: Vec<Record>,
    #[serde(default, rename = "alias")]
    aliases: Vec<Alias>,
    #[serde(default, rename = "union")]
    unions: Vec<Union>,
    #[serde(default, rename = "function")]
    functions_global: Vec<Function>,
    #[serde(default, rename = "function-macro")]
    functions_macro: Vec<FunctionMacro>,
    #[serde(default, rename = "enumeration")]
    enums: Vec<Enumeration>,
    #[serde(default, rename = "class")]
    classes: Vec<Class>,
}
