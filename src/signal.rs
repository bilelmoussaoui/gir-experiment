use serde::Deserialize;

use crate::{
    function::{unwrap_parameters, FunctionReturn, Parameter},
    version::Version,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Emission {
    First,
    Last,
}

#[derive(Debug, Deserialize)]
pub struct Signal {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@when")]
    when: Emission,
    #[serde(default, rename = "@version")]
    version: Option<Version>,
    #[serde(default, rename = "doc")]
    doc: Option<String>,
    #[serde(rename = "return-value")]
    return_value: FunctionReturn,
    #[serde(default, deserialize_with = "unwrap_parameters")]
    parameters: Vec<Parameter>,
}
