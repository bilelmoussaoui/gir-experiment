use serde::Deserialize;

use super::{function::Function, property::Property, signal::Signal};

#[derive(Debug, Deserialize)]
pub struct Prerequisite {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@symbol-prefix")]
    symbol_prefix: String,
    #[serde(rename = "@type")]
    c_type: Option<String>,
    #[serde(rename = "@type-name")]
    type_name: String,
    #[serde(rename = "@get-type")]
    get_type: String,
    #[serde(default, rename = "@type-struct")]
    type_struct: String,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "prerequisite")]
    prerequisites: Vec<Prerequisite>,
    #[serde(default, rename = "function")]
    functions: Vec<Function>,
    #[serde(default, rename = "method")]
    methods: Vec<Function>,
    #[serde(default, rename = "property")]
    properties: Vec<Property>,
    #[serde(default, rename = "signal")]
    signals: Vec<Signal>,
    #[serde(default, rename = "virtual-method")]
    virtual_methods: Vec<Function>,
}
