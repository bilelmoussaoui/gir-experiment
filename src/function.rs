use serde::{Deserialize, Deserializer};

use crate::{r#type::Type, transfer::TransferOwnership, version::Version};

#[derive(Debug, Deserialize)]
pub struct FunctionReturn {
    #[serde(default, rename = "@transfer-ownership")]
    transfer: TransferOwnership,
    #[serde(default, rename = "type")]
    type_: Option<Type>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FunctionScope {
    Call,
    Notified,
    Async,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@transfer-ownership")]
    transfer: Option<TransferOwnership>,
    #[serde(default, rename = "@nullable")]
    nullable: Option<bool>,
    #[serde(default, rename = "@allow-none")]
    allow_none: Option<bool>,
    #[serde(default, rename = "@scope")]
    scope: Option<FunctionScope>,
    #[serde(default, rename = "@closure")]
    closure: Option<u8>,
    #[serde(default, rename = "@destroy")]
    destroy: Option<u8>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "type")]
    type_: Option<Type>,
}

pub(super) fn unwrap_parameters<'de, D>(deserializer: D) -> Result<Vec<Parameter>, D::Error>
where
    D: Deserializer<'de>,
{
    /// Represents <list>...</list>
    #[derive(Deserialize)]
    struct Parameters {
        // default allows empty list
        #[serde(default)]
        parameter: Vec<Parameter>,
    }
    Ok(Parameters::deserialize(deserializer)?.parameter)
}

#[derive(Debug, Deserialize)]
pub struct Function {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@identifier")]
    c_identifier: String,
    #[serde(default, rename = "@version")]
    version: Option<Version>,
    #[serde(default, rename = "@deprecated-version")]
    deprecated_version: Option<Version>,
    #[serde(default, rename = "@deprecated")]
    deprecated: bool,
    #[serde(rename = "return-value")]
    return_value: FunctionReturn,
    #[serde(default, deserialize_with = "unwrap_parameters")]
    parameters: Vec<Parameter>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "doc-deprecated")]
    doc_deprecated: Option<String>,
}
