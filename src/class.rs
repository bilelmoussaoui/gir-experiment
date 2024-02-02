use serde::Deserialize;

use crate::{function::Function, property::Property};

#[derive(Debug, Deserialize)]
pub struct Class {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@symbol-prefix")]
    symbol_prefix: String,
    #[serde(rename = "@type")]
    c_type: Option<String>,
    #[serde(default, rename = "@parent")]
    parent: Option<String>,
    #[serde(rename = "@type-name")]
    type_name: String,
    #[serde(rename = "@get-type")]
    get_type: String,
    #[serde(default, rename = "@fundamental")]
    fundamental: bool,
    #[serde(default, rename = "@abstract")]
    r#abstract: bool,
    #[serde(default, rename = "@ref-func")]
    ref_func: Option<String>,
    #[serde(default, rename = "@unref-func")]
    unref_func: Option<String>,
    #[serde(default, rename = "@set-value-func")]
    set_value_func: Option<String>,
    #[serde(default, rename = "@get-value-func")]
    get_value_func: Option<String>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "function")]
    functions: Vec<Function>,
    #[serde(default, rename = "method")]
    methods: Vec<Function>,
    #[serde(default, rename = "property")]
    properties: Vec<Property>,
    #[serde(default, rename = "virtual-method")]
    virtual_methods: Vec<Function>,
}
