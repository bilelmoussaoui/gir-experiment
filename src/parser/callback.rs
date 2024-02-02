use serde::Deserialize;

use super::function::{unwrap_parameters, FunctionReturn, Parameter};

#[derive(Debug, Deserialize)]
pub struct Callback {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "return-value")]
    return_value: FunctionReturn,
    #[serde(default, deserialize_with = "unwrap_parameters")]
    parameters: Vec<Parameter>,
}
