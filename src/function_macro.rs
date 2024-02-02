use serde::Deserialize;

use crate::{
    function::{unwrap_parameters, Parameter},
    version::Version,
};

#[derive(Debug, Deserialize)]
pub struct FunctionMacro {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@identifier")]
    c_identifier: String,
    #[serde(rename = "@introspectable")]
    introspectable: bool,
    #[serde(default, rename = "@version")]
    version: Option<Version>,
    #[serde(default, deserialize_with = "unwrap_parameters")]
    parameters: Vec<Parameter>,
}
