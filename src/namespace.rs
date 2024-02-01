use serde::Deserialize;

use crate::{
    alias::Alias, class::Class, enums::Enumeration, function::Function,
    function_macro::FunctionMacro, record::Record, union::Union,
};

#[derive(Debug, Deserialize)]
pub struct Namespace {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: f32, // TODO: replace with a version type
    #[serde(rename = "@identifier-prefixes")]
    identifier_prefix: String,
    #[serde(rename = "@symbol-prefixes")]
    symbol_prefix: String,
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
    #[serde(default, rename = "class")]
    classes: Vec<Class>,
    #[serde(default, rename = "enumeration")]
    enums: Vec<Enumeration>,
}
