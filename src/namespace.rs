use serde::Deserialize;

use crate::{
    alias::Alias, class::Class, constant::Constant, enums::Enumeration, function::Function,
    function_macro::FunctionMacro, interface::Interface, record::Record, union::Union,
    version::Version,
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
    #[serde(default, rename = "alias")]
    aliases: Vec<Alias>,
    #[serde(default, rename = "constant")]
    constants: Vec<Constant>,
    #[serde(default, rename = "union")]
    unions: Vec<Union>,
    #[serde(default, rename = "function")]
    functions_global: Vec<Function>,
    #[serde(default, rename = "function-macro")]
    functions_macro: Vec<FunctionMacro>,
    #[serde(default, rename = "record")]
    records: Vec<Record>,
    #[serde(default, rename = "enumeration")]
    enums: Vec<Enumeration>,
    #[serde(default, rename = "class")]
    classes: Vec<Class>,
    #[serde(default, rename = "interface")]
    interfaces: Vec<Interface>,
}
