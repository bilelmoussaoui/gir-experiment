use serde::Deserialize;

use crate::function::{unwrap_parameters, Parameter};

#[derive(Debug, Deserialize)]
pub struct FunctionMacro {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@identifier")]
    c_identifier: String,
    #[serde(rename = "@introspectable")]
    introspectable: bool,
    #[serde(deserialize_with = "unwrap_parameters")]
    parameters: Vec<Parameter>,
}
